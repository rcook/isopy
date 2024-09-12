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
use crate::tng::version::VersionOps;
use anyhow::{bail, Error, Result};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct VersionTriple {
    pub major: i32,
    pub minor: i32,
    pub revision: i32,
}

impl Display for VersionTriple {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}.{}.{}", self.major, self.minor, self.revision)
    }
}

impl FromStr for VersionTriple {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.splitn(3, '.').collect::<Vec<_>>();
        if parts.len() != 3 {
            bail!("Invalid package version {s}")
        }

        let major = parts.get(0).expect("Expected major").parse()?;
        let minor = parts.get(1).expect("Expected minor").parse()?;
        let revision = parts.get(2).expect("Expected revision").parse()?;

        Ok(Self {
            major,
            minor,
            revision,
        })
    }
}

impl VersionOps for VersionTriple {
    fn box_clone(&self) -> Box<dyn VersionOps> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::tng::version_triple::VersionTriple;
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case("1.2.3", VersionTriple { major: 1, minor: 2, revision: 3 })]
    fn from_str(#[case] input: &str, #[case] expected: VersionTriple) -> Result<()> {
        assert_eq!(expected, input.parse()?);
        Ok(())
    }

    #[rstest]
    #[case("")]
    #[case("1")]
    #[case("1.2")]
    #[case("1.2.3.4")]
    #[case("1.2.three")]
    fn from_str_fail(#[case] input: &str) {
        assert!(input.parse::<VersionTriple>().is_err());
    }
}
