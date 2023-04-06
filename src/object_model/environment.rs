use crate::app::App;
use crate::object_model::EnvironmentName;
use crate::result::{user, Result};
use crate::serialization::{AnonymousEnvRecord, NamedEnvRecord, UseRecord};
use crate::util::{path_to_str, read_yaml_file};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Environment {
    pub name: EnvironmentName,
    pub full_python_dir: PathBuf,
}

fn get_use_env_info(app: &App) -> Result<Option<Environment>> {
    let use_dir = app.use_dir(&app.cwd)?;
    let use_config_path = use_dir.join("use.yaml");
    if !use_config_path.is_file() {
        return Ok(None);
    }

    let use_record = read_yaml_file::<UseRecord, _>(&use_config_path)?;
    let named_env_config_path = app.named_env_dir(&use_record.env_name).join("env.yaml");
    if !named_env_config_path.is_file() {
        return Ok(None);
    }

    let named_env_record = read_yaml_file::<NamedEnvRecord, _>(&named_env_config_path)?;
    return Ok(Some(Environment {
        name: named_env_record.name.clone(),
        full_python_dir: app
            .named_env_dir(&named_env_record.name)
            .join(named_env_record.python_dir),
    }));
}

fn get_project_env_info(app: &App) -> Result<Option<Environment>> {
    Ok(match app.read_project(&app.cwd)? {
        None => None,
        Some(project) => {
            let anonymous_env_dir = app.anonymous_env_dir(&project.config_path)?;
            let anonymous_env_config_path = anonymous_env_dir.join("env.yaml");
            if anonymous_env_config_path.is_file() {
                let anonymous_env_record =
                    read_yaml_file::<AnonymousEnvRecord, _>(&anonymous_env_config_path)?;
                Some(Environment {
                    name: EnvironmentName::sanitize(path_to_str(&project.config_path)?),
                    full_python_dir: anonymous_env_dir.join(anonymous_env_record.python_dir),
                })
            } else {
                None
            }
        }
    })
}

pub fn get_env_info(app: &App, env_name_opt: Option<&EnvironmentName>) -> Result<Environment> {
    if let Some(env_name) = env_name_opt {
        match app.read_named_env(env_name)? {
            Some(env_record) => {
                return Ok(Environment {
                    name: env_record.name,
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
