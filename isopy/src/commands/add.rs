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
use crate::package_id::PackageId;
use crate::serialization::{PackageRec, ProjectRec};
use crate::status::Status;
use anyhow::Result;
use joatmon::{read_yaml_file, safe_write_file};
use log::{error, info};

pub fn add(app: &App, package_id: &PackageId) -> Result<Status> {
    let project_config_path = app.cwd.join(&*PROJECT_CONFIG_FILE_NAME);

    let mut packages = if project_config_path.is_file() {
        read_yaml_file::<ProjectRec>(&project_config_path)?.packages
    } else {
        Vec::new()
    };

    let id = package_id.plugin_host().prefix();
    if packages.iter().any(|p| p.id == id) {
        error!("environment already has a package with ID \"{id}\" configured");
        return Ok(Status::Fail);
    }

    packages.push(PackageRec {
        id: String::from(id),
        props: package_id.descriptor().get_project_props()?,
    });

    safe_write_file(
        &project_config_path,
        serde_yaml::to_string(&ProjectRec { packages })?,
        true,
    )?;

    info!(
        "added package \"{id}\" to project configuration file {}",
        project_config_path.display()
    );

    Ok(Status::OK)
}
