use isopy_java::get_package_manager_factory as get_package_manager_factory_java;
use isopy_python::get_package_manager_factory as get_package_manager_factory_python;

fn main() {
    let package_manager_factories = vec![
        get_package_manager_factory_java(),
        get_package_manager_factory_python(),
    ];
    for f in package_manager_factories {
        let package_manager = f.make(f.name());
        package_manager.test();
    }
}
