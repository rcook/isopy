use crate::app::App;
use crate::object_model::Tag;
use crate::result::Result;

pub fn do_info(app: &App) -> Result<()> {
    println!("Working directory: {}", app.cwd.display());
    println!("isopy directory: {}", app.dir.display());
    let project_record = app.read_project_config()?;
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
