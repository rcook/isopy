use crate::result::Result;
use crate::util::path_to_str;
use md5::compute;
use std::path::Path;

pub struct HexDigest(String);

impl HexDigest {
    pub fn from_path<P>(config_path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let digest = compute(path_to_str(config_path.as_ref())?);
        let hex_digest = format!("{:x}", digest);
        Ok(Self(hex_digest))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
