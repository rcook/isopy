use crate::app::App;
use crate::helpers::get_asset;
use crate::object_model::{LastModified, RepositoryName, Tag, Version};
use crate::repository::{GitHubRepository, LocalRepository, Repository};
use crate::result::Result;
use crate::serialization::RepositoryRecord;
use crate::util::download_the_next_generation;
use std::path::Path;

fn make_repository(
    record: RepositoryRecord,
) -> Result<(RepositoryName, bool, Box<dyn Repository>)> {
    Ok(match record {
        RepositoryRecord::GitHub { name, url, enabled } => {
            (name, enabled, Box::new(GitHubRepository::new(url.clone())?))
        }
        RepositoryRecord::Local { name, dir, enabled } => {
            (name, enabled, Box::new(LocalRepository::new(dir)))
        }
    })
}

pub async fn do_scratch(app: &App) -> Result<()> {
    let all_repositories = app
        .read_repositories()?
        .repositories
        .into_iter()
        .map(make_repository)
        .collect::<Result<Vec<_>>>()?;
    let enabled_repositories = all_repositories
        .into_iter()
        .filter(|x| x.1)
        .map(|x| (x.0, x.2))
        .collect::<Vec<_>>();

    for (repository_name, repository) in &enabled_repositories {
        let index_json_path = app.get_index_json_path(repository_name);
        let current_last_modified = app.read_index_last_modified(repository_name)?;
        if let Some(new_last_modified) = download_index(
            &current_last_modified,
            &index_json_path,
            repository.as_ref(),
        )
        .await?
        {
            app.write_index_last_modified(&repository_name, &new_last_modified)?;
        }
    }

    let assets = app.read_assets()?;
    let asset = get_asset(
        &assets,
        &Version::new(3, 11, 1),
        &Some(Tag::parse("20230116")),
    )?;
    for (_repository_name, repository) in &enabled_repositories {
        let mut response = repository.get_asset(&asset).await?;
        download_the_next_generation("asset", &mut response, "foo.tar.gz").await?;
    }
    Ok(())
}

async fn download_index<P>(
    last_modified: &Option<LastModified>,
    index_json_path: P,
    repository: &dyn Repository,
) -> Result<Option<LastModified>>
where
    P: AsRef<Path>,
{
    let response_opt = repository.get_latest_index(last_modified).await?;
    if let Some(mut response) = response_opt {
        download_the_next_generation("index", &mut response, index_json_path).await?;
        Ok(Some(response.last_modified().clone()))
    } else {
        Ok(None)
    }
}
