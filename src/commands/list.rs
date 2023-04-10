use crate::app::App;
use crate::result::Result;

pub async fn do_list(app: &App) -> Result<()> {
    let recs = app.read_named_environments()?;
    if !recs.is_empty() {
        println!("Named environments:");
        for rec in recs {
            println!(
                "  {}, {}, {}, {}",
                rec.name,
                rec.python_dir_rel.display(),
                rec.python_version,
                rec.tag
            );
        }
    }

    let recs = app.read_project_environments()?;
    if !recs.is_empty() {
        println!("Project environments:");
        for rec in recs {
            println!(
                "  {}, {}, {}, {}",
                rec.config_path.display(),
                rec.python_dir_rel.display(),
                rec.python_version,
                rec.tag
            );
        }
    }

    let recs = app.read_uses()?;
    if !recs.is_empty() {
        println!("Uses:");
        for rec in recs {
            println!("  {}, {}", rec.dir.display(), rec.environment_name);
        }
    }

    Ok(())
}
