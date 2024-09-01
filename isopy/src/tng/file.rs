use crate::tng::date_time_format;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct File {
    pub(crate) file_name: String,
    #[serde(with = "date_time_format")]
    pub(crate) downloaded_at: DateTime<Utc>,
}
