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
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Discriminator {
    String(String),
    None,
}

impl Discriminator {
    pub(crate) fn parse(s: &str) -> (Self, &str) {
        if let Some(i) = s.find("a") {
            return (Self::String(String::from(&s[i..])), &s[0..i]);
        }
        if let Some(i) = s.find("rc") {
            return (Self::String(String::from(&s[i..])), &s[0..i]);
        }
        (Self::None, s)
    }
}

impl Display for Discriminator {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::String(value) => write!(f, "{value}")?,
            Self::None => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::too_many_arguments)]
    use super::Discriminator;
    use rstest::rstest;

    #[rstest]
    #[case(
        Discriminator::String(String::from("rc2+20240909")),
        "3.13.0",
        "3.13.0rc2+20240909"
    )]
    #[case(
        Discriminator::String(String::from("a6+20250409")),
        "3.14.0",
        "3.14.0a6+20250409"
    )]
    #[case(Discriminator::None, "3.10.13+20231002", "3.10.13+20231002")]
    fn basics(
        #[case] expected_discriminator: Discriminator,
        #[case] expected_version_str: &str,
        #[case] input: &str,
    ) {
        let result = Discriminator::parse(input);
        assert_eq!(expected_discriminator, result.0);
        assert_eq!(expected_version_str, result.1);
    }
}
