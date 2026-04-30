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
use crate::cache_item::{
    DirectoryCacheItem, FileCacheItem, PaginatedFileCacheItem, add_to_cache_manifest, check_cache,
    make_download_path,
};
use crate::download::download_to_path;
use crate::paginated_download::{
    get_download_paginated_asset_response, get_download_paginated_asset_response_from_dir,
};
use anyhow::{Result, bail};
use async_trait::async_trait;
use chrono::Utc;
use isopy_lib::{
    DownloadAssetOptions, DownloadAssetResponse, DownloadPaginatedAssetOptions,
    DownloadPaginatedAssetResponse, PackageManagerContext, PackageManagerContextOps,
};
use std::fs::{create_dir_all, remove_file};
use std::path::{Path, PathBuf};
use url::Url;

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
}

#[async_trait]
impl PackageManagerContextOps for PackageManagerHelper {
    fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    fn check_asset(&self, url: &Url) -> Result<Option<PathBuf>> {
        check_cache::<FileCacheItem>(&self.base_dir, &self.downloads_dir, url)
    }

    fn check_asset_dir(&self, url: &Url) -> Result<Option<PathBuf>> {
        check_cache::<DirectoryCacheItem>(&self.base_dir, &self.downloads_dir, url)
    }

    fn make_asset_dir(&self, url: &Url, create_new: bool) -> Result<PathBuf> {
        if !create_new
            && let Some(dir) =
                check_cache::<DirectoryCacheItem>(&self.base_dir, &self.downloads_dir, url)?
        {
            return Ok(dir);
        }

        let path = make_download_path(&self.downloads_dir, url)?;
        let created_at = Utc::now();
        create_dir_all(&path)?;

        add_to_cache_manifest(
            &self.base_dir,
            &DirectoryCacheItem {
                url: url.clone(),
                path: path.clone(),
                created_at,
            },
        )?;

        Ok(path)
    }

    async fn download_asset(
        &self,
        url: &Url,
        options: &DownloadAssetOptions,
    ) -> Result<DownloadAssetResponse> {
        if !options.update
            && let Some(path) =
                check_cache::<FileCacheItem>(&self.base_dir, &self.downloads_dir, url)?
        {
            return Ok(DownloadAssetResponse { path });
        }

        let path = make_download_path(&self.downloads_dir, url)?;
        let downloaded_at = Utc::now();
        download_to_path(url, &path, options).await?;
        if let Some(checksum) = &options.checksum
            && !checksum.validate_file(&path).await?
        {
            remove_file(&path)?;
            bail!(
                "checksum validation of {path} failed",
                path = path.display()
            );
        }

        add_to_cache_manifest(
            &self.base_dir,
            &FileCacheItem {
                url: url.clone(),
                path: path.clone(),
                downloaded_at,
            },
        )?;
        Ok(DownloadAssetResponse { path })
    }

    async fn download_paginated_asset(
        &self,
        url: &Url,
        options: &DownloadPaginatedAssetOptions,
    ) -> Result<DownloadPaginatedAssetResponse> {
        if !options.update
            && let Some(dir) =
                check_cache::<PaginatedFileCacheItem>(&self.base_dir, &self.downloads_dir, url)?
        {
            return get_download_paginated_asset_response_from_dir(&dir);
        }

        let path = make_download_path(&self.downloads_dir, url)?;
        create_dir_all(&path)?;

        let response = get_download_paginated_asset_response(url, options, &path).await?;
        let downloaded_at = Utc::now();
        add_to_cache_manifest(
            &self.base_dir,
            &PaginatedFileCacheItem {
                url: url.clone(),
                path,
                downloaded_at,
            },
        )?;

        Ok(response)
    }
}
