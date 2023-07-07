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
use crate::error::IsopyPythonError;
use anyhow::anyhow;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PythonVersion {
    major: i32,
    minor: i32,
    build: i32,
    raw: String,
}

impl PythonVersion {
    #[must_use]
    pub fn new(major: i32, minor: i32, build: i32) -> Self {
        Self {
            major,
            minor,
            build,
            raw: format!("{major}.{minor}.{build}"),
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.raw
    }
}

impl FromStr for PythonVersion {
    type Err = IsopyPythonError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let parts = s.split('.').collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(IsopyPythonError::InvalidVersion(String::from(s)));
        }

        let major = parts[0].parse::<i32>().map_err(|e| anyhow!(e))?;
        let minor = parts[1].parse::<i32>().map_err(|e| anyhow!(e))?;
        let build = parts[2].parse::<i32>().map_err(|e| anyhow!(e))?;

        Ok(Self::new(major, minor, build))
    }
}

impl Display for PythonVersion {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}.{}.{}", self.major, self.minor, self.build)
    }
}

impl<'de> Deserialize<'de> for PythonVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse::<Self>()
            .map_err(serde::de::Error::custom)
    }
}

impl Serialize for PythonVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::PythonVersion;
    use anyhow::Result;

    #[test]
    fn parse() -> Result<()> {
        assert_eq!(
            PythonVersion::new(1, 2, 3),
            "1.2.3".parse::<PythonVersion>()?
        );
        Ok(())
    }

    #[test]
    fn parse_error() {
        assert!("xyz.2.3".parse::<PythonVersion>().is_err());
    }
}
