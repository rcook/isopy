use crate::config::Config;
use crate::error::{user, Result};
use crate::object_model::{AssetFilter, EnvName, Tag, Version};
use crate::util::unpack_file;

pub async fn do_create(
    config: &Config,
    env_name: &EnvName,
    version: &Version,
    tag: &Option<Tag>,
) -> Result<()> {
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

    let output_path = config.assets_dir.join(&asset.name);
    if !output_path.exists() {
        return Err(user(format!(
            "File {} does not exist",
            output_path.display()
        )));
    }

    let env_dir = env_name.dir(&config);
    if env_dir.exists() {
        return Err(user(format!(
            "Environment directory {} already exists",
            env_dir.display()
        )));
    }

    unpack_file(&output_path, env_dir)?;

    Ok(())
}
