use crate::repository::Response;
use crate::result::Result;
use crate::util::{safe_create_file, ContentLength, Indicator};
use std::io::Write;
use std::path::Path;

pub const ISOPY_USER_AGENT: &'static str = "isopy";

pub async fn download_the_next_generation<P>(
    label: &str,
    response: &mut Box<dyn Response>,
    output_path: P,
) -> Result<()>
where
    P: AsRef<Path>,
{
    println!("DOWNLOAD THAT SHIT!");
    let indicator = Indicator::new(response.content_length())?;
    let mut stream = response.bytes_stream()?;
    let mut file = safe_create_file(output_path, true)?;
    let mut downloaded = 0;
    indicator.set_message(format!("Fetching {}", label));
    while let Some(item) = stream.next().await {
        let chunk = item?;
        downloaded += chunk.len() as ContentLength;
        file.write(&chunk)?;
        indicator.set_position(downloaded);
    }
    indicator.set_message(format!("Finished fetching {}", label));
    indicator.finish();
    Ok(())
}
