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
use std::fmt::{Display, Formatter, Result as FmtResult};
use uuid::Uuid;

lazy_static! {
    static ref ENVIRONMENT_NAME_REGEX: Regex = Regex::new("^[A-Za-z0-9-_]+$").unwrap();
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnvironmentName(String);

impl EnvironmentName {
    pub fn parse(s: &str) -> Option<Self> {
        match ENVIRONMENT_NAME_REGEX.is_match(s) {
            true => Some(Self(String::from(s))),
            false => None,
        }
    }

    pub fn random() -> Self {
        let name = format!("env-{}", Uuid::new_v4());
        Self::parse(&name).expect("must be valid")
    }

    pub fn sanitize(s: &str) -> Self {
        let mut s1 = String::with_capacity(s.len());
        for c in s.chars() {
            if char::is_alphanumeric(c) {
                s1.push(c)
            } else {
                s1.push('-')
            }
        }

        Self::parse(&s1).expect("must be valid")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for EnvironmentName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::EnvironmentName;
    use rstest::rstest;

    #[rstest]
    #[case(None, "")]
    #[case(None, " ")]
    #[case(None, " foo ")]
    #[case(Some(EnvironmentName(String::from("foo"))), "foo")]
    #[case(Some(EnvironmentName(String::from("foo-_"))), "foo-_")]
    fn test_basics(#[case] expected_result: Option<EnvironmentName>, #[case] input: &str) {
        assert_eq!(expected_result, EnvironmentName::parse(input))
    }
}
