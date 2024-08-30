use isopy_api::{Host, PackageManager};

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
    fn test(&self, host: &Box<dyn Host>) {
        println!("PYTHON!");
        host.get_file("python");
    }
}
