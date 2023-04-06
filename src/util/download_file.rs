use crate::repository::Response;
use crate::result::{translate_io_error, Result};
use crate::util::{safe_create_file, ContentLength, Indicator};
use std::io::Write;
use std::path::Path;

pub async fn download_stream<P>(
    label: &str,
    response: &mut Box<dyn Response>,
    output_path: P,
) -> Result<()>
where
    P: AsRef<Path>,
{
    let indicator = Indicator::new(response.content_length())?;
    let mut stream = response.bytes_stream()?;
    let mut file = safe_create_file(&output_path, true)?;
    let mut downloaded = 0;
    indicator.set_message(format!("Fetching {}", label));
    while let Some(item) = stream.next().await {
        let chunk = item?;
        downloaded += chunk.len() as ContentLength;
        file.write(&chunk)
            .map_err(|e| translate_io_error(e, &output_path))?;
        indicator.set_position(downloaded);
    }
    indicator.set_message(format!("Finished fetching {}", label));
    indicator.finish();
    Ok(())
}
