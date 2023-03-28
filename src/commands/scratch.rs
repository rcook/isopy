use crate::app::App;
use crate::error::Result;
use crate::util::check_sha256sums;

pub fn do_scratch(app: &App) -> Result<()> {
    check_sha256sums(&app)?;
    Ok(())
}
