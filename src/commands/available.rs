use crate::config::Config;
use crate::error::Result;
use crate::object_model::AssetFilter;

pub fn do_available(config: &Config) -> Result<()> {
    let asset_infos = config.read_asset_infos()?;
    for asset in
        AssetFilter::default_for_platform().filter(asset_infos.iter().map(|x| x).into_iter())
    {
        println!("{:?}", asset)
    }
    Ok(())
}
