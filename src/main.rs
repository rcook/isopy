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
#![allow(unused)]
mod app;
mod cli;
mod commands;
mod object_model;
mod python_info;
mod repository;
mod serialization;
mod shell;
mod util;

use crate::app::App;
use crate::cli::{Args, Command};
use crate::commands::{
    do_available, do_download, do_downloaded, do_exec, do_info, do_init, do_link, do_list,
    do_shell, do_wrap,
};
use crate::python_info::PythonInfo;
use crate::util::{default_isopy_dir, ERROR, OK};
use anyhow::{bail, Result};
use clap::Parser;
use colored::Colorize;
use joat_repo::RepoConfig;
use std::env::current_dir;
use std::process::exit;

async fn run() -> Result<()> {
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

    let app = App::new(cwd, cache_dir, repo);
    match args.command {
        Command::Available => do_available(&app).await?,
        Command::Download { version, tag } => do_download(&app, &version, &tag).await?,
        Command::Downloaded => do_downloaded(&app)?,
        Command::Exec {
            environment_name,
            program,
            args,
        } => do_exec(&app, environment_name.as_ref(), &program, args)?,
        Command::Info => do_info(&app)?,
        Command::Init { version, tag } => do_init(&app, PythonInfo::new(version, tag)).await?,
        Command::Link { meta_id } => do_link(&app, &meta_id)?,
        Command::List => do_list(&app).await?,
        Command::Shell => do_shell(&app)?,
        Command::Wrap {
            wrapper_path,
            script_path,
            base_dir,
        } => do_wrap(&app, &wrapper_path, &script_path, &base_dir)?,
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    exit(match run().await {
        Ok(_) => OK,
        Err(e) => {
            println!("{}", format!("{}", e).bright_red());
            ERROR
        }
    })
}
