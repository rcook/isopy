use super::helpers::get_asset;
use crate::app::App;
use crate::commands::helpers::{download_asset, make_asset_path};
use crate::error::Result;
use crate::serialization::{AnonymousEnvRecord, ProjectRecord};
use crate::util::{path_to_str, safe_write_file, unpack_file};
use md5::compute;
use std::fs::read_to_string;
use std::path::PathBuf;

pub async fn do_init(app: &App) -> Result<()> {
    let project_config_path = app.cwd.join(".isopy.yaml");
    let s = read_to_string(&project_config_path)?;
    let project_record = serde_yaml::from_str::<ProjectRecord>(&s)?;

    let assets = app.read_assets()?;
    let asset = get_asset(&assets, &project_record.python_version, &project_record.tag)?;

    let mut asset_path = make_asset_path(app, &asset);
    if !asset_path.is_file() {
        asset_path = download_asset(app, asset).await?;
    }

    let hex_digest = format!("{:x}", compute(path_to_str(&project_config_path)?));
    let env_dir = app.dir.join("hashed").join(hex_digest);
    unpack_file(&asset_path, &env_dir)?;

    let env_path = env_dir.join("env.yaml");
    let env_record = AnonymousEnvRecord {
        config_path: project_config_path,
        python_dir: PathBuf::from("python"),
        python_version: asset.meta.version.clone(),
        tag: asset.tag.clone(),
    };

    safe_write_file(env_path, serde_yaml::to_string(&env_record)?, false)?;

    Ok(())
}
