use crate::app::App;
use crate::error::{user, Result};
use crate::object_model::EnvName;
use crate::serialization::{AnonymousEnvRecord, NamedEnvRecord, UseRecord};
use serde_yaml::from_str;
use std::fs::read_to_string;
use std::path::PathBuf;

pub const ISOPY_ENV_NAME: &'static str = "ISOPY_ENV";

#[derive(Debug)]
pub struct EnvInfo {
    pub env_name: EnvName,
    pub full_python_dir: PathBuf,
}

fn get_use_env_info(app: &App) -> Result<Option<EnvInfo>> {
    let use_dir = app.use_dir(&app.cwd)?;
    let use_config_path = use_dir.join("use.yaml");
    if !use_config_path.is_file() {
        return Ok(None);
    }

    let use_record = from_str::<UseRecord>(&read_to_string(&use_config_path)?)?;
    let named_env_config_path = app.named_env_dir(&use_record.env_name).join("env.yaml");
    if !named_env_config_path.is_file() {
        return Ok(None);
    }

    let named_env_record = from_str::<NamedEnvRecord>(&read_to_string(&named_env_config_path)?)?;
    return Ok(Some(EnvInfo {
        env_name: named_env_record.name.clone(),
        full_python_dir: app
            .named_env_dir(&named_env_record.name)
            .join(named_env_record.python_dir),
    }));
}

fn get_project_env_info(app: &App) -> Result<Option<EnvInfo>> {
    let project_config_path = app.cwd.join(".isopy.yaml");
    if !project_config_path.is_file() {
        return Ok(None);
    }

    let anonymous_env_dir = app.anonymous_env_dir(&project_config_path)?;
    let anonymous_env_config_path = anonymous_env_dir.join("env.yaml");
    if !anonymous_env_config_path.is_file() {
        return Ok(None);
    }

    let s = read_to_string(anonymous_env_config_path)?;
    let anonymous_env_record = serde_yaml::from_str::<AnonymousEnvRecord>(&s)?;
    return Ok(Some(EnvInfo {
        env_name: EnvName::parse("ANONYMOUS").expect("Must be a valid environment"),
        full_python_dir: anonymous_env_dir.join(anonymous_env_record.python_dir),
    }));
}

pub fn get_env_info(app: &App, env_name_opt: Option<&EnvName>) -> Result<EnvInfo> {
    if let Some(env_name) = env_name_opt {
        match app.read_named_env(env_name)? {
            Some(env_record) => {
                return Ok(EnvInfo {
                    env_name: env_record.name,
                    full_python_dir: app.named_env_dir(env_name).join(env_record.python_dir),
                })
            }
            _ => return Err(user(format!("No environment named {}", env_name))),
        };
    }

    if let Some(env_info) = get_use_env_info(app)? {
        return Ok(env_info);
    }

    if let Some(env_info) = get_project_env_info(app)? {
        return Ok(env_info);
    }

    Err(user(format!(
        "Couldn't infer environment for directory {}",
        app.cwd.display()
    )))
}
