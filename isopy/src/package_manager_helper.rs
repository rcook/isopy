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
use crate::constants::{DOWNLOAD_CACHE_FILE_NAME, ISOPY_USER_AGENT};
use crate::paginated_download::{
    get_download_paginated_asset_response, get_download_paginated_asset_response_from_dir,
};
use crate::serialization::{Directory, Download, File, PaginatedFile};
use anyhow::{Result, anyhow, bail};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use isopy_lib::{
    DownloadAssetOptions, DownloadAssetResponse, DownloadPaginatedAssetOptions,
    DownloadPaginatedAssetResponse, Extent, FileNameParts, PackageManagerContext,
    PackageManagerContextOps, ProgressIndicator, ProgressIndicatorOptionsBuilder,
    error_for_github_rate_limit,
};
use log::info;
use reqwest::Client;
use reqwest::Url as ReqwestUrl;
use reqwest::header::{ACCEPT, USER_AGENT};
use std::collections::HashMap;
use std::fs::{create_dir_all, remove_file};
use std::path::{Path, PathBuf};
use tokio::fs::File as FSFile;
use tokio::io::AsyncWriteExt;
use url::Url;

trait CacheItem {
    fn exists(path: &Path) -> bool;
    fn most_recent_name(download: &Download) -> Option<&str>;
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
                .expect("must have file name")
                .to_str()
                .expect("must be valid string")
                .to_string(),
            downloaded_at: self.downloaded_at,
        }
    }
}

impl CacheItem for FileCacheItem {
    fn exists(path: &Path) -> bool {
        path.is_file()
    }

    fn most_recent_name(download: &Download) -> Option<&str> {
        let mut files = download.files.iter().collect::<Vec<_>>();
        files.sort_by_cached_key(|f| f.downloaded_at);
        files.reverse();
        files.first().map(|f| f.file_name.as_str())
    }

    fn url(&self) -> &Url {
        &self.url
    }

    fn make_download(&self) -> Download {
        Download {
            url: self.url.clone(),
            files: vec![self.make_file()],
            paginated_files: vec![],
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
                .expect("must have directory name")
                .to_str()
                .expect("must be valid string")
                .to_string(),
            created_at: self.created_at,
        }
    }
}

impl CacheItem for DirectoryCacheItem {
    fn exists(path: &Path) -> bool {
        path.is_dir()
    }

    fn most_recent_name(download: &Download) -> Option<&str> {
        let mut directories = download.directories.iter().collect::<Vec<_>>();
        directories.sort_by_cached_key(|f| f.created_at);
        directories.reverse();
        directories.first().map(|d| d.dir_name.as_str())
    }

    fn url(&self) -> &Url {
        &self.url
    }

    fn make_download(&self) -> Download {
        Download {
            url: self.url.clone(),
            files: vec![],
            paginated_files: vec![],
            directories: vec![self.make_directory()],
        }
    }

    fn add_to_downloads(&self, download: &mut Download) {
        download.directories.push(self.make_directory());
    }
}

struct PaginatedFileCacheItem {
    url: Url,
    path: PathBuf,
    downloaded_at: DateTime<Utc>,
}

impl PaginatedFileCacheItem {
    fn make_paginated_file(&self) -> PaginatedFile {
        PaginatedFile {
            dir_name: self
                .path
                .file_name()
                .expect("must have directory name")
                .to_str()
                .expect("must be valid string")
                .to_string(),
            downloaded_at: self.downloaded_at,
        }
    }
}

impl CacheItem for PaginatedFileCacheItem {
    fn exists(path: &Path) -> bool {
        path.is_dir()
    }

    fn most_recent_name(download: &Download) -> Option<&str> {
        let mut paginated_files = download.paginated_files.iter().collect::<Vec<_>>();
        paginated_files.sort_by_cached_key(|f| f.downloaded_at);
        paginated_files.reverse();
        paginated_files.first().map(|f| f.dir_name.as_str())
    }

    fn url(&self) -> &Url {
        &self.url
    }

    fn make_download(&self) -> Download {
        Download {
            url: self.url.clone(),
            files: vec![],
            paginated_files: vec![self.make_paginated_file()],
            directories: vec![],
        }
    }

    fn add_to_downloads(&self, download: &mut Download) {
        download.paginated_files.push(self.make_paginated_file());
    }
}

pub(crate) struct PackageManagerHelper {
    base_dir: PathBuf,
    downloads_dir: PathBuf,
}

impl PackageManagerHelper {
    pub(crate) fn new_context<P: Into<PathBuf>>(base_dir: P) -> PackageManagerContext {
        let base_dir = base_dir.into();
        let downloads_dir = base_dir.join("downloads");
        PackageManagerContext::new(Self {
            base_dir,
            downloads_dir,
        })
    }

    async fn download_to_path(
        url: &Url,
        path: &Path,
        options: &DownloadAssetOptions,
    ) -> Result<()> {
        create_dir_all(
            path.parent().ok_or_else(|| {
                anyhow!("Cannot get parent directory from path {}", path.display())
            })?,
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

    fn load_cache(&self) -> Result<Cache> {
        Cache::load(self.base_dir.join(DOWNLOAD_CACHE_FILE_NAME))
    }

    fn check_cache<C: CacheItem>(&self, url: &Url) -> Result<Option<PathBuf>> {
        let cache = self.load_cache()?;
        let downloads = cache
            .manifest
            .downloads
            .iter()
            .map(|d| (d.url.clone(), d))
            .collect::<HashMap<_, _>>();

        if let Some(download) = downloads.get(url)
            && let Some(name) = C::most_recent_name(download)
        {
            let path = self.downloads_dir.join(name);
            if C::exists(&path) {
                return Ok(Some(path));
            }
            bail!(
                "expected item {path} is missing from cache",
                path = path.display()
            );
        }

        Ok(None)
    }

    fn make_download_path(&self, url: &Url) -> Result<PathBuf> {
        let file_name_parts = FileNameParts::from_url_safe(url)?;
        for i in 0.. {
            let file_name = if i == 0 {
                format!(
                    "{prefix}{parts}",
                    prefix = file_name_parts.prefix,
                    parts = file_name_parts.suffix,
                )
            } else {
                format!(
                    "{prefix}{parts}-{i:05}",
                    prefix = file_name_parts.prefix,
                    parts = file_name_parts.suffix,
                )
            };
            let p = self.downloads_dir.join(file_name);
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
    fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    fn check_asset(&self, url: &Url) -> Result<Option<PathBuf>> {
        self.check_cache::<FileCacheItem>(url)
    }

    fn check_asset_dir(&self, url: &Url) -> Result<Option<PathBuf>> {
        self.check_cache::<DirectoryCacheItem>(url)
    }

    fn make_asset_dir(&self, url: &Url, create_new: bool) -> Result<PathBuf> {
        if !create_new && let Some(dir) = self.check_cache::<DirectoryCacheItem>(url)? {
            return Ok(dir);
        }

        let path = self.make_download_path(url)?;
        let created_at = Utc::now();
        create_dir_all(&path)?;

        self.add_to_cache_manifest(&DirectoryCacheItem {
            url: url.clone(),
            path: path.clone(),
            created_at,
        })?;

        Ok(path)
    }

    async fn download_asset(
        &self,
        url: &Url,
        options: &DownloadAssetOptions,
    ) -> Result<DownloadAssetResponse> {
        if !options.update
            && let Some(path) = self.check_cache::<FileCacheItem>(url)?
        {
            return Ok(DownloadAssetResponse { path });
        }

        let path = self.make_download_path(url)?;
        let downloaded_at = Utc::now();
        Self::download_to_path(url, &path, options).await?;
        if let Some(checksum) = &options.checksum
            && !checksum.validate_file(&path).await?
        {
            remove_file(&path)?;
            bail!(
                "checksum validation of {path} failed",
                path = path.display()
            );
        }

        self.add_to_cache_manifest(&FileCacheItem {
            url: url.clone(),
            path: path.clone(),
            downloaded_at,
        })?;
        Ok(DownloadAssetResponse { path })
    }

    async fn download_paginated_asset(
        &self,
        url: &Url,
        options: &DownloadPaginatedAssetOptions,
    ) -> Result<DownloadPaginatedAssetResponse> {
        if !options.update
            && let Some(dir) = self.check_cache::<PaginatedFileCacheItem>(url)?
        {
            return get_download_paginated_asset_response_from_dir(&dir);
        }

        let path = self.make_download_path(url)?;
        create_dir_all(&path)?;

        let response = get_download_paginated_asset_response(url, options, &path).await?;
        let downloaded_at = Utc::now();
        self.add_to_cache_manifest(&PaginatedFileCacheItem {
            url: url.clone(),
            path,
            downloaded_at,
        })?;

        Ok(response)
    }
}
