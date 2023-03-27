mod cli;
mod commands;
mod config;
mod error;
mod object_model;
mod serialization;
mod util;

use crate::cli::{Args, Command};
use crate::commands::{do_available, do_download, do_downloaded, do_list};
use crate::config::Config;
use crate::error::{could_not_get_isopy_dir, Error, Result};
use clap::Parser;
use colour::red_ln;
use std::path::PathBuf;
use std::process::exit;
use tokio;

fn default_isopy_dir() -> Option<PathBuf> {
    let home_dir = home::home_dir()?;
    let isopy_dir = home_dir.join(".isopy");
    Some(isopy_dir)
}

async fn main_inner() -> Result<()> {
    let args = Args::parse();
    let dir = args
        .dir
        .or_else(default_isopy_dir)
        .ok_or_else(|| could_not_get_isopy_dir("Could not find .isopy directory"))?;
    let config = Config::from_dir(dir);

    match args.command {
        Command::Available => do_available(&config)?,
        Command::Download { version, tag } => do_download(&config, &version, &tag).await?,
        Command::Downloaded => do_downloaded(&config)?,
        Command::List => do_list(&config).await?,
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
            println!("e={:?}", e);
            exitcode::USAGE
        }
    })
}
