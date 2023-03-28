use crate::app::App;
use crate::error::Result;
use crate::object_model::AssetFilter;

pub fn do_available(app: &App) -> Result<()> {
    let assets = app.read_assets()?;
    for asset in AssetFilter::default_for_platform().filter(assets.iter().map(|x| x).into_iter()) {
        println!("{}", asset.name)
    }
    Ok(())
}
