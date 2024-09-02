use anyhow::{Error, Result};
use hex::decode;
use sha2::{Digest, Sha256};
use std::{path::Path, str::FromStr};
use tokio::fs::read;

#[derive(Debug)]
pub struct Checksum(Vec<u8>);

impl Checksum {
    pub async fn validate_file(&self, path: &Path) -> Result<bool> {
        let data = read(path).await?;
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize().to_vec();
        Ok(self.0 == hash)
    }
}

impl FromStr for Checksum {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let value = decode(s)?;
        Ok(Self(value))
    }
}
