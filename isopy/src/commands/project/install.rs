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
use crate::fs::existing;
use crate::registry::Registry;
use crate::status::{return_success, return_user_error, Status};
use anyhow::Result;
use isopy_lib::PluginFactory;
use std::collections::HashMap;

pub async fn install(app: &App) -> Result<Status> {
    if app.repo.get(&app.cwd)?.is_some() {
        return_user_error!("directory {} already has an environment", app.cwd.display());
    }

    let Some(project_rec) = existing(app.read_project_config())? else {
        return_user_error!(
            "no project configuration file in directory {}",
            app.cwd.display()
        );
    };

    let plugin_hosts = Registry::global()
        .plugin_hosts
        .iter()
        .map(|h| (String::from(h.prefix()), h))
        .collect::<HashMap<_, _>>();

    let mut descriptors = Vec::new();
    for package_rec in project_rec.packages {
        let Some(plugin_host) = plugin_hosts.get(&package_rec.id) else {
            return_user_error!(
                "no project configuration file in directory {}",
                app.cwd.display()
            );
        };

        descriptors.push((
            *plugin_host,
            plugin_host.read_project_config(&package_rec.props)?,
        ));
    }

    for (plugin_host, descriptor) in descriptors {
        app.add_package(plugin_host, descriptor.as_ref()).await?;
    }

    return_success!();
}
