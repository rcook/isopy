use crate::app::App;
use crate::error::Result;
use crate::object_model::AssetFilter;
use crate::util::download_file;
use reqwest::Client;

const PYTHON_INDEX_URL: &'static str =
    "https://api.github.com/repos/indygreg/python-build-standalone/releases";

pub async fn do_available(app: &App) -> Result<()> {
    let index_path = app.assets_dir.join("index.json");
    if !index_path.is_file() {
        let client = Client::builder().build()?;
        download_file(&client, PYTHON_INDEX_URL, index_path).await?;
    }

    let assets = app.read_assets()?;
    for asset in AssetFilter::default_for_platform().filter(assets.iter().map(|x| x).into_iter()) {
        println!("{}", asset.name)
    }

    Ok(())
}
