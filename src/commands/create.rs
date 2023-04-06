use crate::app::App;
use crate::helpers::get_asset;
use crate::object_model::{EnvironmentName, Tag, Version};
use crate::result::Result;
use crate::serialization::NamedEnvRecord;
use crate::util::{safe_write_file, unpack_file};
use std::path::PathBuf;

pub async fn do_create(
    app: &App,
    env_name: &EnvironmentName,
    version: &Version,
    tag: &Option<Tag>,
) -> Result<()> {
    let assets = app.read_assets()?;
    let asset = get_asset(&assets, version, tag)?;

    let archive_path = app.assets_dir.join(&asset.name);
    let named_env_dir = app.named_env_dir(&env_name);
    unpack_file(&archive_path, &named_env_dir)?;

    safe_write_file(
        named_env_dir.join("env.yaml"),
        serde_yaml::to_string(&NamedEnvRecord {
            name: env_name.clone(),
            python_dir: PathBuf::from("python"),
            python_version: asset.meta.version.clone(),
            tag: asset.tag.clone(),
        })?,
        false,
    )?;

    Ok(())
}
