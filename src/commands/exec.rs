use crate::app::App;
use crate::object_model::{Environment, EnvironmentName};
use crate::result::Result;
use crate::shell::Command;

pub fn do_exec(
    app: &App,
    environment_name: Option<&EnvironmentName>,
    program: &String,
    args: Vec<String>,
) -> Result<()> {
    let mut command = Command::new(program);
    for arg in args {
        command.arg(arg);
    }

    let environment = Environment::infer(app, environment_name)?;
    command.exec(&environment)?;
    Ok(())
}
