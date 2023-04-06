use super::RepositoryRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RepositoriesRecord {
    #[serde(rename = "repositories")]
    pub repositories: Vec<RepositoryRecord>,
}
