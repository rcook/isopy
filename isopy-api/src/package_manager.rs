use crate::context::Context;
use anyhow::Result;

pub trait PackageManager {
    fn name(&self) -> &str;
    fn test(&self, ctx: &dyn Context) -> Result<()>;
}
