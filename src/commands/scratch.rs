use crate::app::App;
use crate::object_model::{LastModified, RepositoryName};
use crate::repository::{GitHubRepository, LocalRepository, Repository};
use crate::result::Result;
use crate::serialization::{RepositoriesRecord, RepositoryRecord};
use crate::util::{safe_create_file, ContentLength, Indicator};
use std::fs::read_to_string;
use std::io::Write;
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

pub async fn do_scratch<P, Q>(app: &App, _index_json_path1: P, index_json_path2: Q) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let s = read_to_string(app.dir.join("repositories.yaml"))?;
    let doc = serde_yaml::from_str::<RepositoriesRecord>(&s)?;
    let all_repositories = doc
        .repositories
        .into_iter()
        .map(make_repository)
        .collect::<Result<Vec<_>>>()?;
    let enabled_repositories = all_repositories
        .into_iter()
        .filter(|x| x.1)
        .map(|x| (x.0, x.2))
        .collect::<Vec<_>>();
    for rec in enabled_repositories {
        println!("name={}", rec.0);
        test_repository(&None, &index_json_path2, rec.1.as_ref()).await?;
    }
    /*
    let github_last_modified = app.read_index_last_modified()?;
    let github_repository = GitHubRepository::new(RELEASES_URL)?;
    let result =
        test_repository(&github_last_modified, &index_json_path1, github_repository).await?;
    if let Some(last_modified) = result {
        app.write_index_last_modified(&last_modified)?;
    }

    let local_repository = LocalRepository::new("");
    let local_last_modified = Some(to_last_modified(
        &(UNIX_EPOCH + Duration::from_nanos(1680139858761270900)),
    )?);
    let result = test_repository(&local_last_modified, &index_json_path2, local_repository).await?;
    println!("result={:?}", result);
     */
    Ok(())
}

async fn test_repository<P>(
    last_modified: &Option<LastModified>,
    index_json_path: P,
    repository: &dyn Repository,
) -> Result<Option<LastModified>>
where
    P: AsRef<Path>,
{
    let response_opt = repository.get_latest_index(last_modified).await?;
    if let Some(mut response) = response_opt {
        let indicator = Indicator::new(response.content_length())?;
        let mut stream = response.bytes_stream()?;
        let mut file = safe_create_file(index_json_path, false)?;
        let mut downloaded = 0;
        indicator.set_message("Fetching index");
        while let Some(item) = stream.next().await {
            let chunk = item?;
            downloaded += chunk.len() as ContentLength;
            file.write(&chunk)?;
            indicator.set_position(downloaded);
        }
        indicator.set_message("Index fetched");
        indicator.finish();
        Ok(Some(response.last_modified().clone()))
    } else {
        Ok(None)
    }
}
