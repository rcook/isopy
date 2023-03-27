use crate::config::Config;
use crate::error::{user, Result};
use crate::object_model::AssetFilter;
use crate::version::Version;

pub fn do_download(config: &Config, version: &Version, tag_str: &Option<String>) -> Result<()> {
    let asset_infos = config.read_asset_infos()?;
    let mut asset_filter = AssetFilter::default_for_platform();
    asset_filter.version = Some(version.clone());
    asset_filter.tag_str = tag_str.clone();
    let matching_assets = asset_filter.filter(asset_infos.iter().map(|x| x).into_iter());
    let asset = match matching_assets.len() {
        0 => {
            return Err(user(format!(
                "No asset matching version {} and tag {}",
                version,
                tag_str.as_ref().unwrap_or(&String::from("(none)"))
            )))
        }
        1 => matching_assets.first(),
        _ => {
            return Err(user(format!(
                "More than one asset matching version {} and tag {}",
                version,
                tag_str.as_ref().unwrap_or(&String::from("(none)"))
            )))
        }
    };
    println!("asset={:?}", asset);
    todo!()
    /*
    let response = get("https://httpbin.org/ip")?.json::<HttpBinIPResponse>()?;
    println!("{:#?}", response);
    */
}
