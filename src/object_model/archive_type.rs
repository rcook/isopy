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
pub enum ArchiveType {
    TarGZ,
    TarZST,
}

impl ArchiveType {
    pub fn parse<S>(s: S) -> Option<(Self, String)>
    where
        S: AsRef<str>,
    {
        let s0 = s.as_ref();
        for (ext, archive_type) in [(".tar.gz", Self::TarGZ), (".tar.zst", Self::TarZST)] {
            if s0.ends_with(ext) {
                let ext_len = ext.len();
                let base_name = &s0[..s0.len() - ext_len];
                return Some((archive_type, String::from(base_name)));
            }
        }
        None
    }
}
