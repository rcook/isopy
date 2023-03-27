use crate::config::Config;
use crate::error::Result;
use crate::object_model::AssetFilter;

pub fn do_available(config: &Config) -> Result<()> {
    let asset_metas = config.read_asset_metas()?;
    for asset_meta in
        AssetFilter::default_for_platform().filter(asset_metas.iter().map(|x| x).into_iter())
    {
        println!("{}", asset_meta.name)
    }
    Ok(())
}
