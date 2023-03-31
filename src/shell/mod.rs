mod command;
mod proc;
#[cfg(target_os = "windows")]
mod windows;

pub use self::command::Command;
#[cfg(target_os = "windows")]
pub use self::windows::{get_windows_shell_info, WindowsShellKind};
