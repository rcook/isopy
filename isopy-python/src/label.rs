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
use std::cmp::Ordering;
use std::result::Result as StdResult;
use std::str::FromStr;
use std::sync::LazyLock;

use anyhow::{Error, bail};
use regex::Regex;

static NEW_STYLE_LABEL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^\\d{8}$").expect("Invalid regex"));

static OLD_STYLE_LABEL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^\\d{8}T\\d{4}$").expect("Invalid regex"));

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum Label {
    OldStyle(String),
    NewStyle(String),
}

impl Label {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::OldStyle(s) | Self::NewStyle(s) => s,
        }
    }
}

impl FromStr for Label {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        if NEW_STYLE_LABEL_REGEX.is_match(s) {
            Ok(Self::NewStyle(String::from(s)))
        } else if OLD_STYLE_LABEL_REGEX.is_match(s) {
            Ok(Self::OldStyle(String::from(s)))
        } else {
            bail!("Cannot parse {s} as group")
        }
    }
}

impl Ord for Label {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::OldStyle(a), Self::OldStyle(b)) | (Self::NewStyle(a), Self::NewStyle(b)) => {
                a.cmp(b)
            }
            (Self::NewStyle(_), Self::OldStyle(_)) => Ordering::Greater,
            (Self::OldStyle(_), Self::NewStyle(_)) => Ordering::Less,
        }
    }
}

impl PartialOrd for Label {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::label::Label;

    #[test]
    fn basics() {
        let label1 = Label::NewStyle(String::from("label"));
        let label2 = Label::NewStyle(String::from("label"));
        let label3 = Label::OldStyle(String::from("label"));
        assert_eq!(label1, label2);
        assert_ne!(label1, label3);
        assert_ne!(label2, label3);
        assert!(label1 > label3);
        assert!(label3 < label2);
    }

    #[rstest]
    #[case("20250414", Label::NewStyle(String::from("20250414")))]
    #[case("20220227", Label::NewStyle(String::from("20220227")))]
    #[case("20250414T1530", Label::OldStyle(String::from("20250414T1530")))]
    #[case("20240101T0000", Label::OldStyle(String::from("20240101T0000")))]
    fn parse_valid(#[case] input: &str, #[case] expected: Label) -> anyhow::Result<()> {
        assert_eq!(expected, input.parse::<Label>()?);
        Ok(())
    }

    #[rstest]
    #[case("")]
    #[case("2025-04-14")]
    #[case("2025041")]
    #[case("202504140")]
    #[case("20250414T15")]
    #[case("20250414t1530")]
    #[case("label")]
    fn parse_invalid(#[case] input: &str) {
        assert!(input.parse::<Label>().is_err());
    }

    #[test]
    fn new_style_sorts_after_old_style_even_with_earlier_date() {
        let old: Label = "20991231T2359".parse().unwrap();
        let new: Label = "20000101".parse().unwrap();
        assert!(new > old);
    }
}
