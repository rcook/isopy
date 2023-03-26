#![allow(unused_imports)]
mod cli;
mod config;
mod error;
mod parsing;
mod serialization;
mod version;

use crate::cli::Args;
use crate::config::Config;
use crate::error::{could_not_get_isopy_dir, Error, Result};
use crate::parsing::AssetInfo;
use crate::serialization::{HttpBinIPResponse, Package};
use clap::Parser;
use colour::red_ln;
use reqwest::blocking::get;
use serde::Deserialize;
use serde_json::{from_str, Value};
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::PathBuf;

fn default_isopy_dir() -> Option<PathBuf> {
    let home_dir = home::home_dir()?;
    let isopy_dir = home_dir.join(".isopy");
    Some(isopy_dir)
}

fn main_inner() -> Result<()> {
    /*
    let p = current_dir()?;
    */
    let isopy_args = Args::parse();
    let isopy_dir = isopy_args
        .dir
        .or_else(default_isopy_dir)
        .ok_or_else(|| could_not_get_isopy_dir("Could not find .isopy directory"))?;
    let isopy_config = Config::from_dir(isopy_dir);
    let assets_dir = isopy_config.dir.join("assets");
    let index_path = assets_dir.join("index.json");
    let index_json = read_to_string(index_path)?;
    let packages = from_str::<Vec<Package>>(&index_json)?;

    let mut set = HashSet::new();

    for package in packages {
        for asset in package.assets {
            match AssetInfo::from_asset_name(&asset.name) {
                Some(asset_info) => {
                    _ = set.insert(asset_info.flavour);
                }
                _ => {}
            }
        }
    }

    println!("set={:?}", set);

    /*
    let response = get("https://httpbin.org/ip")?.json::<HttpBinIPResponse>()?;
    println!("{:#?}", response);
    */

    Ok(())
}

fn main() {
    match main_inner() {
        Err(Error::Reportable(msg, _)) => red_ln!("{}", msg),
        _ => {}
    }
}
