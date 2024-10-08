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
use anyhow::Error;
use std::ffi::OsString;
use std::path::{Component, Path, PathBuf};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub(crate) enum WrapperFileName {
    FileNameOnly(OsString),
    Path(PathBuf),
}

impl FromStr for WrapperFileName {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let path = Path::new(s);

        let components = path.components().collect::<Vec<_>>();
        if components.len() == 1 {
            if let Some(Component::Normal(c)) = components.first() {
                return Ok(Self::FileNameOnly(c.to_os_string()));
            }
        }

        Ok(Self::Path(path.to_path_buf()))
    }
}
