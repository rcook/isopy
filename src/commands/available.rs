use crate::app::App;
use crate::error::Result;
use crate::object_model::AssetFilter;
use crate::util::safe_write_to_file;
use reqwest::header::{ACCEPT, USER_AGENT};
use reqwest::Client;

const PYTHON_INDEX_URL: &'static str =
    "https://api.github.com/repos/indygreg/python-build-standalone/releases";

pub async fn do_available(app: &App) -> Result<()> {
    let index_path = app.assets_dir.join("index.json");
    if !index_path.is_file() {
        let contents = Client::new()
            .get(PYTHON_INDEX_URL)
            .header(USER_AGENT, "isopy")
            .header(ACCEPT, "application/vnd.github.v3+json")
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        safe_write_to_file(index_path, contents)?;
    }

    let assets = app.read_assets()?;
    for asset in AssetFilter::default_for_platform().filter(assets.iter().map(|x| x).into_iter()) {
        println!("{}", asset.name)
    }

    Ok(())
}
