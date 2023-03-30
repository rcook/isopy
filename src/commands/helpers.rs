use crate::error::{user, Result};
use crate::object_model::{Asset, AssetFilter, Tag, Version};

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
