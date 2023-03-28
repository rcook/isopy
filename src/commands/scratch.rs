use crate::error::Result;
use crate::util::dump_sha256sums;

pub fn do_scratch() -> Result<()> {
    dump_sha256sums(&crate::object_model::Tag::parse("20230116"))?;
    Ok(())
}
