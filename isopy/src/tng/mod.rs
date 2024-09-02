mod app;
mod app_context;
mod app_package_manager;
mod cache_info;
mod date_time_format;
mod download;
mod file;
mod manifest;
mod url_format;

pub(crate) async fn run() -> anyhow::Result<()> {
    use crate::tng::app::App;
    use anyhow::anyhow;
    use dirs::config_dir;

    let app = App::new(
        &config_dir()
            .ok_or_else(|| anyhow!("Could not determine config directory"))?
            .join(".isopy-tng"),
    )
    .await?;

    let package_manager = app.get_package_manager("python").await?;

    package_manager.list_categories().await?;

    package_manager.list_packages().await?;

    {
        use isopy_lib::tng::PackageVersion;
        package_manager
            .download_package(&PackageVersion {
                major: 3,
                minor: 12,
                revision: 5,
            })
            .await?;
    }

    Ok(())
}
