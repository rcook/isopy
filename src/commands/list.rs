use crate::app::App;
use crate::result::Result;

pub async fn do_list(app: &App) -> Result<()> {
    let named_envs = app.read_named_envs()?;
    if named_envs.len() > 0 {
        println!("Named environments:");
        for named_env in named_envs {
            println!(
                "  {}, {}, {}, {}",
                named_env.name,
                named_env.python_dir.display(),
                named_env.python_version,
                named_env.tag
            );
        }
    }

    println!("HELLO");
    let anonymous_envs = app.read_anonymous_envs()?;
    if anonymous_envs.len() > 0 {
        println!("Anonymous environments:");
        for anonymous_env in anonymous_envs {
            println!(
                "  {}, {}, {}, {}",
                anonymous_env.config_path.display(),
                anonymous_env.python_dir.display(),
                anonymous_env.python_version,
                anonymous_env.tag
            );
        }
    }

    println!("HELLO");
    let uses = app.read_uses()?;
    if uses.len() > 0 {
        println!("Uses:");
        for use_ in uses {
            println!("  {}, {}", use_.dir.display(), use_.env_name);
        }
    }

    Ok(())
}
