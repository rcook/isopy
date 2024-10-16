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
use crate::env::EnvKey;
use anyhow::Error;
use colored::Colorize;

#[derive(Debug)]
pub(crate) enum Status {
    Success,
    UserError(String),
}

#[macro_export]
macro_rules! success_quiet {
    () => {
        return ::std::result::Result::Ok($crate::status::Status::Success);
    };
}
pub(crate) use success_quiet;

#[macro_export]
macro_rules! success {
    () => {{
        ::log::info!("isopy completed successfully");
        return ::std::result::Result::Ok($crate::status::Status::Success);
    }};

    ($($arg: tt)*) => {{
        ::log::info!($($arg)*);
        return ::std::result::Result::Ok($crate::status::Status::Success);
    }};
}
pub(crate) use success;

macro_rules! user_error {
    ($($arg: tt)*) => {{
        return ::std::result::Result::Ok($crate::status::Status::UserError(std::format!($($arg)*)));
    }};
}
pub(crate) use user_error;

#[macro_export]
macro_rules! report_install_package_error {
    ($result: expr) => {
        match $result {
            ::std::result::Result::Ok(value) => value,
            ::std::result::Result::Err(::isopy_lib::InstallPackageError::VersionNotFound) => $crate::status::user_error!("Specified version of one or more packages was not found in index"),
            ::std::result::Result::Err(::isopy_lib::InstallPackageError::PackageNotDownloaded) => $crate::status::user_error!("One or more specified packages has not been downloaded: run \"download\" command or run this command with \"--download\" option"),
            ::std::result::Result::Err(::isopy_lib::InstallPackageError::Other(e)) => return ::std::result::Result::Err(e),
        }
    };
}
pub(crate) use report_install_package_error;

pub(crate) fn show_user_error(message: &str) {
    eprintln!("{}", message.to_string().bright_yellow());
}

pub(crate) fn show_error(error: &Error) {
    eprintln!("{}", format!("{error}").bright_red());
    if EnvKey::BacktraceEnabled.to_bool() {
        eprintln!("stack backtrace:\n{}", error.backtrace());
    } else {
        #[cfg(debug_assertions)]
        eprintln!(
            "{}",
            format!(
                "note: run with `{name}={value}` environment variable to display a backtrace",
                name = EnvKey::BacktraceEnabled,
                value = crate::env::BOOL_TRUE_VALUE
            )
            .bright_white()
            .bold()
        );
    }
}
