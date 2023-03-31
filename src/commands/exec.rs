use crate::app::App;
use crate::error::Result;
use crate::object_model::EnvName;
use crate::shell::{get_shell_info, Command};

pub fn do_exec(
    app: &App,
    env_name_opt: Option<&EnvName>,
    program: &String,
    args: Vec<String>,
) -> Result<()> {
    let shell_info = get_shell_info(app, env_name_opt)?;
    let mut command = Command::new(program);
    for arg in args {
        command.arg(arg);
    }

    command.exec(&shell_info)?;
    Ok(())
}
