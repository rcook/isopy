// Copyright (c) 2023 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use crate::app::App;
use crate::object_model::{Project, Tag, Version};
use crate::python_info::PythonInfo;
use crate::serialization::ProjectEnvironmentRecord;
use crate::util::{download_asset, get_asset, unpack_file, PROJECT_CONFIG_FILE_NAME};
use anyhow::{bail, Result};
use joat_repo::DirInfo;
use joatmon::safe_write_file;
use std::path::{Path, PathBuf};

pub async fn do_init(app: &App, python_info: Option<PythonInfo>) -> Result<()> {
    if let Some(temp) = python_info {
        init_project(app, &temp, &app.cwd).await?;
    } else {
        let Some(project) = app.read_project(&app.cwd)? else {
            bail!(
                "Could not find project configuration file {}",
                PROJECT_CONFIG_FILE_NAME
            )
        };
        let python_info = PythonInfo {
            version: project.python_version,
            tag: project.tag,
        };
        init_project(app, &python_info, &project.config_path).await?;
    }

    Ok(())
}

async fn init_project(app: &App, python_info: &PythonInfo, config_path: &Path) -> Result<()> {
    let assets = app.read_assets()?;
    let asset = get_asset(&assets, python_info)?;

    let mut asset_path = app.make_asset_path(asset);
    if !asset_path.is_file() {
        asset_path = download_asset(app, asset).await?;
    }

    let Some(dir_info) = app.repo.init(&app.cwd)? else {
        bail!(
            "Could not initialize metadirectory for directory {}",
            app.cwd.display()
        )
    };

    unpack_file(&asset_path, dir_info.data_dir())?;

    safe_write_file(
        dir_info.data_dir().join("env.yaml"),
        serde_yaml::to_string(&ProjectEnvironmentRecord {
            config_path: config_path.to_path_buf(),
            python_dir_rel: PathBuf::from("python"),
            python_version: asset.meta.version.clone(),
            tag: asset.tag.clone(),
        })?,
        false,
    )?;

    Ok(())
}
