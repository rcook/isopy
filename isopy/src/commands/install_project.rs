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
use crate::constants::PROJECT_CONFIG_FILE_NAME;
use crate::registry::Registry;
use crate::serialization::ProjectRec;
use crate::status::Status;
use anyhow::Result;
use joatmon::read_yaml_file;
use log::error;
use std::collections::HashMap;

pub async fn install_project(app: &App) -> Result<Status> {
    if app.repo.get(&app.cwd)?.is_some() {
        error!("directory {} already has an environment", app.cwd.display());
        return Ok(Status::Fail);
    }

    let project_config_path = app.cwd.join(&*PROJECT_CONFIG_FILE_NAME);
    if !project_config_path.is_file() {
        error!(
            "no project configuration file in directory {}",
            app.cwd.display()
        );
        return Ok(Status::Fail);
    }

    let plugin_hosts = Registry::global()
        .plugin_hosts
        .iter()
        .map(|h| (String::from(h.prefix()), h))
        .collect::<HashMap<_, _>>();

    let mut descriptors = Vec::new();
    let project_rec = read_yaml_file::<ProjectRec>(&project_config_path)?;
    for package_rec in project_rec.packages {
        let Some(plugin_host) = plugin_hosts.get(&package_rec.id) else {
            error!(
                "no project configuration file in directory {}",
                app.cwd.display()
            );
            return Ok(Status::Fail);
        };

        descriptors.push((
            *plugin_host,
            plugin_host.read_project_config(&package_rec.props)?,
        ));
    }

    for (plugin_host, descriptor) in descriptors {
        app.add_package(plugin_host, descriptor.as_ref()).await?;
    }

    Ok(Status::OK)
}
