use crate::app::App;
use crate::helpers::get_asset;
use crate::object_model::{Tag, Version};
use crate::result::{user, Result};
use crate::serialization::ProjectRecord;
use crate::util::{is_already_exists, safe_write_file};

pub fn do_new(app: &App, version: &Version, tag: &Option<Tag>) -> Result<()> {
    let project_config_path = app.get_project_config_path();
    let project_record = ProjectRecord {
        python_version: version.clone(),
        tag: tag.clone(),
    };

    // Make sure there's exactly one asset matching this Python version and tag
    let assets = app.read_assets()?;
    _ = get_asset(&assets, &project_record.python_version, &project_record.tag)?;

    let s = serde_yaml::to_string(&project_record)?;
    match safe_write_file(&project_config_path, s, false) {
        Ok(_) => {
            println!(
                "Wrote project configuration file to {}",
                project_config_path.display()
            );
        }
        Err(e) if is_already_exists(&e) => {
            return Err(user(format!(
                "Project configuration file {} already exists",
                project_config_path.display()
            )))
        }
        e => return e,
    }

    Ok(())
}
