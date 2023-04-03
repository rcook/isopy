use crate::app::App;
use crate::env_info::get_env_info;
use crate::object_model::EnvName;
use crate::result::Result;
use crate::shell::Command;

pub fn do_exec(
    app: &App,
    env_name_opt: Option<&EnvName>,
    program: &String,
    args: Vec<String>,
) -> Result<()> {
    let mut command = Command::new(program);
    for arg in args {
        command.arg(arg);
    }

    let env_info = get_env_info(app, env_name_opt)?;
    command.exec(&env_info)?;
    Ok(())
}
