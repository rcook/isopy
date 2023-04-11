use crate::app::App;
use crate::object_model::{Environment, EnvironmentName};
use crate::result::{user, Result};
use crate::serialization::UseRecord;
use crate::util::safe_write_file;

pub fn do_use(app: &App, environment_name: &EnvironmentName) -> Result<()> {
    let use_yaml_path = app.use_dir(&app.cwd)?.join("use.yaml");
    if use_yaml_path.is_file() {
        return Err(user(format!(
            "Use is already configured for directory {}",
            app.cwd.display()
        )));
    }

    let environment = Environment::infer(app, Some(environment_name))?;

    safe_write_file(
        use_yaml_path,
        serde_yaml::to_string(&UseRecord {
            dir: app.cwd.clone(),
            environment_name: environment.name,
        })?,
        false,
    )?;
    Ok(())
}
