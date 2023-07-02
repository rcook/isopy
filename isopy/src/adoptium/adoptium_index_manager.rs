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
use super::adoptium_client::{all_versions, AdoptiumClient};
use super::query::Query;
use crate::api::adoptium::{Release, Singleton};
use crate::serialization::adoptium::{IndexRec, VersionRec};
use anyhow::Result;
use chrono::{Duration, Utc};
use joatmon::{read_yaml_file, safe_write_file};
use reqwest::Url;
use std::path::{Path, PathBuf};

pub struct AdoptiumIndexManager {
    client: AdoptiumClient,
    index_path: PathBuf,
}

impl AdoptiumIndexManager {
    pub fn new(server_url: &Url, index_path: &Path) -> Self {
        assert!(index_path.is_absolute());
        Self {
            client: AdoptiumClient::new(server_url),
            index_path: index_path.to_path_buf(),
        }
    }

    pub async fn read_versions(&self) -> Result<Vec<VersionRec>> {
        let index = self.read_index().await?;
        let mut versions = index.versions;
        versions.sort_by(|a, b| b.openjdk_version.cmp(&a.openjdk_version));
        Ok(versions)
    }

    pub async fn download_asset(&self, url: &Url, output_path: &Path) -> Result<()> {
        self.client.download_asset(url, output_path).await?;
        Ok(())
    }

    async fn read_index(&self) -> Result<IndexRec> {
        if !self.index_path.exists() {
            return self.refresh_index(false).await;
        }

        let index = read_yaml_file::<IndexRec>(&self.index_path)?;
        if Utc::now() - index.last_updated_at < Duration::hours(12) {
            return Ok(index);
        }

        self.refresh_index(true).await
    }

    async fn refresh_index(&self, overwrite: bool) -> Result<IndexRec> {
        let index = self.download_index().await?;
        safe_write_file(&self.index_path, serde_yaml::to_string(&index)?, overwrite)?;
        Ok(index)
    }

    async fn download_index(&self) -> Result<IndexRec> {
        fn make_version(release: &Release) -> Option<VersionRec> {
            let Some(binary) = release.binaries.single() else {
                return None
            };

            let Some(package) = binary.package.as_ref() else {
                return None
            };

            let Ok(url) = package.link.parse::<Url>() else {
                return None
            };

            let Some(size) = package.size else {
                return None
            };

            let Some(checksum) = package.checksum.as_ref() else {
                return None
            };

            Some(VersionRec {
                openjdk_version: release.version_data.openjdk_version.clone(),
                file_name: PathBuf::from(&package.name),
                url,
                size,
                checksum: String::from(checksum),
            })
        }

        let last_updated_at = Utc::now();

        let versions = self
            .client
            .get_releases(all_versions(), &Query::default())
            .await?
            .iter()
            .filter_map(make_version)
            .collect::<Vec<_>>();

        Ok(IndexRec {
            last_updated_at,
            versions,
        })
    }
}
