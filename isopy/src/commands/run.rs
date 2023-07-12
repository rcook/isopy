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
use crate::dir_info_ext::DirInfoExt;
use crate::shell::Command;
use crate::status::Status;
use anyhow::Result;
use log::error;
use std::ffi::OsString;

pub fn run(app: App, program: &str, args: &[String]) -> Result<Status> {
    let mut command = Command::new(OsString::from(program));
    for arg in args {
        command.arg(OsString::from(arg));
    }

    let Some(dir_info) = app.find_dir_info(&app.cwd, None)? else {
        error!("could not find environment for directory {}", app.cwd.display());
        return Ok(Status::Fail);
    };

    let Some(env_info) = dir_info.make_env_info(None)? else {
        error!("could not get environment info");
        return Ok(Status::Fail);
    };

    // Explicitly drop app so that repository is unlocked in shell
    drop(app);
    command.exec(dir_info.link_id(), dir_info.meta_id(), &env_info)?;
    Ok(Status::OK)
}
