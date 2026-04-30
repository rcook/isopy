// Copyright (c) 2023 Richard Cook
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
use anyhow::{Result, anyhow};
use futures_util::StreamExt;
use isopy_lib::{
    DownloadAssetOptions, Extent, ProgressIndicator, ProgressIndicatorOptionsBuilder,
    error_for_github_rate_limit,
};
use log::info;
use reqwest::Client;
use reqwest::Url as ReqwestUrl;
use reqwest::header::{ACCEPT, USER_AGENT};
use std::fs::create_dir_all;
use std::path::Path;
use tokio::fs::File as FSFile;
use tokio::io::AsyncWriteExt;
use url::Url;

pub(crate) async fn download_to_path(
    url: &Url,
    path: &Path,
    options: &DownloadAssetOptions,
) -> Result<()> {
    create_dir_all(
        path.parent()
            .ok_or_else(|| anyhow!("Cannot get parent directory from path {}", path.display()))?,
    )?;

    info!("Downloading {url}");

    let mut request = Client::new()
        .get(ReqwestUrl::parse(url.as_str())?)
        .header(USER_AGENT, ISOPY_USER_AGENT);

    if let Some(accept) = &options.accept {
        request = request.header(ACCEPT, accept.as_str());
    }

    request = request.query(&options.query);

    let response = request.send().await?;
    error_for_github_rate_limit(&response)?;
    response.error_for_status_ref()?;

    let progress_indicator = ProgressIndicator::new(
        &ProgressIndicatorOptionsBuilder::default()
            .enabled(options.show_progress)
            .extent(
                response
                    .content_length()
                    .map_or(Extent::Unknown, Extent::Bytes),
            )
            .build()?,
    )?;

    let mut stream = response.bytes_stream();
    let mut f = FSFile::create_new(path).await?;
    let mut downloaded = 0;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        downloaded += chunk.len() as u64;
        f.write_all(&chunk).await?;
        progress_indicator.set_progress(downloaded);
    }

    progress_indicator.finish_and_clear();

    info!("Downloaded {url}");

    Ok(())
}
