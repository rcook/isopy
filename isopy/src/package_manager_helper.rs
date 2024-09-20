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
use crate::cache::Cache;
use crate::constants::{CACHE_FILE_NAME, ISOPY_USER_AGENT};
use crate::serialization::{Directory, Download, File};
use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use isopy_lib::{
    DownloadFileOptions, Extent, FileNameParts, GetDirOptions, PackageManagerContext,
    PackageManagerContextOps, ProgressIndicator, ProgressIndicatorOptions,
};
use log::info;
use reqwest::header::{ACCEPT, USER_AGENT};
use reqwest::{Client, StatusCode};
use reqwest::{Response, Url as ReqwestUrl};
use std::collections::HashMap;
use std::fs::remove_file;
use std::path::{Path, PathBuf};
use tokio::fs::{create_dir_all, File as FSFile};
use tokio::io::AsyncWriteExt;
use url::Url;

trait CacheItem {
    fn url(&self) -> &Url;
    fn make_download(&self) -> Download;
    fn add_to_downloads(&self, download: &mut Download);
}

struct FileCacheItem {
    url: Url,
    path: PathBuf,
    downloaded_at: DateTime<Utc>,
}

impl FileCacheItem {
    fn make_file(&self) -> File {
        File {
            file_name: self
                .path
                .file_name()
                .expect("Must have file name")
                .to_str()
                .expect("Must be valid string")
                .to_string(),
            downloaded_at: self.downloaded_at,
        }
    }
}

impl CacheItem for FileCacheItem {
    fn url(&self) -> &Url {
        &self.url
    }

    fn make_download(&self) -> Download {
        Download {
            url: self.url.clone(),
            files: vec![self.make_file()],
            directories: vec![],
        }
    }

    fn add_to_downloads(&self, download: &mut Download) {
        download.files.push(self.make_file());
    }
}

struct DirectoryCacheItem {
    url: Url,
    path: PathBuf,
    created_at: DateTime<Utc>,
}

impl DirectoryCacheItem {
    fn make_directory(&self) -> Directory {
        Directory {
            dir_name: self
                .path
                .file_name()
                .expect("Must have directory name")
                .to_str()
                .expect("Must be valid string")
                .to_string(),
            created_at: self.created_at,
        }
    }
}

impl CacheItem for DirectoryCacheItem {
    fn url(&self) -> &Url {
        &self.url
    }

    fn make_download(&self) -> Download {
        Download {
            url: self.url.clone(),
            files: vec![],
            directories: vec![self.make_directory()],
        }
    }

    fn add_to_downloads(&self, download: &mut Download) {
        download.directories.push(self.make_directory());
    }
}

pub(crate) struct PackageManagerHelper {
    cache_dir: PathBuf,
}

impl PackageManagerHelper {
    pub(crate) fn new<P: Into<PathBuf>>(cache_dir: P) -> PackageManagerContext {
        PackageManagerContext::new(Self {
            cache_dir: cache_dir.into(),
        })
    }

    async fn download_to_path(url: &Url, path: &Path, options: &DownloadFileOptions) -> Result<()> {
        create_dir_all(
            path.parent().ok_or_else(|| {
                anyhow!("Cannot get parent directory from path {}", path.display())
            })?,
        )
        .await?;

        info!("Downloading {url}");

        let mut request = Client::new()
            .get(ReqwestUrl::parse(url.as_str())?)
            .header(USER_AGENT, ISOPY_USER_AGENT);

        if let Some(accept) = &options.accept {
            request = request.header(ACCEPT, accept.as_str());
        };

        request = request.query(&options.query);

        let response = request.send().await?;
        Self::error_for_github_rate_limit(&response)?;
        response.error_for_status_ref()?;

        let progress_indicator = ProgressIndicator::new(
            &ProgressIndicatorOptions::default()
                .enabled(options.show_progress)
                .extent(
                    response
                        .content_length()
                        .map_or(Extent::Unknown, Extent::Bytes),
                ),
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

    fn error_for_github_rate_limit(response: &Response) -> Result<()> {
        if response.status() != StatusCode::FORBIDDEN {
            return Ok(());
        }

        let headers = response.headers();

        if headers.get("x-github-request-id").is_none() {
            return Ok(());
        };

        let Some(h) = headers.get("x-ratelimit-reset") else {
            return Ok(());
        };

        let Ok(s) = h.to_str() else { return Ok(()) };

        let Ok(reset_timestamp) = s.parse::<i64>() else {
            return Ok(());
        };

        let Some(reset_date_time) = DateTime::<Utc>::from_timestamp(reset_timestamp, 0) else {
            return Ok(());
        };

        let Some(h) = headers.get("x-ratelimit-remaining") else {
            return Ok(());
        };

        let Ok(s) = h.to_str() else { return Ok(()) };

        let Ok(value) = s.parse::<i32>() else {
            return Ok(());
        };

        if value != 0 {
            return Ok(());
        }

        bail!(
            "GitHub rate limit was exceeded (limit resets at {reset_date_time}): please try again later!"
        )
    }

    fn load_cache(&self) -> Result<Cache> {
        Cache::load(self.cache_dir.join(CACHE_FILE_NAME))
    }

    fn check_cache_for_file(&self, url: &Url) -> Result<Option<PathBuf>> {
        let cache = self.load_cache()?;
        let downloads = cache
            .manifest
            .downloads
            .iter()
            .map(|d| (d.url.clone(), d))
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

    fn check_cache_for_directory(&self, url: &Url) -> Result<Option<PathBuf>> {
        let cache = self.load_cache()?;
        let downloads = cache
            .manifest
            .downloads
            .iter()
            .map(|d| (d.url.clone(), d))
            .collect::<HashMap<_, _>>();

        if let Some(download) = downloads.get(url) {
            if !download.directories.is_empty() {
                let mut directories = download.directories.iter().collect::<Vec<_>>();
                directories.sort_by_cached_key(|f| f.created_at);
                directories.reverse();
                let directory = directories.first().expect("Must be at least one file");
                let path = self.cache_dir.join(&directory.dir_name);
                if !path.is_dir() {
                    bail!("Directory {} is missing from cache", path.display());
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
            if !p.exists() {
                return Ok(p);
            }
        }
        unreachable!();
    }

    fn add_to_cache_manifest<C: CacheItem>(&self, cache_item: &C) -> Result<()> {
        let mut cache = self.load_cache()?;
        if let Some(d) = cache
            .manifest
            .downloads
            .iter_mut()
            .find(|d| d.url == *cache_item.url())
        {
            cache_item.add_to_downloads(d);
        } else {
            cache.manifest.downloads.push(cache_item.make_download());
        }
        cache.save()?;
        Ok(())
    }
}

#[async_trait]
impl PackageManagerContextOps for PackageManagerHelper {
    async fn download_file(&self, url: &Url, options: &DownloadFileOptions) -> Result<PathBuf> {
        if !options.update {
            if let Some(path) = self.check_cache_for_file(url)? {
                return Ok(path);
            }
        }

        let path = self.make_unique_path(url)?;
        let downloaded_at = Utc::now();
        Self::download_to_path(url, &path, options).await?;
        if let Some(checksum) = &options.checksum {
            if !checksum.validate_file(&path).await? {
                remove_file(&path)?;
                bail!("Checksum validation of {} failed", path.display());
            }
        }

        self.add_to_cache_manifest(&FileCacheItem {
            url: url.clone(),
            path: path.clone(),
            downloaded_at,
        })?;
        Ok(path)
    }

    async fn get_file(&self, url: &Url) -> Result<PathBuf> {
        match self.check_cache_for_file(url)? {
            Some(path) => Ok(path),
            _ => bail!("File at URL {url} not found in cache"),
        }
    }

    async fn get_dir(&self, url: &Url, options: &GetDirOptions) -> Result<PathBuf> {
        if !options.update {
            if let Some(path) = self.check_cache_for_directory(url)? {
                return Ok(path);
            }
        }

        let path = self.make_unique_path(url)?;
        let created_at = Utc::now();
        create_dir_all(&path).await?;
        self.add_to_cache_manifest(&DirectoryCacheItem {
            url: url.clone(),
            path: path.clone(),
            created_at,
        })?;
        Ok(path)
    }
}
