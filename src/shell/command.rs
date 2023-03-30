use super::{ShellInfo, ISOPY_ENV_NAME};
use crate::app::App;
use crate::error::Result;
use crate::util::path_to_str;
use std::env::{set_var, var};
use std::ffi::OsString;
use std::process::ExitStatus;

pub struct Command {
    args: Vec<OsString>,
}

impl Command {
    pub fn new<S>(program: S) -> Self
    where
        S: Into<OsString>,
    {
        Self {
            args: vec![program.into()],
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
    pub fn exec(&self, app: &App, shell_info: &ShellInfo) -> Result<ExitStatus> {
        use exec::execvp;

        set_var(ISOPY_ENV_NAME, shell_info.env_name.as_str());

        let mut new_path = String::new();
        let python_bin_dir = app
            .env_dir(&shell_info.env_name)
            .join(&shell_info.python_dir)
            .join("bin");
        new_path.push_str(path_to_str(&python_bin_dir)?);
        new_path.push(':');
        new_path.push_str(&var("PATH")?);
        set_var("PATH", new_path);

        let shell = var("SHELL")?;
        let _ = execvp(&shell, self.args.iter());
        unreachable!()
    }

    #[cfg(any(target_os = "windows"))]
    pub fn exec(&self, app: &App, shell_info: &ShellInfo) -> Result<ExitStatus> {
        use crate::shell::{get_windows_shell_info, WindowsShellKind};
        use std::process::Command;

        let windows_shell_info = get_windows_shell_info()?;

        set_var(ISOPY_ENV_NAME, shell_info.env_name.as_str());

        let mut new_path = String::new();
        let python_bin_dir = app
            .env_dir(&shell_info.env_name)
            .join(&shell_info.full_python_dir);
        new_path.push_str(path_to_str(&python_bin_dir)?);
        new_path.push(';');
        let python_scripts_dir = python_bin_dir.join("Scripts");
        new_path.push_str(path_to_str(&python_scripts_dir)?);
        new_path.push(';');
        new_path.push_str(&var("PATH")?);
        set_var("PATH", new_path);

        Ok(match windows_shell_info.kind {
            WindowsShellKind::Cmd => Command::new(windows_shell_info.path).arg("/k").status()?,
            WindowsShellKind::PowerShell => Command::new(windows_shell_info.path)
                .arg("-NoExit")
                .arg("-NoProfile")
                .status()?,
        })
    }
}