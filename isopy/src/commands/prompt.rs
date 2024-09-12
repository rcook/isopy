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
use crate::args::PromptConfig;
use crate::dir_info_ext::DirInfoExt;
use crate::fs::existing;
use crate::shell::IsopyEnv;
use crate::status::{return_success_quiet, Status};
use anyhow::Result;

pub(crate) fn do_prompt(app: &App, prompt_config: &PromptConfig) -> Result<Status> {
    let isopy_env = IsopyEnv::get_vars()?;
    let env = if let Some(d) = app.find_dir_info(isopy_env.as_ref())? {
        existing(d.read_env_config())?
    } else {
        None
    };

    let has_project_config_file = app.has_project_config_file();

    let prompt_message = match (isopy_env.is_some(), env.is_some(), has_project_config_file) {
        (true, true, _) => Some(prompt_config.shell_message.as_deref().unwrap_or("(isopy)")),
        (true, false, _) => Some(prompt_config.error_message.as_deref().unwrap_or("(!)")),
        (false, true, _) => Some(prompt_config.available_message.as_deref().unwrap_or("(*)")),
        (_, _, true) => Some(
            prompt_config
                .config_message
                .as_deref()
                .unwrap_or("(config)"),
        ),
        (false, false, false) => None,
    };

    if let Some(m) = prompt_message {
        print!(
            "{before}{m}{after}",
            before = prompt_config.before.as_deref().unwrap_or_default(),
            after = prompt_config.after.as_deref().unwrap_or_default()
        );
    }

    return_success_quiet!();
}
