use crate::app::App;
use crate::helpers::{download_asset, get_asset};
use crate::object_model::Project;
use crate::probe::PROJECT_CONFIG_FILE_NAME;
use crate::result::{user, Result};
use crate::serialization::AnonymousEnvRecord;
use crate::util::{safe_write_file, unpack_file};
use std::path::PathBuf;

pub async fn do_init(app: &App) -> Result<()> {
    match app.read_project(&app.cwd)? {
        None => Err(user(format!(
            "Could not find project configuration file {}",
            PROJECT_CONFIG_FILE_NAME
        ))),
        Some(project) => Ok(init_project(app, &project).await?),
    }
}

async fn init_project(app: &App, project: &Project) -> Result<()> {
    let assets = app.read_assets()?;
    let asset = get_asset(&assets, &project.python_version, &project.tag)?;

    let mut asset_path = app.make_asset_path(&asset);
    if !asset_path.is_file() {
        asset_path = download_asset(app, asset).await?;
    }

    let anonymous_env_dir = app.anonymous_env_dir(&project.config_path)?;
    unpack_file(&asset_path, &anonymous_env_dir)?;

    safe_write_file(
        anonymous_env_dir.join("env.yaml"),
        serde_yaml::to_string(&AnonymousEnvRecord {
            config_path: project.config_path.clone(),
            python_dir: PathBuf::from("python"),
            python_version: asset.meta.version.clone(),
            tag: asset.tag.clone(),
        })?,
        false,
    )?;

    Ok(())
}
