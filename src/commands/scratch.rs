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
use crate::object_model::{LastModified, Tag, Version};
use crate::repository::Repository as RepositoryTrait;
use crate::util::{download_stream, get_asset};
use anyhow::Result;
use std::path::Path;

pub async fn do_scratch(app: &App) -> Result<()> {
    let repositories = app.read_repositories()?;

    for repository in &repositories {
        let index_json_path = app.get_index_json_path(&repository.name);
        let current_last_modified = app.read_index_last_modified(&repository.name)?;
        if let Some(new_last_modified) = download_index(
            &current_last_modified,
            &index_json_path,
            repository.repository.as_ref(),
        )
        .await?
        {
            app.write_index_last_modified(&repository.name, &new_last_modified)?;
        }
    }

    let assets = app.read_assets()?;
    let asset = get_asset(
        &assets,
        &Version::new(3, 11, 1),
        &Some(Tag::parse("20230116")),
    )?;
    for repository in &repositories {
        let mut response = repository.repository.get_asset(asset).await?;
        download_stream("asset", &mut response, "foo.tar.gz").await?;
    }
    Ok(())
}

async fn download_index<P>(
    last_modified: &Option<LastModified>,
    index_json_path: P,
    repository: &dyn RepositoryTrait,
) -> Result<Option<LastModified>>
where
    P: AsRef<Path>,
{
    let response_opt = repository.get_latest_index(last_modified).await?;
    if let Some(mut response) = response_opt {
        download_stream("index", &mut response, index_json_path).await?;
        Ok(Some(
            response
                .last_modified()
                .as_ref()
                .expect("must be present")
                .clone(),
        ))
    } else {
        Ok(None)
    }
}
