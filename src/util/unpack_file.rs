use crate::error::Result;
use crate::ui::ProgressIndicator;
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
        create_dir_all(&dir)?;
        entry.unpack(&path)?;
        Ok(())
    }

    // Open once to get number of entries (is this necessary?)
    let file = File::open(&path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    let size = archive.entries()?.count();

    // Open a second time to unpack
    let file = File::open(&path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    let indicator = ProgressIndicator::new(Some(size as u64))?;
    indicator.set_message(format!("Unpacking {}", path.as_ref().display()));

    for (idx, mut entry) in archive.entries()?.filter_map(|e| e.ok()).enumerate() {
        let path = dir.as_ref().join(entry.path()?);
        unpack_entry(&mut entry, path)?;
        indicator.set_position(idx as u64);
    }

    indicator.finish_with_message("Done");

    Ok(())
}
