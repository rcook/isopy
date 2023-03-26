use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HttpBinIPResponse {
    #[serde(rename = "origin")]
    pub origin: String,
}
