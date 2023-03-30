use super::helpers::get_asset;
use crate::app::App;
use crate::error::Result;
use crate::object_model::{EnvName, Tag, Version};
use crate::serialization::EnvRecord;
use crate::util::{safe_write_to_file, unpack_file};
use std::path::PathBuf;

pub async fn do_create(
    app: &App,
    env_name: &EnvName,
    version: &Version,
    tag: &Option<Tag>,
) -> Result<()> {
    let assets = app.read_assets()?;
    let asset = get_asset(&assets, version, tag)?;

    let archive_path = app.assets_dir.join(&asset.name);
    let env_dir = app.env_dir(&env_name);
    unpack_file(&archive_path, &env_dir)?;

    let env_path = env_dir.join("env.yaml");
    let env_record = EnvRecord {
        name: env_name.clone(),
        python_dir: PathBuf::from("python"),
        python_version: asset.meta.version.clone(),
        tag: asset.tag.clone(),
    };

    safe_write_to_file(env_path, serde_yaml::to_string(&env_record)?, false)?;

    Ok(())
}
