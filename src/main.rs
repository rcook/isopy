mod app;
mod cli;
mod commands;
mod constants;
mod helpers;
mod object_model;
mod probe;
mod repository;
mod result;
mod serialization;
mod shell;
mod util;

use crate::app::App;
use crate::cli::{Args, Command};
use crate::commands::{
    do_available, do_create, do_download, do_downloaded, do_exec, do_generate_repositories_yaml,
    do_info, do_init, do_list, do_new, do_scratch, do_shell, do_use, do_wrap,
};
use crate::constants::{GENERAL_ERROR, OK, USAGE};
use crate::probe::default_isopy_dir;
use crate::result::{could_not_infer_isopy_dir, Error, Result};
use clap::Parser;
use colour::red_ln;
use std::env::current_dir;
use std::process::exit;
use tokio;

async fn run() -> Result<()> {
    let cwd = current_dir()?;
    let args = Args::parse();
    let dir = args
        .dir
        .or_else(default_isopy_dir)
        .ok_or_else(|| could_not_infer_isopy_dir())?;
    let app = App::new(cwd, dir);

    match args.command {
        Command::Available => do_available(&app).await?,
        Command::Create {
            env_name,
            version,
            tag,
        } => do_create(&app, &env_name, &version, &tag).await?,
        Command::Download { version, tag } => do_download(&app, &version, &tag).await?,
        Command::Downloaded => do_downloaded(&app)?,
        Command::Exec {
            env_name,
            program,
            args,
        } => do_exec(&app, env_name.as_ref(), &program, args)?,
        Command::GenerateRepositoriesYaml {
            local_repository_dir,
        } => do_generate_repositories_yaml(&app, local_repository_dir)?,
        Command::Info => do_info(&app)?,
        Command::Init => do_init(&app).await?,
        Command::List => do_list(&app).await?,
        Command::New { version, tag } => do_new(&app, &version, &tag)?,
        Command::Scratch => do_scratch(&app).await?,
        Command::Shell { env_name } => do_shell(&app, env_name.as_ref())?,
        Command::Use { env_name } => do_use(&app, &env_name)?,
        Command::Wrap {
            wrapper_path,
            script_path,
            base_dir,
        } => do_wrap(&app, &wrapper_path, &script_path, &base_dir)?,
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    exit(match run().await {
        Ok(_) => OK,
        Err(Error::User { message }) => {
            red_ln!("{}", message);
            USAGE
        }
        Err(Error::Reportable { message, .. }) => {
            red_ln!("{}", message);
            GENERAL_ERROR
        }
        Err(e) => {
            red_ln!("Unhandled error: {:#?}", e);
            GENERAL_ERROR
        }
    })
}
