use crate::app::App;
use crate::env_info::{get_env_info, ISOPY_ENV_NAME};
use crate::error::{user, Result};
use crate::object_model::EnvName;
use crate::shell::Command;
use std::env::{var, VarError};

pub fn do_shell(app: &App, env_name_opt: Option<&EnvName>) -> Result<()> {
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
