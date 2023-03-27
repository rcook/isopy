use crate::config::Config;
use crate::error::Result;
use crate::serialization::EnvRecord;
use serde_yaml::from_str;
use std::fs::{read_dir, read_to_string};

pub async fn do_list(config: &Config) -> Result<()> {
    for d in read_dir(&config.envs_dir)? {
        let env_path = d?.path().join("env.yaml");
        if env_path.is_file() {
            let s = read_to_string(&env_path)?;
            let env = from_str::<EnvRecord>(&s)?;
            println!(
                "{}: {}, {}, {}, {}",
                env_path.display(),
                env.name,
                env.python_dir.display(),
                env.python_version,
                env.tag
            );
        }
    }

    Ok(())
}
