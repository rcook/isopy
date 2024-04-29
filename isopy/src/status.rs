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

pub enum Status {
    Success,
    UserError,
}

macro_rules! return_success_quiet {
    () => {{
        return Ok(Status::Success);
    }};
}
pub(crate) use return_success_quiet;

macro_rules! return_success {
    () => {{
        log::info!("isopy completed successfully");
        return Ok(Status::Success);
    }};

    ($($arg: tt)*) => {{
        log::info!($($arg)*);
        return Ok(Status::Success);
    }};
}
pub(crate) use return_success;

macro_rules! return_user_error {
    ($($arg: tt)*) => {{
        log::error!($($arg)*);
        return Ok(Status::UserError);
    }};
}
pub(crate) use return_user_error;

pub fn show_error(error: &anyhow::Error) {
    eprintln!("{}", format!("{error}").bright_red());
    if read_env_bool(ISOPY_BACKTRACE_ENV_NAME) {
        eprintln!("stack backtrace:\n{}", error.backtrace());
    } else {
        #[cfg(debug_assertions)]
        eprintln!("{}", format!("note: run with `{ISOPY_BACKTRACE_ENV_NAME}={}` environment variable to display a backtrace", crate::env::BOOL_TRUE_VALUE).bright_white().bold());
    }
}
