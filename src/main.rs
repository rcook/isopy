mod cli;
mod commands;
mod config;
mod error;
mod object_model;
mod serialization;
mod version;

use crate::cli::{Args, Command};
use crate::commands::{do_available, do_download, do_downloaded};
use crate::config::Config;
use crate::error::{could_not_get_isopy_dir, Error, Result};
use clap::Parser;
use colour::red_ln;
use std::path::PathBuf;

fn default_isopy_dir() -> Option<PathBuf> {
    let home_dir = home::home_dir()?;
    let isopy_dir = home_dir.join(".isopy");
    Some(isopy_dir)
}

fn main_inner() -> Result<()> {
    let args = Args::parse();
    let dir = args
        .dir
        .or_else(default_isopy_dir)
        .ok_or_else(|| could_not_get_isopy_dir("Could not find .isopy directory"))?;
    let config = Config::from_dir(dir);

    match args.command {
        Command::Available => do_available(&config)?,
        Command::Download => do_download(&config)?,
        Command::Downloaded => do_downloaded(&config)?,
    }

    Ok(())
}

fn main() {
    match main_inner() {
        Err(Error::Reportable(msg, _)) => red_ln!("{}", msg),
        _ => {}
    }
}
