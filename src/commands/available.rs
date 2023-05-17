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
use crate::object_model::AssetFilter;
use crate::util::{download_stream, print};
use anyhow::{anyhow, Result};
use joatmon::safe_back_up;
use log::info;
use std::fs::remove_file;

pub async fn do_available(app: &App) -> Result<()> {
    update_index_if_necessary(app).await?;
    show_available_downloads(app)?;
    Ok(())
}

async fn update_index_if_necessary(app: &App) -> Result<()> {
    let repositories = app.read_repositories()?;
    let repository = repositories
        .first()
        .ok_or(anyhow!("No asset repositories are configured"))?;

    let current_last_modified = app.read_index_last_modified(&repository.name)?;
    if let Some(mut response) = repository
        .repository
        .get_latest_index(&current_last_modified)
        .await?
    {
        let index_json_path = app.get_index_json_path(&repository.name);
        if index_json_path.exists() {
            let safe_back_up_path = safe_back_up(&index_json_path)?;
            info!(
                "Index file {} backed up to {}",
                index_json_path.display(),
                safe_back_up_path.display()
            );
            remove_file(&index_json_path)?;
        }

        download_stream("index", &mut response, index_json_path).await?;
        if let Some(last_modified) = response.last_modified() {
            app.write_index_last_modified(&repository.name, last_modified)?;
        }
    }

    Ok(())
}

fn show_available_downloads(app: &App) -> Result<()> {
    let assets = app.read_assets()?;
    for asset in AssetFilter::default_for_platform().filter(assets.iter()) {
        print(&asset.name)
    }
    Ok(())
}
