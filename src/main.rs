mod app;
mod cli;
mod commands;
mod error;
mod object_model;
mod serialization;
mod shell;
mod ui;
mod util;

use crate::app::App;
use crate::cli::{Args, Command};
use crate::commands::{
    do_available, do_create, do_download, do_downloaded, do_exec, do_info, do_init, do_list,
    do_new, do_shell, do_use,
};
use crate::error::{could_not_get_isopy_dir, Error, Result};
use clap::Parser;
use colour::red_ln;
use commands::do_scratch;
use std::env::current_dir;
use std::path::PathBuf;
use std::process::exit;
use tokio;

fn default_isopy_dir() -> Option<PathBuf> {
    let home_dir = home::home_dir()?;
    let isopy_dir = home_dir.join(".isopy");
    Some(isopy_dir)
}

async fn main_inner() -> Result<()> {
    let cwd = current_dir()?;
    let args = Args::parse();
    let dir = args
        .dir
        .or_else(default_isopy_dir)
        .ok_or_else(|| could_not_get_isopy_dir("Could not find .isopy directory"))?;
    let app = App::new(cwd, dir);

    match args.command {
        Command::Available => do_available(&app).await?,
        Command::Create {
            env_name,
            version,
            tag,
        } => do_create(&app, &env_name, &version, &tag).await?,
        Command::Download { version, tag } => do_download(&app, &version, &tag).await?,
        Command::Downloaded => do_downloaded(&app)?,
        Command::Exec { env_name } => do_exec(&app, &env_name)?,
        Command::Info => do_info(&app)?,
        Command::Init => do_init(&app)?,
        Command::List => do_list(&app).await?,
        Command::New { version, tag } => do_new(&app, &version, &tag)?,
        Command::Scratch => do_scratch(&app)?,
        Command::Shell { env_name } => do_shell(&app, &env_name)?,
        Command::Use => do_use(&app)?,
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    exit(match main_inner().await {
        Ok(_) => exitcode::OK,
        Err(Error::Reportable(msg, _)) => {
            red_ln!("{}", msg);
            exitcode::USAGE
        }
        Err(Error::User(msg)) => {
            red_ln!("{}", msg);
            exitcode::USAGE
        }
        e => {
            red_ln!("Unhandled error: {:#?}", e);
            exitcode::USAGE
        }
    })
}
