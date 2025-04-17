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
use crate::constants::DEFAULT_MONIKER_CONFIG_NAME;
use crate::moniker::Moniker;
use crate::print::{humanize_size_base_2, make_list_table};
use crate::status::{success, StatusResult};
use crate::table::{table_columns, table_divider, table_headings, Table};
use anyhow::bail;
use anyhow::Result;
use colored::{ColoredString, Colorize};
use isopy_lib::{
    ListPackagesOptions, ListPackagesOptionsBuilder, PackageInfo, Plugin, SourceFilter, TagFilter,
};
use std::fs::metadata;
use url::{Host, Url};

pub(crate) async fn do_packages(
    app: &App,
    moniker: &Option<Moniker>,
    source_filter: SourceFilter,
    tag_filter: &TagFilter,
    verbose: bool,
) -> StatusResult {
    async fn list_packages(
        table: &mut Table,
        app: &App,
        moniker: &Moniker,
        source_filter: SourceFilter,
        tag_filter: &TagFilter,
        options: &ListPackagesOptions,
        verbose: bool,
    ) -> Result<()> {
        let plugin = app.plugin_manager.get_plugin(moniker);
        let packages = app
            .plugin_manager
            .new_package_manager(moniker, &app.config_dir)
            .list_packages(source_filter, tag_filter, options)
            .await?;

        let packages = match source_filter {
            SourceFilter::All => {
                // List local packages before remote packages
                let mut partitions = packages
                    .into_iter()
                    .partition::<Vec<_>, _>(|p| p.path.is_some());
                partitions.0.append(&mut partitions.1);
                partitions.0
            }
            SourceFilter::Local | SourceFilter::Remote => packages,
        };

        add_plugin_rows(table, moniker, plugin, &packages, source_filter, verbose)?;
        Ok(())
    }

    let moniker = match moniker {
        Some(m) => Some(m.clone()),
        None => {
            if let Some(value) = app.get_config_value(DEFAULT_MONIKER_CONFIG_NAME)? {
                Some(value.parse()?)
            } else {
                None
            }
        }
    };

    let mut table = make_list_table();
    let options = ListPackagesOptionsBuilder::default()
        .show_progress(app.show_progress)
        .build()?;

    match moniker {
        Some(moniker) => {
            list_packages(
                &mut table,
                app,
                &moniker,
                source_filter,
                tag_filter,
                &options,
                verbose,
            )
            .await?;
        }
        None => {
            for moniker in Moniker::iter_enabled() {
                list_packages(
                    &mut table,
                    app,
                    &moniker,
                    source_filter,
                    tag_filter,
                    &options,
                    verbose,
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
    packages: &Vec<PackageInfo>,
    source_filter: SourceFilter,
    verbose: bool,
) -> Result<()> {
    fn make_package_id(moniker: &Moniker, package: &PackageInfo) -> String {
        match package.version.label() {
            Some(label) => format!(
                "{}:{}:{}",
                moniker.as_str(),
                package.version.as_str(),
                label
            ),
            None => format!("{}:{}", moniker.as_str(), package.version.as_str()),
        }
    }

    fn make_package_summary(package: &PackageInfo, verbose: bool) -> Result<String> {
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

        match package.path.as_ref() {
            Some(p) => {
                let size = metadata(p)?.len();
                Ok(format!(
                    "{} ({})",
                    "Available locally".bright_white().bold(),
                    humanize_size_base_2(size).cyan()
                ))
            }
            None => Ok(format!("{}", format_url(&package.url, verbose)?.white())),
        }
    }

    if packages.is_empty() {
        match source_filter {
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
            SourceFilter::Remote => table_divider!(
                table,
                "No remote packages found for {} ({})",
                moniker.as_str().cyan(),
                plugin.url().as_str().bright_magenta()
            ),
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
