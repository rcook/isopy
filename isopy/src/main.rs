use isopy_java::get_package_manager_factory as get_package_manager_factory_java;
use isopy_python::get_package_manager_factory as get_package_manager_factory_python;

fn main() {
    let package_factories = vec![
        get_package_manager_factory_java(),
        get_package_manager_factory_python(),
    ];
    for f in package_factories {
        println!("name={}", f.name());
    }
}
