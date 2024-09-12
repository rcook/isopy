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
use crate::print::{humanize_size_base_2, make_list_table};
use crate::status::{return_success, Status};
use crate::table::{table_columns, table_divider, table_headings, Table};
use crate::tng::Moniker;
use anyhow::{anyhow, Result};
use colored::Colorize;
use isopy_lib::tng::{PackageFilter, PackageSummary, Plugin};
use std::fs::metadata;
use strum::IntoEnumIterator;

pub(crate) async fn packages(
    app: &App,
    moniker: &Option<Moniker>,
    filter: PackageFilter,
    tags: &Option<Vec<String>>,
    verbose: bool,
) -> Result<Status> {
    async fn list_plugin_packages(
        table: &mut Table,
        app: &App,
        moniker: &Moniker,
        filter: PackageFilter,
        tags: &Option<Vec<String>>,
        verbose: bool,
    ) -> Result<()> {
        let plugin = app.plugin_manager().get_plugin(moniker);
        let package_summaries = app
            .plugin_manager()
            .new_package_manager(moniker, app.config_dir())
            .list_packages(filter, tags)
            .await?;
        add_plugin_rows(table, moniker, plugin, &package_summaries, filter, verbose)?;
        Ok(())
    }

    let mut table = make_list_table();

    match moniker {
        Some(moniker) => {
            list_plugin_packages(&mut table, app, moniker, filter, tags, verbose).await?;
        }
        None => {
            for moniker in Moniker::iter() {
                list_plugin_packages(&mut table, app, &moniker, filter, tags, verbose).await?;
            }
        }
    }

    table.print();

    return_success!();
}

fn add_plugin_rows(
    table: &mut Table,
    moniker: &Moniker,
    plugin: &Plugin,
    package_summaries: &Vec<PackageSummary>,
    filter: PackageFilter,
    verbose: bool,
) -> Result<()> {
    fn make_package_id(moniker: &Moniker, package_summary: &PackageSummary) -> String {
        format!(
            "{}:{}",
            moniker.as_str(),
            **package_summary.version()
        )
    }

    fn make_package_info(package_summary: &PackageSummary, verbose: bool) -> Result<String> {
        match package_summary.path() {
            Some(p) => {
                let path = if verbose {
                    p.to_str().ok_or_else(|| {
                        anyhow!("Could not convert path {} to string", p.display())
                    })?
                } else {
                    p.file_name()
                        .ok_or_else(|| {
                            anyhow!("Could not get file name from path {}", p.display())
                        })?
                        .to_str()
                        .ok_or_else(|| {
                            anyhow!(
                                "Could not convert file name of path {} to string",
                                p.display()
                            )
                        })?
                };
                let size = metadata(p)?.len();
                Ok(format!(
                    "{} ({})",
                    path.bright_white().bold(),
                    humanize_size_base_2(size).cyan()
                ))
            }
            None => Ok(format!("{}", package_summary.url().as_str().white())),
        }
    }

    if package_summaries.is_empty() {
        match filter {
            PackageFilter::Local => table_divider!(
                table,
                "No local packages found for {} ({})",
                moniker.as_str().cyan(),
                plugin.url().as_str().bright_magenta()
            ),
            PackageFilter::All => table_divider!(
                table,
                "No packages (local or remote) found for {} ({})",
                moniker.as_str().cyan(),
                plugin.url().as_str().bright_magenta()
            ),
            PackageFilter::Remote => todo!(),
        }
    } else {
        table_divider!(
            table,
            "{} ({})",
            moniker.as_str().cyan(),
            plugin.url().as_str().bright_magenta()
        );

        table_headings!(table, "Package ID", "Details");
        for package_summary in package_summaries {
            let package_id = make_package_id(moniker, package_summary);
            let package_info = make_package_info(package_summary, verbose)?;
            table_columns!(table, package_id, package_info);
        }
    }

    Ok(())
}
