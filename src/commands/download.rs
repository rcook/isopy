use crate::app::App;
use crate::object_model::{Tag, Version};
use crate::result::Result;
use crate::util::{download_asset, get_asset};

pub async fn do_download(app: &App, version: &Version, tag: &Option<Tag>) -> Result<()> {
    let assets = app.read_assets()?;
    let asset = get_asset(&assets, version, tag)?;
    download_asset(app, asset).await?;
    Ok(())
}
