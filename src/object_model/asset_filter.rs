use super::{
    Arch, ArchiveType, Asset, Family, Flavour, Platform, Subflavour, Tag, Variant, Version, OS,
};
use std::iter::Iterator;

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
    pub fn all() -> Self {
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

    #[cfg(target_os = "linux")]
    pub fn default_for_platform() -> Self {
        Self {
            archive_type: Some(ArchiveType::TarGZ),
            family: Some(Family::CPython),
            version: None,
            tag: None,
            arch: Some(Arch::X86_64),
            platform: Some(Platform::Unknown),
            os: Some(OS::Linux),
            flavour: Some(Flavour::GNU),
            subflavour0: None,
            subflavour1: None,
            variant: Some(Variant::InstallOnly),
        }
    }

    #[cfg(target_os = "macos")]
    pub fn default_for_platform() -> Self {
        Self {
            archive_type: Some(ArchiveType::TarGZ),
            family: Some(Family::CPython),
            version: None,
            tag: None,
            arch: Some(Arch::X86_64),
            platform: Some(Platform::Apple),
            os: Some(OS::Darwin),
            flavour: None,
            subflavour0: None,
            subflavour1: None,
            variant: Some(Variant::InstallOnly),
        }
    }

    #[cfg(target_os = "windows")]
    pub fn default_for_platform() -> Self {
        Self {
            archive_type: Some(ArchiveType::TarGZ),
            family: Some(Family::CPython),
            version: None,
            tag: None,
            arch: Some(Arch::X86_64),
            platform: Some(Platform::PC),
            os: Some(OS::Windows),
            flavour: Some(Flavour::MSVC),
            subflavour0: Some(Subflavour::Shared),
            subflavour1: None,
            variant: Some(Variant::InstallOnly),
        }
    }

    pub fn filter<'a, A>(&self, assets: A) -> Vec<&'a Asset>
    where
        A: IntoIterator<Item = &'a Asset>,
    {
        fn predicate(this: &AssetFilter, asset: &Asset) -> bool {
            if !this
                .archive_type
                .as_ref()
                .map(|x| asset.meta.archive_type == *x)
                .unwrap_or(true)
            {
                return false;
            }

            if !this
                .family
                .as_ref()
                .map(|x| asset.meta.family == *x)
                .unwrap_or(true)
            {
                return false;
            }

            if !this
                .version
                .as_ref()
                .map(|x| asset.meta.version == *x)
                .unwrap_or(true)
            {
                return false;
            }

            if !this.tag.as_ref().map(|x| asset.tag == *x).unwrap_or(true) {
                return false;
            }

            if !this
                .arch
                .as_ref()
                .map(|x| asset.meta.arch == *x)
                .unwrap_or(true)
            {
                return false;
            }

            if !this
                .platform
                .as_ref()
                .map(|x| asset.meta.platform == *x)
                .unwrap_or(true)
            {
                return false;
            }

            if !this
                .os
                .as_ref()
                .map(|x| asset.meta.os == *x)
                .unwrap_or(true)
            {
                return false;
            }

            if !this
                .flavour
                .as_ref()
                .map(|x| asset.meta.flavour.as_ref() == Some(x))
                .unwrap_or(true)
            {
                return false;
            }

            if !this
                .subflavour0
                .as_ref()
                .map(|x| asset.meta.subflavour0.as_ref() == Some(x))
                .unwrap_or(true)
            {
                return false;
            }

            if !this
                .subflavour1
                .as_ref()
                .map(|x| asset.meta.subflavour1.as_ref() == Some(x))
                .unwrap_or(true)
            {
                return false;
            }

            if !this
                .variant
                .as_ref()
                .map(|x| asset.meta.variant.as_ref() == Some(x))
                .unwrap_or(true)
            {
                return false;
            }

            true
        }

        assets.into_iter().filter(|x| predicate(self, &x)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::super::{ArchiveType, Asset, AssetFilter, AssetMeta, Tag};
    use reqwest::Url;

    #[test]
    fn test_basics() {
        let (a0, a1, a2, a3) = make_test_artifacts();

        assert_eq!(
            vec![&a0, &a1, &a2, &a3],
            AssetFilter::all().filter(vec![&a0, &a1, &a2, &a3])
        );

        let mut asset_filter = AssetFilter::all();
        asset_filter.archive_type = Some(ArchiveType::TarGZ);
        assert_eq!(vec![&a1], asset_filter.filter(vec![&a0, &a1, &a2]));

        let mut asset_filter = AssetFilter::all();
        asset_filter.archive_type = Some(ArchiveType::TarZST);
        assert_eq!(vec![&a0, &a2], asset_filter.filter(vec![&a0, &a1, &a2]));

        let mut asset_filter = AssetFilter::all();
        asset_filter.tag = Some(Tag::parse("20210724T1424"));
        assert_eq!(vec![&a3], asset_filter.filter(vec![&a0, &a1, &a2, &a3]))
    }

    fn make_test_artifacts() -> (Asset, Asset, Asset, Asset) {
        let a0 = Asset {
            name: String::from(""),
            tag: Tag::parse("tag"),
            url: Url::parse("https://httpbin.org").expect("Must parse"),
            size: 0,
            meta: AssetMeta::parse(
                "cpython-3.10.9+20230116-aarch64-apple-darwin-debug-full.tar.zst",
            )
            .expect("Should parse"),
        };
        let a1 = Asset {
            name: String::from(""),
            tag: Tag::parse("tag"),
            url: Url::parse("https://httpbin.org").expect("Must parse"),
            size: 0,
            meta: AssetMeta::parse(
                "cpython-3.10.9+20230116-aarch64-apple-darwin-install_only.tar.gz",
            )
            .expect("Should parse"),
        };
        let a2 = Asset {
            name: String::from(""),
            tag: Tag::parse("tag"),
            url: Url::parse("https://httpbin.org").expect("Must parse"),
            size: 0,
            meta: AssetMeta::parse(
                "cpython-3.10.2-aarch64-apple-darwin-debug-20220220T1113.tar.zst",
            )
            .expect("Should parse"),
        };
        let a3 = Asset {
            name: String::from(""),
            tag: Tag::parse("20210724T1424"),
            url: Url::parse("https://httpbin.org").expect("Must parse"),
            size: 0,
            meta: AssetMeta::parse(
                "cpython-3.9.6-x86_64-unknown-linux-gnu-install_only-20210724T1424.tar.gz",
            )
            .expect("Should parse"),
        };
        (a0, a1, a2, a3)
    }
}
