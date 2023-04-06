use crate::app::App;
use crate::object_model::Tag;
use crate::result::Result;

pub fn do_info(app: &App) -> Result<()> {
    println!("Working directory: {}", app.cwd.display());
    println!("isopy directory: {}", app.dir.display());
    match app.read_project(&app.cwd)? {
        Some(project) => {
            println!(
                "Project configuration file: {}",
                project.config_path.display()
            );
            println!("Python version: {}", project.python_version);
            println!(
                "Build tag: {}",
                project
                    .tag
                    .as_ref()
                    .map(Tag::to_string)
                    .unwrap_or(String::from("(none)"))
            );
            let dir = app.project_environment_dir(&project.config_path)?;
            println!("Environment directory: {}", dir.display());
        }
        None => {}
    }
    Ok(())
}
