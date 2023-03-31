use crate::app::App;
use crate::error::Result;
use std::path::PathBuf;

pub fn do_wrap(
    _app: &App,
    wrapper_path: &PathBuf,
    script_path: &PathBuf,
    base_dir: &PathBuf,
) -> Result<()> {
    println!(
        "wrapper_path={:?} script_path={:?} base_dir={:?}",
        wrapper_path, script_path, base_dir
    );
    Ok(())
}
