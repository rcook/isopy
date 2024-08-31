use crate::context::Context;
use crate::package_manager::PackageManager;
use anyhow::Result;

pub type PackageManagerFactoryFn = fn(ctx: &dyn Context) -> Result<Box<dyn PackageManager>>;

pub struct PackageManagerFactory {
    make_fn: PackageManagerFactoryFn,
}

impl PackageManagerFactory {
    pub fn new(make_fn: PackageManagerFactoryFn) -> Self {
        Self { make_fn }
    }

    pub fn make(&self, ctx: &dyn Context) -> Result<Box<dyn PackageManager>> {
        (self.make_fn)(ctx)
    }
}
