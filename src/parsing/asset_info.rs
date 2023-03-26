use crate::version::Version;
use std::collections::{hash_set, HashSet};

const EXTS: [&str; 2] = [".tar.gz", ".tar.zst"];
const SUBFLAVOURS: [&str; 1] = ["full"]; // Should be a set!

#[derive(Debug, PartialEq)]
pub struct AssetInfo {
    pub name: String,
    pub family: Family,
    pub version: Version,
    pub tag: Tag,
    pub arch: Arch,
    pub platform: Platform,
    pub os: OS,
    pub flavour: Flavour,
    subflavour: Option<Subflavour>,
    ext: Ext,
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

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Flavour(String);

impl Flavour {
    fn from_str<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }
}

#[derive(Debug, PartialEq)]
struct Subflavour(String);

impl Subflavour {
    fn from_str<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }
}

#[derive(Debug, PartialEq)]
struct Ext(String);

impl Ext {
    fn from_str<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }
}

#[allow(unused)]
impl AssetInfo {
    pub fn from_asset_name(s: &str) -> Option<Self> {
        fn definitely_not_an_asset(s: &str) -> bool {
            if "libuuid-1.0.3.tar.gz" == s {
                true
            } else if "SHA256SUMS" == s {
                true
            } else {
                s.ends_with(".sha256")
            }
        }

        if definitely_not_an_asset(s) {
            return None;
        }

        let mut ext = None::<Ext>;
        let mut ext_len = 0;
        for e in EXTS {
            if (s.ends_with(e)) {
                ext = Some(Ext::from_str(e));
                ext_len = e.len();
                break;
            }
        }
        if ext.is_none() {
            return None;
        }

        let ext = ext.unwrap();
        let base_name = &s[..s.len() - ext_len];

        let mut iter = base_name.split("-").into_iter();

        let family = Family::from_str(iter.next()?)?;

        let version_tag = iter.next()?;

        let parts = version_tag.split("+").collect::<Vec<_>>();
        let (version_str, mut tag) = match parts.len() {
            1 => (parts[0], None),
            2 => (parts[0], Some(Tag::NewStyle(String::from(parts[1])))),
            _ => return None,
        };

        let version = Version::parse(version_str)?;

        let arch = Arch::from_str(iter.next()?)?;

        let platform = Platform::from_str(iter.next()?)?;

        let os = OS::from_str(iter.next()?)?;

        let flavour = Flavour::from_str(iter.next()?);

        let (subflavour, tag) = match iter.next() {
            Some(temp) => {
                let mut sf_temp = None::<Subflavour>;
                for sf in SUBFLAVOURS {
                    if sf == temp {
                        sf_temp = Some(Subflavour::from_str(temp))
                    }
                }

                if sf_temp.is_none() {
                    (None, Some(Tag::OldStyle(String::from(temp))))
                } else {
                    (sf_temp, tag)
                }
            }
            None => (None, tag),
        };

        Some(Self {
            name: String::from(s),
            family: family,
            version: version,
            tag: tag.expect("Must be set"),
            arch: arch,
            platform: platform,
            os: os,
            flavour: flavour,
            subflavour: subflavour,
            ext: ext,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Arch, AssetInfo, Ext, Family, Flavour, Platform, Subflavour, Tag, OS};
    use crate::version::Version;
    use rstest::rstest;

    #[rstest]
    #[case(
        AssetInfo {
            name: String::from("cpython-3.10.9+20230116-aarch64-apple-darwin-debug-full.tar.zst"),
            family: Family::from_str("cpython").expect("Should parse"),
            version: Version::parse("3.10.9").expect("Should parse"),
            tag: Tag::NewStyle(String::from("20230116")),
            arch: Arch::from_str("aarch64").expect("Should parse"),
            platform: Platform::from_str("apple").expect("Should parse"),
            os: OS::from_str("darwin").expect("Should parse"),
            flavour: Flavour::from_str("debug"),
            subflavour: Some(Subflavour::from_str("full")),
            ext: Ext::from_str(".tar.zst"),
        },
        "cpython-3.10.9+20230116-aarch64-apple-darwin-debug-full.tar.zst"
    )]
    #[case(
        AssetInfo {
            name: String::from("cpython-3.10.9+20230116-aarch64-apple-darwin-install_only.tar.gz"),
            family: Family::from_str("cpython").expect("Should parse"),
            version: Version::parse("3.10.9").expect("Should parse"),
            tag: Tag::NewStyle(String::from("20230116")),
            arch: Arch::from_str("aarch64").expect("Should parse"),
            platform: Platform::from_str("apple").expect("Should parse"),
            os: OS::from_str("darwin").expect("Should parse"),
            flavour: Flavour::from_str("install_only"),
            subflavour: None,
            ext: Ext::from_str(".tar.gz"),
        },
        "cpython-3.10.9+20230116-aarch64-apple-darwin-install_only.tar.gz"
    )]
    #[case(
        AssetInfo {
            name: String::from("cpython-3.10.2-aarch64-apple-darwin-debug-20220220T1113.tar.zst"),
            family: Family::from_str("cpython").expect("Should parse"),
            version: Version::parse("3.10.2").expect("Should parse"),
            tag: Tag::OldStyle(String::from("20220220T1113")),
            arch: Arch::from_str("aarch64").expect("Should parse"),
            platform: Platform::from_str("apple").expect("Should parse"),
            os: OS::from_str("darwin").expect("Should parse"),
            flavour: Flavour::from_str("debug"),
            subflavour: None,
            ext: Ext::from_str(".tar.zst"),
        },
        "cpython-3.10.2-aarch64-apple-darwin-debug-20220220T1113.tar.zst"
    )]
    fn test_from_asset_name(#[case] expected_result: AssetInfo, #[case] input: &str) {
        assert_eq!(Some(expected_result), AssetInfo::from_asset_name(input))
    }
}
