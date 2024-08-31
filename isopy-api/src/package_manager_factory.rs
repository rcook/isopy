use crate::PackageManager;
use anyhow::Result;

pub type PackageManagerFactoryFn = fn(name: String) -> Result<Box<dyn PackageManager>>;

pub struct PackageManagerFactory {
    name: String,
    make_fn: PackageManagerFactoryFn,
}

impl PackageManagerFactory {
    pub fn new<S>(name: S, make_fn: PackageManagerFactoryFn) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            make_fn,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn make<S>(&self, name: S) -> Result<Box<dyn PackageManager>>
    where
        S: Into<String>,
    {
        (self.make_fn)(name.into())
    }
}
