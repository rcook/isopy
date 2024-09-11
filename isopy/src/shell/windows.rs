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
use crate::shell::proc::{get_parent_pid, get_pid, get_process_from_pid};
use anyhow::{bail, Result};
use lazy_static::lazy_static;
use same_file::is_same_file;
use std::env::var;
use std::path::PathBuf;
use sysinfo::System;

#[derive(Debug)]
pub(crate) struct WindowsShellInfo {
    pub(crate) path: PathBuf,
    pub(crate) kind: WindowsShellKind,
}

#[derive(Debug)]
pub(crate) enum WindowsShellKind {
    Cmd,
    PowerShell,
}

lazy_static! {
    static ref POWERSHELL_PATH: PathBuf =
        PathBuf::from(var("WINDIR").expect("lazy_static: WINDIR must be defined"))
            .join("System32")
            .join("WindowsPowerShell")
            .join("v1.0")
            .join("powershell.exe");
    static ref CMD_PATH: PathBuf =
        PathBuf::from(var("ComSpec").expect("lazy_static: ComSpec must be defined"));
}

pub(crate) fn get_windows_shell_info() -> Result<WindowsShellInfo> {
    let mut system = System::new();
    let pid = get_pid()?;
    let mut process = get_process_from_pid(&mut system, pid)?;
    loop {
        if process.name().is_empty() {
            bail!("Failed to determine parent shell");
        }

        if let Some(process_exe) = process.exe() {
            if is_same_file(&*POWERSHELL_PATH, process_exe)? {
                return Ok(WindowsShellInfo {
                    path: POWERSHELL_PATH.clone(),
                    kind: WindowsShellKind::PowerShell,
                });
            }

            if is_same_file(&*CMD_PATH, process_exe)? {
                return Ok(WindowsShellInfo {
                    path: CMD_PATH.clone(),
                    kind: WindowsShellKind::Cmd,
                });
            }

            if process.name() == "pwsh.exe" {
                return Ok(WindowsShellInfo {
                    path: process_exe.to_path_buf(),
                    kind: WindowsShellKind::PowerShell,
                });
            }
        }

        let parent_pid = get_parent_pid(process)?;
        process = get_process_from_pid(&mut system, parent_pid)?;
    }
}

#[cfg(test)]
mod tests {
    use crate::shell::windows::{CMD_PATH, POWERSHELL_PATH};

    #[test]
    fn cmd() {
        _ = format!("{}", CMD_PATH.display());
    }

    #[test]
    fn cmd_path() {
        _ = format!("{}", CMD_PATH.as_path().display());
    }

    #[test]
    fn powershell() {
        _ = format!("{}", POWERSHELL_PATH.display());
    }

    #[test]
    fn powershell_path() {
        _ = format!("{}", POWERSHELL_PATH.as_path().display());
    }
}
