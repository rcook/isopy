use crate::config::Config;
use crate::error::Result;
use crate::object_model::EnvName;
use std::env::{set_var, var};

pub fn do_shell(config: &Config, env_name: &EnvName) -> Result<()> {
    do_shell_platform(config, env_name)
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn do_shell_platform(config: &Config, env_name: &EnvName) -> Result<()> {
    use exec::execvp;

    // TODO:
    //let temp = var("ISOPY_ENV");

    let shell = var("SHELL")?;
    println!("shell={}", shell);
    set_var("ISOPY_ENV", env_name.as_str());
    let python_bin_dir = env_name.dir(config).join("python/bin");
    let path = var("PATH")?;

    let mut new_path = String::new();
    // TODO: Is this right?
    new_path.push_str(&python_bin_dir.into_os_string().into_string().unwrap());
    new_path.push(':');
    new_path.push_str(&path);
    set_var("PATH", new_path);

    let _ = execvp(shell, ["bash"]);
    unreachable!()
}
