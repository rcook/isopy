use crate::config::Config;
use crate::error::{user, Result};
use crate::object_model::{AssetFilter, Tag, Version};
use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

pub async fn do_create(config: &Config, version: &Version, tag: &Option<Tag>) -> Result<()> {
    let assets = config.read_assets()?;
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

    let output_path = config.assets_dir.join(&asset.name);

    if !output_path.exists() {
        return Err(user(format!(
            "File {} does not exist",
            output_path.display()
        )));
    }

    let tar_gz = File::open(output_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack("TEMP")?;

    Ok(())
}
