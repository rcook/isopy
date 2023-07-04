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
use crate::args::PackageFilter;
use crate::constants::RELEASES_URL;
use crate::print::print;
use crate::python::{Asset, AssetFilter};
use crate::registry::ProductDescriptor;
use crate::status::Status;
use anyhow::{anyhow, Result};
use colored::Colorize;
use isopy_lib::download_stream;
use isopy_openjdk::adoptium::AdoptiumIndexManager;
use isopy_openjdk::OpenJdkDescriptor;
use isopy_python::PythonDescriptor;
use std::cmp::Ordering;

pub async fn do_available(app: &App, package_filter: PackageFilter) -> Result<Status> {
    match package_filter {
        PackageFilter::All => {
            show_python_index(app).await?;
            show_openjdk_index(app).await?;
        }
        PackageFilter::Python => show_python_index(app).await?,
        PackageFilter::OpenJdk => show_openjdk_index(app).await?,
    }
    Ok(Status::OK)
}

async fn show_python_index(app: &App) -> Result<()> {
    print(&format!(
        "{} ({})",
        "Python".cyan(),
        RELEASES_URL.as_str().bright_magenta()
    ));

    update_index_if_necessary(app).await?;
    show_available_downloads(app)?;

    Ok(())
}

async fn show_openjdk_index(app: &App) -> Result<()> {
    let manager = AdoptiumIndexManager::new_default(app.repo.shared_dir());

    print(&format!(
        "{} ({})",
        "OpenJDK".cyan(),
        manager.server_url().as_str().bright_magenta()
    ));

    for version in &manager.read_versions().await? {
        let product_descriptor = ProductDescriptor::OpenJdk(OpenJdkDescriptor {
            version: version.openjdk_version.clone(),
        });

        print(&format!(
            "  {:<30} {}",
            format!("{product_descriptor}").bright_yellow(),
            version.file_name.display()
        ));
    }

    Ok(())
}

async fn update_index_if_necessary(app: &App) -> Result<()> {
    let repositories = app.foo.read_repositories(&app.repo.shared_dir())?;
    let repository = repositories
        .first()
        .ok_or_else(|| anyhow!("No asset repositories are configured"))?;

    let releases_path = app.releases_path(&repository.name);
    let current_last_modified = if releases_path.is_file() {
        app.read_index_last_modified(&repository.name)?
    } else {
        None
    };

    if let Some(mut response) = repository
        .repository
        .get_latest_index(&current_last_modified)
        .await?
    {
        download_stream("release index", &mut response, &releases_path).await?;
        if let Some(last_modified) = response.last_modified() {
            app.write_index_last_modified(&repository.name, last_modified)?;
        }
    }

    Ok(())
}

fn show_available_downloads(app: &App) -> Result<()> {
    fn compare_by_version_and_tag(a: &Asset, b: &Asset) -> Ordering {
        match a.meta.version.cmp(&b.meta.version) {
            Ordering::Equal => a.tag.cmp(&b.tag),
            result => result,
        }
    }

    let mut assets = app.foo.read_assets(&app.repo.shared_dir())?;
    assets.sort_by(|a, b| compare_by_version_and_tag(b, a));

    for asset in AssetFilter::default_for_platform().filter(assets.iter()) {
        let product_descriptor = ProductDescriptor::Python(PythonDescriptor {
            version: asset.meta.version.clone(),
            tag: Some(asset.tag.clone()),
        });

        print(&format!(
            "  {:<30} {}",
            format!("{product_descriptor}").bright_yellow(),
            asset.name
        ));
    }
    Ok(())
}
