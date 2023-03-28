use crate::app::App;
use crate::error::{user, Result};
use crate::object_model::EnvName;
use crate::serialization::{EnvRecord, HashedEnvRecord, UseRecord};
use crate::util::path_to_str;
use md5::compute;
use std::env::{set_var, var, VarError};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

const ISOPY_ENV_NAME: &'static str = "ISOPY_ENV";

struct ShellInfo {
    env_name: EnvName,
    python_dir: PathBuf,
}

fn get_use_shell_info(app: &App) -> Result<Option<ShellInfo>> {
    let hex_digest = format!("{:x}", compute(path_to_str(&app.cwd)?));
    let use_config_path = app.uses_dir.join(&hex_digest).join("use.yaml");
    if !use_config_path.is_file() {
        return Ok(None);
    }

    let s = read_to_string(use_config_path)?;
    let use_record = serde_yaml::from_str::<UseRecord>(&s)?;
    let env_config_path = app
        .envs_dir
        .join(use_record.env_name.as_str())
        .join("env.yaml");
    if !env_config_path.is_file() {
        return Ok(None);
    }

    let s = read_to_string(env_config_path)?;
    let env_record = serde_yaml::from_str::<EnvRecord>(&s)?;
    return Ok(Some(ShellInfo {
        env_name: env_record.name,
        python_dir: env_record.python_dir,
    }));
}

fn get_project_shell_info(app: &App) -> Result<Option<ShellInfo>> {
    let project_config_path = app.cwd.join(".isopy.yaml");
    if !project_config_path.is_file() {
        return Ok(None);
    }

    let hex_digest = format!("{:x}", compute(path_to_str(&project_config_path)?));
    let env_config_path = app.hashed_dir.join(&hex_digest).join("env.yaml");
    if !env_config_path.is_file() {
        return Ok(None);
    }

    let s = read_to_string(env_config_path)?;
    let hashed_env_record = serde_yaml::from_str::<HashedEnvRecord>(&s)?;
    return Ok(Some(ShellInfo {
        env_name: EnvName::parse(&hex_digest).expect("Must be a valid environment"),
        python_dir: hashed_env_record.python_dir,
    }));
}

fn get_shell_info(app: &App) -> Result<Option<ShellInfo>> {
    if let Some(shell_info) = get_use_shell_info(app)? {
        return Ok(Some(shell_info));
    }

    if let Some(shell_info) = get_project_shell_info(app)? {
        return Ok(Some(shell_info));
    }

    Ok(None)
}

pub fn do_shell(app: &App, env_name_opt: &Option<EnvName>) -> Result<()> {
    match var(ISOPY_ENV_NAME) {
        Ok(_) => {
            return Err(user("You are already in an isopy shell"));
        }
        Err(VarError::NotPresent) => {}
        Err(e) => return Err(e)?,
    }

    match env_name_opt {
        Some(env_name) => {
            let env = match app.read_env(env_name)? {
                Some(x) => x,
                _ => return Err(user(format!("No environment named {}", env_name))),
            };

            do_shell_platform(app, &env.name.as_str(), &env.python_dir)?;
        }
        _ => match get_shell_info(app)? {
            Some(shell_info) => {
                do_shell_platform(app, shell_info.env_name.as_str(), &shell_info.python_dir)?;
            }
            _ => todo!(),
        },
    };

    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn do_shell_platform<P>(app: &App, env_name: &str, python_dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    use exec::execvp;

    let shell = var("SHELL")?;
    set_var(ISOPY_ENV_NAME, env_name);
    let python_bin_dir = app.envs_dir.join(env_name).join(&python_dir).join("bin");
    let mut new_path = String::new();
    new_path.push_str(path_to_str(&python_bin_dir)?);
    new_path.push(':');
    new_path.push_str(&var("PATH")?);
    set_var("PATH", new_path);

    let _ = execvp(shell, ["bash"]);
    unreachable!()
}
