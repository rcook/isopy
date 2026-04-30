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
use anyhow::{Result, bail};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Extra {
    Stable,
    ReleaseCandidate(u32),
    Beta(u32),
}

impl PartialOrd for Extra {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Extra {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Stable => match other {
                Self::Stable => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Self::ReleaseCandidate(this) => match other {
                Self::Stable => Ordering::Less,
                Self::ReleaseCandidate(that) => this.cmp(that),
                Self::Beta(_) => Ordering::Greater,
            },
            Self::Beta(this) => match other {
                Self::Beta(that) => this.cmp(that),
                _ => Ordering::Less,
            },
        }
    }
}

pub fn parse_last_part(label: &str, raw: &str, s: &str) -> Result<(u32, Extra)> {
    let mut iter = s.chars();
    let mut prefix = String::new();
    let mut rest = String::new();

    for c in iter.by_ref() {
        if !c.is_ascii_digit() {
            rest.push(c);
            break;
        }
        prefix.push(c);
    }

    for c in iter {
        rest.push(c);
    }

    let value = prefix.parse()?;

    Ok(if rest.is_empty() {
        (value, Extra::Stable)
    } else if let Some(rest) = rest.strip_prefix("rc") {
        let value1 = rest.parse()?;
        (value, Extra::ReleaseCandidate(value1))
    } else if let Some(rest) = rest.strip_prefix("beta") {
        let value1 = rest.parse()?;
        (value, Extra::Beta(value1))
    } else {
        bail!("Invalid {label} version {raw}")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Extra::Beta(1), Extra::Beta(1), Ordering::Equal)]
    #[case(Extra::Beta(1), Extra::Beta(2), Ordering::Less)]
    #[case(Extra::Beta(2), Extra::Beta(1), Ordering::Greater)]
    #[case(Extra::Beta(1), Extra::ReleaseCandidate(1), Ordering::Less)]
    #[case(Extra::Beta(1), Extra::Stable, Ordering::Less)]
    #[case(
        Extra::ReleaseCandidate(1),
        Extra::ReleaseCandidate(1),
        Ordering::Equal
    )]
    #[case(Extra::ReleaseCandidate(1), Extra::ReleaseCandidate(2), Ordering::Less)]
    #[case(
        Extra::ReleaseCandidate(2),
        Extra::ReleaseCandidate(1),
        Ordering::Greater
    )]
    #[case(Extra::ReleaseCandidate(1), Extra::Beta(1), Ordering::Greater)]
    #[case(Extra::ReleaseCandidate(1), Extra::Stable, Ordering::Less)]
    #[case(Extra::Stable, Extra::Stable, Ordering::Equal)]
    #[case(Extra::Stable, Extra::ReleaseCandidate(1), Ordering::Greater)]
    #[case(Extra::Stable, Extra::Beta(1), Ordering::Greater)]
    fn extra_ordering(#[case] a: Extra, #[case] b: Extra, #[case] expected: Ordering) {
        assert_eq!(expected, a.cmp(&b));
        assert_eq!(Some(expected), a.partial_cmp(&b));
    }

    #[rstest]
    #[case("5", 5, Extra::Stable)]
    #[case("0", 0, Extra::Stable)]
    #[case("123", 123, Extra::Stable)]
    #[case("5rc1", 5, Extra::ReleaseCandidate(1))]
    #[case("5rc0", 5, Extra::ReleaseCandidate(0))]
    #[case("5beta1", 5, Extra::Beta(1))]
    #[case("5beta0", 5, Extra::Beta(0))]
    fn parse_last_part_valid(
        #[case] input: &str,
        #[case] expected_value: u32,
        #[case] expected_extra: Extra,
    ) -> Result<()> {
        let (value, extra) = parse_last_part("Test", "raw", input)?;
        assert_eq!(expected_value, value);
        assert_eq!(expected_extra, extra);
        Ok(())
    }

    #[rstest]
    #[case("")]
    #[case("abc")]
    #[case("5alpha1")]
    #[case("5dev1")]
    fn parse_last_part_invalid(#[case] input: &str) {
        assert!(parse_last_part("Test", "raw", input).is_err());
    }

    #[test]
    fn parse_last_part_error_includes_label() {
        let err = parse_last_part("Go", "go1.21xyz", "21xyz").unwrap_err();
        assert!(err.to_string().contains("Go"));
        assert!(err.to_string().contains("go1.21xyz"));
    }
}
