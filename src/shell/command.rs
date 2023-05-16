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
#![allow(unused)]
use crate::object_model::Environment;
use crate::util::path_to_str;
use anyhow::Result;
use std::env::{set_var, var};
use std::ffi::OsString;
use std::process::ExitStatus;

pub const ISOPY_ENV_NAME: &str = "ISOPY_ENV";

pub struct Command {
    program: Option<OsString>,
    args: Vec<OsString>,
}

impl Command {
    pub fn new<S>(program: S) -> Self
    where
        S: Into<OsString>,
    {
        Self {
            program: Some(program.into()),
            args: Vec::new(),
        }
    }

    pub fn new_shell() -> Self {
        Self {
            program: None,
            args: Vec::new(),
        }
    }

    pub fn arg<S>(&mut self, arg: S) -> &mut Self
    where
        S: Into<OsString>,
    {
        self.args.push(arg.into());
        self
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub fn exec(&self, environment: &Environment) -> Result<ExitStatus> {
        use exec::execvp;
        use std::iter::once;

        set_var(ISOPY_ENV_NAME, environment.name.as_str());

        let mut new_path = String::new();
        let python_bin_dir = environment.full_python_dir.join("bin");
        new_path.push_str(path_to_str(&python_bin_dir)?);
        new_path.push(':');
        new_path.push_str(&var("PATH")?);
        set_var("PATH", new_path);

        match &self.program {
            Some(program) => {
                let _ = execvp(program, once(program).chain(self.args.iter()));
                unreachable!()
            }
            None => {
                let shell = OsString::from(var("SHELL")?);
                let _ = execvp(&shell, once(&shell).chain(self.args.iter()));
                unreachable!()
            }
        }
    }

    #[cfg(any(target_os = "windows"))]
    pub fn exec(&self, environment: &Environment) -> Result<ExitStatus> {
        use crate::shell::{get_windows_shell_info, WindowsShellKind};
        use std::process::Command;

        let windows_shell_info = get_windows_shell_info()?;

        set_var(ISOPY_ENV_NAME, environment.name.as_str());

        let mut new_path = String::new();
        new_path.push_str(path_to_str(&environment.full_python_dir)?);
        new_path.push(';');
        let python_scripts_dir = environment.full_python_dir.join("Scripts");
        new_path.push_str(path_to_str(&python_scripts_dir)?);
        new_path.push(';');
        new_path.push_str(&var("PATH")?);
        set_var("PATH", new_path);

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
