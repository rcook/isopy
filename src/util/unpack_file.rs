use crate::result::{translate_io_error, Result};
use crate::util::{open_file, ContentLength, Indicator};
use flate2::read::GzDecoder;
use std::fs::{create_dir_all, File};
use std::path::{Path, PathBuf};
use tar::{Archive, Entry};

pub fn unpack_file<P, Q>(path: P, dir: Q) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    fn unpack_entry(entry: &mut Entry<GzDecoder<File>>, path: PathBuf) -> Result<()> {
        let mut dir = path.clone();
        dir.pop();
        create_dir_all(&dir).map_err(|e| translate_io_error(e, &dir))?;
        entry
            .unpack(&path)
            .map_err(|e| translate_io_error(e, &path))?;
        Ok(())
    }

    // Open once to get number of entries (is this necessary?)
    let file = open_file(&path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    let size = archive
        .entries()
        .map_err(|e| translate_io_error(e, &path))?
        .count();

    // Open a second time to unpack
    let file = open_file(&path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    let indicator = Indicator::new(Some(size as ContentLength))?;
    indicator.set_message(format!("Unpacking {}", path.as_ref().display()));

    for (idx, mut entry) in archive
        .entries()
        .map_err(|e| translate_io_error(e, &path))?
        .filter_map(|e| e.ok())
        .enumerate()
    {
        let path = dir
            .as_ref()
            .join(entry.path().map_err(|e| translate_io_error(e, &dir))?);
        unpack_entry(&mut entry, path)?;
        indicator.set_position(idx as ContentLength);
    }

    indicator.finish();

    Ok(())
}
