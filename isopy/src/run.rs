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
use crate::commands::*;
use crate::constants::CACHE_DIR;
use crate::env::set_up_env;
use crate::moniker::Moniker;
use crate::status::Status;
use crate::terminal::reset_terminal;
use anyhow::{bail, Result};
use clap::Parser;
use joat_logger::init_ui;
use joat_repo::RepoConfig;
use log::{set_max_level, LevelFilter};
use std::env::current_dir;
use std::path::PathBuf;

fn set_up() -> Result<()> {
    reset_terminal();
    init_ui(true)?;
    set_max_level(LevelFilter::Trace);
    set_up_env()?;
    Ok(())
}

fn default_cache_dir() -> Option<PathBuf> {
    let home_dir = home::home_dir()?;
    let isopy_dir = home_dir.join(&*CACHE_DIR);
    Some(isopy_dir)
}

pub(crate) async fn run() -> Result<Status> {
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

    let app = App::new(cwd, &cache_dir, repo)?;
    do_it(app, args.command).await
}

async fn do_it(app: App, command: Command) -> Result<Status> {
    use crate::args::Command::*;
    use crate::args::EnvCommand;
    use crate::args::ProjectCommand;

    match command {
        Check { clean, .. } => do_check(&app, clean),
        Completions { shell } => Ok(do_completions(shell)),
        Delete { project_dir } => do_delete(&app, &project_dir).await,
        Download { package_id, tags } => do_download(&app, &package_id, &tags).await,
        Env { command } => match command {
            EnvCommand::Install { package_id } => do_env_install(&app, &package_id).await,
            EnvCommand::List { verbose, .. } => do_env_list(&app, verbose),
            EnvCommand::Link { dir_id } => do_env_link(&app, &dir_id),
        },
        Packages {
            moniker,
            filter,
            tags,
            verbose,
            ..
        } => {
            do_packages(
                &app,
                &moniker.map(Into::<Moniker>::into),
                filter.into(),
                &tags,
                verbose,
            )
            .await
        }
        Info => do_info(&app),
        Project { command } => match command {
            ProjectCommand::Add { package_id } => do_project_add(&app, &package_id),
            ProjectCommand::Install => do_project_install(&app).await,
        },
        Prompt(prompt_config) => do_prompt(&app, &prompt_config),
        Run { program, args } => do_run(app, &program, &args),
        Scratch => do_scratch(&app).await,
        Shell { verbose, .. } => do_shell(app, verbose),
        Tags { moniker } => do_tags(&app, &moniker.map(Into::<Moniker>::into)).await,
        Update { moniker } => do_update(&app, &moniker.map(Into::<Moniker>::into)).await,
        Wrap {
            wrapper_file_name,
            script_path,
            base_dir,
            platform,
            shell,
            force,
            ..
        } => do_wrap(
            &app,
            &wrapper_file_name,
            &script_path,
            &base_dir,
            platform.into(),
            shell.into(),
            force,
        ),
    }
}
