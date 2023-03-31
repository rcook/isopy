use crate::app::App;
use crate::env_info::get_env_info;
use crate::error::{user, Result};
use crate::object_model::EnvName;
use crate::serialization::UseRecord;
use crate::util::{path_to_str, safe_write_file};
use md5::compute;

pub fn do_use(app: &App, env_name: &EnvName) -> Result<()> {
    let hex_digest = format!("{:x}", compute(path_to_str(&app.cwd)?));

    let use_yaml_path = app.uses_dir.join(&hex_digest).join("use.yaml");
    if use_yaml_path.is_file() {
        return Err(user(format!(
            "Use is already configured for directory {}",
            app.cwd.display()
        )));
    }

    let env_info = get_env_info(app, Some(env_name))?;

    safe_write_file(
        use_yaml_path,
        serde_yaml::to_string(&UseRecord {
            dir: app.cwd.clone(),
            env_name: env_info.env_name,
        })?,
        false,
    )?;
    Ok(())
}
