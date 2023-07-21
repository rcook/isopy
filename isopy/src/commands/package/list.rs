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
use crate::plugin_host::PluginHostRef;
use crate::print::{make_list_table, prettify_descriptor, prettify_package};
use crate::registry::Registry;
use crate::status::Status;
use crate::table::{table_divider, table_row, Table};
use anyhow::Result;
use colored::Colorize;
use isopy_lib::{Package, PluginFactory};

pub enum ListType {
    LocalOnly,
    All,
}

pub async fn list(app: &App, list_type: ListType, verbose: bool) -> Result<Status> {
    let mut table = make_list_table();

    for plugin_host in &Registry::global().plugin_hosts {
        let plugin_dir = app.repo.shared_dir().join(plugin_host.prefix());
        let plugin = plugin_host.make_plugin(&plugin_dir);

        let packages = match list_type {
            ListType::LocalOnly => plugin.get_downloaded_packages().await?,
            ListType::All => plugin.get_available_packages().await?,
        };

        add_plugin_rows(&mut table, plugin_host, &packages, verbose)?;
    }

    table.print();

    Ok(Status::OK)
}

fn add_plugin_rows(
    table: &mut Table,
    plugin_host: &PluginHostRef,
    packages: &Vec<Package>,
    verbose: bool,
) -> Result<()> {
    if packages.is_empty() {
        table_divider!(
            table,
            "No packages found for {} ({})",
            plugin_host.name().cyan(),
            plugin_host.source_url().as_str().bright_magenta()
        );
    } else {
        table_divider!(
            table,
            "{} ({})",
            plugin_host.name().cyan(),
            plugin_host.source_url().as_str().bright_magenta()
        );

        for package in packages {
            let descriptor_pretty = prettify_descriptor(plugin_host, package);
            let package_pretty = prettify_package(package, verbose)?;
            table_row!(table, descriptor_pretty, package_pretty);
        }
    }

    Ok(())
}
