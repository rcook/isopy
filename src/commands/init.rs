use crate::app::{App, PROJECT_CONFIG_FILE_NAME};
use crate::helpers::{download_asset, get_asset};
use crate::result::Result;
use crate::serialization::AnonymousEnvRecord;
use crate::util::{safe_write_file, unpack_file};
use std::path::PathBuf;

pub async fn do_init(app: &App) -> Result<()> {
    let project_record = app.read_project_config()?;

    let assets = app.read_assets()?;
    let asset = get_asset(&assets, &project_record.python_version, &project_record.tag)?;

    let mut asset_path = app.make_asset_path(&asset);
    if !asset_path.is_file() {
        asset_path = download_asset(app, asset).await?;
    }

    let anonymous_env_dir = app.anonymous_env_dir(&PROJECT_CONFIG_FILE_NAME)?;
    unpack_file(&asset_path, &anonymous_env_dir)?;

    let project_config_path = app.get_project_config_path();

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
