use crate::error::{fatal, Result};
use std::ffi::OsStr;
use std::path::Path;

#[allow(unused)]
pub fn path_to_str(p: &Path) -> Result<&str> {
    p.to_str().ok_or(fatal("Failed to convert path"))
}

#[allow(unused)]
pub fn osstr_to_str(s: &OsStr) -> Result<&str> {
    s.to_str().ok_or(fatal("Failed to convert OS string"))
}
