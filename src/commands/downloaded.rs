use crate::config::Config;
use crate::error::Result;
use crate::object_model::AssetMeta;
use std::fs::read_dir;

pub fn do_downloaded(config: &Config) -> Result<()> {
    for e in read_dir(&config.assets_dir)? {
        let temp = e?;
        let asset_name = String::from(temp.file_name().to_str().expect("Must be a valid string"));
        if let Some(_) = AssetMeta::parse(&asset_name) {
            println!("{}", asset_name)
        }
    }
    Ok(())
}
