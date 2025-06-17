use crate::config::Config;
use crate::error::Error;

pub mod error;
pub(crate) mod config;
pub(crate) mod evenflow;

pub fn run() -> Result<(), Error> {
    let config = config::get_config()?;
    match config {
        Config::Evenflow(config) => { evenflow::run(config)?; }
    }
    Ok(())
}