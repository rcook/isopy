use crate::tng::file::File;
use crate::tng::url_format;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Download {
    #[serde(with = "url_format")]
    pub(crate) url: Url,
    pub(crate) files: Vec<File>,
}
