struct AssetInfo {}

#[allow(unused)]
impl AssetInfo {
    fn from_asset_name(s: &str) -> bool {
        let temp = s.split(".");
        false
    }
}

#[cfg(test)]
mod tests {
    use super::AssetInfo;
    use rstest::rstest;

    #[rstest]
    #[case(
        false,
        "cpython-3.10.9+20230116-aarch64-apple-darwin-debug-full.tar.zst"
    )]
    fn test_from_asset_name(#[case] expected_result: bool, #[case] input: &str) {
        assert_eq!(expected_result, AssetInfo::from_asset_name(input))
    }
}
