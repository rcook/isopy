use crate::config::Config;
use crate::error::{user, Result};
use crate::object_model::{AssetFilter, Tag, Version};

pub fn do_download(config: &Config, version: &Version, tag: &Option<Tag>) -> Result<()> {
    let asset_names = config.read_asset_names()?;
    let mut asset_filter = AssetFilter::default_for_platform();
    asset_filter.version = Some(version.clone());
    asset_filter.tag = tag.clone();
    let matching_asset_names = asset_filter.filter(asset_names.iter().map(|x| x).into_iter());
    let asset_name = match matching_asset_names.len() {
        1 => matching_asset_names.first().expect("Must exist"),
        0 => {
            return Err(user(format!(
                "No asset matching version {} and tag {}",
                version,
                tag.as_ref()
                    .map(Tag::to_string)
                    .unwrap_or(String::from("(none)"))
            )))
        }
        _ => {
            return Err(user(format!(
                "More than one asset matching version {} and tag {}",
                version,
                tag.as_ref()
                    .map(Tag::to_string)
                    .unwrap_or(String::from("(none)"))
            )))
        }
    };
    println!("asset_name={:?}", asset_name);
    todo!()
    /*
    let response = get("https://httpbin.org/ip")?.json::<HttpBinIPResponse>()?;
    println!("{:#?}", response);
    */
}
