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
use crate::prerelease_kind::PrereleaseKind;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct PrereleaseInfo {
    kind: PrereleaseKind,
    number: i32,
}

impl PrereleaseInfo {
    pub(crate) const fn new(kind: PrereleaseKind, number: i32) -> Self {
        Self { kind, number }
    }
}

impl Display for PrereleaseInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            PrereleaseKind::Alpha => write!(f, "a{}", self.number),
            PrereleaseKind::Beta => write!(f, "b{}", self.number),
            PrereleaseKind::ReleaseCandidate => write!(f, "rc{}", self.number),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PrereleaseInfo;
    use crate::prerelease_kind::PrereleaseKind;

    #[test]
    fn order() {
        let info1 = PrereleaseInfo::new(PrereleaseKind::ReleaseCandidate, 2);
        let info2 = PrereleaseInfo::new(PrereleaseKind::ReleaseCandidate, 10);
        let info3 = PrereleaseInfo::new(PrereleaseKind::Alpha, 100);
        assert!(info1 < info2);
        assert!(info2 > info1);
        assert!(info1 > info3);
        assert!(info3 < info1);
        assert!(info2 > info3);
        assert!(info3 < info2);
    }
}
