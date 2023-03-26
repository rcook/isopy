#[allow(unused)]
fn parse_asset_name(s: &str) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::parse_asset_name;
    use rstest::rstest;

    #[rstest]
    #[case(
        false,
        "cpython-3.10.9+20230116-aarch64-apple-darwin-debug-full.tar.zst"
    )]
    fn test_parse_asset_name(#[case] expected_result: bool, #[case] input: &str) {
        assert_eq!(expected_result, parse_asset_name(input))
    }
}
