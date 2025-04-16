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
use std::fmt::{Display, Formatter, Result as FmtResult};

#[allow(unused)]
pub(crate) struct MavenVersion {
    major: u32,
    minor: Option<u32>,
}

impl MavenVersion {
    #[allow(unused)]
    #[must_use]
    pub(crate) const fn new(major: u32, minor: Option<u32>) -> Self {
        Self { major, minor }
    }
}

impl Display for MavenVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(minor) = self.minor {
            write!(f, "{}.{}", self.major, minor)
        } else {
            write!(f, "{}", self.major)
        }
    }
}
