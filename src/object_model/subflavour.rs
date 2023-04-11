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
#[derive(Debug, PartialEq)]
pub enum Subflavour {
    Debug,
    NoOpt,
    PgoLto,
    Pgo,
    Lto,
    Shared,
    Static,
}

impl Subflavour {
    pub fn parse<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Some(match s.as_ref() {
            "debug" => Self::Debug,
            "noopt" => Self::NoOpt,
            "pgo+lto" => Self::PgoLto,
            "pgo" => Self::Pgo,
            "lto" => Self::Lto,
            "shared" => Self::Shared,
            "static" => Self::Static,
            _ => return None,
        })
    }
}
