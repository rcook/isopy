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
use anyhow::{bail, Error, Result};
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::Path;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const PYTHON_STR: &str = "python";

#[derive(Clone, Debug, EnumIter, ValueEnum)]

pub(crate) enum Moniker {
    #[clap(name = PYTHON_STR)]
    Python,
}

impl Moniker {
    pub(crate) const fn as_str(&self) -> &str {
        match self {
            Self::Python => PYTHON_STR,
        }
    }

    pub(crate) fn dir(&self) -> &Path {
        Path::new(self.as_str())
    }
}

impl Display for Moniker {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Moniker {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        for value in Self::iter() {
            if value.as_str() == s {
                return Ok(value);
            }
        }
        bail!("Invalid package manager moniker {s}")
    }
}
