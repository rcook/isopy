use crate::app::App;
use crate::object_model::LastModified;
use crate::repository::{GitHubRepository, LocalRepository, Repository};
use crate::result::Result;
use crate::util::{safe_create_file, to_last_modified, ContentLength, Indicator};
use std::io::Write;
use std::path::Path;
use std::time::{Duration, UNIX_EPOCH};

const RELEASES_URL: &'static str =
    "https://api.github.com/repos/indygreg/python-build-standalone/releases/";

pub async fn do_scratch<P, Q, R>(
    app: &App,
    local_repository_dir: P,
    index_json_path1: Q,
    index_json_path2: R,
) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    R: AsRef<Path>,
{
    let github_last_modified = app.read_index_last_modified()?;
    let github_repository = GitHubRepository::new(RELEASES_URL)?;
    let result =
        test_repository(&github_last_modified, &index_json_path1, github_repository).await?;
    if let Some(last_modified) = result {
        app.write_index_last_modified(&last_modified)?;
    }

    let local_repository = LocalRepository::new(local_repository_dir.as_ref());
    let local_last_modified = Some(to_last_modified(
        &(UNIX_EPOCH + Duration::from_nanos(1680139858761270900)),
    )?);
    let result = test_repository(&local_last_modified, &index_json_path2, local_repository).await?;
    println!("result={:?}", result);
    Ok(())
}

async fn test_repository<P>(
    last_modified: &Option<LastModified>,
    index_json_path: P,
    repository: impl Repository,
) -> Result<Option<LastModified>>
where
    P: AsRef<Path>,
{
    let response_opt = repository.get_latest_index(last_modified).await?;
    if let Some((new_last_modified, content_length, mut response)) = response_opt {
        let indicator = Indicator::new(content_length)?;
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
        Ok(Some(new_last_modified))
    } else {
        Ok(None)
    }
}
