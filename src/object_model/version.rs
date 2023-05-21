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
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct Version {
    major: i32,
    minor: i32,
    build: i32,
    value: String,
}

impl Version {
    pub fn new(major: i32, minor: i32, build: i32) -> Self {
        Self {
            major,
            minor,
            build,
            value: format!("{major}.{minor}.{build}"),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts = s.split('.').collect::<Vec<_>>();
        if parts.len() != 3 {
            bail!("invalid version \"{}\"", s)
        }

        let major = parts[0].parse()?;
        let minor = parts[1].parse()?;
        let build = parts[2].parse()?;

        Ok(Self::new(major, minor, build))
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}.{}.{}", self.major, self.minor, self.build)
    }
}

#[cfg(test)]
mod tests {
    use super::Version;
    use anyhow::Result;

    #[test]
    fn test_parse() -> Result<()> {
        assert_eq!(Version::new(1, 2, 3), "1.2.3".parse::<Version>()?);
        Ok(())
    }
}
