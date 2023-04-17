// Copyright (c) 2023 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use crate::app::App;
use crate::object_model::EnvironmentName;
use crate::serialization::{NamedEnvironmentRecord, ProjectEnvironmentRecord, UseRecord};
use crate::util::path_to_str;
use anyhow::{bail, Result};
use joatmon::read_yaml_file;
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
                _ => bail!("No environment named {}", n),
            };
        }

        if let Some(environment) = Self::try_use(app)? {
            return Ok(environment);
        }

        if let Some(environment) = Self::try_project(app)? {
            return Ok(environment);
        }

        bail!(
            "Couldn't infer environment for directory {}",
            app.cwd.display()
        )
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
        Ok(Some(Environment {
            name: rec.name.clone(),
            full_python_dir: app
                .named_environment_dir(&rec.name)
                .join(rec.python_dir_rel),
        }))
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
