use crate::tng::download::Download;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Manifest {
    pub(crate) downloads: Vec<Download>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            downloads: Vec::new(),
        }
    }
}
