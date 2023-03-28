use crate::config::Config;
use crate::error::{user, Result};
use crate::object_model::EnvName;
use crate::serialization::HashedEnvRecord;
use crate::util::path_to_str;
use md5::compute;
use std::env::{set_var, var, VarError};
use std::fs::read_to_string;
use std::path::Path;

const ISOPY_ENV_NAME: &'static str = "ISOPY_ENV";

pub fn do_shell(config: &Config, env_name_opt: &Option<EnvName>) -> Result<()> {
    match var(ISOPY_ENV_NAME) {
        Ok(_) => {
            return Err(user("You are already in an isopy shell"));
        }
        Err(VarError::NotPresent) => {}
        Err(e) => return Err(e)?,
    }

    match env_name_opt {
        Some(env_name) => {
            let env = match config.read_env(env_name)? {
                Some(x) => x,
                _ => return Err(user(format!("No environment named {}", env_name))),
            };

            do_shell_platform(config, &env.name.as_str(), &env.python_dir)?;
        }
        _ => {
            let config_path = config.cwd.join(".isopy.yaml");
            let hex_digest = format!("{:x}", compute(path_to_str(&config_path)?));
            let env_config_path = config.dir.join("hashed").join(hex_digest).join("env.yaml");
            let s = read_to_string(env_config_path)?;
            let hashed_env_record = serde_yaml::from_str::<HashedEnvRecord>(&s)?;

            do_shell_platform(
                config,
                path_to_str(&hashed_env_record.config_path)?,
                &hashed_env_record.python_dir,
            )?;
        }
    };

    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn do_shell_platform<P>(config: &Config, env_name: &str, python_dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    use exec::execvp;

    let shell = var("SHELL")?;
    set_var(ISOPY_ENV_NAME, env_name);
    let python_bin_dir = config.envs_dir.join(env_name).join(&python_dir).join("bin");
    let mut new_path = String::new();
    new_path.push_str(path_to_str(&python_bin_dir)?);
    new_path.push(':');
    new_path.push_str(&var("PATH")?);
    set_var("PATH", new_path);

    let _ = execvp(shell, ["bash"]);
    unreachable!()
}
