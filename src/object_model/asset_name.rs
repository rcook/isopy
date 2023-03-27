use super::attributes::{Arch, ArchiveType, Family, Flavour, Platform, Subflavour, Variant, OS};
use super::tag::Tag;
use super::version::Version;

#[derive(Debug, PartialEq)]
pub struct AssetName {
    pub name: String,
    pub archive_type: ArchiveType,
    pub family: Family,
    pub version: Version,
    pub tag: Tag,
    pub arch: Arch,
    pub platform: Platform,
    pub os: OS,
    pub flavour: Option<Flavour>,
    pub subflavour0: Option<Subflavour>,
    pub subflavour1: Option<Subflavour>,
    pub variant: Option<Variant>,
}

#[allow(unused)]
impl AssetName {
    pub fn definitely_not_an_asset_name(s: &str) -> bool {
        if "libuuid-1.0.3.tar.gz" == s {
            true
        } else if "SHA256SUMS" == s {
            true
        } else {
            s.ends_with(".sha256")
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        fn parse_version_and_tag_opt(s: &str) -> Option<(Version, Option<Tag>)> {
            let parts = s.split("+").collect::<Vec<_>>();
            let (version_str, tag) = match parts.len() {
                1 => (parts[0], None),
                2 => (parts[0], Some(Tag::parse(parts[1]))),
                _ => return None,
            };

            let version = Version::parse(version_str)?;
            Some((version, tag))
        }

        let (archive_type, base_name) = ArchiveType::parse(s)?;

        let mut iter = base_name.split("-").into_iter();

        let family = Family::parse(iter.next()?)?;
        let (version, mut tag_opt) = parse_version_and_tag_opt(iter.next()?)?;
        let arch = Arch::parse(iter.next()?)?;
        let platform = Platform::parse(iter.next()?)?;
        let os = OS::parse(iter.next()?)?;

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
            let temp = iter.next()?;
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
            unimplemented!()
        }

        if iter.next().is_some() {
            unimplemented!()
        }

        Some(Self {
            name: String::from(s),
            archive_type: archive_type,
            family: family,
            version: version,
            tag: tag_opt.expect("Should be set"),
            arch: arch,
            platform: platform,
            os: os,
            flavour: flavour_opt,
            subflavour0: subflavour0_opt,
            subflavour1: subflavour1_opt,
            variant: variant_opt,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::attributes::{Arch, ArchiveType, Family, Platform, Subflavour, Variant, OS};
    use super::super::tag::Tag;
    use super::super::version::Version;
    use super::AssetName;
    use rstest::rstest;

    #[rstest]
    #[case(
        AssetName {
            name: String::from("cpython-3.10.9+20230116-aarch64-apple-darwin-debug-full.tar.zst"),
            archive_type: ArchiveType::TarZST,
            family: Family::CPython,
            version: Version::new(3, 10, 9),
            tag: Tag::parse("20230116"),
            arch: Arch::AArch64,
            platform: Platform::Apple,
            os: OS::Darwin,
            flavour: None,
            subflavour0: Some(Subflavour::Debug),
            subflavour1: None,
            variant: Some(Variant::Full),
        },
        "cpython-3.10.9+20230116-aarch64-apple-darwin-debug-full.tar.zst"
    )]
    #[case(
        AssetName {
            name: String::from("cpython-3.10.9+20230116-aarch64-apple-darwin-install_only.tar.gz"),
            archive_type: ArchiveType::TarGZ,
            family: Family::CPython,
            version: Version::new(3, 10, 9),
            tag: Tag::parse("20230116"),
            arch: Arch::AArch64,
            platform: Platform::Apple,
            os: OS::Darwin,
            flavour: None,
            subflavour0: None,
            subflavour1: None,
            variant: Some(Variant::InstallOnly),
        },
        "cpython-3.10.9+20230116-aarch64-apple-darwin-install_only.tar.gz"
    )]
    #[case(
        AssetName {
            name: String::from("cpython-3.10.2-aarch64-apple-darwin-debug-20220220T1113.tar.zst"),
            archive_type: ArchiveType::TarZST,
            family: Family::CPython,
            version: Version::new(3, 10, 2),
            tag: Tag::parse("20220220T1113"),
            arch: Arch::AArch64,
            platform: Platform::Apple,
            os: OS::Darwin,
            flavour: None,
            subflavour0: Some(Subflavour::Debug),
            subflavour1: None,
            variant: None,
        },
        "cpython-3.10.2-aarch64-apple-darwin-debug-20220220T1113.tar.zst"
    )]
    #[case(
        AssetName {
            name: String::from("cpython-3.9.6-x86_64-apple-darwin-install_only-20210724T1424.tar.gz"),
            archive_type: ArchiveType::TarGZ,
            family: Family::CPython,
            version: Version::parse("3.9.6").expect("Should parse"),
            tag: Tag::parse("20210724T1424"),
            arch: Arch::X86_64,
            platform: Platform::Apple,
            os: OS::Darwin,
            flavour: None,
            subflavour0: None,
            subflavour1: None,
            variant: Some(Variant::InstallOnly),
        },
        "cpython-3.9.6-x86_64-apple-darwin-install_only-20210724T1424.tar.gz"
    )]
    fn test_parse(#[case] expected_result: AssetName, #[case] input: &str) {
        assert_eq!(Some(expected_result), AssetName::parse(input))
    }
}
