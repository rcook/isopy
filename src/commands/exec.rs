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
use crate::constants::ENV_FILE_NAME;
use crate::serialization::EnvRec;
use crate::shell::Command;
use crate::status::Status;
use crate::{app::App, shell::make_python_path_dirs};
use anyhow::{bail, Result};
use joatmon::read_yaml_file;
use std::ffi::OsString;
use std::path::Path;

pub fn do_exec(app: App, program: &str, args: &[String]) -> Result<Status> {
    let mut command = Command::new(OsString::from(program));
    for arg in args {
        command.arg(OsString::from(arg));
    }

    let Some(dir_info) = app.find_dir_info(&app.cwd,None)? else {
        bail!("Could not find environment for directory {}", app.cwd.display())
    };

    let data_dir = dir_info.data_dir();
    let env_rec = read_yaml_file::<EnvRec>(&data_dir.join(ENV_FILE_NAME))?;

    let Some(rec) = env_rec.python else {
        bail!("No Python configured for directory {}", app.cwd.display())
    };

    // Explicitly drop app so that repository is unlocked in shell
    drop(app);

    command.exec(
        dir_info.link_id(),
        dir_info.meta_id(),
        &make_python_path_dirs(dir_info.data_dir(), &rec)
            .iter()
            .map(|p| p as &Path)
            .collect::<Vec<_>>(),
        &[],
    )?;
    Ok(Status::OK)
}
