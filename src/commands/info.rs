use crate::app::App;
use crate::error::Result;
use crate::object_model::Tag;
use crate::serialization::ProjectRecord;
use std::fs::read_to_string;

pub fn do_info(app: &App) -> Result<()> {
    let config_path = app.cwd.join(".isopy.yaml");
    let s = read_to_string(&config_path)?;
    let project_record = serde_yaml::from_str::<ProjectRecord>(&s)?;
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
