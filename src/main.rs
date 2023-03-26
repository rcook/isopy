mod error;

use crate::error::{could_not_get_isopy_dir, Error, Result};
use clap::Parser;
use colour::red_ln;
use serde_json::{from_str, Value};
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct IsopyArgs {
    dir: Option<PathBuf>,
}

#[derive(Debug)]
struct IsopyConfig {
    dir: PathBuf,
}

impl IsopyConfig {
    fn from_dir(dir: PathBuf) -> Self {
        Self { dir: dir }
    }
}

fn default_isopy_dir() -> Option<PathBuf> {
    let home_dir = home::home_dir()?;
    let isopy_dir = home_dir.join(".isopy");
    Some(isopy_dir)
}

fn main_inner() -> Result<()> {
    /*
    let p = current_dir()?;
    */
    let isopy_args = IsopyArgs::parse();
    let isopy_dir = isopy_args
        .dir
        .or_else(default_isopy_dir)
        .ok_or_else(|| could_not_get_isopy_dir("Could not find .isopy directory"))?;
    let isopy_config = IsopyConfig::from_dir(isopy_dir);
    let assets_dir = isopy_config.dir.join("assets");
    let index_path = assets_dir.join("index.json");
    let index_json = read_to_string(index_path)?;
    let index_vec = from_str::<Vec<Value>>(&index_json)?;
    let first_value = &index_vec[0];
    for key in first_value.as_object().unwrap().keys() {
        println!("{:?}", key);
    }
    Ok(())
}

fn main() {
    match main_inner() {
        Err(Error::Reportable(msg, _)) => red_ln!("{}", msg),
        _ => {}
    }
}
