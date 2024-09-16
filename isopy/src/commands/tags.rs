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
use crate::status::{return_success, Status};
use anyhow::Result;
use colored::Colorize;
use strum::IntoEnumIterator;

pub(crate) async fn do_tags(app: &App, moniker: &Option<Moniker>) -> Result<Status> {
    async fn list_tags(app: &App, moniker: &Moniker) -> Result<()> {
        let tags = app
            .plugin_manager()
            .new_package_manager(moniker, app.config_dir())
            .list_tags()
            .await?;

        println!("Package manager: {}", moniker.as_str().bright_magenta());

        if !tags.tags().is_empty() {
            println!("  Tags:");
            for tag in tags.tags() {
                println!("    {}", tag.cyan());
            }
        }

        if !tags.default_tags().is_empty() {
            println!("  Default tags:");
            for tag in tags.default_tags() {
                println!("    {}", tag.cyan());
            }
        }

        if !tags.other_tags().is_empty() {
            println!("  Other tags:");
            for tag in tags.other_tags() {
                println!("    {}", tag.cyan());
            }
        }

        Ok(())
    }

    match moniker {
        Some(moniker) => {
            list_tags(app, moniker).await?;
        }
        None => {
            for moniker in Moniker::iter() {
                list_tags(app, &moniker).await?;
            }
        }
    }

    return_success!();
}
