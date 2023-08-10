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
use anyhow::Result;
use isopy_lib::EnvInfo;
use std::env::{join_paths, set_var, split_paths, var_os};
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitStatus;

use super::IsopyEnv;

pub struct Command {
    program: Option<OsString>,
    args: Vec<OsString>,
}

impl Command {
    pub const fn new(program: OsString) -> Self {
        Self {
            program: Some(program),
            args: Vec::new(),
        }
    }

    pub const fn new_shell() -> Self {
        Self {
            program: None,
            args: Vec::new(),
        }
    }

    pub fn arg(&mut self, arg: OsString) -> &mut Self {
        self.args.push(arg);
        self
    }

    pub fn exec(&self, isopy_env: &IsopyEnv, env_info: &EnvInfo) -> Result<ExitStatus> {
        prepend_paths(&env_info.path_dirs)?;

        isopy_env.set_vars();

        for (key, value) in &env_info.vars {
            set_var(key, value);
        }

        self.exec_impl()
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn exec_impl(&self) -> Result<ExitStatus> {
        use anyhow::{anyhow, bail};
        use exec::execvp;
        use std::iter::once;

        let p = if let Some(program) = &self.program {
            program.clone()
        } else {
            var_os("SHELL").ok_or_else(|| anyhow!("SHELL environment variable is not available"))?
        };

        let err = execvp(&p, once(&p).chain(self.args.iter()));
        bail!(err);
    }

    #[cfg(target_os = "windows")]
    fn exec_impl(&self) -> Result<ExitStatus> {
        use crate::shell::{get_windows_shell_info, WindowsShellKind};
        use std::process::Command;

        let windows_shell_info = get_windows_shell_info()?;
        let mut command = match &self.program {
            Some(program) => match windows_shell_info.kind {
                WindowsShellKind::Cmd => {
                    let mut command = Command::new(windows_shell_info.path);
                    command.arg("/k");
                    command
                }
                WindowsShellKind::PowerShell => {
                    let mut command = Command::new(windows_shell_info.path);
                    command.arg("-NoProfile");
                    command.arg(program);
                    command
                }
            },
            None => match windows_shell_info.kind {
                WindowsShellKind::Cmd => {
                    let mut command = Command::new(windows_shell_info.path);
                    command.arg("/k");
                    command
                }
                WindowsShellKind::PowerShell => {
                    let mut command = Command::new(windows_shell_info.path);
                    command.arg("-NoExit");
                    command.arg("-NoProfile");
                    command
                }
            },
        };

        for arg in &self.args {
            command.arg(arg);
        }

        Ok(command.status()?)
    }
}

fn prepend_paths(paths: &[PathBuf]) -> Result<()> {
    let mut new_paths = paths.to_owned();
    if let Some(path) = var_os("PATH") {
        new_paths.extend(split_paths(&path));
    }

    set_var("PATH", join_paths(new_paths)?);

    Ok(())
}
