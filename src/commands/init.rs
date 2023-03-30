use super::helpers::get_asset;
use crate::app::App;
use crate::commands::helpers::{download_asset, make_asset_path};
use crate::error::Result;
use crate::serialization::{AnonymousEnvRecord, ProjectRecord};
use crate::util::{safe_write_file, unpack_file};
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

    let anonymous_env_dir = app.anonymous_env_dir(&project_config_path)?;
    unpack_file(&asset_path, &anonymous_env_dir)?;

    safe_write_file(
        anonymous_env_dir.join("env.yaml"),
        serde_yaml::to_string(&AnonymousEnvRecord {
            config_path: project_config_path,
            python_dir: PathBuf::from("python"),
            python_version: asset.meta.version.clone(),
            tag: asset.tag.clone(),
        })?,
        false,
    )?;

    Ok(())
}
