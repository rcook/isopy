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
use crate::status::{return_success, Status};
use anyhow::Result;
use log::info;

pub(crate) async fn download(app: &App, package_id: &PackageId) -> Result<Status> {
    fn parse_package_id(package_id: &PackageId) -> (&str, String) {
        (
            package_id.plugin_host().prefix(),
            package_id.descriptor().to_string(),
        )
    }

    let (moniker, version_str) = parse_package_id(package_id);
    let plugin = app.plugin_manager().get_plugin(moniker)?;
    let version = plugin.parse_version(&version_str)?;
    let manager = plugin.new_manager();
    manager.download_package(&version).await?;
    info!("Package {moniker}:{} is now available locally", *version);
    return_success!();
}
