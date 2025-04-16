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
use crate::package_availability::PackageAvailability;
use crate::version::Version;
use std::path::PathBuf;
use url::Url;

pub struct PackageInfo {
    pub availability: PackageAvailability,
    pub name: String,
    pub url: Url,
    pub version: Version,
    pub path: Option<PathBuf>,
}

impl PackageInfo {
    pub fn new<S: Into<String>, P: Into<PathBuf>>(
        availability: PackageAvailability,
        name: S,
        url: &Url,
        version: Version,
        path: Option<P>,
    ) -> Self {
        Self {
            availability,
            name: name.into(),
            url: url.clone(),
            version,
            path: path.map(Into::into),
        }
    }
}
