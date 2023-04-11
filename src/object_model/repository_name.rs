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
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REPOSITORY_NAME_REGEX: Regex = Regex::new("^[A-Za-z0-9-_]+$").unwrap();
}

const DEFAULT_REPOSITORY_NAME: &str = "default";

#[derive(Clone, Debug, PartialEq)]
pub enum RepositoryName {
    Default,
    Named(String),
}

impl RepositoryName {
    pub fn parse(s: &str) -> Option<Self> {
        if s.eq_ignore_ascii_case(DEFAULT_REPOSITORY_NAME) {
            Some(Self::Default)
        } else {
            match REPOSITORY_NAME_REGEX.is_match(s) {
                true => Some(Self::Named(String::from(s))),
                false => None,
            }
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Default => DEFAULT_REPOSITORY_NAME,
            Self::Named(s) => s,
        }
    }
}

impl std::fmt::Display for RepositoryName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::RepositoryName;
    use rstest::rstest;

    #[rstest]
    #[case(None, "")]
    #[case(None, " ")]
    #[case(None, " foo ")]
    #[case(Some(RepositoryName::Named(String::from("foo"))), "foo")]
    #[case(Some(RepositoryName::Named(String::from("foo-_"))), "foo-_")]
    #[case(Some(RepositoryName::Default), "DEFAULT")]
    #[case(Some(RepositoryName::Default), "default")]
    fn test_basics(#[case] expected_result: Option<RepositoryName>, #[case] input: String) {
        assert_eq!(expected_result, RepositoryName::parse(&input))
    }
}
