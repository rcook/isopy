use super::helpers::{create_env_dir, get_asset};
use crate::app::App;
use crate::error::Result;
use crate::object_model::EnvName;
use crate::serialization::{HashedEnvRecord, ProjectRecord};
use crate::util::path_to_str;
use md5::compute;
use std::fs::read_to_string;
use std::path::PathBuf;

pub fn do_use(app: &App, _env_name_opt: &Option<EnvName>) -> Result<()> {
    let config_path = app.cwd.join(".isopy.yaml");
    let s = read_to_string(&config_path)?;
    let project_record = serde_yaml::from_str::<ProjectRecord>(&s)?;

    let assets = app.read_assets()?;
    let asset = get_asset(&assets, &project_record.python_version, &project_record.tag)?;

    let archive_path = app.assets_dir.join(&asset.name);
    let hex_digest = format!("{:x}", compute(path_to_str(&config_path)?));
    let env_dir = app.dir.join("hashed").join(hex_digest);

    create_env_dir(&archive_path, &env_dir)?;

    let env_path = env_dir.join("env.yaml");
    let env_record = HashedEnvRecord {
        config_path: config_path,
        python_dir: PathBuf::from("python"),
        python_version: asset.meta.version.clone(),
        tag: asset.tag.clone(),
    };
    std::fs::write(env_path, serde_yaml::to_string(&env_record)?)?;

    Ok(())
}