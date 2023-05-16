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
use crate::cli::PythonVersion;
use crate::object_model::{Project, Tag, Version};
use crate::serialization::{ProjectEnvironmentRecord, ProjectRecord};
use crate::util::{download_asset, get_asset, init_project, unpack_file, PROJECT_CONFIG_FILE_NAME};
use anyhow::{bail, Result};
use joat_repo::DirInfo;
use joatmon::read_yaml_file;
use std::path::{Path, PathBuf};

pub async fn do_init_config(app: &App) -> Result<()> {
    let config_path = app.cwd.join(PROJECT_CONFIG_FILE_NAME);

    if let Some(dir_info) = app.repo.get(&app.cwd)? {
        bail!(
            "Directory {} already has Python environment",
            app.cwd.display()
        )
    }

    let rec = read_yaml_file::<ProjectRecord, _>(&config_path)?;

    let python_version = PythonVersion {
        version: rec.python_version,
        tag: rec.tag,
    };

    init_project(app, &python_version, &config_path).await?;

    Ok(())
}
