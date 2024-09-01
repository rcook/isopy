mod app;
mod app_context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await?;
    Ok(())
}

async fn run() -> anyhow::Result<()> {
    use crate::app::App;
    use anyhow::anyhow;
    use dirs::config_dir;
    use isopy_api::PackageVersion;

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
            minor: 8,
            revision: 12,
        },
    )
    .await?;

    Ok(())
}
