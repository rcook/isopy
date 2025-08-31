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
use crate::package_id::PackageId;
use crate::serialization::Project;
use crate::status::{StatusResult, success, user_error};
use anyhow::{Error, bail};
use log::info;
use std::io::{Error as IoError, ErrorKind};

fn is_io_error_kind(e: &Error, error_kind: ErrorKind) -> bool {
    match e.root_cause().downcast_ref::<IoError>() {
        Some(e) => e.kind() == error_kind,
        None => false,
    }
}

pub fn do_project(app: &App, package_id: &PackageId) -> StatusResult {
    let mut package_ids = match app.read_project_config() {
        Ok(project) => project.package_ids,
        Err(e) if is_io_error_kind(&e, ErrorKind::NotFound) => Vec::new(),
        Err(e) => bail!(e),
    };

    if package_ids.iter().any(|p| p.moniker == package_id.moniker) {
        user_error!(
            "Environment already has a package from package manager \"{}\"",
            package_id.moniker.as_str()
        );
    }

    package_ids.push(package_id.clone());

    app.write_project_config(&Project { package_ids }, true)?;
    info!(
        "Added package \"{}\" to project at {}",
        package_id,
        app.cwd.display()
    );
    success!();
}
