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
use crate::tng::cache_info::CacheInfo;
use crate::tng::consts::{CACHE_FILE_NAME, ISOPY_USER_AGENT};
use crate::tng::download::Download;
use crate::tng::file::File;
use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use isopy_lib::tng::{Context, DownloadOptions, FileNameParts};
use reqwest::header::{ACCEPT, USER_AGENT};
use reqwest::Client;
use reqwest::Url as ReqwestUrl;
use std::collections::HashMap;
use std::fs::{create_dir_all, remove_file, write};
use std::path::{Path, PathBuf};
use url::Url;

pub(crate) struct AppContext {
    cache_dir: PathBuf,
}

impl AppContext {
    pub(crate) fn new<P>(cache_dir: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            cache_dir: cache_dir.into(),
        }
    }

    async fn download_to_path(url: &Url, path: &Path, options: &DownloadOptions) -> Result<()> {
        create_dir_all(
            path.parent().ok_or_else(|| {
                anyhow!("Cannot get parent directory from path {}", path.display())
            })?,
        )?;

        println!("Downloading {url}");

        let mut request = Client::new()
            .get(ReqwestUrl::parse(url.as_str())?)
            .header(USER_AGENT, ISOPY_USER_AGENT);

        if let Some(accept) = &options.accept {
            request = request.header(ACCEPT, accept.as_str())
        };

        let response = request.send().await?;
        response.error_for_status_ref()?;

        let data = response.bytes().await?;
        write(&path, data)?;

        println!("Downloaded {url}");

        Ok(())
    }

    fn load_cache(&self) -> Result<CacheInfo> {
        CacheInfo::load(self.cache_dir.join(CACHE_FILE_NAME))
    }

    fn check_cache(&self, url: &Url) -> Result<Option<PathBuf>> {
        let cache = self.load_cache()?;
        let downloads = cache
            .manifest
            .downloads
            .iter()
            .map(|f| (f.url.clone(), f))
            .collect::<HashMap<_, _>>();

        if let Some(download) = downloads.get(url) {
            if !download.files.is_empty() {
                let mut files = download.files.iter().collect::<Vec<_>>();
                files.sort_by_cached_key(|f| f.downloaded_at);
                files.reverse();

                let file = files.first().expect("Must be at least one file");
                let path = self.cache_dir.join(&file.file_name);
                if !path.is_file() {
                    bail!("File {} is missing from cache", path.display());
                }
                return Ok(Some(path));
            }
        }

        Ok(None)
    }

    fn make_unique_path(&self, url: &Url) -> Result<PathBuf> {
        let file_name_parts = FileNameParts::from_url_safe(url)?;
        for i in 0.. {
            let file_name = if i == 0 {
                format!("{}{}", file_name_parts.prefix, file_name_parts.suffix)
            } else {
                format!(
                    "{}-{i:05}{}",
                    file_name_parts.prefix, file_name_parts.suffix
                )
            };
            let p = self.cache_dir.join(file_name);
            if !p.is_file() {
                return Ok(p);
            }
        }
        unreachable!();
    }

    fn update_cache(&self, url: &Url, path: &Path, downloaded_at: &DateTime<Utc>) -> Result<()> {
        let file = File {
            file_name: path
                .file_name()
                .expect("Must have file name")
                .to_str()
                .expect("Must be valid string")
                .to_string(),
            downloaded_at: downloaded_at.clone(),
        };

        let mut cache = self.load_cache()?;
        if let Some(d) = cache.manifest.downloads.iter_mut().find(|d| d.url == *url) {
            d.files.push(file);
        } else {
            cache.manifest.downloads.push(Download {
                url: url.clone(),
                files: vec![file],
            });
        }
        cache.save()?;
        Ok(())
    }
}

#[async_trait]
impl Context for AppContext {
    async fn download_file(&self, url: &Url, options: &DownloadOptions) -> Result<PathBuf> {
        if !options.update {
            if let Some(path) = self.check_cache(url)? {
                return Ok(path);
            }
        }

        let path = self.make_unique_path(url)?;
        let downloaded_at = Utc::now();

        Self::download_to_path(url, &path, &options).await?;
        if let Some(checksum) = &options.checksum {
            if !checksum.validate_file(&path).await? {
                remove_file(&path)?;
                bail!("Checksum validation of {} failed", path.display());
            }
        }

        self.update_cache(url, &path, &downloaded_at)?;

        Ok(path)
    }

    async fn get_file(&self, url: &Url) -> Result<PathBuf> {
        match self.check_cache(url)? {
            Some(path) => Ok(path),
            _ => bail!("File at URL {url} not found in cache"),
        }
    }
}