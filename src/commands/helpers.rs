use crate::app::App;
use crate::error::{fatal, user, Result};
use crate::object_model::{Asset, AssetFilter, Tag, Version};
use crate::util::{download_file, validate_sha256_checksum};
use reqwest::Client;
use std::fs::remove_file;
use std::path::PathBuf;

pub fn get_asset<'a>(
    assets: &'a Vec<Asset>,
    version: &Version,
    tag: &Option<Tag>,
) -> Result<&'a Asset> {
    let mut asset_filter = AssetFilter::default_for_platform();
    asset_filter.version = Some(version.clone());
    asset_filter.tag = tag.clone();
    let matching_assets = asset_filter.filter(assets.iter().map(|x| x).into_iter());
    match matching_assets.len() {
        1 => return Ok(matching_assets.first().expect("Must exist")),
        0 => {
            return Err(user(format!(
                "No asset matching version {} and tag {}",
                version,
                tag.as_ref()
                    .map(Tag::to_string)
                    .unwrap_or(String::from("(none)"))
            )))
        }
        _ => {
            return Err(user(format!(
                "More than one asset matching version {} and tag {}",
                version,
                tag.as_ref()
                    .map(Tag::to_string)
                    .unwrap_or(String::from("(none)"))
            )))
        }
    }
}

pub async fn download_asset(app: &App, asset: &Asset) -> Result<PathBuf> {
    let asset_path = app.make_asset_path(asset);
    if asset_path.exists() {
        return Err(user(format!(
            "Asset {} already downloaded",
            asset_path.display()
        )));
    }

    let client = Client::builder().build()?;
    download_file(&client, asset.url.clone(), &asset_path, true).await?;

    let is_valid = validate_sha256_checksum(&asset_path, &asset.tag)?;
    if !is_valid {
        remove_file(&asset_path)?;
        return Err(fatal(format!(
            "SHA256 checksum validation failed on {}",
            asset_path.display()
        )));
    }

    println!(
        "SHA256 checksum validation succeeded on {}",
        asset_path.display()
    );

    Ok(asset_path)
}
