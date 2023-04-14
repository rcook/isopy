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
mod app;
mod cli;
mod commands;
mod object_model;
mod repository;
mod serialization;
mod shell;
mod util;

use crate::app::App;
use crate::cli::{Args, Command};
use crate::commands::{
    do_available, do_create, do_download, do_downloaded, do_exec, do_generate_repositories_yaml,
    do_info, do_init, do_list, do_new, do_scratch, do_shell, do_use, do_wrap,
};
use crate::util::{default_isopy_dir, ERROR, OK};
use anyhow::{anyhow, Result};
use clap::Parser;
use colored::Colorize;
use std::env::current_dir;
use std::process::exit;

async fn run() -> Result<()> {
    let cwd = current_dir()?;
    let args = Args::parse();
    let dir = args.dir.or_else(default_isopy_dir).ok_or(anyhow!(
        "Could not infer isopy cache directory location: please specify using --dir option"
    ))?;
    let app = App::new(cwd, dir);

    match args.command {
        Command::Available => do_available(&app).await?,
        Command::Create {
            environment_name,
            version,
            tag,
        } => do_create(&app, &environment_name, &version, &tag).await?,
        Command::Download { version, tag } => do_download(&app, &version, &tag).await?,
        Command::Downloaded => do_downloaded(&app)?,
        Command::Exec {
            environment_name,
            program,
            args,
        } => do_exec(&app, environment_name.as_ref(), &program, args)?,
        Command::GenerateRepositoriesYaml {
            local_repository_dir,
        } => do_generate_repositories_yaml(&app, local_repository_dir)?,
        Command::Info => do_info(&app)?,
        Command::Init => do_init(&app).await?,
        Command::List => do_list(&app).await?,
        Command::New { version, tag } => do_new(&app, &version, &tag)?,
        Command::Scratch => do_scratch(&app).await?,
        Command::Shell { environment_name } => do_shell(&app, environment_name.as_ref())?,
        Command::Use { environment_name } => do_use(&app, &environment_name)?,
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
