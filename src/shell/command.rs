use crate::object_model::Environment;
use crate::result::Result;
use crate::util::path_to_str;
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
