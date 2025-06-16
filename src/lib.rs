use crate::conf::Config;
use crate::error::Error;

pub mod error;
pub(crate) mod conf;
pub(crate) mod evenflow;

pub fn run() -> Result<(), Error> {
    let config = conf::get_config()?;
    match config {
        Config::Evenflow => { evenflow::run()?; }
    }
    Ok(())
}