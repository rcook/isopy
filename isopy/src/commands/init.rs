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
use crate::status::{success, user_error, StatusResult};
use isopy_lib::{
    DownloadPackageOptionsBuilder, GetPackageStateOptionsBuilder, InstallPackageOptionsBuilder,
    TagFilter,
};

pub(crate) async fn do_init(app: &App, download: bool) -> StatusResult {
    if app.repo().get(app.cwd())?.is_some() {
        user_error!(
            "Project in directory {} already has an environment",
            app.cwd().display()
        );
    }

    let Some(project) = existing(app.read_project_config())? else {
        user_error!(
            "No project configuration file in directory {}: create one with \"isopy project <PACKAGE-ID>\"",
            app.cwd().display()
        );
    };

    if download {
        let download_package_options = DownloadPackageOptionsBuilder::default()
            .show_progress(app.show_progress())
            .build()?;

        for package_id in &project.package_ids {
            if download {
                app.plugin_manager()
                    .new_package_manager(package_id.moniker(), app.config_dir())
                    .download_package(
                        package_id.version(),
                        &TagFilter::default(),
                        &download_package_options,
                    )
                    .await?;
            }
        }
    } else {
        let get_package_state_options = GetPackageStateOptionsBuilder::default()
            .show_progress(app.show_progress())
            .build()?;

        let mut unavailable_package_ids = Vec::new();
        for package_id in &project.package_ids {
            let package = app
                .get_package_state(
                    package_id.moniker(),
                    package_id.version(),
                    &get_package_state_options,
                )
                .await?;
            if package.is_none() {
                unavailable_package_ids.push(package_id.to_string());
            }
        }

        if !unavailable_package_ids.is_empty() {
            let package_id_str = unavailable_package_ids.join(", ");
            user_error!("The following package(s) have not been downloaded: {package_id_str}; use \"isopy download <PACKAGE-ID>\" or \"isopy init --download\" command to download missing packages");
        }
    }

    let install_package_options = InstallPackageOptionsBuilder::default()
        .show_progress(app.show_progress())
        .build()?;

    for package_id in &project.package_ids {
        app.install_package(
            package_id.moniker(),
            package_id.version(),
            &install_package_options,
        )
        .await?;
    }

    success!();
}
