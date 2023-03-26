#[derive(Debug, PartialEq)]
struct AssetInfo {
    family: Family,
    version: Version,
    tag: Tag,
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
struct Version(String);

impl Version {
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

#[allow(unused)]
impl AssetInfo {
    fn from_asset_name(s: &str) -> Option<Self> {
        let mut iter = s.split("-").into_iter();

        let family = Family::from_str(match iter.next() {
            Some(x) => x,
            None => return None,
        });

        let version_tag = match iter.next() {
            Some(x) => x,
            None => return None,
        };

        let parts = version_tag.split("+").collect::<Vec<_>>();
        if parts.len() != 2 {
            return None;
        }

        let version = Version::from_str(parts[0]);
        let tag = Tag::from_str(parts[1]);

        Some(Self {
            family: family,
            version: version,
            tag: tag,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{AssetInfo, Family, Tag, Version};
    use rstest::rstest;

    #[rstest]
    #[case(
        AssetInfo {
            family: Family::from_str("cpython"),
            version: Version::from_str("3.10.9"),
            tag: Tag::from_str("20230116")
        },
        "cpython-3.10.9+20230116-aarch64-apple-darwin-debug-full.tar.zst"
    )]
    fn test_from_asset_name(#[case] expected_result: AssetInfo, #[case] input: &str) {
        assert_eq!(Some(expected_result), AssetInfo::from_asset_name(input))
    }
}
