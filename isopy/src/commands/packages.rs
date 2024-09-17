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
use crate::status::{return_success, Status};
use crate::table::{table_columns, table_divider, table_headings, Table};
use anyhow::bail;
use anyhow::Result;
use colored::{ColoredString, Colorize};
use isopy_lib::{
    ListPackagesOptions, ListPackagesOptionsBuilder, OptionalTags, PackageFilter, PackageSummary,
    Plugin,
};
use std::fs::metadata;
use strum::IntoEnumIterator;
use url::{Host, Url};

pub(crate) async fn do_packages(
    app: &App,
    moniker: &Option<Moniker>,
    filter: PackageFilter,
    tags: &OptionalTags,
) -> Result<Status> {
    async fn list_packages(
        table: &mut Table,
        app: &App,
        moniker: &Moniker,
        filter: PackageFilter,
        tags: &OptionalTags,
        options: &ListPackagesOptions,
    ) -> Result<()> {
        let plugin = app.plugin_manager().get_plugin(moniker);
        let package_summaries = app
            .plugin_manager()
            .new_package_manager(moniker, app.config_dir())
            .list_packages(filter, tags, options)
            .await?;
        add_plugin_rows(table, moniker, plugin, &package_summaries, filter)?;
        Ok(())
    }

    let mut table = make_list_table();
    let options = ListPackagesOptionsBuilder::default()
        .show_progress(app.show_progress())
        .build()?;

    match moniker {
        Some(moniker) => {
            list_packages(&mut table, app, moniker, filter, tags, &options).await?;
        }
        None => {
            for moniker in Moniker::iter() {
                list_packages(&mut table, app, &moniker, filter, tags, &options).await?;
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
) -> Result<()> {
    fn make_package_id(moniker: &Moniker, package_summary: &PackageSummary) -> String {
        format!(
            "{}:{}:{}",
            moniker.as_str(),
            **package_summary.version(),
            package_summary.label()
        )
    }

    fn make_package_info(package_summary: &PackageSummary) -> Result<String> {
        fn format_url(url: &Url) -> Result<ColoredString> {
            let scheme = url.scheme();
            let Some(Host::Domain(domain)) = url.host() else {
                bail!("Unsupport URL type {url}")
            };

            let path = url.path();
            Ok(match path.rfind('/') {
                Some(i) => format!("{scheme}://{domain}/{}{}", "...".cyan(), &path[i..]).white(),
                None => format!("{scheme}://{domain}{path}").white(),
            })
        }

        match package_summary.path() {
            Some(p) => {
                let size = metadata(p)?.len();
                Ok(format!(
                    "{} ({})",
                    "Available locally".bright_white().bold(),
                    humanize_size_base_2(size).cyan()
                ))
            }
            None => Ok(format!("{}", format_url(package_summary.url())?.white())),
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
            let package_info = make_package_info(package_summary)?;
            table_columns!(table, package_id, package_info);
        }
    }

    Ok(())
}
