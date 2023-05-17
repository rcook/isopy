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
use crate::app::App;
use crate::cli::PythonVersion;
use crate::object_model::{Asset, AssetFilter, Tag};
use crate::util::{download_stream, validate_sha256_checksum};
use anyhow::{anyhow, bail, Result};
use log::info;
use std::fs::remove_file;
use std::path::PathBuf;

pub fn get_asset<'a>(assets: &'a [Asset], python_version: &PythonVersion) -> Result<&'a Asset> {
    let mut asset_filter = AssetFilter::default_for_platform();
    asset_filter.version = Some(python_version.version.clone());
    asset_filter.tag = python_version.tag.clone();
    let matching_assets = asset_filter.filter(assets.iter());
    match matching_assets.len() {
        1 => return Ok(matching_assets.first().expect("Must exist")),
        0 => bail!(
            "No asset matching version {} and tag {}",
            python_version.version,
            python_version
                .tag
                .as_ref()
                .map(Tag::to_string)
                .unwrap_or(String::from("(none)"))
        ),
        _ => bail!(
            "More than one asset matching version {} and tag {}",
            python_version.version,
            python_version
                .tag
                .as_ref()
                .map(Tag::to_string)
                .unwrap_or(String::from("(none)"))
        ),
    }
}

pub async fn download_asset(app: &App, asset: &Asset) -> Result<PathBuf> {
    let repositories = app.read_repositories()?;
    let repository = repositories
        .first()
        .ok_or(anyhow!("No asset repositories are configured"))?;

    let asset_path = app.make_asset_path(asset);
    if asset_path.exists() {
        bail!("Asset {} already downloaded", asset_path.display());
    }

    let mut response = repository.repository.get_asset(asset).await?;
    download_stream("asset", &mut response, &asset_path).await?;

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
