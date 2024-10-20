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
use crate::status::{success, StatusResult};
use anyhow::Result;
use colored::Colorize;
use isopy_lib::{ListTagsOptions, ListTagsOptionsBuilder};

pub(crate) async fn do_tags(app: &App, moniker: &Option<Moniker>) -> StatusResult {
    async fn list_tags(app: &App, moniker: &Moniker, options: &ListTagsOptions) -> Result<()> {
        let tags = app
            .plugin_manager()
            .new_package_manager(moniker, app.config_dir())
            .list_tags(options)
            .await?;

        println!(
            "{}: {}",
            "Package manager".bright_green(),
            moniker.as_str().bright_magenta()
        );

        if !tags.tags().is_empty() {
            println!("  {}:", "Tags".bright_yellow());
            for tag in tags.tags() {
                println!("    {}", tag.cyan());
            }
        }

        if !tags.default_tags().is_empty() {
            println!("  {}:", "Default tags".bright_yellow());
            for tag in tags.default_tags() {
                println!("    {}", tag.cyan());
            }
        }

        if !tags.other_tags().is_empty() {
            println!("  {}:", "Other tags".bright_yellow());
            for tag in tags.other_tags() {
                println!("    {}", tag.cyan());
            }
        }

        Ok(())
    }

    let options = ListTagsOptionsBuilder::default()
        .show_progress(app.show_progress())
        .build()?;

    match moniker {
        Some(moniker) => {
            list_tags(app, moniker, &options).await?;
        }
        None => {
            for moniker in Moniker::iter_enabled() {
                list_tags(app, &moniker, &options).await?;
            }
        }
    }

    success!();
}
