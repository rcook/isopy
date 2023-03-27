use crate::error::Result;
use flate2::read::GzDecoder;
use std::fs::File;
use std::path::Path;
use tar::Archive;

pub fn unpack_file<P, Q>(path: P, dir: Q) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let file = File::open(path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    archive.unpack(dir)?;
    Ok(())
}
