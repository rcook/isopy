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
use isopy_lib::VersionOps;
use serde::Deserialize;
use std::any::Any;
use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Version {
    #[allow(unused)]
    #[serde(rename = "major")]
    pub(crate) major: u32,

    #[allow(unused)]
    #[serde(rename = "minor")]
    pub(crate) minor: u32,

    #[allow(unused)]
    #[serde(rename = "build")]
    pub(crate) build: u32,

    #[allow(unused)]
    #[allow(clippy::struct_field_names)]
    #[serde(rename = "openjdk_version")]
    pub(crate) openjdk_version: String,

    #[allow(unused)]
    #[serde(rename = "optional")]
    pub(crate) optional: Option<String>,

    #[allow(unused)]
    #[serde(rename = "pre")]
    pub(crate) pre: Option<String>,

    #[allow(unused)]
    #[serde(rename = "security")]
    pub(crate) security: u32,

    #[allow(unused)]
    #[serde(rename = "semver")]
    pub(crate) semver: String,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.semver)
    }
}

impl VersionOps for Version {
    fn as_str(&self) -> Cow<'_, String> {
        Cow::Owned(format!("{self}"))
    }

    fn label(&self) -> Option<Cow<'_, String>> {
        None
    }

    fn box_clone(&self) -> Box<dyn VersionOps> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
