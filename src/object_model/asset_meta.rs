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
use super::{Arch, ArchiveType, Family, Flavour, Platform, Subflavour, Tag, Variant, Version, OS};
use anyhow::{bail, Result};

#[derive(Debug, PartialEq)]
pub struct AssetMeta {
    pub archive_type: ArchiveType,
    pub family: Family,
    pub version: Version,
    pub arch: Arch,
    pub platform: Platform,
    pub os: OS,
    pub flavour: Option<Flavour>,
    pub subflavour0: Option<Subflavour>,
    pub subflavour1: Option<Subflavour>,
    pub variant: Option<Variant>,
    pub parsed_tag: Tag,
}

impl AssetMeta {
    pub fn definitely_not_an_asset_name(s: &str) -> bool {
        if "libuuid-1.0.3.tar.gz" == s || "SHA256SUMS" == s {
            true
        } else {
            s.ends_with(".sha256")
        }
    }

    pub fn parse(s: &str) -> Result<Self> {
        fn wrap<'a>(s: &str, label: &str, result: Option<&'a str>) -> Result<&'a str> {
            match result {
                Some(value) => Ok(value),
                None => bail!("Missing {} from asset name \"{}\"", label, s),
            }
        }

        fn parse_version_and_tag_opt(s: &str) -> Result<(Version, Option<Tag>)> {
            let parts = s.split('+').collect::<Vec<_>>();
            let (version_str, tag) = match parts.len() {
                1 => (parts[0], None),
                2 => (parts[0], Some(Tag::parse(parts[1]))),
                _ => bail!("Invalid version/tag \"{}\"", s),
            };

            let version = Version::parse(version_str)?;
            Ok((version, tag))
        }

        let (archive_type, base_name) = ArchiveType::parse(s)?;

        let mut iter = base_name.split('-');

        let family = Family::parse(wrap(s, "family", iter.next())?)?;
        let (version, mut tag_opt) =
            parse_version_and_tag_opt(wrap(s, "version/tag", iter.next())?)?;
        let arch = Arch::parse(wrap(s, "architecture", iter.next())?)?;
        let platform = Platform::parse(wrap(s, "platform", iter.next())?)?;
        let os = OS::parse(wrap(s, "OS", iter.next())?)?;

        let mut need_flavour = true;
        let mut flavour_opt = None;
        let mut need_subflavour0 = true;
        let mut subflavour0_opt = None;
        let mut need_subflavour1 = true;
        let mut subflavour1_opt = None;
        let mut need_variant = true;
        let mut variant_opt = None;
        let mut need_tag = tag_opt.is_none();
        while need_flavour || need_subflavour0 || need_subflavour1 || need_variant || need_tag {
            let temp = wrap(s, "flavour/subflavour/variant/tag", iter.next())?;

            if need_flavour {
                need_flavour = false;
                flavour_opt = Flavour::parse(temp);
                if flavour_opt.is_some() {
                    continue;
                }
            }

            if need_subflavour0 {
                need_subflavour0 = false;
                subflavour0_opt = Subflavour::parse(temp);
                if subflavour0_opt.is_some() {
                    continue;
                }
            }

            if need_subflavour1 {
                need_subflavour1 = false;
                subflavour1_opt = Subflavour::parse(temp);
                if subflavour1_opt.is_some() {
                    continue;
                }
            }

            if need_variant {
                need_variant = false;
                variant_opt = Variant::parse(temp);
                if variant_opt.is_some() {
                    continue;
                }
            }

            if need_tag {
                need_tag = false;
                tag_opt = Some(Tag::parse(temp));
                continue;
            }

            unreachable!()
        }

        let tag = tag_opt.expect("tag must be present");
        assert!(iter.next().is_none());

        Ok(Self {
            archive_type,
            family,
            version,
            arch,
            platform,
            os,
            flavour: flavour_opt,
            subflavour0: subflavour0_opt,
            subflavour1: subflavour1_opt,
            variant: variant_opt,
            parsed_tag: tag,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::{
        Arch, ArchiveType, AssetMeta, Family, Platform, Subflavour, Tag, Variant, Version, OS,
    };
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(
        AssetMeta {
            archive_type: ArchiveType::TarZST,
            family: Family::CPython,
            version: Version::new(3, 10, 9),
            arch: Arch::AArch64,
            platform: Platform::Apple,
            os: OS::Darwin,
            flavour: None,
            subflavour0: Some(Subflavour::Debug),
            subflavour1: None,
            variant: Some(Variant::Full),
            parsed_tag: Tag::parse("20230116")
        },
        "cpython-3.10.9+20230116-aarch64-apple-darwin-debug-full.tar.zst"
    )]
    #[case(
        AssetMeta {
            archive_type: ArchiveType::TarGZ,
            family: Family::CPython,
            version: Version::new(3, 10, 9),
            arch: Arch::AArch64,
            platform: Platform::Apple,
            os: OS::Darwin,
            flavour: None,
            subflavour0: None,
            subflavour1: None,
            variant: Some(Variant::InstallOnly),
            parsed_tag: Tag::parse("20230116")
        },
        "cpython-3.10.9+20230116-aarch64-apple-darwin-install_only.tar.gz"
    )]
    #[case(
        AssetMeta {
            archive_type: ArchiveType::TarZST,
            family: Family::CPython,
            version: Version::new(3, 10, 2),
            arch: Arch::AArch64,
            platform: Platform::Apple,
            os: OS::Darwin,
            flavour: None,
            subflavour0: Some(Subflavour::Debug),
            subflavour1: None,
            variant: None,
            parsed_tag: Tag::parse("20220220T1113")
        },
        "cpython-3.10.2-aarch64-apple-darwin-debug-20220220T1113.tar.zst"
    )]
    #[case(
        AssetMeta {
            archive_type: ArchiveType::TarGZ,
            family: Family::CPython,
            version: Version::parse("3.9.6").expect("test: should be valid version"),
            arch: Arch::X86_64,
            platform: Platform::Apple,
            os: OS::Darwin,
            flavour: None,
            subflavour0: None,
            subflavour1: None,
            variant: Some(Variant::InstallOnly),
            parsed_tag: Tag::parse("20210724T1424")
        },
        "cpython-3.9.6-x86_64-apple-darwin-install_only-20210724T1424.tar.gz"
    )]
    fn test_parse(#[case] expected_result: AssetMeta, #[case] input: &str) -> Result<()> {
        assert_eq!(expected_result, AssetMeta::parse(input)?);
        Ok(())
    }

    #[rstest]
    #[case(true, "libuuid-1.0.3.tar.gz")]
    #[case(true, "SHA256SUMS")]
    #[case(true, "foo.sha256")]
    #[case(false, "foo")]
    fn test_definitely_not_an_asset_name(#[case] expected_result: bool, #[case] input: &str) {
        assert_eq!(
            expected_result,
            AssetMeta::definitely_not_an_asset_name(input)
        )
    }
}
