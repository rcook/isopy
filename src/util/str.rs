use crate::error::{fatal, Result};
use std::ffi::{OsStr, OsString};
use std::path::Path;

#[allow(unused)]
pub fn path_to_str(p: &Path) -> Result<&str> {
    p.to_str().ok_or(fatal("Failed to convert path"))
}

#[allow(unused)]
pub fn osstr_to_str(s: &OsStr) -> Result<&str> {
    s.to_str().ok_or(fatal("Failed to convert OS string"))
}

#[allow(unused)]
pub fn osstring_to_string(s: OsString) -> Result<String> {
    s.into_string()
        .map_err(|_| fatal("Failed to convert OS string"))
}
