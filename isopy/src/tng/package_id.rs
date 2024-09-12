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
use crate::tng::moniker::Moniker;
use crate::tng::plugin_manager::PluginManager;
use anyhow::{bail, Error, Result};
use isopy_lib::tng::Version;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub(crate) struct PackageId {
    moniker: Moniker,
    version: Version,
}

impl PackageId {
    pub(crate) const fn moniker(&self) -> &Moniker {
        &self.moniker
    }

    pub(crate) const fn version(&self) -> &Version {
        &self.version
    }
}

impl FromStr for PackageId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let Some((moniker_str, version_str)) = s.split_once(':') else {
            bail!("Invalid package ID {s}")
        };

        let moniker = moniker_str.parse::<Moniker>()?;
        let plugin_manager = PluginManager::new();
        let plugin = plugin_manager.get_plugin(&moniker);

        let Ok(version) = plugin.parse_version(version_str) else {
            bail!("Invalid version string {s} for plugin {moniker}");
        };

        Ok(Self { moniker, version })
    }
}

impl Display for PackageId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}:{}", self.moniker, *self.version)
    }
}
