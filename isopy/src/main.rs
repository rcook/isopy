use isopy_api::Host;
use isopy_java::get_package_manager_factory as get_package_manager_factory_java;
use isopy_python::get_package_manager_factory as get_package_manager_factory_python;

struct IsopyHost {}

impl Host for IsopyHost {
    fn get_file(&self, url: &str) {
        println!("get {url}");
    }
}

fn main() {
    let package_manager_factories = vec![
        get_package_manager_factory_java(),
        get_package_manager_factory_python(),
    ];

    let host = Box::new(IsopyHost {}) as Box<dyn Host>;

    for f in package_manager_factories {
        let package_manager = f.make(f.name());
        package_manager.test(&host);
    }
}
