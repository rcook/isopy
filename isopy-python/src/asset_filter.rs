// Copyright (c) 2023 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use crate::asset::Asset;
use crate::python_standalone_builds::api::{
    Arch, ArchiveType, Family, Flavour, Platform, Subflavour, Variant, OS,
};
use crate::python_version::PythonVersion;
use crate::tag::Tag;
use std::iter::Iterator;

pub struct AssetFilter {
    pub archive_type: Option<ArchiveType>,
    pub family: Option<Family>,
    pub version: Option<PythonVersion>,
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
    #[cfg(target_os = "linux")]
    #[must_use]
    pub const fn default_for_platform() -> Self {
        Self {
            archive_type: Some(ArchiveType::TarGZ),
            family: Some(Family::CPython),
            version: None,
            tag: None,
            #[cfg(target_arch = "aarch64")]
            arch: Some(Arch::AArch64),
            #[cfg(target_arch = "x86_64")]
            arch: Some(Arch::X86_64),
            platform: Some(Platform::Unknown),
            os: Some(OS::Linux),
            flavour: Some(Flavour::Gnu),
            subflavour0: None,
            subflavour1: None,
            variant: Some(Variant::InstallOnly),
        }
    }

    #[cfg(target_os = "macos")]
    pub const fn default_for_platform() -> Self {
        Self {
            archive_type: Some(ArchiveType::TarGZ),
            family: Some(Family::CPython),
            version: None,
            tag: None,
            #[cfg(target_arch = "aarch64")]
            arch: Some(Arch::AArch64),
            #[cfg(target_arch = "x86_64")]
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
    pub const fn default_for_platform() -> Self {
        Self {
            archive_type: Some(ArchiveType::TarGZ),
            family: Some(Family::CPython),
            version: None,
            tag: None,
            #[cfg(target_arch = "aarch64")]
            arch: Some(Arch::AArch64),
            #[cfg(target_arch = "x86_64")]
            arch: Some(Arch::X86_64),
            platform: Some(Platform::Pc),
            os: Some(OS::Windows),
            flavour: Some(Flavour::Msvc),
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
                .map_or(true, |x| asset.meta.archive_type == *x)
            {
                return false;
            }

            if !this
                .family
                .as_ref()
                .map_or(true, |x| asset.meta.family == *x)
            {
                return false;
            }

            if !this
                .version
                .as_ref()
                .map_or(true, |x| asset.meta.version == *x)
            {
                return false;
            }

            if !this.tag.as_ref().map_or(true, |x| asset.tag == *x) {
                return false;
            }

            if !this.arch.as_ref().map_or(true, |x| asset.meta.arch == *x) {
                return false;
            }

            if !this
                .platform
                .as_ref()
                .map_or(true, |x| asset.meta.platform == *x)
            {
                return false;
            }

            if !this.os.as_ref().map_or(true, |x| asset.meta.os == *x) {
                return false;
            }

            if !this
                .flavour
                .as_ref()
                .map_or(true, |x| asset.meta.flavour.as_ref() == Some(x))
            {
                return false;
            }

            if !this
                .subflavour0
                .as_ref()
                .map_or(true, |x| asset.meta.subflavour0.as_ref() == Some(x))
            {
                return false;
            }

            if !this
                .subflavour1
                .as_ref()
                .map_or(true, |x| asset.meta.subflavour1.as_ref() == Some(x))
            {
                return false;
            }

            if !this
                .variant
                .as_ref()
                .map_or(true, |x| asset.meta.variant.as_ref() == Some(x))
            {
                return false;
            }

            true
        }

        assets.into_iter().filter(|x| predicate(self, x)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::AssetFilter;
    use crate::asset::Asset;
    use crate::asset_meta::AssetMeta;
    use crate::python_standalone_builds::api::ArchiveType;
    use crate::tag::Tag;
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
        asset_filter.tag = Some(
            "20210724T1424"
                .parse::<Tag>()
                .expect("test: must be valid tag"),
        );
        assert_eq!(vec![&a3], asset_filter.filter(vec![&a0, &a1, &a2, &a3]));
    }

    fn make_test_artifacts() -> (Asset, Asset, Asset, Asset) {
        let a0 = Asset {
            name: String::new(),
            tag: "tag".parse::<Tag>().expect("test: must be valid tag"),
            url: Url::parse("https://httpbin.org").expect("test: must be valid URL"),
            size: 0,
            meta: "cpython-3.10.9+20230116-aarch64-apple-darwin-debug-full.tar.zst"
                .parse::<AssetMeta>()
                .expect("test: must be valid asset name"),
        };
        let a1 = Asset {
            name: String::new(),
            tag: "tag".parse::<Tag>().expect("test: must be valid tag"),
            url: Url::parse("https://httpbin.org").expect("test: must be valid URL"),
            size: 0,
            meta: "cpython-3.10.9+20230116-aarch64-apple-darwin-install_only.tar.gz"
                .parse::<AssetMeta>()
                .expect("test: must be valid asset name"),
        };
        let a2 = Asset {
            name: String::new(),
            tag: "tag".parse::<Tag>().expect("test: must be valid tag"),
            url: Url::parse("https://httpbin.org").expect("test: must be valid URL"),
            size: 0,
            meta: "cpython-3.10.2-aarch64-apple-darwin-debug-20220220T1113.tar.zst"
                .parse::<AssetMeta>()
                .expect("test: must be valid asset name"),
        };
        let a3 = Asset {
            name: String::new(),
            tag: "20210724T1424"
                .parse::<Tag>()
                .expect("test: must be valid tag"),
            url: Url::parse("https://httpbin.org").expect("test: must be valid URL"),
            size: 0,
            meta: "cpython-3.9.6-x86_64-unknown-linux-gnu-install_only-20210724T1424.tar.gz"
                .parse::<AssetMeta>()
                .expect("test: must be valid asset name"),
        };
        (a0, a1, a2, a3)
    }
}
