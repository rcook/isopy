use super::proc::{get_parent_pid, get_pid, get_process_from_pid};
use crate::error::{fatal, Result};
use lazy_static::lazy_static;
use same_file::is_same_file;
use std::env::var;
use std::path::{Path, PathBuf};
use sysinfo::{ProcessExt, System, SystemExt};

#[derive(Debug)]
pub struct WindowsShellInfo {
    pub path: &'static Path,
    pub kind: WindowsShellKind,
}

#[derive(Debug)]
pub enum WindowsShellKind {
    Cmd,
    PowerShell,
}

lazy_static! {
    static ref POWERSHELL_PATH: PathBuf =
        PathBuf::from(var("WINDIR").expect("WINDIR must be defined"))
            .join("System32")
            .join("WindowsPowerShell")
            .join("v1.0")
            .join("powershell.exe");
    static ref POWERSHELL: WindowsShellInfo = WindowsShellInfo {
        path: &POWERSHELL_PATH,
        kind: WindowsShellKind::PowerShell
    };
    static ref CMD_PATH: PathBuf = PathBuf::from(var("ComSpec").expect("ComSpec must be defined"));
    static ref CMD: WindowsShellInfo = WindowsShellInfo {
        path: &CMD_PATH,
        kind: WindowsShellKind::Cmd
    };
}

pub fn get_windows_shell_info() -> Result<&'static WindowsShellInfo> {
    let mut system = System::new();
    let pid = get_pid()?;
    let mut process = get_process_from_pid(&mut system, pid)?;
    loop {
        if process.name().is_empty() {
            return Err(fatal("Failed to determine parent shell"));
        }

        if is_same_file(&*POWERSHELL.path, &process.exe())? {
            return Ok(&POWERSHELL);
        }

        if is_same_file(&*CMD.path, &process.exe())? {
            return Ok(&CMD);
        }

        let parent_pid = get_parent_pid(process)?;
        process = get_process_from_pid(&mut system, parent_pid)?;
    }
}
