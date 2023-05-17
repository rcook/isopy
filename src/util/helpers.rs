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
use crate::constants::ENV_FILE_NAME;
use crate::object_model::{Asset, AssetFilter, Tag};
use crate::serialization::EnvRec;
use crate::util::{download_stream, unpack_file, validate_sha256_checksum};
use anyhow::{anyhow, bail, Result};
use joatmon::safe_write_file;
use log::info;
use std::fs::remove_file;
use std::path::{Path, PathBuf};

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

pub async fn init_project(
    app: &App,
    python_version: &PythonVersion,
    config_path: &Path,
) -> Result<()> {
    let assets = app.read_assets()?;
    let asset = get_asset(&assets, python_version)?;

    let mut asset_path = app.make_asset_path(asset);
    if !asset_path.is_file() {
        asset_path = download_asset(app, asset).await?;
    }

    let Some(dir_info) = app.repo.init(&app.cwd)? else {
        bail!(
            "Could not initialize metadirectory for directory {}",
            app.cwd.display()
        )
    };

    unpack_file(&asset_path, dir_info.data_dir())?;

    safe_write_file(
        dir_info.data_dir().join(ENV_FILE_NAME),
        serde_yaml::to_string(&EnvRec {
            config_path: config_path.to_path_buf(),
            python_dir_rel: PathBuf::from("python"),
            version: asset.meta.version.clone(),
            tag: asset.tag.clone(),
        })?,
        false,
    )?;

    Ok(())
}
