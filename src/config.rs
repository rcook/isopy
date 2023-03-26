use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub dir: PathBuf,
}

impl Config {
    pub fn from_dir(dir: PathBuf) -> Self {
        Self { dir: dir }
    }
}
