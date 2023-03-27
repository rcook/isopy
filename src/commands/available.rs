use crate::config::Config;
use crate::error::Result;
use crate::object_model::AssetFilter;

pub fn do_available(config: &Config) -> Result<()> {
    let assets = config.read_assets()?;
    for asset in AssetFilter::default_for_platform().filter(assets.iter().map(|x| x).into_iter()) {
        println!("{}", asset.name)
    }
    Ok(())
}
