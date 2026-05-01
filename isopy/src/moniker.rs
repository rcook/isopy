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
use crate::env::EnvKey;
use anyhow::{Error, bail};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::Path;
use std::result::Result as StdResult;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const GO: &str = "go";
const JAVA: &str = "java";
const PYTHON: &str = "python";

#[derive(Clone, Debug, Deserialize, EnumIter, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
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

    /// Env var that enables this moniker, or `None` if it's always enabled.
    pub(crate) const fn enable_env_var(&self) -> Option<EnvKey> {
        match self {
            Self::Go => Some(EnvKey::GoEnabled),
            Self::Java => Some(EnvKey::JavaEnabled),
            Self::Python => None,
        }
    }

    pub(crate) fn is_enabled(&self) -> bool {
        self.enable_env_var().is_none_or(EnvKey::is_true)
    }

    pub(crate) fn iter_enabled() -> impl Iterator<Item = Self> {
        Self::iter().filter(Self::is_enabled)
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
        for member in Self::iter() {
            if member.as_str().eq_ignore_ascii_case(s) {
                return if member.is_enabled() {
                    Ok(member)
                } else {
                    let name = member.as_str();
                    let env_key = member
                        .enable_env_var()
                        .expect("disabled moniker must have an enable env var");
                    let env = env_key.name();
                    bail!("{name} plugin is not enabled; set {env}=true to enable (experimental)")
                };
            }
        }
        bail!("Invalid package manager moniker {s}")
    }
}
