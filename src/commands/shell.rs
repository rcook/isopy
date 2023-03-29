use crate::app::App;
use crate::error::{user, Result};
use crate::object_model::EnvName;
use crate::shell::{get_shell_info, ISOPY_ENV_NAME};
use crate::util::path_to_str;
use std::env::{set_var, var, VarError};
use std::path::Path;
use std::process::ExitStatus;

pub fn do_shell(app: &App, env_name_opt: &Option<EnvName>) -> Result<()> {
    match var(ISOPY_ENV_NAME) {
        Ok(_) => {
            return Err(user("You are already in an isopy shell"));
        }
        Err(VarError::NotPresent) => {}
        Err(e) => return Err(e)?,
    }

    let shell_info = get_shell_info(app, env_name_opt)?;
    do_shell_platform(app, shell_info.env_name.as_str(), &shell_info.python_dir)?;
    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn do_shell_platform<P>(app: &App, env_name: &str, python_dir: P) -> Result<ExitStatus>
where
    P: AsRef<Path>,
{
    use exec::execvp;

    set_var(ISOPY_ENV_NAME, env_name);

    let mut new_path = String::new();
    let python_bin_dir = app.envs_dir.join(env_name).join(&python_dir).join("bin");
    new_path.push_str(path_to_str(&python_bin_dir)?);
    new_path.push(':');
    new_path.push_str(&var("PATH")?);
    set_var("PATH", new_path);

    let shell = var("SHELL")?;
    let _ = execvp(&shell, [&shell]);
    unreachable!()
}

#[cfg(any(target_os = "windows"))]
fn do_shell_platform<P>(app: &App, env_name: &str, python_dir: P) -> Result<ExitStatus>
where
    P: AsRef<Path>,
{
    use crate::util::{get_windows_shell_info, WindowsShellKind};
    use std::process::Command;

    let shell_info = get_windows_shell_info()?;

    set_var(ISOPY_ENV_NAME, env_name);

    let mut new_path = String::new();
    let python_bin_dir = app.envs_dir.join(env_name).join(&python_dir).join("bin");
    new_path.push_str(path_to_str(&python_bin_dir)?);
    new_path.push(';');
    let python_scripts_dir = python_bin_dir.join("Scripts");
    new_path.push_str(path_to_str(&python_scripts_dir)?);
    new_path.push(';');
    new_path.push_str(&var("PATH")?);
    set_var("PATH", new_path);

    Ok(match shell_info.kind {
        WindowsShellKind::Cmd => Command::new(shell_info.path).arg("/k").status()?,
        WindowsShellKind::PowerShell => Command::new(shell_info.path)
            .arg("-NoExit")
            .arg("-NoProfile")
            .status()?,
    })
}
