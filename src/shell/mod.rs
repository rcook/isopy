mod exec;
mod proc;
mod shell_info;
mod unix;
mod windows;

pub use self::exec::exec;
pub use self::shell_info::{get_shell_info, ShellInfo, ISOPY_ENV_NAME};
