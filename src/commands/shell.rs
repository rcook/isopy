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
use crate::object_model::{Environment, EnvironmentName};
use crate::serialization::ProjectEnvironmentRecord;
use crate::shell::{Command, ISOPY_ENV_NAME};
use crate::util::find_dir_info;
use crate::util::path_to_str;
use anyhow::{bail, Result};
use joatmon::read_yaml_file;
use std::env::{var, VarError};

pub fn do_shell(app: &App) -> Result<()> {
    match var(ISOPY_ENV_NAME) {
        Ok(_) => {
            bail!("You are already in an isopy shell");
        }
        Err(VarError::NotPresent) => {}
        Err(e) => return Err(e)?,
    }

    let Some(dir_info) = find_dir_info(&app.repo, &app.cwd)? else {
        bail!("Could not find environment for directory {}", app.cwd.display())
    };

    let env_path = dir_info.data_dir().join("env.yaml");
    let rec = read_yaml_file::<ProjectEnvironmentRecord, _>(&env_path)?;
    let environment = Environment {
        name: EnvironmentName::sanitize(path_to_str(&env_path)?),
        full_python_dir: dir_info.data_dir().join(rec.python_dir_rel),
    };

    Command::new_shell().exec(&environment)?;
    Ok(())
}
