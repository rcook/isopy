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
use crate::constants::{ENV_FILE_NAME, ISOPY_ENV_NAME};
use crate::serialization::EnvRec;
use crate::status::Status;
use anyhow::Result;
use colored::Colorize;
use joatmon::read_yaml_file;
use std::env::{var, VarError};

pub fn do_prompt(app: &App) -> Result<Status> {
    let isopy_env = match var(ISOPY_ENV_NAME) {
        Ok(s) => Some(s),
        Err(VarError::NotPresent) => None,
        Err(e) => return Err(e)?,
    };

    let env_rec_opt = app
        .find_dir_info(&app.cwd, isopy_env.clone())?
        .map(|d| d.data_dir().join(ENV_FILE_NAME))
        .filter(|p| p.is_file())
        .map(|p| read_yaml_file::<EnvRec>(&p))
        .transpose()?;

    let mut prompt = String::new();

    if isopy_env.is_some() {
        if let Some(env_rec) = env_rec_opt {
            if let Some(rec) = env_rec.python {
                prompt.push_str(&format!("python-{}", rec.version));
            }
        } else {
            prompt.push_str("unknown-python");
        }
    } else if let Some(env_rec) = env_rec_opt {
        if let Some(rec) = env_rec.python {
            prompt.push_str(&format!(
                "Run \"isopy shell\" to use Python {}",
                rec.version
            ));
        }
    }

    if !prompt.is_empty() {
        print!("{} ", prompt.bright_magenta());
    }

    Ok(Status::OK)
}