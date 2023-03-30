use crate::app::App;
use crate::error::{user, Result};
use crate::object_model::EnvName;
use crate::serialization::{AnonymousEnvRecord, NamedEnvRecord, UseRecord};
use crate::util::path_to_str;
use md5::compute;
use serde_yaml::from_str;
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

    let use_record = from_str::<UseRecord>(&read_to_string(&use_config_path)?)?;
    let named_env_config_path = app.named_env_dir(&use_record.env_name).join("env.yaml");
    if !named_env_config_path.is_file() {
        return Ok(None);
    }

    let named_env_record = from_str::<NamedEnvRecord>(&read_to_string(&named_env_config_path)?)?;
    return Ok(Some(ShellInfo {
        env_name: named_env_record.name.clone(),
        full_python_dir: app
            .named_env_dir(&named_env_record.name)
            .join(named_env_record.python_dir),
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
    let hashed_env_record = serde_yaml::from_str::<AnonymousEnvRecord>(&s)?;
    return Ok(Some(ShellInfo {
        env_name: EnvName::parse(&hex_digest).expect("Must be a valid environment"),
        full_python_dir: app
            .hashed_dir
            .join(&hex_digest)
            .join(hashed_env_record.python_dir),
    }));
}

pub fn get_shell_info(app: &App, env_name_opt: Option<&EnvName>) -> Result<ShellInfo> {
    if let Some(env_name) = env_name_opt {
        match app.read_named_env(env_name)? {
            Some(env_record) => {
                return Ok(ShellInfo {
                    env_name: env_record.name,
                    full_python_dir: app.named_env_dir(env_name).join(env_record.python_dir),
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
