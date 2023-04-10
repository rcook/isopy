use crate::app::App;
use crate::helpers::get_asset;
use crate::object_model::{LastModified, Tag, Version};
use crate::repository::Repository as RepositoryTrait;
use crate::result::Result;
use crate::util::download_stream;
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
