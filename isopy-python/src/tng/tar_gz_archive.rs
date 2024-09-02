use anyhow::Result;
use std::path::Path;

pub(crate) async fn unpack(archive_path: &Path, dir: &Path) -> Result<()> {
    todo!(
        "Unpack .tar.gz archive {} to {}",
        archive_path.display(),
        dir.display()
    )
}
