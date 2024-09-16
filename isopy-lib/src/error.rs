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
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstallPackageError {
    #[error("Version not found")]
    VersionNotFound,

    #[error("Package not downloaded")]
    PackageNotDownloaded,

    #[error("Other")]
    Other(#[from] anyhow::Error),
}

#[macro_export]
macro_rules! install_package_bail {
    ($msg: literal $(,)?) => {
        return std::result::Result::Err($crate::InstallPackageError::Other(anyhow::anyhow!($msg)))
    };
    ($err: expr $(,)?) => {
        return std::result::Result::Err($crate::InstallPackageError::Other(anyhow::anyhow!($err)));
    };
    ($fmt: expr, $($arg: tt)*) => {
        return std::result::Result::Err($crate::InstallPackageError::Other(anyhow::anyhow!($fmt, $($arg)*)));
    };
}

#[macro_export]
macro_rules! install_package_error {
    ($err: expr) => {
        InstallPackageError::Other(anyhow::anyhow!($err))
    };
}
