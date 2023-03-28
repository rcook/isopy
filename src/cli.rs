use crate::object_model::EnvName;
use crate::object_model::{Tag, Version};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::result::Result;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short = 'd', long = "dir")]
    pub dir: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Available,
    Create {
        #[arg(value_parser = parse_env_name)]
        env_name: EnvName,
        #[arg(value_parser = parse_version)]
        version: Version,
        #[arg(short = 't', long = "tag", value_parser = parse_tag)]
        tag: Option<Tag>,
    },
    Download {
        #[arg(value_parser = parse_version)]
        version: Version,
        #[arg(short = 't', long = "tag", value_parser = parse_tag)]
        tag: Option<Tag>,
    },
    Downloaded,
    Info,
    Init,
    List,
    New {
        #[arg(value_parser = parse_version)]
        version: Version,
        #[arg(short = 't', long = "tag", value_parser = parse_tag)]
        tag: Option<Tag>,
    },
    Scratch,
    Shell {
        #[arg(short = 'e', long = "env", value_parser = parse_env_name)]
        env_name: Option<EnvName>,
    },
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
