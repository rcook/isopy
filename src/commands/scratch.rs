use crate::repository::{GitHubRepository, LocalRepository, Repository};
use crate::result::Result;
use crate::util::{safe_create_file, ContentLength, Indicator};
use std::io::Write;
use std::path::Path;

pub async fn do_scratch<P, Q, R>(
    local_repository_dir: P,
    index_json_path1: Q,
    index_json_path2: R,
) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    R: AsRef<Path>,
{
    let github_repository = GitHubRepository::new("http://localhost:8000")?;
    let local_repository = LocalRepository::new(local_repository_dir.as_ref());
    test_repository(&index_json_path1, github_repository).await?;
    test_repository(&index_json_path2, local_repository).await?;
    Ok(())
}

async fn test_repository<P>(index_json_path: P, repository: impl Repository) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut response = repository.get_index().await?;
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
        std::thread::sleep(std::time::Duration::from_millis(10))
    }
    indicator.set_message("Index fetched");
    indicator.finish();
    Ok(())
}
