use crate::app::App;
use crate::helpers::get_asset;
use crate::object_model::{Tag, Version};
use crate::probe::make_project_config_path;
use crate::result::Result;
use crate::serialization::ProjectRecord;
use crate::util::safe_write_file;

pub fn do_new(app: &App, version: &Version, tag: &Option<Tag>) -> Result<()> {
    let project_config_path = make_project_config_path(&app.cwd);
    let project_record = ProjectRecord {
        python_version: version.clone(),
        tag: tag.clone(),
    };

    // Make sure there's exactly one asset matching this Python version and tag
    let assets = app.read_assets()?;
    _ = get_asset(&assets, &project_record.python_version, &project_record.tag)?;

    let s = serde_yaml::to_string(&project_record)?;
    safe_write_file(&project_config_path, s, false)?;
    println!(
        "Wrote project configuration file to {}",
        project_config_path.display()
    );

    Ok(())
}
