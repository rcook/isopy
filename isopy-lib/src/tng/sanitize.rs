pub struct SanitizeOptions {
    pub retain_dots: bool,
}

impl SanitizeOptions {
    pub fn retain_dots(mut self, value: bool) -> Self {
        self.retain_dots = value;
        self
    }
}

impl Default for SanitizeOptions {
    fn default() -> Self {
        Self { retain_dots: false }
    }
}

pub fn sanitize_with_options(s: &str, options: &SanitizeOptions) -> String {
    let mut in_placeholder = false;
    let mut output = String::new();
    for ch in s.chars() {
        if ch.is_alphanumeric() || options.retain_dots && ch == '.' {
            output.push(ch);
            in_placeholder = false;
        } else {
            if !in_placeholder {
                output.push('_');
                in_placeholder = true;
            }
        }
    }
    output
}

pub fn sanitize(s: &str) -> String {
    sanitize_with_options(s, &SanitizeOptions::default())
}

#[cfg(test)]
mod tests {
    use crate::tng::sanitize::{sanitize, sanitize_with_options, SanitizeOptions};
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case("www.foo.com", "www.foo.com")]
    #[case("www.foo.-com", "www.foo._com")]
    #[case("www_foo_com", "www_foo_com")]
    #[case("www__foo__com", "www_foo_com")]
    #[case("www___foo___com", "www_foo_com")]
    #[case("www__.foo__.com", "www_.foo_.com")]
    fn test_sanitize_with_options(#[case] input: &str, #[case] expected: &str) -> Result<()> {
        let options = SanitizeOptions::default().retain_dots(true);
        assert_eq!(expected, sanitize_with_options(input, &options));
        Ok(())
    }

    #[rstest]
    #[case("www.foo.com", "www_foo_com")]
    #[case("www.foo.-com", "www_foo_com")]
    #[case("www_foo_com", "www_foo_com")]
    #[case("www__foo__com", "www_foo_com")]
    #[case("www___foo___com", "www_foo_com")]
    #[case("www__.foo__.com", "www_foo_com")]
    fn test_sanitize(#[case] input: &str, #[case] expected: &str) -> Result<()> {
        assert_eq!(expected, sanitize(input));
        Ok(())
    }
}
