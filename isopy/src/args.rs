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
use crate::env::{ISOPY_CONFIG_DIR_ENV_NAME, ISOPY_LOG_LEVEL_ENV_NAME};
use crate::moniker::Moniker;
use crate::package_id::PackageId;
use crate::wrapper_file_name::WrapperFileName;
use clap::{ArgAction, Args as ClapArgs, Parser, Subcommand, ValueEnum};
use clap_complete::Shell as ClapCompleteShell;
use isopy_lib::{Platform as IsopyLibPlatform, Shell as IsopyLibShell};
use joat_repo::MetaId;
use log::LevelFilter;
use path_absolutize::Absolutize;
use std::path::PathBuf;
use std::result::Result;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
const PACKAGE_HOME_PAGE: &str = env!("CARGO_PKG_HOMEPAGE");
const PACKAGE_BUILD_VERSION: Option<&str> = option_env!("RUST_TOOL_ACTION_BUILD_VERSION");

#[derive(Debug, Parser)]
#[command(
    name = PACKAGE_NAME,
    version = PACKAGE_VERSION,
    about = format!("{PACKAGE_DESCRIPTION} {PACKAGE_VERSION}"),
    after_help = format!("{PACKAGE_HOME_PAGE}\nhttps://github.com/rcook/isopy{}", PACKAGE_BUILD_VERSION.map(|x| format!("\n\n{}", x)).unwrap_or_else(|| String::from("")))
)]
pub(crate) struct Args {
    #[arg(
        global = true,
        help = "Path to isopy configuration directory",
        short = 'd',
        long = "config-dir",
        value_parser = parse_absolute_path,
        env = ISOPY_CONFIG_DIR_ENV_NAME
    )]
    pub(crate) config_dir: Option<PathBuf>,

    #[arg(
        global = true,
        help = "Path to working directory",
        short = 'c',
        long = "cwd",
        value_parser = parse_absolute_path
    )]
    pub(crate) cwd: Option<PathBuf>,

    #[arg(
        global = true,
        help = "Logging level",
        short = 'l',
        long = "level",
        default_value_t = LogLevel::Info,
        value_enum,
        env = ISOPY_LOG_LEVEL_ENV_NAME
    )]
    pub(crate) log_level: LogLevel,

    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
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

    #[command(name = "completions", about = "Generate shell completions")]
    Completions {
        #[arg(help = "Shell", long = "shell", value_enum)]
        shell: Option<ClapCompleteShell>,
    },

    #[command(
        name = "delete",
        about = "Delete environment corresponding to project directory"
    )]
    Delete {
        #[arg(help = "Project directory", value_parser = parse_absolute_path)]
        project_dir: PathBuf,
    },

    #[command(name = "download", about = "Download package")]
    Download {
        #[arg(help = "Package ID")]
        package_id: PackageId,

        #[arg(
            help = "Optional tags",
            short = 't',
            long = "tags",
            num_args = 0..,
            value_delimiter = ','
        )]
        tags: Option<Vec<String>>,
    },

    #[command(name = "env-init", about = "Install package into environment")]
    EnvInit {
        #[arg(help = "Package ID")]
        package_id: PackageId,

        // Reference: https://jwodder.github.io/kbits/posts/clap-bool-negate/
        // --download/--no-download with default of "false"
        #[arg(
            help = "Download package if required",
            long = "download",
            overrides_with = "_no_download",
            default_value_t = false
        )]
        download: bool,

        #[arg(help = "Do not download package if missing", long = "no-download")]
        _no_download: bool,
    },

    #[command(name = "env-list", about = "List environments")]
    EnvList {
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

    #[command(
        name = "init",
        about = "Install configured project packages into environment"
    )]
    Init {
        // Reference: https://jwodder.github.io/kbits/posts/clap-bool-negate/
        // --download/--no-download with default of "false"
        #[arg(
            help = "Download package if required",
            long = "download",
            overrides_with = "_no_download",
            default_value_t = false
        )]
        download: bool,

        #[arg(help = "Do not download package if missing", long = "no-download")]
        _no_download: bool,
    },

    #[command(
        name = "link",
        about = "Use existing environment for current directory"
    )]
    Link {
        #[arg(help = "Directory ID", value_parser = parse_meta_id)]
        dir_id: MetaId,
    },

    #[command(name = "packages", about = "List packages")]
    Packages {
        #[arg(help = "Package manager")]
        moniker: Option<Moniker>,

        #[arg(
            help = "Subset of packages to list",
            short = 'f',
            long = "filter",
            default_value_t = PackageFilter::Local,
            value_enum
        )]
        filter: PackageFilter,

        #[arg(
            help = "Optional tags",
            short = 't',
            long = "tags",
            num_args = 0..,
            value_delimiter = ','
        )]
        tags: Option<Vec<String>>,
    },

    #[command(name = "project", about = "Configure project")]
    Project {
        #[arg(help = "Package ID")]
        package_id: PackageId,
    },

    #[command(name = "prompt", about = "Show brief information in shell prompt")]
    Prompt(PromptConfig),

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

    #[command(name = "tags", about = "List tags")]
    Tags {
        #[arg(help = "Package manager")]
        moniker: Option<Moniker>,
    },

    #[command(name = "update", about = "Update package indices")]
    Update {
        #[arg(help = "Package manager")]
        moniker: Option<Moniker>,
    },

    #[command(
        name = "wrap",
        about = "Generate environment wrapper script in bin directory for script"
    )]
    Wrap {
        #[arg(help = "Wrapper file name")]
        wrapper_file_name: WrapperFileName,

        #[arg(help = "Script path", value_parser = parse_absolute_path)]
        script_path: PathBuf,

        #[arg(help = "Base directory", value_parser = parse_absolute_path)]
        base_dir: PathBuf,

        #[arg(
            help = "Platform",
            short = 'p',
            long = "platform",
            default_value_t = Platform::default(),
            value_enum
        )]
        platform: Platform,

        #[arg(
            help = "Shell script type",
            short = 's',
            long = "shell",
            default_value_t = Shell::default(),
            value_enum
        )]
        shell: Shell,

        // Reference: https://jwodder.github.io/kbits/posts/clap-bool-negate/
        #[arg(
            help = "Force overwrite of output file",
            short = 'f',
            long = "force",
            overrides_with = "_no_force"
        )]
        force: bool,

        #[arg(help = "Do not force overwrite of output file", long = "no-force")]
        _no_force: bool,
    },
}

#[derive(ClapArgs, Debug)]
pub(crate) struct PromptConfig {
    #[arg(
        help = "String to output before non-empty prompt",
        short = 'b',
        long = "before"
    )]
    pub(crate) before: Option<String>,

    #[arg(
        help = "String to output after non-empty prompt",
        short = 'a',
        long = "after"
    )]
    pub(crate) after: Option<String>,

    #[arg(
        help = "Message to display when running in isopy shell",
        long = "shell"
    )]
    pub(crate) shell_message: Option<String>,

    #[arg(
        help = "Message to display when isopy environment available",
        long = "available"
    )]
    pub(crate) available_message: Option<String>,

    #[arg(
        help = "Message to display if isopy configuration file is available",
        long = "config"
    )]
    pub(crate) config_message: Option<String>,

    #[arg(help = "Message to display when isopy error occurs", long = "error")]
    pub(crate) error_message: Option<String>,
}

#[derive(Clone, Debug, ValueEnum)]
pub(crate) enum LogLevel {
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

#[derive(Clone, Debug, ValueEnum)]
pub(crate) enum Platform {
    #[clap(name = "linux")]
    Linux,

    #[clap(name = "macos")]
    MacOS,

    #[clap(name = "windows")]
    Windows,
}

impl Default for Platform {
    #[cfg(target_os = "linux")]
    fn default() -> Self {
        Self::Linux
    }

    #[cfg(target_os = "macos")]
    fn default() -> Self {
        Self::MacOS
    }

    #[cfg(target_os = "windows")]
    fn default() -> Self {
        Self::Windows
    }
}

impl From<Platform> for IsopyLibPlatform {
    fn from(value: Platform) -> Self {
        match value {
            Platform::Linux => Self::Linux,
            Platform::MacOS => Self::MacOS,
            Platform::Windows => Self::Windows,
        }
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub(crate) enum Shell {
    #[clap(name = "bash")]
    Bash,

    #[clap(name = "cmd")]
    Cmd,
}

impl Default for Shell {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn default() -> Self {
        Self::Bash
    }

    #[cfg(target_os = "windows")]
    fn default() -> Self {
        Self::Cmd
    }
}

impl From<Shell> for IsopyLibShell {
    fn from(value: Shell) -> Self {
        match value {
            Shell::Bash => Self::Bash,
            Shell::Cmd => Self::Cmd,
        }
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub(crate) enum PackageFilter {
    #[clap(name = "all")]
    All,

    #[clap(name = "local")]
    Local,

    #[clap(name = "remote")]
    Remote,
}

impl From<PackageFilter> for isopy_lib::PackageFilter {
    fn from(value: PackageFilter) -> Self {
        match value {
            PackageFilter::All => Self::All,
            PackageFilter::Local => Self::Local,
            PackageFilter::Remote => Self::Remote,
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
