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
mod app;
mod app_context;
mod app_package_manager;
mod cache_info;
mod consts;
mod date_time_format;
mod download;
mod file;
mod manifest;
mod url_format;

pub(crate) async fn run() -> anyhow::Result<()> {
    use crate::tng::app::App;
    use anyhow::anyhow;
    use dirs::config_dir;
    use isopy_lib2::tng::PackageVersion;
    use std::env::current_dir;

    let app = App::new(
        &config_dir()
            .ok_or_else(|| anyhow!("Could not determine config directory"))?
            .join(".isopy-tng"),
    )
    .await?;

    let package_manager = app.get_package_manager("python").await?;

    package_manager.list_categories().await?;

    package_manager.list_packages().await?;

    package_manager
        .download_package(&PackageVersion {
            major: 3,
            minor: 12,
            revision: 5,
        })
        .await?;

    package_manager
        .install_package(
            &PackageVersion {
                major: 3,
                minor: 12,
                revision: 5,
            },
            &current_dir()?.join("TEST"),
        )
        .await?;

    Ok(())
}
