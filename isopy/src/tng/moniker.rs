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
use crate::tng::plugin_manager::PluginManager;
use anyhow::{bail, Error, Result};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub(crate) struct Moniker(String);

impl Moniker {
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Moniker {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if PluginManager::new().get_plugin(s).is_err() {
            bail!("Unknown plugin moniker {s}");
        };

        Ok(Self(String::from(s)))
    }
}

impl Display for Moniker {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl AsRef<Path> for Moniker {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}
