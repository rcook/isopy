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
use crate::app::App;
use crate::constants::ISOPY_ENV_NAME;
use crate::dir_info_ext::DirInfoExt;
use crate::fs::existing;
use crate::status::{return_success_quiet, Status};
use anyhow::Result;
use colored::Colorize;
use std::env::{var, VarError};

pub fn prompt(app: &App) -> Result<Status> {
    let isopy_env = match var(ISOPY_ENV_NAME) {
        Ok(s) => Some(s),
        Err(VarError::NotPresent) => None,
        Err(e) => return Err(e)?,
    };

    let env_rec = if let Some(d) = app.find_dir_info(isopy_env.clone())? {
        existing(d.read_env_config())?
    } else {
        None
    };

    let prompt_message = match (isopy_env.is_some(), env_rec.is_some()) {
        (true, true) => Some("env"),
        (true, false) => Some("error"),
        (false, true) => Some("env available"),
        (false, false) => None,
    };

    if let Some(m) = prompt_message {
        print!("{} ", format!("(isopy {m})").bright_magenta());
    }

    return_success_quiet!();
}
