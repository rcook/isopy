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
use anyhow::{bail, Error};
use regex::Regex;
use std::cmp::Ordering;
use std::result::Result as StdResult;
use std::str::FromStr;
use std::sync::LazyLock;

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
    use super::Label;
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
}
