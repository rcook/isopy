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
use crate::shell::Command;
use crate::status::Status;
use anyhow::{bail, Result};
use joatmon::read_yaml_file;
use std::env::{var, VarError};

pub fn do_shell(app: App) -> Result<Status> {
    match var(ISOPY_ENV_NAME) {
        Ok(_) => {
            bail!("You are already in an isopy shell");
        }
        Err(VarError::NotPresent) => {}
        Err(e) => return Err(e)?,
    }

    let Some(dir_info) = app.find_dir_info( &app.cwd)? else {
        bail!("Could not find environment for directory {}", app.cwd.display())
    };

    let data_dir = dir_info.data_dir();
    let rec = read_yaml_file::<EnvRec, _>(&data_dir.join(ENV_FILE_NAME))?;
    let python_dir = data_dir.join(rec.python_dir_rel);

    // Explicitly drop app so that repository is unlocked in shell
    drop(app);

    Command::new_shell().exec(dir_info.link_id(), dir_info.meta_id(), &python_dir)?;
    Ok(Status::OK)
}
