use crate::app::App;
use crate::error::{user, Result};
use crate::object_model::{Tag, Version};
use crate::serialization::ProjectRecord;
use crate::util::{is_already_exists, safe_write_to_file};

pub fn do_new(app: &App, version: &Version, tag: &Option<Tag>) -> Result<()> {
    let config_path = app.cwd.join(".isopy.yaml");
    let project_record = ProjectRecord {
        python_version: version.clone(),
        tag: tag.clone(),
    };

    let s = serde_yaml::to_string(&project_record)?;
    match safe_write_to_file(&config_path, s) {
        Ok(_) => {
            println!(
                "Wrote project configuration file to {}",
                config_path.display()
            );
        }
        Err(e) if is_already_exists(&e) => {
            return Err(user(format!(
                "Project configuration file {} already exists",
                config_path.display()
            )))
        }
        e => return e,
    }

    Ok(())
}
