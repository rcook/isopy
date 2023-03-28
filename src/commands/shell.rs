use crate::config::Config;
use crate::error::{user, Result};
use crate::object_model::EnvName;
use crate::util::path_to_str;
use std::env::{set_var, var, VarError};

const ISOPY_ENV_NAME: &'static str = "ISOPY_ENV";

pub fn do_shell(config: &Config, env_name: &EnvName) -> Result<()> {
    do_shell_platform(config, env_name)
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn do_shell_platform(config: &Config, env_name: &EnvName) -> Result<()> {
    use exec::execvp;

    match var(ISOPY_ENV_NAME) {
        Ok(_) => {
            return Err(user("You are already in an isopy shell"));
        }
        Err(VarError::NotPresent) => {}
        Err(e) => return Err(e)?,
    }

    let shell = var("SHELL")?;
    set_var(ISOPY_ENV_NAME, env_name.as_str());
    let python_bin_dir = env_name.dir(config).join("python/bin");
    let mut new_path = String::new();
    new_path.push_str(path_to_str(&python_bin_dir)?);
    new_path.push(':');
    new_path.push_str(&var("PATH")?);
    set_var("PATH", new_path);

    let _ = execvp(shell, ["bash"]);
    unreachable!()
}
