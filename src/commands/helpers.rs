use crate::error::{user, Result};
use crate::object_model::{Asset, AssetFilter, Tag, Version};
use crate::util::unpack_file;
use std::path::Path;

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

pub fn create_env_dir<P, Q>(archive_path: P, env_dir: Q) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    if !archive_path.as_ref().exists() {
        return Err(user(format!(
            "File {} does not exist",
            archive_path.as_ref().display()
        )));
    }

    if env_dir.as_ref().exists() {
        return Err(user(format!(
            "Environment directory {} already exists",
            env_dir.as_ref().display()
        )));
    }

    unpack_file(&archive_path, &env_dir)?;
    Ok(())
}
