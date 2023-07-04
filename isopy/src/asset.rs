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
use crate::checksum::validate_sha256_checksum;
use crate::python::{Asset, AssetFilter};
use crate::repository_info::RepositoryInfo;
use anyhow::{bail, Result};
use isopy_lib::download_stream;
use isopy_python::{PythonDescriptor, Tag};
use log::info;
use std::fs::remove_file;
use std::path::{Path, PathBuf};

pub fn get_asset<'a>(assets: &'a [Asset], descriptor: &PythonDescriptor) -> Result<&'a Asset> {
    let mut asset_filter = AssetFilter::default_for_platform();
    asset_filter.version = Some(descriptor.version.clone());
    asset_filter.tag = descriptor.tag.clone();
    let matching_assets = asset_filter.filter(assets.iter());

    if matching_assets.len() > 1 {
        bail!(
            "More than one asset matching version {} and tag {}",
            descriptor.version,
            descriptor
                .tag
                .as_ref()
                .map_or_else(|| String::from("(none)"), Tag::to_string)
        )
    }

    if let Some(asset) = matching_assets.first() {
        return Ok(asset);
    }

    bail!(
        "No asset matching version {} and tag {}",
        descriptor.version,
        descriptor
            .tag
            .as_ref()
            .map_or_else(|| String::from("(none)"), Tag::to_string)
    );
}

pub async fn download_asset(
    repository: &RepositoryInfo,
    asset: &Asset,
    shared_dir: &Path,
) -> Result<PathBuf> {
    let asset_path = shared_dir.join(&asset.name);
    if asset_path.exists() {
        info!("Asset {} already downloaded", asset_path.display());
        return Ok(asset_path);
    }

    let mut response = repository.repository.get_asset(asset).await?;
    download_stream("Python asset", &mut response, &asset_path).await?;

    let is_valid = validate_sha256_checksum(&asset_path, &asset.tag)?;
    if !is_valid {
        remove_file(&asset_path)?;
        bail!(
            "SHA256 checksum validation failed on {}",
            asset_path.display()
        );
    }

    info!(
        "SHA256 checksum validation succeeded on {}",
        asset_path.display()
    );

    Ok(asset_path)
}
