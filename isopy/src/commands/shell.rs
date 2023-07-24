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
use crate::constants::ISOPY_ENV_NAME;
use crate::dir_info_ext::DirInfoExt;
use crate::shell::Command;
use crate::status::{return_success, return_user_error, Status};
use anyhow::{bail, Result};
use colored::Colorize;
use log::info;
use std::env::{var, VarError};

pub fn shell(app: App, verbose: bool) -> Result<Status> {
    match var(ISOPY_ENV_NAME) {
        Ok(_) => {
            bail!("you are already in an isopy shell");
        }
        Err(VarError::NotPresent) => {}
        Err(e) => return Err(e)?,
    }

    let Some(dir_info) = app.find_dir_info(None)? else {
        return_user_error!(
            "could not find environment for directory {}",
            app.cwd().display()
        );
    };

    let Some(env_info) = dir_info.make_env_info(None)? else {
        return_user_error!("could not get environment info");
    };

    if verbose {
        info!("{}", "Starting isopy environment shell".bright_green());

        if !env_info.path_dirs.is_empty() {
            info!("{}", "Path directories:".bright_yellow());
            for path_dir in &env_info.path_dirs {
                info!("  {}", format!("{}", path_dir.display()).yellow());
            }
        }

        if !env_info.vars.is_empty() {
            info!("{}", "Additional environment variables:".bright_yellow());
            for (k, v) in &env_info.vars {
                info!("  {}", format!("{k} = {v}").yellow());
            }
        }
    }

    // Explicitly drop app so that repository is unlocked in shell
    drop(app);
    Command::new_shell().exec(dir_info.link_id(), dir_info.meta_id(), &env_info)?;
    return_success!();
}
