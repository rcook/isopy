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
use crate::plugin::Plugin;
use crate::status::Status;
use anyhow::Result;
use log::error;
use std::path::PathBuf;
use std::rc::Rc;

pub async fn do_init_config(app: &App) -> Result<Status> {
    if app.repo.get(&app.cwd)?.is_some() {
        error!("directory {} already has an environment", app.cwd.display());
        return Ok(Status::Fail);
    }

    let Some((plugin, project_config_path)) = get_plugin_and_project_config_path(app) else {
        error!("no project configuration file was found in {}", app.cwd.display());
        return Ok(Status::Fail)
    };

    let descriptor = plugin
        .product
        .read_project_config_file(&project_config_path)?;

    app.init_project(&plugin, descriptor.as_ref()).await?;

    Ok(Status::OK)
}

fn get_plugin_and_project_config_path(app: &App) -> Option<(Rc<Plugin>, PathBuf)> {
    for plugin in &app.registry.plugins {
        let project_config_path = app.cwd.join(plugin.product.project_config_file_name());
        if project_config_path.is_file() {
            return Some((Rc::clone(plugin), project_config_path));
        }
    }

    None
}
