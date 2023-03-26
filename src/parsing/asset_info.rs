use crate::version::Version;

#[derive(Debug, PartialEq)]
pub struct AssetInfo {
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

#[derive(Debug, PartialEq)]
pub enum ArchiveType {
    TarGZ,
    TarZST,
}

impl ArchiveType {
    fn from_asset_name<S>(s: S) -> Option<(Self, String)>
    where
        S: AsRef<str>,
    {
        let s0 = s.as_ref();
        for (ext, archive_type) in [(".tar.gz", Self::TarGZ), (".tar.zst", Self::TarZST)] {
            if s0.ends_with(ext) {
                let ext_len = ext.len();
                let base_name = &s0[..s0.len() - ext_len];
                return Some((archive_type, String::from(base_name)));
            }
        }
        None
    }
}

#[derive(Debug, PartialEq)]
pub enum Family {
    CPython,
}

impl Family {
    fn from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Some(match s.as_ref() {
            "cpython" => Self::CPython,
            _ => return None,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Tag {
    NewStyle(String),
    OldStyle(String),
}

#[derive(Debug, PartialEq)]
pub enum Arch {
    AArch64,
    I686,
    X86_64,
    X86_64V2,
    X86_64V3,
    X86_64V4,
}

impl Arch {
    fn from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Some(match s.as_ref() {
            "aarch64" => Self::AArch64,
            "i686" => Self::I686,
            "x86_64" => Self::X86_64,
            "x86_64_v2" => Self::X86_64V2,
            "x86_64_v3" => Self::X86_64V3,
            "x86_64_v4" => Self::X86_64V4,
            _ => return None,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Platform {
    PC,
    Apple,
    Unknown,
}

impl Platform {
    fn from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Some(match s.as_ref() {
            "pc" => Self::PC,
            "apple" => Self::Apple,
            "unknown" => Self::Unknown,
            _ => return None,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum OS {
    Darwin,
    Linux,
    Windows,
}

impl OS {
    fn from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Some(match s.as_ref() {
            "darwin" => Self::Darwin,
            "linux" => Self::Linux,
            "windows" => Self::Windows,
            _ => return None,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Flavour {
    GNU,
    MSVC,
    MUSL,
}

impl Flavour {
    fn from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Some(match s.as_ref() {
            "gnu" => Self::GNU,
            "msvc" => Self::MSVC,
            "musl" => Self::MUSL,
            _ => return None,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Subflavour {
    Debug,
    NoOpt,
    PGOLTO,
    PGO,
    LTO,
    Shared,
    Static,
}

impl Subflavour {
    fn from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Some(match s.as_ref() {
            "debug" => Self::Debug,
            "noopt" => Self::NoOpt,
            "pgo+lto" => Self::PGOLTO,
            "pgo" => Self::PGO,
            "lto" => Self::LTO,
            "shared" => Self::Shared,
            "static" => Self::Static,
            _ => return None,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Variant {
    InstallOnly,
    Full,
}

impl Variant {
    fn from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Some(match s.as_ref() {
            "install_only" => Self::InstallOnly,
            "full" => Self::Full,
            _ => return None,
        })
    }
}

#[allow(unused)]
impl AssetInfo {
    pub fn definitely_not_an_asset(s: &str) -> bool {
        if "libuuid-1.0.3.tar.gz" == s {
            true
        } else if "SHA256SUMS" == s {
            true
        } else {
            s.ends_with(".sha256")
        }
    }

    pub fn from_asset_name(s: &str) -> Option<Self> {
        fn parse_version_and_tag_opt(s: &str) -> Option<(Version, Option<Tag>)> {
            let parts = s.split("+").collect::<Vec<_>>();
            let (version_str, tag) = match parts.len() {
                1 => (parts[0], None),
                2 => (parts[0], Some(Tag::NewStyle(String::from(parts[1])))),
                _ => return None,
            };

            let version = Version::parse(version_str)?;
            Some((version, tag))
        }

        let (archive_type, base_name) = ArchiveType::from_asset_name(s)?;

        let mut iter = base_name.split("-").into_iter();

        let family = Family::from_str(iter.next()?)?;
        let (version, mut tag_opt) = parse_version_and_tag_opt(iter.next()?)?;
        let arch = Arch::from_str(iter.next()?)?;
        let platform = Platform::from_str(iter.next()?)?;
        let os = OS::from_str(iter.next()?)?;

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
                flavour_opt = Flavour::from_str(temp);
                if flavour_opt.is_some() {
                    continue;
                }
            }
            if need_subflavour0 {
                need_subflavour0 = false;
                subflavour0_opt = Subflavour::from_str(temp);
                if subflavour0_opt.is_some() {
                    continue;
                }
            }
            if need_subflavour1 {
                need_subflavour1 = false;
                subflavour1_opt = Subflavour::from_str(temp);
                if subflavour1_opt.is_some() {
                    continue;
                }
            }
            if need_variant {
                need_variant = false;
                variant_opt = Variant::from_str(temp);
                if variant_opt.is_some() {
                    continue;
                }
            }
            if need_tag {
                need_tag = false;
                tag_opt = Some(Tag::OldStyle(String::from(temp)));
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
    use super::{
        Arch, ArchiveType, AssetInfo, Family, Flavour, Platform, Subflavour, Tag, Variant, OS,
    };
    use crate::version::Version;
    use rstest::rstest;

    #[rstest]
    #[case(
        AssetInfo {
            name: String::from("cpython-3.10.9+20230116-aarch64-apple-darwin-debug-full.tar.zst"),
            archive_type: ArchiveType::TarZST,
            family: Family::CPython,
            version: Version::parse("3.10.9").expect("Should parse"),
            tag: Tag::NewStyle(String::from("20230116")),
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
        AssetInfo {
            name: String::from("cpython-3.10.9+20230116-aarch64-apple-darwin-install_only.tar.gz"),
            archive_type: ArchiveType::TarGZ,
            family: Family::CPython,
            version: Version::parse("3.10.9").expect("Should parse"),
            tag: Tag::NewStyle(String::from("20230116")),
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
        AssetInfo {
            name: String::from("cpython-3.10.2-aarch64-apple-darwin-debug-20220220T1113.tar.zst"),
            archive_type: ArchiveType::TarZST,
            family: Family::CPython,
            version: Version::parse("3.10.2").expect("Should parse"),
            tag: Tag::OldStyle(String::from("20220220T1113")),
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
        AssetInfo {
            name: String::from("cpython-3.9.6-x86_64-apple-darwin-install_only-20210724T1424.tar.gz"),
            archive_type: ArchiveType::TarGZ,
            family: Family::CPython,
            version: Version::parse("3.9.6").expect("Should parse"),
            tag: Tag::OldStyle(String::from("20210724T1424")),
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
    fn test_from_asset_name(#[case] expected_result: AssetInfo, #[case] input: &str) {
        assert_eq!(Some(expected_result), AssetInfo::from_asset_name(input))
    }
}
