mod tng;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Ok(crate::tng::run().await?)
}
