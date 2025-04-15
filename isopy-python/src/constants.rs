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
use std::ffi::OsString;
use std::sync::LazyLock;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub(crate) static PYTHON_BIN_FILE_NAME: LazyLock<OsString> =
    LazyLock::new(|| OsString::from("python3"));

#[cfg(target_os = "windows")]
pub(crate) static PYTHON_BIN_FILE_NAME: LazyLock<OsString> =
    LazyLock::new(|| OsString::from("python.exe"));

pub(crate) static PYTHON_SCRIPT_EXT: LazyLock<OsString> = LazyLock::new(|| OsString::from("py"));

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub(crate) const PLATFORM_TAGS: [&str; 5] = ["aarch64", "unknown", "linux", "gnu", "install_only"];

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub(crate) const PLATFORM_TAGS: [&str; 5] = ["x86_64", "unknown", "linux", "gnu", "install_only"];

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub(crate) const PLATFORM_TAGS: [&str; 4] = ["aarch64", "apple", "darwin", "install_only"];

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub(crate) const PLATFORM_TAGS: [&str; 4] = ["x86_64", "apple", "darwin", "install_only"];

#[cfg(target_os = "windows")]
pub(crate) const PLATFORM_TAGS: [&str; 6] =
    ["x86_64", "pc", "windows", "msvc", "shared", "install_only"];
