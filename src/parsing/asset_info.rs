#[derive(Debug, PartialEq)]
struct AssetInfo {
    family: String,
    version: String,
    tag: String,
}

#[allow(unused)]
impl AssetInfo {
    fn from_asset_name(s: &str) -> Option<Self> {
        let mut iter = s.split("-").into_iter();

        let family = match iter.next() {
            Some(x) => x,
            None => return None,
        };

        let version_tag = match iter.next() {
            Some(x) => x,
            None => return None,
        };

        let parts = version_tag.split("+").collect::<Vec<_>>();
        if parts.len() != 2 {
            return None;
        }

        let version = parts[0];
        let tag = parts[1];

        Some(Self {
            family: String::from(family),
            version: String::from(version),
            tag: String::from(tag),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::AssetInfo;
    use rstest::rstest;

    #[rstest]
    #[case(
        AssetInfo {
            family:String::from("cpython"),
            version:String::from("3.10.9"),
            tag:String::from("20230116")
        },
        "cpython-3.10.9+20230116-aarch64-apple-darwin-debug-full.tar.zst"
    )]
    fn test_from_asset_name(#[case] expected_result: AssetInfo, #[case] input: &str) {
        assert_eq!(Some(expected_result), AssetInfo::from_asset_name(input))
    }
}
