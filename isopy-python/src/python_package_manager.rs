use isopy_api::PackageManager;

pub struct PythonPackageManager {
    #[allow(unused)]
    name: String,
}

impl PythonPackageManager {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self { name: name.into() }
    }
}

impl PackageManager for PythonPackageManager {
    fn test(&self) {
        println!("PYTHON!");
    }
}
