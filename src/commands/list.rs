use crate::app::App;
use crate::error::Result;

pub async fn do_list(app: &App) -> Result<()> {
    for env in app.read_envs()? {
        println!(
            "{}, {}, {}, {}",
            env.name,
            env.python_dir.display(),
            env.python_version,
            env.tag
        );
    }

    Ok(())
}
