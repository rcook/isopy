use crate::app::App;
use crate::object_model::{Environment, EnvironmentName};
use crate::result::{user, Result};
use crate::shell::{Command, ISOPY_ENV_NAME};
use std::env::{var, VarError};

pub fn do_shell(app: &App, environment_nam: Option<&EnvironmentName>) -> Result<()> {
    match var(ISOPY_ENV_NAME) {
        Ok(_) => {
            return Err(user("You are already in an isopy shell"));
        }
        Err(VarError::NotPresent) => {}
        Err(e) => return Err(e)?,
    }

    let environment = Environment::infer(app, environment_nam)?;
    Command::new_shell().exec(&environment)?;
    Ok(())
}
