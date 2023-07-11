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
use crate::print::print_packages;
use crate::registry::Registry;
use crate::status::Status;
use anyhow::Result;
use isopy_lib::PluginFactory;

pub async fn downloaded(app: &App, verbose: bool) -> Result<Status> {
    for plugin_host in &Registry::global().plugin_hosts {
        let plugin_dir = app.repo.shared_dir().join(plugin_host.prefix());
        let plugin = plugin_host.make_plugin(&plugin_dir);
        let packages = plugin.get_downloaded_packages().await?;
        print_packages(plugin_host, &packages, verbose)?;
    }

    Ok(Status::OK)
}
