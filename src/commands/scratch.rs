use crate::config::Config;
use crate::error::Result;
use crate::util::check_sha256sums;

pub fn do_scratch(config: &Config) -> Result<()> {
    check_sha256sums(&config)?;
    Ok(())
}
