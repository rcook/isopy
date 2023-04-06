use crate::result::Result;
use std::path::PathBuf;

pub const ISOPY_DIR_NAME: &'static str = ".isopy";

pub const PROJECT_CONFIG_FILE_NAME: &'static str = ".python-version.yaml";

pub fn default_isopy_dir() -> Option<PathBuf> {
    let home_dir = home::home_dir()?;
    let isopy_dir = home_dir.join(ISOPY_DIR_NAME);
    Some(isopy_dir)
}

pub fn make_project_config_path<P>(project_dir: P) -> PathBuf
where
    P: Into<PathBuf>,
{
    project_dir.into().join(PROJECT_CONFIG_FILE_NAME)
}

pub fn find_project_config_path<P>(start_dir: P) -> Result<Option<PathBuf>>
where
    P: Into<PathBuf>,
{
    let mut dir = start_dir.into();
    loop {
        let project_config_path = make_project_config_path(&dir);
        if project_config_path.is_file() {
            return Ok(Some(project_config_path));
        }
        if !dir.pop() {
            return Ok(None);
        }
    }
}
