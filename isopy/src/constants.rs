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
use std::path::PathBuf;

lazy_static! {
    pub static ref CACHE_DIR: PathBuf = PathBuf::from(".isopy");
    pub static ref ENV_CONFIG_FILE_NAME: OsString = OsString::from("env.yaml");
    pub static ref PROJECT_CONFIG_FILE_NAME: OsString = OsString::from(".isopy.yaml");
}

pub const PYTHON_DESCRIPTOR_PREFIX: &str = "python";

pub const JDK_DESCRIPTOR_PREFIX: &str = "jdk";

pub const JRE_DESCRIPTOR_PREFIX: &str = "jre";

pub const EXECUTABLE_MASK: u32 = 0o100;
