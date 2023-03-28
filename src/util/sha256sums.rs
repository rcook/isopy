use crate::error::{fatal, Result};
use crate::object_model::Tag;
use include_dir::{include_dir, Dir};

static SHA256SUMS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/sha256sums");

pub fn dump_sha256sums(tag: &Tag) -> Result<()> {
    let file_name = format!("{}.sha256sums", tag.as_str());
    let f = SHA256SUMS_DIR
        .get_file(file_name)
        .ok_or(fatal("Resource not found"))?;
    let s = f
        .contents_utf8()
        .ok_or(fatal("Resource could not be decoded at UTF-8"))?;
    println!("s={:?}", s);
    Ok(())
}
