use isopy_api::PackageManager;

pub struct JavaPackageManager {
    #[allow(unused)]
    name: String,
}

impl JavaPackageManager {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self { name: name.into() }
    }
}

impl PackageManager for JavaPackageManager {
    fn test(&self) {
        println!("JAVA!");
    }
}
