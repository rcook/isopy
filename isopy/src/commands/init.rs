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
use crate::status::{report_install_package_error, return_success, return_user_error, Status};
use anyhow::Result;
use isopy_lib::{DownloadPackageOptions, InstallPackageOptions};

pub(crate) async fn do_init(app: &App, download: bool) -> Result<Status> {
    if app.repo().get(app.cwd())?.is_some() {
        return_user_error!(
            "Project in directory {} already has an environment",
            app.cwd().display()
        );
    }

    let Some(project) = existing(app.read_project_config())? else {
        return_user_error!(
            "No project configuration file in directory {}",
            app.cwd().display()
        );
    };

    let download_package_options = DownloadPackageOptions::default().show_progress(true);
    let install_package_options = InstallPackageOptions::default().show_progress(true);

    for package_id in project.package_ids {
        if download {
            app.plugin_manager()
                .new_package_manager(package_id.moniker(), app.config_dir())
                .download_package(package_id.version(), &None, &download_package_options)
                .await?;
        }

        report_install_package_error!(
            app.install_package(
                package_id.moniker(),
                package_id.version(),
                &install_package_options,
            )
            .await
        );
    }

    return_success!();
}
