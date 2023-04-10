use crate::app::App;
use crate::object_model::AssetFilter;
use crate::result::{user, Result};
use crate::util::download_stream;

pub async fn do_available(app: &App) -> Result<()> {
    update_index_if_necessary(app).await?;
    show_available_downloads(app)?;
    Ok(())
}

async fn update_index_if_necessary(app: &App) -> Result<()> {
    let repositories = app.read_repositories()?;
    let repository = repositories
        .first()
        .ok_or(user("No asset repositories are configured"))?;

    let current_last_modified = app.read_index_last_modified(&repository.name)?;
    if let Some(mut response) = repository
        .repository
        .get_latest_index(&current_last_modified)
        .await?
    {
        let index_json_path = app.get_index_json_path(&repository.name);
        download_stream("index", &mut response, index_json_path).await?;
        if let Some(last_modified) = response.last_modified() {
            app.write_index_last_modified(&repository.name, last_modified)?;
        }
    }

    Ok(())
}

fn show_available_downloads(app: &App) -> Result<()> {
    let assets = app.read_assets()?;
    for asset in AssetFilter::default_for_platform().filter(assets.iter()) {
        println!("{}", asset.name)
    }
    Ok(())
}
