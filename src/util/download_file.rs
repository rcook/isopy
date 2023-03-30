use crate::error::{user, Result};
use crate::ui::ProgressIndicator;
use futures_util::StreamExt;
use reqwest::{Client, IntoUrl};
use std::cmp::min;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub async fn download_file<U, P>(client: &Client, url: U, output_path: P) -> Result<()>
where
    U: IntoUrl,
    P: AsRef<Path>,
{
    let temp_url = url.into_url()?;
    let url_str = String::from(temp_url.as_str());
    let response = client.get(temp_url).send().await?.error_for_status()?;
    let size_opt = response.content_length();

    let indicator = ProgressIndicator::new(size_opt)?;
    indicator.set_message(format!("Downloading {}", url_str));

    let mut file = File::create(&output_path)?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;

        downloaded = match size_opt {
            Some(size) => min(downloaded + chunk.len() as u64, size),
            None => downloaded + chunk.len() as u64,
        };

        indicator.set_position(downloaded);
    }

    indicator.finish_with_message(format!(
        "Downloaded {} to {}",
        url_str,
        output_path.as_ref().display()
    ));

    Ok(())
}
