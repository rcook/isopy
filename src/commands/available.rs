use crate::config::Config;
use crate::error::Result;
use crate::object_model::AssetFilter;

pub fn do_available(config: &Config) -> Result<()> {
    let asset_names = config.read_asset_names()?;
    for asset_name in
        AssetFilter::default_for_platform().filter(asset_names.iter().map(|x| x).into_iter())
    {
        println!("{}", asset_name.name)
    }
    Ok(())
}
