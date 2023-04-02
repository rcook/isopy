use crate::app::App;
use crate::constants::RELEASES_URL;
use crate::object_model::RepositoryName;
use crate::result::Result;
use crate::serialization::{RepositoriesRecord, RepositoryRecord};
use crate::util::{dir_url, safe_write_file};
use std::path::{Path, PathBuf};

pub fn do_generate_repositories_yaml<P>(app: &App, local_repository_dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    safe_write_file(
        app.dir.join("repositories.yaml"),
        serde_yaml::to_string(&RepositoriesRecord {
            repositories: vec![
                RepositoryRecord::GitHub {
                    name: RepositoryName::parse("default").expect("must parse"),
                    url: dir_url(RELEASES_URL)?,
                    enabled: true,
                },
                RepositoryRecord::Local {
                    name: RepositoryName::parse("local").expect("must parse"),
                    dir: PathBuf::from(local_repository_dir.as_ref()),
                    enabled: true,
                },
            ],
        })?,
        false,
    )?;
    Ok(())
}
