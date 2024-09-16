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
use crate::env::{read_env_bool, ISOPY_BACKTRACE_ENV_NAME};
use colored::Colorize;

#[derive(Debug)]
pub(crate) enum Status {
    Success,
    UserError,
}

macro_rules! return_success_quiet {
    () => {
        return Ok($crate::status::Status::Success);
    };
}
pub(crate) use return_success_quiet;

macro_rules! return_success {
    () => {{
        log::info!("isopy completed successfully");
        return Ok($crate::status::Status::Success);
    }};

    ($($arg: tt)*) => {{
        log::info!($($arg)*);
        return Ok($crate::status::Status::Success);
    }};
}
pub(crate) use return_success;

macro_rules! return_user_error {
    ($($arg: tt)*) => {{
        log::error!($($arg)*);
        return Ok($crate::status::Status::UserError);
    }};
}
pub(crate) use return_user_error;

macro_rules! report_install_package_error {
    ($result: expr) => {
        match $result {
            Ok(value) => value,
            Err(isopy_lib::InstallPackageError::VersionNotFound) => $crate::status::return_user_error!("Specified version of one or more packages was not found in index"),
            Err(isopy_lib::InstallPackageError::PackageNotDownloaded) => $crate::status::return_user_error!("One or more specified packages has not been downloaded: run \"download\" command or run this command with \"--download\" option"),
            Err(isopy_lib::InstallPackageError::Other(e)) => return Err(e),
        }
    };
}
pub(crate) use report_install_package_error;

pub(crate) fn show_error(error: &anyhow::Error) {
    eprintln!("{}", format!("{error}").bright_red());
    if read_env_bool(ISOPY_BACKTRACE_ENV_NAME) {
        eprintln!("stack backtrace:\n{}", error.backtrace());
    } else {
        #[cfg(debug_assertions)]
        eprintln!("{}", format!("note: run with `{ISOPY_BACKTRACE_ENV_NAME}={}` environment variable to display a backtrace", crate::env::BOOL_TRUE_VALUE).bright_white().bold());
    }
}
