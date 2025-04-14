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
use crate::discriminant::Discriminant;
use crate::release_group::ReleaseGroup;
use anyhow::Error;
use isopy_lib::{VersionOps, VersionTriple};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct PythonVersion {
    version: VersionTriple,
    discriminant: Discriminant,
    release_group: Option<ReleaseGroup>,
}

impl PythonVersion {
    pub(crate) const fn version(&self) -> &VersionTriple {
        &self.version
    }

    pub(crate) const fn discriminant(&self) -> &Discriminant {
        &self.discriminant
    }

    pub(crate) const fn release_group(&self) -> &Option<ReleaseGroup> {
        &self.release_group
    }
}

impl Display for PythonVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}{}", self.version, self.discriminant)?;
        if let Some(group) = &self.release_group {
            write!(f, ":{}", group.as_str())?;
        }
        Ok(())
    }
}

impl FromStr for PythonVersion {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let (prefix, release_group) = match s.rsplit_once(':') {
            Some((prefix, suffix)) => (prefix, Some(suffix.parse()?)),
            None => (s, None),
        };

        let (discriminant, prefix) = Discriminant::parse(prefix);
        let version = prefix.parse()?;
        Ok(Self {
            version,
            discriminant,
            release_group,
        })
    }
}

impl VersionOps for PythonVersion {
    fn box_clone(&self) -> Box<dyn VersionOps> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
