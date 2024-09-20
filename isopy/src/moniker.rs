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
use crate::env::{read_env_bool, ISOPY_JAVA_ENV_NAME};
use anyhow::{bail, Error};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::Path;
use std::result::Result as StdResult;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const GO: &str = "go";
const JAVA: &str = "java";
const PYTHON: &str = "python";

#[derive(Clone, Debug, EnumIter, PartialEq)]

pub(crate) enum Moniker {
    Go,
    Java,
    Python,
}

impl Moniker {
    pub(crate) const fn as_str(&self) -> &str {
        match self {
            Self::Go => GO,
            Self::Java => JAVA,
            Self::Python => PYTHON,
        }
    }

    pub(crate) fn dir(&self) -> &Path {
        Path::new(self.as_str())
    }

    pub(crate) fn iter_enabled() -> impl Iterator<Item = Self> {
        let java_enabled = read_env_bool(ISOPY_JAVA_ENV_NAME);
        Self::iter().filter(move |member| match member {
            Self::Java if java_enabled => true,
            Self::Java if !java_enabled => false,
            _ => true,
        })
    }
}

impl Display for Moniker {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Moniker {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        for member in Self::iter_enabled() {
            if member.as_str().eq_ignore_ascii_case(s) {
                return Ok(member);
            }
        }
        bail!("Invalid package manager moniker {s}")
    }
}
