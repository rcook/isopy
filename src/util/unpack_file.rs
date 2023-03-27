use crate::error::Result;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::path::{Path, PathBuf};
use tar::Archive;

pub fn unpack_file<P, Q>(path: P, dir: Q) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    fn make_progress_bar<P>(path: P) -> Result<ProgressBar>
    where
        P: AsRef<Path>,
    {
        let progress_bar = ProgressBar::new(100);
        progress_bar.set_style(ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
            .progress_chars("#>-"));
        progress_bar.set_message(format!("Unpacking {}", path.as_ref().display()));
        Ok(progress_bar)
    }

    let progress_bar = make_progress_bar(&path)?;
    let file = File::open(path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf> {
            let path = entry.path()?.into_owned();
            println!("path={:?}", path);
            entry.unpack(&path)?;
            Ok(path)
        })
        .for_each(|x| println!("> {:?}", x));

    // progress_bar.set_position(100);

    progress_bar.finish_with_message("Done");

    Ok(())
}
