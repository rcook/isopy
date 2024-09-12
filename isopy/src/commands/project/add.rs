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
use crate::serialization::{Package2, Project};
use crate::status::{return_success, return_user_error, Status};
use crate::tng::PackageId;
use anyhow::Result;
use log::info;

pub(crate) fn add(app: &App, package_id: &PackageId) -> Result<Status> {
    let mut packages = existing(app.read_project_config())?.map_or_else(Vec::new, |p| p.packages);

    let moniker_str = package_id.moniker().as_str();
    if packages.iter().any(|p| p.moniker == moniker_str) {
        return_user_error!(
            "Environment already has a package from package manager \"{moniker_str}\""
        );
    }

    packages.push(Package2 {
        moniker: String::from(moniker_str),
        version: package_id.version().to_string(),
    });

    app.write_project_config(&Project { packages }, true)?;
    info!(
        "Added package \"{moniker_str}\" to project at {}",
        app.cwd().display()
    );
    return_success!();
}
