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
use crate::adoptium::AdoptiumIndexManager;
use crate::openjdk_descriptor::OpenJdkDescriptor;
use anyhow::anyhow;
use async_trait::async_trait;
use isopy_lib::{
    verify_sha256_file_checksum, Descriptor, DownloadAssetError, DownloadAssetResult,
    ParseDescriptorError, ParseDescriptorResult, Product,
};
use log::info;
use std::fs::remove_file;
use std::path::{Path, PathBuf};

const NAME: &str = "OpenJDK";

pub struct OpenJdk;

impl Default for OpenJdk {
    fn default() -> Self {
        Self
    }
}

impl OpenJdk {
    async fn download_openjdk(
        &self,
        descriptor: &OpenJdkDescriptor,
        shared_dir: &Path,
    ) -> DownloadAssetResult<PathBuf> {
        let manager = AdoptiumIndexManager::new_default(shared_dir);

        let versions = manager.read_versions().await?;
        let Some(version) = versions
            .iter()
            .find(|x| x.openjdk_version == descriptor.version) else {
            return Err(DownloadAssetError::VersionNotFound(descriptor.version.to_string()));
        };

        let asset_path = shared_dir.join(&version.file_name);
        if asset_path.exists() {
            info!("Asset {} already downloaded", version.file_name.display());
            return Ok(asset_path);
        }

        manager.download_asset(&version.url, &asset_path).await?;

        let is_valid = verify_sha256_file_checksum(&version.checksum, &asset_path)?;
        if !is_valid {
            remove_file(&asset_path).map_err(|e| DownloadAssetError::Other(anyhow!(e)))?;
            return Err(DownloadAssetError::ChecksumValidationFailed(
                asset_path.display().to_string(),
            ));
        }

        info!(
            "SHA256 checksum validation succeeded on {}",
            asset_path.display()
        );

        Ok(asset_path)
    }
}

#[async_trait]
impl Product for OpenJdk {
    fn name(&self) -> &str {
        NAME
    }

    fn parse_descriptor(&self, s: &str) -> ParseDescriptorResult<Box<dyn Descriptor>> {
        Ok(Box::new(
            s.parse::<OpenJdkDescriptor>()
                .map_err(|e| ParseDescriptorError::Other(anyhow!(e)))?,
        ))
    }

    async fn download_asset(
        &self,
        descriptor: &dyn Descriptor,
        shared_dir: &Path,
    ) -> DownloadAssetResult<PathBuf> {
        let descriptor = descriptor
            .as_any()
            .downcast_ref::<OpenJdkDescriptor>()
            .expect("must be OpenJdkDescriptor");
        self.download_openjdk(descriptor, shared_dir).await
    }
}
