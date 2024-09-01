mod app;
mod app_context;

pub(crate) async fn run() -> anyhow::Result<()> {
    use crate::tng::app::App;
    use anyhow::anyhow;
    use dirs::config_dir;
    use isopy_lib::tng::PackageVersion;

    App::new(
        &config_dir()
            .ok_or_else(|| anyhow!("Could not determine config directory"))?
            .join(".isopy-tng"),
    )
    .await?
    .download_package(
        "python",
        &PackageVersion {
            major: 3,
            minor: 9,
            revision: 13,
        },
    )
    .await?;

    Ok(())
}
