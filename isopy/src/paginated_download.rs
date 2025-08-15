// Copyright (c) 2025 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use crate::constants::ISOPY_USER_AGENT;
use anyhow::bail;
use anyhow::{Context, Result};
use futures_util::StreamExt;
use isopy_lib::pagination::PAGINATION_PART_PREFIX;
use isopy_lib::{DownloadPaginatedAssetOptions, DownloadPaginatedAssetResponse, LinkHeader};
use log::info;
use reqwest::Client;
use reqwest::Url as ReqwestUrl;
use reqwest::header::{ACCEPT, USER_AGENT};
use std::fs::read_dir;
use std::path::Path;
use tokio::fs::{File as TokioFsFile, create_dir_all as tokio_fs_create_dir_all};
use tokio::io::AsyncWriteExt;
use url::Url;

pub fn get_download_paginated_asset_response_from_dir(
    dir: &Path,
) -> Result<DownloadPaginatedAssetResponse> {
    let mut parts = Vec::new();
    for entry in
        read_dir(dir).with_context(|| format!("directory {dir} not found", dir = dir.display()))?
    {
        let entry = entry?;
        if let Some(s) = entry.file_name().to_str()
            && let Some(suffix) = s.strip_prefix(PAGINATION_PART_PREFIX)
            && let Ok(index) = suffix.parse::<usize>()
        {
            parts.push((entry.path(), index));
        }
    }

    if parts.is_empty() {
        bail!("no paginated download found at {dir}", dir = dir.display())
    }

    parts.sort_by(|(_, index_a), (_, index_b)| index_a.cmp(index_b));

    Ok(DownloadPaginatedAssetResponse {
        dir: dir.to_path_buf(),
        parts: parts.into_iter().map(|p| p.0).collect(),
    })
}

pub async fn get_download_paginated_asset_response(
    url: &Url,
    options: &DownloadPaginatedAssetOptions,
    dir: &Path,
) -> Result<DownloadPaginatedAssetResponse> {
    let mut page = 1;
    let mut url = Some(ReqwestUrl::parse(url.as_str())?);

    let mut parts = Vec::new();
    while let Some(ref u) = url {
        let output_path = dir.join(make_file_name(page));
        page += 1;
        url = download_part(u.clone(), options, &output_path).await?;
        parts.push(output_path);
    }

    Ok(DownloadPaginatedAssetResponse {
        dir: dir.to_path_buf(),
        parts,
    })
}

fn make_file_name(page: usize) -> String {
    format!("{PAGINATION_PART_PREFIX}{page:04}")
}

async fn download_part(
    url: Url,
    options: &DownloadPaginatedAssetOptions,
    output_path: &Path,
) -> Result<Option<Url>> {
    if let Some(dir) = output_path.parent() {
        tokio_fs_create_dir_all(dir).await?;
    }

    info!("downloading from {url}");

    let mut request = Client::new().get(url).header(USER_AGENT, ISOPY_USER_AGENT);

    if let Some(accept) = &options.accept {
        request = request.header(ACCEPT, accept.as_str());
    }

    let response = request.send().await?;
    if let Some(f) = &options.check {
        f(&response)?;
    }

    let next_url = LinkHeader::from_response(&response)?.and_then(|h| h.next);

    let mut stream = response.bytes_stream();
    let mut f = TokioFsFile::create_new(output_path).await?;
    let mut downloaded = 0;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        downloaded += chunk.len() as u64;
        f.write_all(&chunk).await?;
    }

    info!(
        "downloaded total {downloaded} bytes to {p}",
        p = output_path.display()
    );

    Ok(next_url)
}
