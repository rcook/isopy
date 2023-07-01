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
use crate::constants::ENV_FILE_NAME;
use crate::constants::ISOPY_ENV_NAME;
use crate::serialization::EnvRec;
use crate::serialization::OpenJdkEnvRec;
use crate::serialization::PythonEnvRec;
use crate::shell::make_openjdk_path_dirs;
use crate::shell::make_python_path_dirs;
use crate::shell::Command;
use crate::status::Status;
use anyhow::{anyhow, bail, Result};
use joat_repo::DirInfo;
use joatmon::read_yaml_file;
use std::env::{var, VarError};
use std::path::Path;

pub fn do_shell(app: App) -> Result<Status> {
    match var(ISOPY_ENV_NAME) {
        Ok(_) => {
            bail!("You are already in an isopy shell");
        }
        Err(VarError::NotPresent) => {}
        Err(e) => return Err(e)?,
    }

    let Some(dir_info) = app.find_dir_info(&app.cwd, None)? else {
        bail!("Could not find environment for directory {}", app.cwd.display())
    };

    let env_rec = read_yaml_file::<EnvRec>(&dir_info.data_dir().join(ENV_FILE_NAME))?;

    if let Some(rec) = env_rec.python {
        do_shell_python(app, &rec, &dir_info)?;
        return Ok(Status::OK);
    };

    if let Some(rec) = env_rec.openjdk {
        do_shell_openjdk(app, &rec, &dir_info)?;
        return Ok(Status::OK);
    }

    bail!(
        "Environment for directory {} is incorrectly configured",
        app.cwd.display()
    )
}

fn do_shell_python(app: App, rec: &PythonEnvRec, dir_info: &DirInfo) -> Result<()> {
    // Explicitly drop app so that repository is unlocked in shell
    drop(app);

    Command::new_shell().exec(
        dir_info.link_id(),
        dir_info.meta_id(),
        &make_python_path_dirs(dir_info.data_dir(), rec)
            .iter()
            .map(|p| p as &Path)
            .collect::<Vec<_>>(),
        &[],
    )?;
    Ok(())
}

fn do_shell_openjdk(app: App, rec: &OpenJdkEnvRec, dir_info: &DirInfo) -> Result<()> {
    // Explicitly drop app so that repository is unlocked in shell
    drop(app);

    let openjdk_dir = dir_info.data_dir().join(&rec.dir);
    let openjdk_dir_str = openjdk_dir
        .to_str()
        .ok_or_else(|| anyhow!("could not convert path to string"))?;

    Command::new_shell().exec(
        dir_info.link_id(),
        dir_info.meta_id(),
        &make_openjdk_path_dirs(dir_info.data_dir(), rec)
            .iter()
            .map(|p| p as &Path)
            .collect::<Vec<_>>(),
        &[("JAVA_HOME", openjdk_dir_str)],
    )?;
    Ok(())
}
