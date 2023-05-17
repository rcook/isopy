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
use crate::backtrace::init_backtrace;
use crate::cli::{Args, Command};
use crate::commands::{
    do_available, do_download, do_downloaded, do_exec, do_gen_config, do_info, do_init,
    do_init_config, do_link, do_list, do_shell, do_wrap,
};
use crate::logging::init_logging;
use crate::status::Status;
use crate::ui::reset_terminal;
use crate::util::default_isopy_dir;
use anyhow::{bail, Result};
use clap::Parser;
use joat_repo::RepoConfig;
use log::LevelFilter;
use std::env::current_dir;

pub async fn run() -> Result<Status> {
    init_backtrace();
    reset_terminal();
    init_logging(LevelFilter::Info)?;

    let args = Args::parse();

    let cwd = match args.cwd {
        Some(c) => c,
        None => current_dir()?,
    };

    let Some(cache_dir) = args.cache_dir.or_else(default_isopy_dir) else {
        bail!("Could not infer isopy cache directory location: please specify using --dir option")
    };

    let Some(repo)= RepoConfig::default(&cache_dir, None).repo()? else{
        bail!("Could not get repository")
    };

    let app = App::new(cwd, repo);
    match args.command {
        Command::Available => do_available(&app).await,
        Command::Download(python_version) => do_download(&app, &python_version).await,
        Command::Downloaded => do_downloaded(&app),
        Command::Exec { program, args } => do_exec(&app, &program, &args),
        Command::GenConfig(python_version) => do_gen_config(&app, &python_version).await,
        Command::Info => do_info(&app),
        Command::Init(python_version) => do_init(&app, &python_version).await,
        Command::InitConfig => do_init_config(&app).await,
        Command::Link { meta_id } => do_link(&app, &meta_id),
        Command::List => do_list(&app).await,
        Command::Shell => do_shell(&app),
        Command::Wrap {
            wrapper_path,
            script_path,
            base_dir,
        } => do_wrap(&app, &wrapper_path, &script_path, &base_dir),
    }
}
