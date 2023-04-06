use crate::app::App;
use crate::object_model::{get_env_info, EnvironmentName};
use crate::result::{user, Result};
use crate::shell::{Command, ISOPY_ENV_NAME};
use std::env::{var, VarError};

pub fn do_shell(app: &App, env_name_opt: Option<&EnvironmentName>) -> Result<()> {
    match var(ISOPY_ENV_NAME) {
        Ok(_) => {
            return Err(user("You are already in an isopy shell"));
        }
        Err(VarError::NotPresent) => {}
        Err(e) => return Err(e)?,
    }

    let env_info = get_env_info(app, env_name_opt)?;
    Command::new_shell().exec(&env_info)?;
    Ok(())
}
