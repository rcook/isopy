use crate::app::App;
use crate::error::Result;

pub async fn do_list(app: &App) -> Result<()> {
    for named_env in app.read_named_envs()? {
        println!(
            "{}, {}, {}, {}",
            named_env.name,
            named_env.python_dir.display(),
            named_env.python_version,
            named_env.tag
        );
    }

    Ok(())
}
