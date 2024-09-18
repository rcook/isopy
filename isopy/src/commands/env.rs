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
use crate::status::{report_install_package_error, success, Status};
use anyhow::Result;
use isopy_lib::{DownloadPackageOptionsBuilder, InstallPackageOptionsBuilder};

pub(crate) async fn do_env(app: &App, package_id: &PackageId, download: bool) -> Result<Status> {
    let download_package_options = DownloadPackageOptionsBuilder::default()
        .show_progress(app.show_progress())
        .build()?;
    let install_package_options = InstallPackageOptionsBuilder::default()
        .show_progress(app.show_progress())
        .build()?;

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
            &install_package_options
        )
        .await
    );

    success!()
}
