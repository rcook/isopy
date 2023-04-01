use crate::app::App;
use crate::object_model::{AssetFilter, LastModified};
use crate::result::Result;
use crate::serialization::IndexRecord;
use crate::util::{download_file, safe_write_file, ISOPY_USER_AGENT};
use reqwest::header::{IF_MODIFIED_SINCE, LAST_MODIFIED, USER_AGENT};
use reqwest::{Client, StatusCode};

const LATEST_RELEASE_URL: &'static str =
    "https://api.github.com/repos/indygreg/python-build-standalone/releases/latest";

const RELEASES_URL: &'static str =
    "https://api.github.com/repos/indygreg/python-build-standalone/releases";

pub async fn do_available(app: &App) -> Result<()> {
    let index_yaml_path = app.assets_dir.join("index.yaml");
    let index_json_path = app.assets_dir.join("index.json");

    let last_modified_opt = app.index_last_modified()?;

    let client = Client::new();

    let mut request = client
        .head(LATEST_RELEASE_URL)
        .header(USER_AGENT, ISOPY_USER_AGENT);
    if let Some(last_modified) = last_modified_opt {
        request = request.header(IF_MODIFIED_SINCE, last_modified.as_str());
    }

    let response = request.send().await?.error_for_status()?;
    if response.status() != StatusCode::NOT_MODIFIED {
        println!("New releases are available");
        if let Some(last_modified) = response.headers().get(LAST_MODIFIED) {
            safe_write_file(
                &index_yaml_path,
                serde_yaml::to_string(&IndexRecord {
                    last_modified: LastModified::parse(last_modified.to_str()?),
                })?,
                true,
            )?;

            download_file(&client, RELEASES_URL, index_json_path, true).await?;
        }
    }

    let assets = app.read_assets()?;
    for asset in AssetFilter::default_for_platform().filter(assets.iter().map(|x| x).into_iter()) {
        println!("{}", asset.name)
    }

    Ok(())
}
