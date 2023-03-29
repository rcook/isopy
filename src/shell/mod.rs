mod command;
mod proc;
mod shell_info;
#[cfg(target_os = "windows")]
mod windows;

pub use self::command::Command;
pub use self::shell_info::{get_shell_info, ShellInfo, ISOPY_ENV_NAME};
#[cfg(target_os = "windows")]
pub use self::windows::{get_windows_shell_info, WindowsShellKind};
