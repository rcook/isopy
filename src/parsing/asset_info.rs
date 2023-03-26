use crate::version::Version;

const SUPPORTED_EXTS: [&str; 1] = [".tar.zst"];

#[derive(Debug, PartialEq)]
pub struct AssetInfo {
    family: Family,
    version: Version,
    tag: Tag,
    arch: Arch,
    platform: Platform,
    os: OS,
    flavour: Flavour,
    subflavour: Subflavour,
    ext: Ext,
}

#[derive(Debug, PartialEq)]
struct Family(String);

impl Family {
    fn from_str<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }
}

#[derive(Debug, PartialEq)]
struct Tag(String);

impl Tag {
    fn from_str<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }
}

#[derive(Debug, PartialEq)]
struct Arch(String);

impl Arch {
    fn from_str<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }
}

#[derive(Debug, PartialEq)]
struct Platform(String);

impl Platform {
    fn from_str<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }
}

#[derive(Debug, PartialEq)]
struct OS(String);

impl OS {
    fn from_str<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }
}

#[derive(Debug, PartialEq)]
struct Flavour(String);

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
        let mut ext = None::<Ext>;
        let mut ext_len = 0;
        for e in SUPPORTED_EXTS {
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
        let s = &s[..s.len() - ext_len];

        let mut iter = s.split("-").into_iter();

        let family = Family::from_str(iter.next()?);

        let version_tag = iter.next()?;

        let parts = version_tag.split("+").collect::<Vec<_>>();
        if parts.len() != 2 {
            return None;
        }

        let version = Version::parse(parts[0])?;

        let tag = Tag::from_str(parts[1]);

        let arch = Arch::from_str(iter.next()?);

        let platform = Platform::from_str(iter.next()?);

        let os = OS::from_str(iter.next()?);

        let flavour = Flavour::from_str(iter.next()?);

        let subflavour = Subflavour::from_str(iter.next()?);

        Some(Self {
            family: family,
            version: version,
            tag: tag,
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
            family: Family::from_str("cpython"),
            version: Version::parse("3.10.9").expect("Should parse"),
            tag: Tag::from_str("20230116"),
            arch: Arch::from_str("aarch64"),
            platform: Platform::from_str("apple"),
            os: OS::from_str("darwin"),
            flavour: Flavour::from_str("debug"),
            subflavour: Subflavour::from_str("full"),
            ext: Ext::from_str(".tar.zst"),
        },
        "cpython-3.10.9+20230116-aarch64-apple-darwin-debug-full.tar.zst"
    )]
    fn test_from_asset_name(#[case] expected_result: AssetInfo, #[case] input: &str) {
        assert_eq!(Some(expected_result), AssetInfo::from_asset_name(input))
    }
}
