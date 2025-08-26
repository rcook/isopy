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
mod app;
mod args;
mod bool_util;
mod cache;
mod commands;
mod constants;
mod date_time_format;
mod dir_info_ext;
mod env;
mod fs;
mod moniker;
mod package_id;
mod package_manager_helper;
mod paginated_download;
mod plugin_manager;
mod print;
mod run;
mod serialization;
mod shell;
mod status;
mod table;
mod terminal;
mod ui;
mod wrapper_file_name;

#[tokio::main]
async fn main() -> std::process::ExitCode {
    use crate::env::EnvKey;
    use crate::run::run;
    use crate::status::Status::{Success, UserError};
    use anyhow::Error;
    use colored::Colorize;
    use std::process::ExitCode;

    fn show_message(message: &str) {
        println!("{}", message.to_string().bright_green());
    }

    fn show_user_error(message: &str) {
        eprintln!("{}", message.to_string().bright_yellow());
    }

    fn show_error(error: &Error) {
        eprintln!("{}", error.to_string().bright_red());
        if EnvKey::BacktraceEnabled.is_true() {
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

    const USER_ERROR: u8 = 2;

    match run().await {
        Ok(Success(Some(message))) => {
            show_message(&message);
            ExitCode::SUCCESS
        }
        Ok(Success(None)) => ExitCode::SUCCESS,
        Ok(UserError(message)) => {
            show_user_error(&message);
            ExitCode::from(USER_ERROR)
        }
        Err(e) => {
            show_error(&e);
            ExitCode::FAILURE
        }
    }
}
