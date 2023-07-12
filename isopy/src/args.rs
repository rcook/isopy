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
use crate::package_id::PackageId;
use clap::{ArgAction, Parser, Subcommand, ValueEnum};
use joat_repo::MetaId;
use log::LevelFilter;
use path_absolutize::Absolutize;
use std::ffi::OsString;
use std::path::PathBuf;
use std::result::Result;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
const PACKAGE_HOME_PAGE: &str = env!("CARGO_PKG_HOMEPAGE");
const PACKAGE_BUILD_VERSION: Option<&str> = option_env!("RUST_TOOL_ACTION_BUILD_VERSION");

#[derive(Parser, Debug)]
#[command(
    name = PACKAGE_NAME,
    version = PACKAGE_VERSION,
    about = format!("{} {}", PACKAGE_DESCRIPTION, PACKAGE_VERSION),
    after_help = format!("{}\nhttps://github.com/rcook/isopy{}", PACKAGE_HOME_PAGE, PACKAGE_BUILD_VERSION.map(|x| format!("\n\n{}", x)).unwrap_or(String::from("")))
)]
pub struct Args {
    #[arg(
        global = true,
        help = "Path to isopy cache directory",
        short = 'd',
        long = "cache-dir",
        value_parser = parse_absolute_path,
        env = "ISOPY_CACHE_DIR"
    )]
    pub cache_dir: Option<PathBuf>,

    #[arg(
        global = true,
        help = "Path to working directory",
        short = 'c',
        long = "cwd",
        value_parser = parse_absolute_path
    )]
    pub cwd: Option<PathBuf>,

    #[arg(
        global = true,
        help = "Logging level",
        short = 'l',
        long = "level",
        default_value_t = LogLevel::Info,
        value_enum,
        env = "ISOPY_LOG_LEVEL"
    )]
    pub log_level: LogLevel,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(name = "add", about = "Add package to project")]
    Add {
        #[arg(help = "Package ID")]
        package_id: PackageId,
    },

    #[command(name = "available", about = "List packages available for download")]
    Available {
        // Reference: https://jwodder.github.io/kbits/posts/clap-bool-negate/
        // --verbose/--no-verbose with default of "false"
        #[arg(
            help = "Show detailed output",
            long = "verbose",
            overrides_with = "_no_verbose",
            default_value_t = false
        )]
        verbose: bool,

        #[arg(help = "Show brief output", long = "no-verbose")]
        _no_verbose: bool,
    },

    #[command(
        name = "check",
        about = "Check integrity of metadata directory and optionally clean up"
    )]
    Check {
        // Reference: https://jwodder.github.io/kbits/posts/clap-bool-negate/
        // --clean/--no-clean with default of "false"
        #[arg(
            help = "Clean up cache directory",
            long = "clean",
            overrides_with = "_no_clean",
            default_value_t = false
        )]
        clean: bool,

        #[arg(help = "Do not clean up cache directory", long = "no-clean")]
        _no_clean: bool,
    },

    #[command(name = "download", about = "Download package")]
    Download {
        #[arg(help = "Package ID")]
        package_id: PackageId,
    },

    #[command(name = "downloaded", about = "List locally downloaded packages")]
    Downloaded {
        // Reference: https://jwodder.github.io/kbits/posts/clap-bool-negate/
        // --verbose/--no-verbose with default of "false"
        #[arg(
            help = "Show detailed output",
            long = "verbose",
            overrides_with = "_no_verbose",
            default_value_t = false
        )]
        verbose: bool,

        #[arg(help = "Show brief output", long = "no-verbose")]
        _no_verbose: bool,
    },

    #[command(name = "info", about = "Show information")]
    Info,

    #[command(name = "install", about = "Install package into environment")]
    Install {
        #[arg(help = "Package ID")]
        package_id: PackageId,
    },

    #[command(
        name = "install-project",
        about = "Install project packages into environment"
    )]
    InstallProject,

    #[command(
        name = "link",
        about = "Use existing environment for current directory"
    )]
    Link {
        #[arg(help = "Directory ID", value_parser = parse_meta_id)]
        dir_id: MetaId,
    },

    #[command(name = "list", about = "List environments")]
    List,

    #[command(name = "prompt", about = "Show brief information in shell prompt")]
    Prompt,

    #[command(name = "run", about = "Run command in environment")]
    Run {
        #[arg(help = "Program")]
        program: String,

        #[arg(
            help = "Arguments",
            trailing_var_arg = true,
            allow_hyphen_values = true
        )]
        args: Vec<String>,
    },

    #[command(name = "scratch", about = "Experimental stuff")]
    Scratch,

    #[command(name = "shell", about = "Start environment shell")]
    Shell {
        // Reference: https://jwodder.github.io/kbits/posts/clap-bool-negate/
        // --verbose/--no-verbose with default of "true"
        #[arg(
            help = "Show detailed output",
            long = "no-verbose",
            default_value_t = true,
            action = ArgAction::SetFalse
        )]
        verbose: bool,

        #[arg(
            help = "Show brief output",
            long = "verbose",
            overrides_with = "verbose"
        )]
        _no_verbose: bool,
    },

    #[command(
        name = "wrap-command",
        about = "Generate environment wrapper script in bin directory for command"
    )]
    WrapCommand {
        #[arg(help = "Wrapper file name")]
        wrapper_file_name: OsString,

        #[arg(help = "Command")]
        command: String,

        #[arg(help = "Base directory", value_parser = parse_absolute_path)]
        base_dir: PathBuf,

        // Reference: https://jwodder.github.io/kbits/posts/clap-bool-negate/
        // --force/--no-force with default of "false"
        #[arg(
            help = "Force overwrite of output file",
            short = 'f',
            long = "force",
            overrides_with = "_no_force",
            default_value_t = false
        )]
        force: bool,

        #[arg(help = "Do not force overwrite of output file", long = "no-force")]
        _no_force: bool,
    },

    #[command(
        name = "wrap-script",
        about = "Generate environment wrapper script in bin directory for script"
    )]
    WrapScript {
        #[arg(help = "Wrapper file name")]
        wrapper_file_name: OsString,

        #[arg(help = "Script path", value_parser = parse_absolute_path)]
        script_path: PathBuf,

        #[arg(help = "Base directory", value_parser = parse_absolute_path)]
        base_dir: PathBuf,

        // Reference: https://jwodder.github.io/kbits/posts/clap-bool-negate/
        #[arg(
            help = "Force overwrite of output file",
            short = 'f',
            long = "force",
            overrides_with = "_no_force"
        )]
        force: bool,

        #[arg(help = "Dot not force overwrite of output file", long = "no-force")]
        _no_force: bool,
    },
}

#[derive(Clone, Debug, ValueEnum)]
pub enum LogLevel {
    #[clap(name = "off")]
    Off,

    #[clap(name = "error")]
    Error,

    #[clap(name = "warn")]
    Warn,

    #[clap(name = "info")]
    Info,

    #[clap(name = "debug")]
    Debug,

    #[clap(name = "trace")]
    Trace,
}

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Off => Self::Off,
            LogLevel::Error => Self::Error,
            LogLevel::Warn => Self::Warn,
            LogLevel::Info => Self::Info,
            LogLevel::Debug => Self::Debug,
            LogLevel::Trace => Self::Trace,
        }
    }
}

fn parse_absolute_path(s: &str) -> Result<PathBuf, String> {
    PathBuf::from(s)
        .absolutize()
        .map_err(|_| String::from("invalid path"))
        .map(|x| x.to_path_buf())
}

fn parse_meta_id(s: &str) -> Result<MetaId, String> {
    s.parse::<MetaId>()
        .map_err(|_| String::from("invalid meta ID"))
}
