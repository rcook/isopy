use crate::app::App;
use crate::object_model::EnvironmentName;
use crate::result::{user, Result};
use crate::serialization::{NamedEnvironmentRecord, ProjectEnvironmentRecord, UseRecord};
use crate::util::{path_to_str, read_yaml_file};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Environment {
    pub name: EnvironmentName,
    pub full_python_dir: PathBuf,
}

impl Environment {
    pub fn infer(app: &App, environment_name: Option<&EnvironmentName>) -> Result<Environment> {
        if let Some(n) = environment_name {
            match app.read_named_environment(n)? {
                Some(rec) => {
                    return Ok(Environment {
                        name: rec.name,
                        full_python_dir: app.named_environment_dir(n).join(rec.python_dir_rel),
                    })
                }
                _ => return Err(user(format!("No environment named {}", n))),
            };
        }

        if let Some(environment) = Self::try_use(app)? {
            return Ok(environment);
        }

        if let Some(environment) = Self::try_project(app)? {
            return Ok(environment);
        }

        Err(user(format!(
            "Couldn't infer environment for directory {}",
            app.cwd.display()
        )))
    }

    fn try_use(app: &App) -> Result<Option<Environment>> {
        let config_path = app.use_dir(&app.cwd)?.join("use.yaml");
        if !config_path.is_file() {
            return Ok(None);
        }

        let rec = read_yaml_file::<UseRecord, _>(&config_path)?;
        let config_path = app
            .named_environment_dir(&rec.environment_name)
            .join("env.yaml");
        if !config_path.is_file() {
            return Ok(None);
        }

        let rec = read_yaml_file::<NamedEnvironmentRecord, _>(&config_path)?;
        return Ok(Some(Environment {
            name: rec.name.clone(),
            full_python_dir: app
                .named_environment_dir(&rec.name)
                .join(rec.python_dir_rel),
        }));
    }

    fn try_project(app: &App) -> Result<Option<Environment>> {
        Ok(match app.read_project(&app.cwd)? {
            None => None,
            Some(project) => {
                let dir = app.project_environment_dir(&project.config_path)?;
                let config_path = dir.join("env.yaml");
                if config_path.is_file() {
                    let rec = read_yaml_file::<ProjectEnvironmentRecord, _>(&config_path)?;
                    Some(Environment {
                        name: EnvironmentName::sanitize(path_to_str(&project.config_path)?),
                        full_python_dir: dir.join(rec.python_dir_rel),
                    })
                } else {
                    None
                }
            }
        })
    }
}
