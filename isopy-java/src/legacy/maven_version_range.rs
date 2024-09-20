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
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use std::fmt::{Display, Formatter, Result as FmtResult};

const MAVEN_VERSION: &AsciiSet = &CONTROLS.add(b'(').add(b')').add(b',').add(b'[').add(b']');

pub enum MavenVersionRange {
    #[allow(unused)]
    OpenJdkVersion(String),
    #[allow(unused)]
    Value(MavenVersionValue),
    Range(MavenVersionLimit, MavenVersionLimit),
}

impl MavenVersionRange {
    #[must_use]
    pub fn to_path_segment(&self) -> String {
        utf8_percent_encode(&self.to_string(), MAVEN_VERSION).to_string()
    }
}

impl Display for MavenVersionRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use MavenVersionLimit::*;

        match self {
            Self::OpenJdkVersion(s) => write!(f, "{s}"),
            Self::Value(value) => {
                write!(f, "{value}")
            }
            Self::Range(lower, upper) => {
                let mut s = String::new();
                match lower {
                    Open(None) => s.push('('),
                    Open(Some(value)) => {
                        s.push('(');
                        s.push_str(&value.to_string());
                    }
                    Closed(None) => s.push('['),
                    Closed(Some(value)) => {
                        s.push('[');
                        s.push_str(&value.to_string());
                    }
                }
                s.push(',');
                match upper {
                    Open(None) => s.push(')'),
                    Open(Some(value)) => {
                        s.push_str(&value.to_string());
                        s.push(')');
                    }
                    Closed(None) => s.push(']'),
                    Closed(Some(value)) => {
                        s.push_str(&value.to_string());
                        s.push(']');
                    }
                }
                write!(f, "{s}")
            }
        }
    }
}

pub struct MavenVersionValue {
    pub major: u32,
    pub minor: Option<u32>,
}

impl MavenVersionValue {
    #[must_use]
    pub const fn new(major: u32, minor: Option<u32>) -> Self {
        Self { major, minor }
    }
}

impl Display for MavenVersionValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(minor) = self.minor {
            write!(f, "{}.{}", self.major, minor)
        } else {
            write!(f, "{}", self.major)
        }
    }
}

pub enum MavenVersionLimit {
    Open(Option<MavenVersionValue>),
    Closed(Option<MavenVersionValue>),
}

#[cfg(test)]
mod tests {
    use super::MavenVersionLimit::{Closed, Open};
    use super::MavenVersionRange::{self, OpenJdkVersion, Range, Value};
    use super::MavenVersionValue;
    use rstest::rstest;

    #[rstest]
    #[case(
        "arbitrary/string",
        "arbitrary/string",
        OpenJdkVersion(String::from("arbitrary/string"))
    )]
    #[case("10", "10", Value(MavenVersionValue::new(10, None)))]
    #[case("1.2", "1.2", Value(MavenVersionValue::new(1, Some(2))))]
    #[case("(,)", "%28%2C%29", Range(Open(None), Open(None)))]
    #[case("[,]", "%5B%2C%5D", Range(Closed(None), Closed(None)))]
    #[case(
        "(22,)",
        "%2822%2C%29",
        Range(Open(Some(MavenVersionValue::new(22, None))), Open(None))
    )]
    #[case(
        "(22.33,)",
        "%2822.33%2C%29",
        Range(Open(Some(MavenVersionValue::new(22, Some(33)))), Open(None))
    )]
    #[case(
        "(,1000000.0]",
        "%28%2C1000000.0%5D",
        Range(Open(None), Closed(Some(MavenVersionValue::new(1_000_000, Some(0)))))
    )]
    fn basics(
        #[case] expected_str: &str,
        #[case] expected_path_segment: &str,
        #[case] version: MavenVersionRange,
    ) {
        assert_eq!(expected_str, version.to_string());
        assert_eq!(expected_path_segment, version.to_path_segment());
    }
}
