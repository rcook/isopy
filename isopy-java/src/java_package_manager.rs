use isopy_api::{Host, PackageManager};

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
    fn test(&self, host: &Box<dyn Host>) {
        println!("JAVA!");
        host.get_file("java");
    }
}
