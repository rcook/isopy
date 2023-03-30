use crate::app::App;
use crate::error::{fatal, user, Result};
use crate::object_model::{AssetFilter, Tag, Version};
use crate::util::{download_file, validate_sha256_checksum};
use reqwest::Client;
use std::fs::remove_file;

pub async fn do_download(app: &App, version: &Version, tag: &Option<Tag>) -> Result<()> {
    let assets = app.read_assets()?;
    let mut asset_filter = AssetFilter::default_for_platform();
    asset_filter.version = Some(version.clone());
    asset_filter.tag = tag.clone();
    let matching_assets = asset_filter.filter(assets.iter().map(|x| x).into_iter());
    let asset = match matching_assets.len() {
        1 => matching_assets.first().expect("Must exist"),
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
    };
    println!("{}", asset.name);

    let output_path = app.assets_dir.join(&asset.name);

    if output_path.exists() {
        return Err(user(format!(
            "File {} already exists",
            output_path.display()
        )));
    }

    let client = Client::builder().build()?;
    download_file(&client, asset.url.clone(), &output_path, true).await?;

    let is_valid = validate_sha256_checksum(&output_path, &asset.tag)?;
    if !is_valid {
        remove_file(&output_path)?;
        return Err(fatal(format!(
            "SHA256 checksum validation failed on {}",
            output_path.display()
        )));
    }

    println!(
        "SHA256 checksum validation succeeded on {}",
        output_path.display()
    );
    Ok(())
}
