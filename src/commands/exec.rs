use crate::app::App;
use crate::error::{user, Result};
use crate::object_model::EnvName;
use crate::shell::{exec, get_shell_info, ISOPY_ENV_NAME};
use std::env::{var, VarError};

pub fn do_exec(app: &App, env_name_opt: &Option<EnvName>) -> Result<()> {
    match var(ISOPY_ENV_NAME) {
        Ok(_) => {
            return Err(user("You are already in an isopy shell"));
        }
        Err(VarError::NotPresent) => {}
        Err(e) => return Err(e)?,
    }

    let shell_info = get_shell_info(app, env_name_opt)?;
    exec(app, &shell_info)?;
    Ok(())
}
