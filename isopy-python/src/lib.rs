mod tng;

pub async fn get_package_manager_factory() -> anyhow::Result<isopy_lib::tng::PackageManagerFactory>
{
    Ok(crate::tng::PythonPackageManagerFactory::new().await?)
}
