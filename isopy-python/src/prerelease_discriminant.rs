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
use crate::prerelease_type::PrereleaseType;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct PrereleaseDiscriminant {
    prerelease_type: PrereleaseType,
    number: i32,
}

impl PrereleaseDiscriminant {
    pub(crate) const fn new(prerelease_type: PrereleaseType, number: i32) -> Self {
        Self {
            prerelease_type,
            number,
        }
    }
}

impl Display for PrereleaseDiscriminant {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.prerelease_type {
            PrereleaseType::Alpha => write!(f, "a{}", self.number),
            PrereleaseType::ReleaseCandidate => write!(f, "rc{}", self.number),
        }
    }
}
