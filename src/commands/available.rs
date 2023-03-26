use crate::config::Config;
use crate::error::Result;

pub fn do_available(_config:&Config) -> Result<()> {
    println!("Hello World");
    Ok(())
}