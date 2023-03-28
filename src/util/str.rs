use crate::error::{fatal, Result};
use std::path::Path;

#[allow(unused)]
pub fn path_to_str(p: &Path) -> Result<&str> {
    p.to_str().ok_or(fatal("Failed to convert path"))
}
