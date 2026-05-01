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
    Checksum, DownloadAssetOptions, DownloadAssetResponse, DownloadPaginatedAssetOptions,
    DownloadPaginatedAssetResponse, PackageManagerContext, PackageManagerContextOps,
};
use log::warn;
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
            && cached_is_valid(&path, options.checksum.as_ref()).await?
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

/// Returns true if the cached file is usable. If a checksum is supplied and
/// validation fails, the stale file is removed so the caller can re-download.
async fn cached_is_valid(path: &Path, checksum: Option<&Checksum>) -> Result<bool> {
    let Some(checksum) = checksum else {
        return Ok(true);
    };
    if checksum.validate_file(path).await? {
        return Ok(true);
    }
    warn!(
        "Cached file {} failed checksum validation; re-downloading",
        path.display()
    );
    remove_file(path)?;
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::cached_is_valid;
    use std::fs::write;
    use tempfile::TempDir;

    const HELLO_SHA256: &str = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";

    #[tokio::test]
    async fn valid_cached_file_passes() -> anyhow::Result<()> {
        let tmp = TempDir::with_prefix("cached-valid-")?;
        let path = tmp.path().join("payload");
        write(&path, b"hello")?;
        let checksum = HELLO_SHA256.parse()?;
        assert!(cached_is_valid(&path, Some(&checksum)).await?);
        assert!(path.exists(), "file must be preserved on valid checksum");
        Ok(())
    }

    #[tokio::test]
    async fn corrupt_cached_file_is_removed() -> anyhow::Result<()> {
        let tmp = TempDir::with_prefix("cached-corrupt-")?;
        let path = tmp.path().join("payload");
        write(&path, b"not-hello")?;
        let checksum = HELLO_SHA256.parse()?;
        assert!(!cached_is_valid(&path, Some(&checksum)).await?);
        assert!(
            !path.exists(),
            "file must be removed when checksum fails so the caller re-downloads"
        );
        Ok(())
    }

    #[tokio::test]
    async fn no_checksum_trusts_cache() -> anyhow::Result<()> {
        let tmp = TempDir::with_prefix("cached-nosum-")?;
        let path = tmp.path().join("payload");
        write(&path, b"whatever")?;
        assert!(cached_is_valid(&path, None).await?);
        assert!(path.exists());
        Ok(())
    }
}
