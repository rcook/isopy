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
use super::api::{ImageType, Release, Singleton};
use super::client::{all_versions, AdoptiumClient, Query};
use crate::constants::{ADOPTIUM_INDEX_FILE_NAME, ADOPTIUM_SERVER_URL, TWELVE_HOURS};
use crate::serialization::{Index, Version};
use anyhow::{bail, Result};
use chrono::Utc;
use joatmon::{read_yaml_file, safe_write_file};
use reqwest::Url;
use std::path::{Path, PathBuf};

pub struct AdoptiumIndexManager {
    offline: bool,
    image_type: ImageType,
    client: AdoptiumClient,
    index_path: PathBuf,
}

impl AdoptiumIndexManager {
    #[must_use]
    pub fn new_default(offline: bool, image_type: ImageType, shared_dir: &Path) -> Self {
        Self::new(
            offline,
            image_type,
            &ADOPTIUM_SERVER_URL,
            &shared_dir.join(&*ADOPTIUM_INDEX_FILE_NAME),
        )
    }

    /// Creates index manager.
    ///
    /// # Panics
    ///
    /// Panics if `index_path` is not absolute.
    #[must_use]
    pub fn new(offline: bool, image_type: ImageType, server_url: &Url, index_path: &Path) -> Self {
        assert!(index_path.is_absolute());
        Self {
            offline,
            image_type,
            client: AdoptiumClient::new(server_url),
            index_path: index_path.to_path_buf(),
        }
    }

    pub async fn read_versions(&self) -> Result<Vec<Version>> {
        let index = self.read_index().await?;
        let mut versions = index.versions;
        versions.sort_by(|a, b| b.openjdk_version.cmp(&a.openjdk_version));
        Ok(versions)
    }

    pub async fn download_asset(&self, url: &Url, output_path: &Path) -> Result<()> {
        if self.offline {
            bail!("cannot download assets in offline mode");
        }

        self.client.download_asset(url, output_path).await?;
        Ok(())
    }

    async fn read_index(&self) -> Result<Index> {
        if !self.index_path.exists() {
            return self.refresh_index(false).await;
        }

        let index = read_yaml_file::<Index>(&self.index_path)?;
        if Utc::now() - index.last_updated_at < *TWELVE_HOURS {
            return Ok(index);
        }

        self.refresh_index(true).await
    }

    async fn refresh_index(&self, overwrite: bool) -> Result<Index> {
        let index = self.download_index().await?;
        safe_write_file(&self.index_path, serde_yaml::to_string(&index)?, overwrite)?;
        Ok(index)
    }

    async fn download_index(&self) -> Result<Index> {
        fn make_version(release: &Release) -> Option<Version> {
            let binary = release.binaries.single()?;

            let package = binary.package.as_ref()?;

            let Ok(url) = package.link.parse::<Url>() else {
                return None;
            };

            let size = package.size?;

            let checksum = package.checksum.as_ref()?;

            Some(Version {
                openjdk_version: release.version_data.openjdk_version.clone(),
                file_name: package.name.clone(),
                url,
                size,
                checksum: String::from(checksum),
            })
        }

        if self.offline {
            bail!("cannot download index in offline mode");
        }

        let last_updated_at = Utc::now();

        let versions = self
            .client
            .get_releases(all_versions(), &Query::new(self.image_type.clone()))
            .await?
            .iter()
            .filter_map(make_version)
            .collect::<Vec<_>>();

        Ok(Index {
            last_updated_at,
            versions,
        })
    }
}
