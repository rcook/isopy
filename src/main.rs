#![allow(unused_imports)]
mod cli;
mod config;
mod error;
mod object_model;
mod serialization;
mod version;

use crate::cli::Args;
use crate::config::Config;
use crate::error::{could_not_get_isopy_dir, Error, Result};
use crate::object_model::{Arch, ArchiveType, AssetInfo, Variant, OS};
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

    for package in packages {
        for asset in package.assets {
            if AssetInfo::definitely_not_an_asset(&asset.name) {
                continue;
            }
            let asset_info = AssetInfo::from_asset_name(&asset.name).expect("Should parse");
            if asset_info.archive_type == ArchiveType::TarGZ
                && asset_info.os == OS::Linux
                && asset_info.arch == Arch::X86_64
                && asset_info.variant == Some(Variant::InstallOnly)
            {
                println!("{:?}", asset_info);
            }
        }
    }

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
