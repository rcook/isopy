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
    do_available, do_check, do_download, do_downloaded, do_exec, do_gen_config, do_info, do_init,
    do_init_config, do_link, do_list, do_prompt, do_scratch, do_shell, do_wrap,
};
use crate::constants::CACHE_DIR_NAME;
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
    let isopy_dir = home_dir.join(CACHE_DIR_NAME);
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
        Command::Available { package_filter } => do_available(&app, package_filter).await,
        Command::Check { clean } => do_check(&app, clean),
        Command::Download { product_descriptor } => do_download(&app, &product_descriptor).await,
        Command::Downloaded => do_downloaded(&app),
        Command::Exec { program, args } => do_exec(app, &program, &args),
        Command::GenConfig {
            python_version,
            force,
        } => do_gen_config(&app, &python_version, force),
        Command::Info => do_info(&app),
        Command::Init(python_version) => do_init(app, &python_version).await,
        Command::InitConfig => do_init_config(app).await,
        Command::Link { meta_id } => do_link(&app, &meta_id),
        Command::List => do_list(&app),
        Command::Prompt => do_prompt(&app),
        Command::Scratch => do_scratch(&app),
        Command::Shell => do_shell(app),
        Command::Wrap {
            wrapper_path,
            script_path,
            base_dir,
        } => do_wrap(&app, &wrapper_path, &script_path, &base_dir),
    }
}
