use crate::config::Config;
use crate::error::Result;

pub async fn do_list(config: &Config) -> Result<()> {
    for env in config.read_envs()? {
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
