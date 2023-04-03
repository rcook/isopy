use crate::app::App;
use crate::object_model::Tag;
use crate::result::Result;
use crate::serialization::ProjectRecord;
use crate::util::read_yaml_file;

pub fn do_info(app: &App) -> Result<()> {
    let project_config_path = app.cwd.join(".isopy.yaml");
    let project_record = read_yaml_file::<ProjectRecord, _>(&project_config_path)?;
    println!("python_version: {}", project_record.python_version);
    println!(
        "tag: {}",
        project_record
            .tag
            .as_ref()
            .map(Tag::to_string)
            .unwrap_or(String::from("(none)"))
    );
    Ok(())
}
