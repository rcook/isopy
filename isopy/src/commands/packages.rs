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
use crate::moniker::Moniker;
use crate::print::{humanize_size_base_2, make_list_table};
use crate::status::{success, Status};
use crate::table::{table_columns, table_divider, table_headings, Table};
use anyhow::bail;
use anyhow::Result;
use colored::{ColoredString, Colorize};
use isopy_lib::{
    ListPackageStatesOptions, ListPackageStatesOptionsBuilder, PackageState, Plugin, SourceFilter,
    TagFilter,
};
use std::fs::metadata;
use url::{Host, Url};

pub(crate) async fn do_packages(
    app: &App,
    moniker: &Option<Moniker>,
    filter: SourceFilter,
    tag_filter: &TagFilter,
    verbose: bool,
) -> Result<Status> {
    async fn list_packages(
        table: &mut Table,
        app: &App,
        moniker: &Moniker,
        filter: SourceFilter,
        tag_filter: &TagFilter,
        options: &ListPackageStatesOptions,
        verbose: bool,
    ) -> Result<()> {
        let plugin = app.plugin_manager().get_plugin(moniker);
        let packages = app
            .plugin_manager()
            .new_package_manager(moniker, app.config_dir())
            .list_package_states(filter, tag_filter, options)
            .await?;
        add_plugin_rows(table, moniker, plugin, &packages, filter, verbose)?;
        Ok(())
    }

    let mut table = make_list_table();
    let options = ListPackageStatesOptionsBuilder::default()
        .show_progress(app.show_progress())
        .build()?;

    match moniker {
        Some(moniker) => {
            list_packages(
                &mut table, app, moniker, filter, tag_filter, &options, verbose,
            )
            .await?;
        }
        None => {
            for moniker in Moniker::iter_enabled() {
                list_packages(
                    &mut table, app, &moniker, filter, tag_filter, &options, verbose,
                )
                .await?;
            }
        }
    }

    table.print();

    success!();
}

fn add_plugin_rows(
    table: &mut Table,
    moniker: &Moniker,
    plugin: &Plugin,
    packages: &Vec<PackageState>,
    filter: SourceFilter,
    verbose: bool,
) -> Result<()> {
    fn make_package_id(moniker: &Moniker, package: &PackageState) -> String {
        match package.label() {
            Some(label) => format!("{}:{}:{}", moniker.as_str(), **package.version(), label),
            None => format!("{}:{}", moniker.as_str(), **package.version()),
        }
    }

    fn make_package_summary(package: &PackageState, verbose: bool) -> Result<String> {
        fn format_url(url: &Url, verbose: bool) -> Result<ColoredString> {
            fn truncated_format(scheme: &str, domain: &str, path: &str) -> ColoredString {
                match path.rfind('/') {
                    Some(i) => {
                        format!("{scheme}://{domain}/{}{}", "...".cyan(), &path[i..]).white()
                    }
                    None => verbose_format(scheme, domain, path),
                }
            }

            fn verbose_format(scheme: &str, domain: &str, path: &str) -> ColoredString {
                format!("{scheme}://{domain}{path}").white()
            }

            let scheme = url.scheme();
            let Some(Host::Domain(domain)) = url.host() else {
                bail!("Unsupport URL type {url}")
            };

            let path = url.path();

            Ok(if verbose {
                verbose_format(scheme, domain, path)
            } else {
                truncated_format(scheme, domain, path)
            })
        }

        match package.path() {
            Some(p) => {
                let size = metadata(p)?.len();
                Ok(format!(
                    "{} ({})",
                    "Available locally".bright_white().bold(),
                    humanize_size_base_2(size).cyan()
                ))
            }
            None => Ok(format!("{}", format_url(package.url(), verbose)?.white())),
        }
    }

    if packages.is_empty() {
        match filter {
            SourceFilter::Local => table_divider!(
                table,
                "No local packages found for {} ({})",
                moniker.as_str().cyan(),
                plugin.url().as_str().bright_magenta()
            ),
            SourceFilter::All => table_divider!(
                table,
                "No packages (local or remote) found for {} ({})",
                moniker.as_str().cyan(),
                plugin.url().as_str().bright_magenta()
            ),
            SourceFilter::Remote => todo!(),
        }
    } else {
        table_divider!(
            table,
            "{} ({})",
            moniker.as_str().cyan(),
            plugin.url().as_str().bright_magenta()
        );

        table_headings!(table, "Package ID", "Details");
        for package in packages {
            let package_id = make_package_id(moniker, package);
            let package_summary = make_package_summary(package, verbose)?;
            table_columns!(table, package_id, package_summary);
        }
    }

    Ok(())
}
