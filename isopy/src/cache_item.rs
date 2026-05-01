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
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use chrono::{DateTime, Utc};
use isopy_lib::FileNameParts;
use url::Url;

use crate::cache::Cache;
use crate::constants::DOWNLOAD_CACHE_FILE_NAME;
use crate::serialization::{Directory, Download, File, PaginatedFile};

pub(crate) trait CacheItem {
    fn exists(path: &Path) -> bool;
    fn most_recent_name(download: &Download) -> Option<&str>;
    fn url(&self) -> &Url;
    fn make_download(&self) -> Download;
    fn add_to_downloads(&self, download: &mut Download);
}

pub(crate) struct FileCacheItem {
    pub(crate) url: Url,
    pub(crate) path: PathBuf,
    pub(crate) downloaded_at: DateTime<Utc>,
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

pub(crate) struct DirectoryCacheItem {
    pub(crate) url: Url,
    pub(crate) path: PathBuf,
    pub(crate) created_at: DateTime<Utc>,
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

pub(crate) struct PaginatedFileCacheItem {
    pub(crate) url: Url,
    pub(crate) path: PathBuf,
    pub(crate) downloaded_at: DateTime<Utc>,
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

pub(crate) fn check_cache<C: CacheItem>(
    base_dir: &Path,
    downloads_dir: &Path,
    url: &Url,
) -> Result<Option<PathBuf>> {
    let cache = Cache::load(base_dir.join(DOWNLOAD_CACHE_FILE_NAME))?;
    let downloads = cache
        .manifest
        .downloads
        .iter()
        .map(|d| (d.url.clone(), d))
        .collect::<HashMap<_, _>>();

    if let Some(download) = downloads.get(url)
        && let Some(name) = C::most_recent_name(download)
    {
        let path = downloads_dir.join(name);
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

pub(crate) fn make_download_path(downloads_dir: &Path, url: &Url) -> Result<PathBuf> {
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
        let p = downloads_dir.join(file_name);
        if !p.exists() {
            return Ok(p);
        }
    }
    unreachable!();
}

pub(crate) fn add_to_cache_manifest<C: CacheItem>(base_dir: &Path, cache_item: &C) -> Result<()> {
    let mut cache = Cache::load(base_dir.join(DOWNLOAD_CACHE_FILE_NAME))?;
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
