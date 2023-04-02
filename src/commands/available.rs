use crate::app::App;
use crate::constants::{LATEST_RELEASE_URL, RELEASES_URL};
use crate::object_model::{AssetFilter, LastModified, RepositoryName};
use crate::result::Result;
use crate::util::{download_file, ISOPY_USER_AGENT};
use reqwest::header::{IF_MODIFIED_SINCE, LAST_MODIFIED, USER_AGENT};
use reqwest::{Client, StatusCode};

pub async fn do_available(app: &App) -> Result<()> {
    let index_json_path = app.assets_dir.join("index.json");

    let last_modified_opt = app.read_index_last_modified(&RepositoryName::Default)?;

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
        if let Some(h) = response.headers().get(LAST_MODIFIED) {
            download_file(&client, RELEASES_URL, index_json_path, true).await?;
            let last_modified = LastModified::parse(h.to_str()?);
            app.write_index_last_modified(&RepositoryName::Default, &last_modified)?;
        }
    }

    let assets = app.read_assets()?;
    for asset in AssetFilter::default_for_platform().filter(assets.iter().map(|x| x).into_iter()) {
        println!("{}", asset.name)
    }

    Ok(())
}
