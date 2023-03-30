use crate::error::Result;
use crate::ui::ProgressIndicator;
use crate::util::safe_create_file;
use futures_util::StreamExt;
use reqwest::header::USER_AGENT;
use reqwest::{Client, IntoUrl};
use std::cmp::min;
use std::io::Write;
use std::path::Path;

pub const ISOPY_USER_AGENT: &'static str = "isopy";

pub async fn download_file<U, P>(
    client: &Client,
    url: U,
    output_path: P,
    overwrite: bool,
) -> Result<()>
where
    U: IntoUrl,
    P: AsRef<Path>,
{
    let temp_url = url.into_url()?;
    let url_str = String::from(temp_url.as_str());
    let request = client.get(temp_url).header(USER_AGENT, ISOPY_USER_AGENT);
    let response = request.send().await?.error_for_status()?;
    let size_opt = response.content_length();

    let indicator = ProgressIndicator::new(size_opt)?;
    indicator.set_message(format!("Downloading {}", url_str));

    let mut file = safe_create_file(&output_path, overwrite)?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;

        match size_opt {
            Some(size) => {
                downloaded = min(downloaded + chunk.len() as u64, size);
                indicator.set_position(downloaded);
            }
            None => {
                downloaded += chunk.len() as u64;
                indicator.set_message(format!("Downloaded {} bytes", downloaded))
            }
        };
    }

    indicator.finish_with_message(format!(
        "Downloaded {} to {}",
        url_str,
        output_path.as_ref().display()
    ));

    Ok(())
}
