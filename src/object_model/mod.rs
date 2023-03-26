mod asset_info;
mod attributes;

pub use self::asset_info::AssetInfo;
pub use self::attributes::{
    Arch, ArchiveType, Family, Flavour, Platform, Subflavour, Tag, Variant, OS,
};
