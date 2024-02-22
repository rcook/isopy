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
use isopy_lib::{serializable_newtype, TryToString};
use std::ffi::{OsStr, OsString};
use std::result::Result as StdResult;
use std::str::FromStr;

serializable_newtype!(WrapperFileName, OsString);

impl WrapperFileName {
    pub fn as_os_str(&self) -> &OsStr {
        self.0.as_os_str()
    }
}

impl FromStr for WrapperFileName {
    type Err = anyhow::Error;

    //#[cfg(any(target_os = "linux", target_os = "macos"))]
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        Ok(Self(OsString::from_str(s)?))
    }

    // TBD: Defer this check until time of creation of the file
    /*
    #[cfg(target_os = "windows")]
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        use anyhow::bail;

        let temp = s.to_lowercase();

        #[allow(clippy::case_sensitive_file_extension_comparisons)]
        if !temp.ends_with(".bat") && !temp.ends_with(".cmd") {
            bail!("wrapper file name must have .bat or .cmd extension");
        }

        Ok(Self(OsString::from_str(s)?))
    }
    */
}

impl TryToString for WrapperFileName {
    fn to_string_lossy(&self) -> String {
        self.0.to_string_lossy().to_string()
    }

    fn try_to_string(&self) -> Option<String> {
        self.0.to_str().map(String::from)
    }
}
