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
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Extra {
    Stable,
    ReleaseCandidate(u32),
    Beta(u32),
}

impl PartialOrd for Extra {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Extra {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Stable => match other {
                Self::Stable => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Self::ReleaseCandidate(this) => match other {
                Self::Stable => Ordering::Less,
                Self::ReleaseCandidate(that) => this.cmp(that),
                Self::Beta(_) => Ordering::Greater,
            },
            Self::Beta(this) => match other {
                Self::Beta(that) => this.cmp(that),
                _ => Ordering::Less,
            },
        }
    }
}
