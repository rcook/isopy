use super::asset_info::AssetInfo;
use super::attributes::{
    Arch, ArchiveType, Family, Flavour, Platform, Subflavour, Tag, Variant, OS,
};
use crate::version::Version;
use std::iter::Iterator;

#[allow(unused)]
pub struct AssetFilter {
    pub archive_type: Option<ArchiveType>,
    pub family: Option<Family>,
    pub version: Option<Version>,
    pub tag: Option<Tag>,
    pub arch: Option<Arch>,
    pub platform: Option<Platform>,
    pub os: Option<OS>,
    pub flavour: Option<Flavour>,
    pub subflavour0: Option<Subflavour>,
    pub subflavour1: Option<Subflavour>,
    pub variant: Option<Variant>,
}

impl AssetFilter {
    #[allow(unused)]
    fn default() -> Self {
        Self {
            archive_type: None,
            family: None,
            version: None,
            tag: None,
            arch: None,
            platform: None,
            os: None,
            flavour: None,
            subflavour0: None,
            subflavour1: None,
            variant: None,
        }
    }

    #[allow(unused)]
    fn filter<'a, A>(&self, asset_infos: A) -> Vec<&'a AssetInfo>
    where
        A: IntoIterator<Item = &'a AssetInfo>,
    {
        fn predicate(this: &AssetFilter, item: &&AssetInfo) -> bool {
            if !this
                .archive_type
                .as_ref()
                .map(|x| item.archive_type == *x)
                .unwrap_or(true)
            {
                return false;
            }
            true
        }

        asset_infos
            .into_iter()
            .filter(|x| predicate(self, x))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::super::asset_info::AssetInfo;
    use super::super::attributes::{
        Arch, ArchiveType, Family, Flavour, Platform, Subflavour, Tag, Variant, OS,
    };
    use super::AssetFilter;
    use crate::version::Version;
    use rstest::rstest;

    #[test]
    fn test_basics() {
        let (a0, a1, a2) = make_test_artifacts();
        let mut asset_filter = AssetFilter::default();
        asset_filter.archive_type = Some(ArchiveType::TarGZ);
        assert_eq!(vec![&a1], asset_filter.filter(vec![&a0, &a1, &a2]))
    }

    fn make_test_artifacts() -> (AssetInfo, AssetInfo, AssetInfo) {
        let a0 = AssetInfo {
            name: String::from("cpython-3.10.9+20230116-aarch64-apple-darwin-debug-full.tar.zst"),
            archive_type: ArchiveType::TarZST,
            family: Family::CPython,
            version: Version::new(3, 10, 9),
            tag: Tag::NewStyle(String::from("20230116")),
            arch: Arch::AArch64,
            platform: Platform::Apple,
            os: OS::Darwin,
            flavour: None,
            subflavour0: Some(Subflavour::Debug),
            subflavour1: None,
            variant: Some(Variant::Full),
        };
        let a1 = AssetInfo {
            name: String::from("cpython-3.10.9+20230116-aarch64-apple-darwin-install_only.tar.gz"),
            archive_type: ArchiveType::TarGZ,
            family: Family::CPython,
            version: Version::new(3, 10, 9),
            tag: Tag::NewStyle(String::from("20230116")),
            arch: Arch::AArch64,
            platform: Platform::Apple,
            os: OS::Darwin,
            flavour: None,
            subflavour0: None,
            subflavour1: None,
            variant: Some(Variant::InstallOnly),
        };
        let a2 = AssetInfo {
            name: String::from("cpython-3.10.2-aarch64-apple-darwin-debug-20220220T1113.tar.zst"),
            archive_type: ArchiveType::TarZST,
            family: Family::CPython,
            version: Version::new(3, 10, 2),
            tag: Tag::OldStyle(String::from("20220220T1113")),
            arch: Arch::AArch64,
            platform: Platform::Apple,
            os: OS::Darwin,
            flavour: None,
            subflavour0: Some(Subflavour::Debug),
            subflavour1: None,
            variant: None,
        };
        (a0, a1, a2)
    }
}
