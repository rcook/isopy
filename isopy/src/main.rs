mod app;
mod app_context;

use crate::app::App;
use anyhow::{anyhow, Result};
use app_context::AppContext;
use dirs::config_dir;
use std::rc::Rc;

fn main() -> Result<()> {
    let app = Rc::new(App::new(
        config_dir()
            .ok_or_else(|| anyhow!("Could not determine config directory"))?
            .join(".isopy-tng"),
    ));

    for f in app.package_manager_factories() {
        println!("Factory: {}", f.name());
    }

    let package_manager_factory = app.get_package_manager_factory("python")?;
    let package_manager = package_manager_factory.make(package_manager_factory.name())?;
    let ctx = AppContext::new(app, package_manager.name());
    package_manager.test(&ctx)?;

    Ok(())
}
