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
use crate::maven_version::MavenVersion;
use crate::maven_version_limit::MavenVersionLimit;
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use std::fmt::{Display, Formatter, Result as FmtResult};

const MAVEN_VERSION: &AsciiSet = &CONTROLS.add(b'(').add(b')').add(b',').add(b'[').add(b']');

#[allow(unused)]
pub(crate) enum MavenVersionRange {
    #[allow(unused)]
    OpenJdkVersion(String),
    #[allow(unused)]
    Version(MavenVersion),
    VersionRange(MavenVersionLimit, MavenVersionLimit),
}

impl MavenVersionRange {
    #[allow(unused)]
    #[must_use]
    pub(crate) fn to_path_segment(&self) -> String {
        utf8_percent_encode(&self.to_string(), MAVEN_VERSION).to_string()
    }
}

impl Display for MavenVersionRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use crate::maven_version_limit::MavenVersionLimit::*;

        match self {
            Self::OpenJdkVersion(s) => write!(f, "{s}"),
            Self::Version(value) => {
                write!(f, "{value}")
            }
            Self::VersionRange(lower, upper) => {
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

#[cfg(test)]
mod tests {
    use crate::maven_version::MavenVersion;
    use crate::maven_version_limit::MavenVersionLimit::{Closed, Open};
    use crate::maven_version_range::MavenVersionRange::{
        self, OpenJdkVersion, Version, VersionRange,
    };
    use rstest::rstest;

    #[rstest]
    #[case(
        "arbitrary/string",
        "arbitrary/string",
        OpenJdkVersion(String::from("arbitrary/string"))
    )]
    #[case("10", "10", Version(MavenVersion::new(10, None)))]
    #[case("1.2", "1.2", Version(MavenVersion::new(1, Some(2))))]
    #[case("(,)", "%28%2C%29", VersionRange(Open(None), Open(None)))]
    #[case("[,]", "%5B%2C%5D", VersionRange(Closed(None), Closed(None)))]
    #[case(
        "(22,)",
        "%2822%2C%29",
        VersionRange(Open(Some(MavenVersion::new(22, None))), Open(None))
    )]
    #[case(
        "(22.33,)",
        "%2822.33%2C%29",
        VersionRange(Open(Some(MavenVersion::new(22, Some(33)))), Open(None))
    )]
    #[case(
        "(,1000000.0]",
        "%28%2C1000000.0%5D",
        VersionRange(Open(None), Closed(Some(MavenVersion::new(1_000_000, Some(0)))))
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
