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
use crate::object_model::{
    Arch, ArchiveType, AssetFilter, AssetInfo, Family, Flavour, Platform, Tag, Variant, OS,
};
use crate::serialization::{HttpBinIPResponse, Package};
use crate::version::Version;
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

    let mut asset_infos = Vec::new();
    for package in packages {
        for asset in package.assets {
            if !AssetInfo::definitely_not_an_asset(&asset.name) {
                asset_infos.push(AssetInfo::from_asset_name(&asset.name).expect("Should parse"));
            }
        }
    }
    println!("count={}", asset_infos.len());

    let mut asset_filter = AssetFilter::default();
    asset_filter.archive_type = Some(ArchiveType::TarGZ);
    asset_filter.family = Some(Family::CPython);
    asset_filter.version = Some(Version::new(3, 11, 1));
    asset_filter.tag = Some(Tag::NewStyle(String::from("20230116")));
    asset_filter.arch = Some(Arch::X86_64);
    asset_filter.platform = Some(Platform::Unknown);
    asset_filter.os = Some(OS::Linux);
    asset_filter.flavour = Some(Flavour::GNU);
    asset_filter.variant = Some(Variant::InstallOnly);

    let filtered_asset_infos = asset_filter.filter(asset_infos.iter().map(|x| x).into_iter());
    println!("filtered_count={}", filtered_asset_infos.len());
    for a in filtered_asset_infos {
        println!("a={:?}", a)
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
