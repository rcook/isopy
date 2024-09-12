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
#[derive(Default)]
pub struct SanitizeOptions {
    pub retain_dots: bool,
}

impl SanitizeOptions {
    #[must_use]
    pub fn retain_dots(mut self, value: bool) -> Self {
        self.retain_dots = value;
        self
    }
}

#[must_use]
pub fn sanitize_with_options(s: &str, options: &SanitizeOptions) -> String {
    let mut in_placeholder = false;
    let mut output = String::new();
    for ch in s.chars() {
        if ch.is_alphanumeric() || options.retain_dots && ch == '.' {
            output.push(ch);
            in_placeholder = false;
        } else if !in_placeholder {
            output.push('_');
            in_placeholder = true;
        }
    }
    output
}

#[must_use]
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
