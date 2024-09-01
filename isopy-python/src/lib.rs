mod archive_full_version;
mod archive_group;
mod archive_info;
mod archive_metadata;
mod archive_type;
mod python_package_manager;
mod python_package_manager_factory;

pub async fn get_package_manager_factory() -> anyhow::Result<isopy_lib::PackageManagerFactory> {
    Ok(python_package_manager_factory::PythonPackageManagerFactory::new().await?)
}
