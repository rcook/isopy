pub struct PackageManagerFactory {
    name: String,
}

impl PackageManagerFactory {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
