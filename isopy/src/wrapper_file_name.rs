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
use serde::de::Error;
use serde::{Deserialize, Serialize};
use std::ffi::{OsStr, OsString};
use std::fmt::{Display, Error as FmtError, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct WrapperFileName(OsString);

impl WrapperFileName {
    pub fn as_os_str(&self) -> &OsStr {
        self.0.as_os_str()
    }
}

impl FromStr for WrapperFileName {
    type Err = anyhow::Error;

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(OsString::from_str(s)?))
    }

    #[cfg(target_os = "windows")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use anyhow::bail;

        let temp = s.to_lowercase();

        #[allow(clippy::case_sensitive_file_extension_comparisons)]
        if !temp.ends_with(".bat") && !temp.ends_with(".cmd") {
            bail!("wrapper file name must have .bat or .cmd extension");
        }

        Ok(Self(OsString::from_str(s)?))
    }
}

impl Display for WrapperFileName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0.to_str().ok_or_else(FmtError::default)?)
    }
}

impl<'de> Deserialize<'de> for WrapperFileName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse::<Self>()
            .map_err(Error::custom)
    }
}

impl Serialize for WrapperFileName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(
            self.0.to_str().ok_or_else(|| {
                serde::ser::Error::custom("could not serialize wrapper file name")
            })?,
        )
    }
}
