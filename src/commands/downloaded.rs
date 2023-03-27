use crate::config::Config;
use crate::error::Result;
use crate::object_model::AssetMeta;
use std::fs::read_dir;

pub fn do_downloaded(config: &Config) -> Result<()> {
    let assets_dir = config.dir.join("assets");
    for e in read_dir(assets_dir)? {
        let temp = e?;
        if let Some(asset_meta) =
            AssetMeta::parse(temp.file_name().to_str().expect("Must be a valid string"))
        {
            println!("{}", asset_meta.name)
        }
    }
    Ok(())
}
