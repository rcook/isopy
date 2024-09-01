mod java_package_manager;
mod java_package_manager_factory;

pub async fn get_package_manager_factory() -> anyhow::Result<isopy_api::PackageManagerFactory> {
    Ok(java_package_manager_factory::JavaPackageManagerFactory::new().await?)
}
