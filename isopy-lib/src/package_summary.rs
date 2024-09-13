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
use crate::package_kind::PackageKind;
use crate::version::Version;
use std::path::PathBuf;
use url::Url;

pub struct PackageSummary {
    kind: PackageKind,
    name: String,
    url: Url,
    version: Version,
    label: String,
    path: Option<PathBuf>,
}

impl PackageSummary {
    pub fn new<S1: Into<String>, S2: Into<String>, P: Into<PathBuf>>(
        kind: PackageKind,
        name: S1,
        url: &Url,
        version: Version,
        label: S2,
        path: Option<P>,
    ) -> Self {
        Self {
            kind,
            name: name.into(),
            url: url.clone(),
            version,
            label: label.into(),
            path: path.map(std::convert::Into::into),
        }
    }
    #[must_use]
    pub const fn kind(&self) -> PackageKind {
        self.kind
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn url(&self) -> &Url {
        &self.url
    }

    #[must_use]
    pub const fn version(&self) -> &Version {
        &self.version
    }

    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub const fn path(&self) -> &Option<PathBuf> {
        &self.path
    }
}
