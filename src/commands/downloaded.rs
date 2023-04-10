use crate::app::App;
use crate::object_model::AssetMeta;
use crate::result::Result;
use std::fs::read_dir;

pub fn do_downloaded(app: &App) -> Result<()> {
    for e in read_dir(&app.assets_dir)? {
        let temp = e?;
        let asset_name = String::from(temp.file_name().to_str().expect("Must be a valid string"));
        if AssetMeta::parse(&asset_name).is_some() {
            println!("{}", asset_name)
        }
    }
    Ok(())
}
