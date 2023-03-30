use crate::app::App;
use crate::error::{user, Result};
use crate::object_model::EnvName;
use crate::serialization::{EnvRecord, HashedEnvRecord, UseRecord};
use crate::util::path_to_str;
use md5::compute;
use std::fs::read_to_string;
use std::path::PathBuf;

pub const ISOPY_ENV_NAME: &'static str = "ISOPY_ENV";

#[derive(Debug)]
pub struct ShellInfo {
    pub env_name: EnvName,
    pub full_python_dir: PathBuf,
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
        full_python_dir: app.envs_dir.join(&hex_digest).join(env_record.python_dir),
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
        full_python_dir: app
            .hashed_dir
            .join(&hex_digest)
            .join(hashed_env_record.python_dir),
    }));
}

pub fn get_shell_info(app: &App, env_name_opt: &Option<EnvName>) -> Result<ShellInfo> {
    if let Some(env_name) = env_name_opt {
        match app.read_env(&env_name)? {
            Some(env_record) => {
                return Ok(ShellInfo {
                    env_name: env_record.name,
                    full_python_dir: app
                        .envs_dir
                        .join(env_name.as_str())
                        .join(env_record.python_dir),
                })
            }
            _ => return Err(user(format!("No environment named {}", env_name))),
        };
    }

    if let Some(shell_info) = get_use_shell_info(app)? {
        return Ok(shell_info);
    }

    if let Some(shell_info) = get_project_shell_info(app)? {
        return Ok(shell_info);
    }

    Err(user(format!(
        "Couldn't infer environment for directory {}",
        app.cwd.display()
    )))
}
