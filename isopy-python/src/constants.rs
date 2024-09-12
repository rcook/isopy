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
use lazy_static::lazy_static;
use std::ffi::OsString;

lazy_static! {
    pub(crate) static ref PYTHON_BIN_FILE_NAME: OsString = python_bin_file_name();
    pub(crate) static ref PYTHON_SCRIPT_EXT: OsString = OsString::from("py");
}

pub(crate) const PLUGIN_NAME: &str = "Python";

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn python_bin_file_name() -> OsString {
    OsString::from("python3")
}

#[cfg(target_os = "windows")]
fn python_bin_file_name() -> OsString {
    OsString::from("python.exe")
}
