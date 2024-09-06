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
use crate::status::{return_success, Status};
use crate::tng::Moniker;
use anyhow::Result;
use isopy_lib::tng::{PackageFilter, PackageKind};
use log::info;
use strum::IntoEnumIterator;

pub(crate) async fn packages(
    app: &App,
    moniker: &Option<Moniker>,
    filter: PackageFilter,
) -> Result<Status> {
    async fn list_packages(app: &App, moniker: &Moniker, filter: PackageFilter) -> Result<()> {
        info!("Package manager {moniker}");
        for package_summary in app
            .plugin_manager()
            .new_package_manager(moniker, app.config_dir())
            .list_packages(filter)
            .await?
        {
            if package_summary.kind() == PackageKind::Local {
                println!("{} (downloaded locally)", package_summary.name())
            } else {
                println!("{}", package_summary.name())
            }
            println!("  {}", package_summary.url())
        }
        Ok(())
    }

    match moniker {
        Some(moniker) => {
            list_packages(app, moniker, filter).await?;
        }
        None => {
            for moniker in Moniker::iter() {
                list_packages(app, &moniker, filter).await?;
            }
        }
    }

    return_success!();
}