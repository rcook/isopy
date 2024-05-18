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
use crate::shell::{Command, IsopyEnv};
use crate::status::{return_success, return_user_error, Status};
use anyhow::Result;
use colored::Colorize;
use ctrlc::set_handler;
use log::info;

pub fn shell(app: App, verbose: bool) -> Result<Status> {
    if let Some(isopy_env) = IsopyEnv::get_vars()? {
        if let Some(link) = app.find_link(isopy_env.link_id())? {
            return_user_error!(
                "you are already in the isopy shell for project {}",
                link.project_dir().display()
            );
        }

        return_user_error!(
            "you are already in an isopy shell (metadirectory ID {}",
            isopy_env.meta_id()
        );
    };

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

    let isopy_env = IsopyEnv::from_dir_info(&dir_info);

    // Explicitly drop app so that repository is unlocked in shell
    drop(app);

    // Handler to prevent isopy.exe from being killed by Ctrl+C from child process
    set_handler(move || {})?;

    Command::new_shell().exec(&isopy_env, &env_info)?;
    return_success!();
}
