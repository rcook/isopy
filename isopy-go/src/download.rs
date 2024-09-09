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
use crate::api::{File, Version};
use crate::constants::DOWNLOADS_URL;
use crate::error::other_error;
use crate::filter::Filter;
use crate::result::IsopyGoResult;
use crate::serialization::{Index, Version as Version_serialization};
use chrono::Utc;
use isopy_lib::dir_url;
use joatmon::safe_write_file;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Client, Response, Url};
use std::path::Path;

pub async fn download_index(filter: &Filter, index_path: &Path) -> IsopyGoResult<()> {
    fn transform(
        server_url: &Url,
        filter: &Filter,
        version: &Version,
        file: &File,
    ) -> IsopyGoResult<Option<Version_serialization>> {
        let Some(os) = file.os.as_ref() else {
            return Ok(None);
        };

        if *os != filter.os {
            return Ok(None);
        }

        let Some(arch) = file.arch.as_ref() else {
            return Ok(None);
        };

        if *arch != filter.arch {
            return Ok(None);
        }

        if file.kind != filter.kind {
            return Ok(None);
        }

        let Some(checksum) = file.sha256.as_ref() else {
            return Ok(None);
        };

        let url = server_url.join(&file.file_name).map_err(other_error)?;
        Ok(Some(Version_serialization {
            version: version.version.clone(),
            stable: version.stable,
            file_name: file.file_name.clone(),
            url,
            size: file.size,
            checksum: checksum.clone(),
        }))
    }

    if index_path.exists() {
        return Ok(());
    }

    // URL must have trailing slash
    let server_url = dir_url(&DOWNLOADS_URL);

    let client = Client::new();
    let versions = client
        .get(server_url.clone())
        .query(&[("include", "all"), ("mode", "json")])
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .and_then(Response::error_for_status)
        .map_err(other_error)?
        .json::<Vec<Version>>()
        .await
        .map_err(other_error)?;

    let mut version_recs = Vec::new();

    for version in versions {
        for file in &version.files {
            if let Some(version_rec) = transform(&server_url, filter, &version, file)? {
                version_recs.push(version_rec);
            }
        }
    }

    let last_updated_at = Utc::now();
    let index_rec = Index {
        last_updated_at,
        versions: version_recs,
    };

    safe_write_file(
        index_path,
        serde_yaml::to_string(&index_rec).map_err(other_error)?,
        false,
    )
    .map_err(other_error)?;

    Ok(())
}
