// Copyright (c) 2023 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use crate::app::App;
use crate::args::{Args, Command, PackageFilter};
use crate::constants::CONFIG_DIR_NAME;
use crate::env::set_up_env;
use crate::status::StatusResult;
use crate::terminal::reset_terminal;
use anyhow::{bail, Result};
use clap::Parser;
use isopy_lib::TagFilter;
use joat_logger::init_ui;
use joat_repo::RepoConfig;
use log::{set_max_level, LevelFilter};
use std::env::current_dir;
use std::path::PathBuf;

fn set_up() -> Result<()> {
    reset_terminal();
    init_ui(true)?;
    set_max_level(LevelFilter::Trace);
    set_up_env()?;
    Ok(())
}

fn default_config_dir() -> Option<PathBuf> {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    {
        Some(dirs::config_dir()?.join(CONFIG_DIR_NAME))
    }

    #[cfg(target_os = "macos")]
    {
        Some(dirs::home_dir()?.join(".config").join(CONFIG_DIR_NAME))
    }
}

pub(crate) async fn run() -> StatusResult {
    set_up()?;

    let args = Args::parse();

    set_max_level(args.log_level.into());

    let Some(config_dir) = args.config_dir.or_else(default_config_dir) else {
        bail!("Could not infer isopy cache directory location: please specify using --dir option")
    };

    let repo = match RepoConfig::default(&config_dir, None).repo() {
        Ok(Some(r)) => r,
        Ok(None) => {
            bail!("Could not acquire lock on repository: is another instance of isopy running?")
        }
        Err(e) => bail!(e),
    };

    let cwd = match args.cwd {
        Some(c) => c,
        None => current_dir()?,
    };

    let app = App::new(&cwd, &config_dir, repo, args.show_progress);
    run_command(app, args.command).await
}

async fn run_command(app: App, command: Command) -> StatusResult {
    use crate::args::Command::*;
    use crate::commands::{
        do_check, do_completions, do_download, do_env, do_info, do_init, do_link, do_list,
        do_packages, do_project, do_prompt, do_remove, do_run, do_scratch, do_shell, do_tags,
        do_update, do_wrap,
    };

    match command {
        Check { clean, .. } => do_check(&app, clean),
        Completions { shell } => do_completions(shell),
        Download { package_id, tags } => {
            do_download(&app, &package_id, &TagFilter::new(tags)).await
        }
        Env {
            package_id,
            download,
            ..
        } => do_env(&app, &package_id, download).await,
        Info => do_info(&app),
        Init { download, .. } => do_init(&app, download).await,
        Link { dir_id } => do_link(&app, &dir_id),
        List { verbose, .. } => do_list(&app, verbose),
        Packages {
            moniker,
            filter,
            tags,
            verbose,
            ..
        } => {
            do_packages(
                &app,
                &moniker,
                PackageFilter::to_source_filter(filter),
                &TagFilter::new(tags),
                verbose,
            )
            .await
        }
        Project { package_id } => do_project(&app, &package_id),
        Prompt(prompt_config) => do_prompt(&app, &prompt_config),
        Remove { project_dir } => do_remove(&app, &project_dir).await,
        Run { program, args } => do_run(app, &program, &args),
        Scratch => do_scratch(&app).await,
        Shell { verbose, .. } => do_shell(app, verbose),
        Tags { moniker } => do_tags(&app, &moniker).await,
        Update { moniker } => do_update(&app, &moniker).await,
        Wrap {
            wrapper_file_name,
            script_path,
            platform,
            shell,
            force,
            ..
        } => do_wrap(
            &app,
            &wrapper_file_name,
            &script_path,
            platform.into(),
            shell.into(),
            force,
        ),
    }
}
