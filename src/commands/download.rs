use super::helpers::{download_asset, get_asset};
use crate::app::App;
use crate::error::Result;
use crate::object_model::{Tag, Version};

pub async fn do_download(app: &App, version: &Version, tag: &Option<Tag>) -> Result<()> {
    let assets = app.read_assets()?;
    let asset = get_asset(&assets, &version, &tag)?;
    download_asset(app, asset).await?;
    Ok(())
}
