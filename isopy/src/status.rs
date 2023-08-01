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
use crate::env::RUST_BACKTRACE_ENV_NAME;
use colored::Colorize;
use std::env::var;

const RUST_BACKTRACE_ENV_TRUE_VALUE: &str = "1";

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

#[cfg(debug_assertions)]
pub fn init_backtrace() {
    use crate::env::{read_env_var_bool, ISOPY_BYPASS_ENV_ENV_NAME};
    use std::env::{set_var, VarError};

    if !read_env_var_bool(ISOPY_BYPASS_ENV_ENV_NAME)
        && var(RUST_BACKTRACE_ENV_NAME) == Err(VarError::NotPresent)
    {
        set_var(RUST_BACKTRACE_ENV_NAME, RUST_BACKTRACE_ENV_TRUE_VALUE);
    }
}

#[cfg(not(debug_assertions))]
pub fn init_backtrace() {}

pub fn show_error(error: &anyhow::Error) {
    eprintln!("{}", format!("{error}").bright_red());
    if is_backtrace_enabled() {
        eprintln!("stack backtrace:\n{}", error.backtrace());
    } else {
        #[cfg(debug_assertions)]
        eprintln!("{}", format!("Set environment variable {RUST_BACKTRACE_ENV_NAME}={RUST_BACKTRACE_ENV_TRUE_VALUE} to see backtrace").bright_white().bold());
    }
}

fn is_backtrace_enabled() -> bool {
    var(RUST_BACKTRACE_ENV_NAME) == Ok(String::from("1"))
}
