use crate::config::Config;
use crate::error::Result;
use crate::util::dump_sha256sums;

pub fn do_scratch(config: &Config) -> Result<()> {
    dump_sha256sums(&config, &crate::object_model::Tag::parse("20230116"))?;
    Ok(())
}
