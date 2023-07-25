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
use crate::args::{Args, Command};
use crate::commands::env::{install as env_install, link as env_link, list as env_list};
use crate::commands::package::{download as package_download, list as package_list, ListType};
use crate::commands::project::{add as project_add, install as project_install};
use crate::commands::wrap::{wrap, WrapTarget};
use crate::commands::{check, completions, info, prompt, run as run_command, scratch, shell};
use crate::constants::CACHE_DIR;
use crate::status::{init_backtrace, Status};
use crate::terminal::reset_terminal;
use anyhow::{bail, Result};
use clap::Parser;
use joat_logger::init_ui;
use joat_repo::RepoConfig;
use log::{set_max_level, LevelFilter};
use std::env::current_dir;
use std::path::PathBuf;

fn set_up() -> Result<()> {
    init_backtrace();
    reset_terminal();
    init_ui(true)?;
    set_max_level(LevelFilter::Trace);
    Ok(())
}

fn default_cache_dir() -> Option<PathBuf> {
    let home_dir = home::home_dir()?;
    let isopy_dir = home_dir.join(&*CACHE_DIR);
    Some(isopy_dir)
}

pub async fn run() -> Result<Status> {
    set_up()?;

    let args = Args::parse();

    set_max_level(args.log_level.into());

    let Some(cache_dir) = args.cache_dir.or_else(default_cache_dir) else {
        bail!("Could not infer isopy cache directory location: please specify using --dir option")
    };

    let repo = match RepoConfig::default(&cache_dir, None).repo() {
        Ok(Some(r)) => r,
        Ok(None) => {
            bail!("Could not acquire lock on repository: is another instance of isopy running?")
        }
        Err(e) => bail!(e),
    };

    let cwd = match args.cwd {
        Some(c) => c,
        None => current_dir()?,
    };

    let app = App::new(cwd, &cache_dir, repo);
    do_it(app, args.command).await
}

async fn do_it(app: App, command: Command) -> Result<Status> {
    use crate::args::Command::*;
    use crate::args::EnvCommand;
    use crate::args::PackageCommand;
    use crate::args::ProjectCommand;
    use crate::args::WrapCommand;

    match command {
        Check { clean, .. } => check(&app, clean),
        Completions { shell } => Ok(completions(shell)),
        Env { command } => match command {
            EnvCommand::Install { package_id } => env_install(&app, &package_id).await,
            EnvCommand::List { verbose, .. } => env_list(&app, verbose),
            EnvCommand::Link { dir_id } => env_link(&app, &dir_id),
        },
        Info => info(&app),
        Package { command } => match command {
            PackageCommand::Download { package_id } => package_download(&app, &package_id).await,
            PackageCommand::List { verbose, all, .. } => {
                package_list(
                    &app,
                    if all {
                        ListType::All
                    } else {
                        ListType::LocalOnly
                    },
                    verbose,
                )
                .await
            }
        },
        Project { command } => match command {
            ProjectCommand::Add { package_id } => project_add(&app, &package_id),
            ProjectCommand::Install => project_install(&app).await,
        },
        Prompt(prompt_config) => prompt(&app, &prompt_config),
        Run { program, args } => run_command(app, &program, &args),
        Scratch => scratch(&app),
        Shell { verbose, .. } => shell(app, verbose),
        Wrap { command } => match command {
            WrapCommand::Command {
                wrapper_file_name,
                command,
                base_dir,
                force,
                ..
            } => wrap(
                &app,
                &wrapper_file_name,
                &WrapTarget::Command(command),
                &base_dir,
                force,
            ),
            WrapCommand::Script {
                wrapper_file_name,
                script_path,
                base_dir,
                force,
                ..
            } => wrap(
                &app,
                &wrapper_file_name,
                &WrapTarget::Script(script_path),
                &base_dir,
                force,
            ),
        },
    }
}
