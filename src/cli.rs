use crate::object_model::EnvName;
use crate::object_model::{Tag, Version};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::result::Result;

#[derive(Parser, Debug)]
#[command(about = "Isolated Python Tool Rust Edition!", version)]
pub struct Args {
    #[arg(help = "Path to isopy cache directory", short = 'd', long = "dir")]
    pub dir: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(about = "List Python packages available for download")]
    Available,
    #[command(about = "Create named Python environment")]
    Create {
        #[arg(help = "Environment name", value_parser = parse_env_name)]
        env_name: EnvName,
        #[arg(help = "Python version", value_parser = parse_version)]
        version: Version,
        #[arg(help = "Build tag", short = 't', long = "tag", value_parser = parse_tag)]
        tag: Option<Tag>,
    },
    #[command(about = "Download Python package")]
    Download {
        #[arg(help = "Python version", value_parser = parse_version)]
        version: Version,
        #[arg(help = "Build tag", short = 't', long = "tag", value_parser = parse_tag)]
        tag: Option<Tag>,
    },
    #[command(about = "List downloaded Python package")]
    Downloaded,
    #[command(about = "Show info about current Python environment")]
    Info,
    #[command(about = "Initialize current Python environment")]
    Init,
    #[command(about = "List named Python environments")]
    List,
    #[command(about = "Create project Python environment")]
    New {
        #[arg(help = "Python version", value_parser = parse_version)]
        version: Version,
        #[arg(help = "Build tag", short = 't', long = "tag", value_parser = parse_tag)]
        tag: Option<Tag>,
    },
    #[command(about = "Experimental")]
    Scratch,
    #[command(about = "Start shell for current Python environment")]
    Shell {
        #[arg(help = "Environment name", short = 'e', long = "env", value_parser = parse_env_name)]
        env_name: Option<EnvName>,
    },
    #[command(about = "Use specified named Python environment for current directory")]
    Use,
}

fn parse_env_name(s: &str) -> Result<EnvName, String> {
    EnvName::parse(s).ok_or(String::from("invalid env name"))
}

fn parse_version(s: &str) -> Result<Version, String> {
    Version::parse(s).ok_or(String::from("invalid version"))
}

fn parse_tag(s: &str) -> Result<Tag, String> {
    Ok(Tag::parse(s))
}
