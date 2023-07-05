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
use crate::print::print;
use crate::registry::DescriptorInfo;
use crate::status::Status;
use anyhow::Result;
use colored::Colorize;
use isopy_python::constants::RELEASES_URL;
use std::rc::Rc;

pub async fn do_available(app: &App) -> Result<Status> {
    for product_info in &app.registry.product_infos {
        let package_infos = product_info
            .product
            .get_package_infos(app.repo.shared_dir())
            .await?;
        if !package_infos.is_empty() {
            print(&format!(
                "{} ({})",
                product_info.product.name().cyan(),
                RELEASES_URL.as_str().bright_magenta()
            ));

            for package_info in package_infos {
                let descriptor_info = DescriptorInfo {
                    product_info: Rc::clone(product_info),
                    descriptor: package_info.descriptor,
                };
                print(&format!(
                    "  {:<30} {}",
                    format!("{descriptor_info}").bright_yellow(),
                    package_info.file_name.display()
                ));
            }
        }
    }
    Ok(Status::OK)
}

/*
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

async fn update_index_if_necessary(app: &App) -> Result<()> {
    let repositories = Foo::read_repositories(app.repo.shared_dir())?;
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

    let mut assets = Foo::read_assets(app.repo.shared_dir())?;
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
*/
