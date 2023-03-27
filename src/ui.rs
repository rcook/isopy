use crate::error::Result;
use indicatif::{ProgressBar, ProgressStyle};

pub fn make_progress_bar(size: u64) -> Result<ProgressBar> {
    let progress_bar = ProgressBar::new(size);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
        .progress_chars("#>-"));
    Ok(progress_bar)
}
