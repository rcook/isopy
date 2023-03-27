use crate::error::{user, Result};
use crate::ui::make_progress_bar;
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
    let response = client.get(temp_url).send().await?;
    let size = response
        .content_length()
        .ok_or(user("Failed to get content length"))?;

    let progress_bar = make_progress_bar(size)?;
    progress_bar.set_message(format!("Downloading {}", url_str));

    let mut file = File::create(&output_path)?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        let new = min(downloaded + (chunk.len() as u64), size);
        downloaded = new;
        progress_bar.set_position(new);
    }

    progress_bar.finish_with_message(format!(
        "Downloaded {} to {}",
        url_str,
        output_path.as_ref().display()
    ));

    Ok(())
}
