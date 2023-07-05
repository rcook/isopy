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
use crate::constants::{ENV_FILE_NAME, ISOPY_ENV_NAME};
use crate::registry::Registry;
use crate::serialization::EnvRec;
use crate::shell::Command;
use crate::status::Status;
use anyhow::{bail, Result};
use joatmon::read_yaml_file;
use log::error;
use std::env::{var, VarError};
use std::path::Path;

pub fn do_shell(app: App, package_dir_id: &Option<String>) -> Result<Status> {
    match var(ISOPY_ENV_NAME) {
        Ok(_) => {
            bail!("You are already in an isopy shell");
        }
        Err(VarError::NotPresent) => {}
        Err(e) => return Err(e)?,
    }

    let Some(dir_info) = app.find_dir_info(&app.cwd, None)? else {
        error!("could not find environment for directory {}", app.cwd.display());
        return Ok(Status::Fail);
    };

    let env_rec = read_yaml_file::<EnvRec>(&dir_info.data_dir().join(ENV_FILE_NAME))?;

    let package_dir_rec = match package_dir_id.as_ref() {
        Some(id) => env_rec.package_dirs.iter().find(|p| p.id == *id),
        None => env_rec.package_dirs.first(),
    };

    let Some(package_dir_rec) = package_dir_rec else {
        if let Some(package_dir_id) = package_dir_id {
            error!("could not find package directory with ID {}", package_dir_id);
        } else {
            error!("could not find default package directory");
        }
        return Ok(Status::Fail);
    };

    let Some(env_info) = Registry::global().blah(dir_info.data_dir(), package_dir_rec)? else {
        error!("could not get environment info");
        return Ok(Status::Fail);
    };

    // Explicitly drop app so that repository is unlocked in shell
    drop(app);

    Command::new_shell().exec(
        dir_info.link_id(),
        dir_info.meta_id(),
        &env_info
            .path_dirs
            .iter()
            .map(|p| p as &Path)
            .collect::<Vec<_>>(),
        &env_info
            .envs
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect::<Vec<_>>(),
    )?;

    Ok(Status::OK)
}
