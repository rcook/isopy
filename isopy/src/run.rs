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
use crate::backtrace::init_backtrace;
use crate::commands::{
    add, check, download, info, install, install_project, link, list, list_available_packages,
    list_downloaded_packages, prompt, run as run_command, scratch, shell, wrap,
};
use crate::constants::CACHE_DIR;
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

    let app = App::new(cwd, repo);
    do_it(app, args.command).await
}

async fn do_it(app: App, command: Command) -> Result<Status> {
    match command {
        Command::Add { package_id } => add(&app, &package_id),
        Command::Check { clean } => check(&app, clean),
        Command::Download { package_id } => download(&app, &package_id).await,
        Command::Info => info(&app),
        Command::Install { package_id } => install(&app, &package_id).await,
        Command::InstallProject => install_project(&app).await,
        Command::List => list(&app),
        Command::ListAvailablePackages { verbose } => list_available_packages(&app, verbose).await,
        Command::ListDownloadedPackages { verbose } => {
            list_downloaded_packages(&app, verbose).await
        }
        Command::Link { dir_id } => link(&app, &dir_id),
        Command::Prompt => prompt(&app),
        Command::Run { program, args } => run_command(app, &program, &args),
        Command::Scratch => scratch(&app),
        Command::Shell => shell(app),
        Command::Wrap {
            wrapper_path,
            script_path,
            base_dir,
        } => wrap(&app, &wrapper_path, &script_path, &base_dir),
    }
}
