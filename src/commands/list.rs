use crate::config::Config;
use crate::error::Result;
use crate::object_model::EnvName;
use crate::util::osstr_to_str;
use std::fs::read_dir;

pub async fn do_list(config: &Config) -> Result<()> {
    for d in read_dir(&config.envs_dir)? {
        let env_name = match EnvName::parse(osstr_to_str(&d?.file_name())?) {
            Some(x) => x,
            None => continue,
        };

        let env = match config.read_env(&env_name)? {
            Some(x) => x,
            None => continue,
        };

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
