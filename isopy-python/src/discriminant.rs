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
use crate::prerelease_discriminant::PrereleaseDiscriminant;
use crate::prerelease_type::PrereleaseType;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Discriminant {
    Prerelease(PrereleaseDiscriminant),
    None,
}

impl Discriminant {
    pub(crate) const fn prerelease(prerelease_type: PrereleaseType, number: i32) -> Self {
        Self::Prerelease(PrereleaseDiscriminant::new(prerelease_type, number))
    }
}

impl Display for Discriminant {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Prerelease(d) => write!(f, "{d}")?,
            Self::None => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Discriminant;
    use crate::prerelease_type::PrereleaseType;

    #[test]
    fn order() {
        let discriminant1 = Discriminant::prerelease(PrereleaseType::Alpha, 10);
        let discriminant2 = Discriminant::prerelease(PrereleaseType::Alpha, 6);
        let discriminant3 = Discriminant::prerelease(PrereleaseType::Alpha, 5);
        let discriminant4 = Discriminant::prerelease(PrereleaseType::ReleaseCandidate, 3);
        let discriminant5 = Discriminant::prerelease(PrereleaseType::ReleaseCandidate, 10);
        assert!(discriminant1 > discriminant2);
        assert!(discriminant2 > discriminant3);
        assert!(discriminant4 > discriminant1);
        assert!(discriminant5 > discriminant4);
        assert!(discriminant1 < Discriminant::None);
        assert!(discriminant2 < Discriminant::None);
        assert!(discriminant3 < Discriminant::None);
        assert!(discriminant4 < Discriminant::None);
        assert!(discriminant5 < Discriminant::None);
    }
}
