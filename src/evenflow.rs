use crate::config::EvenflowConfig;
use crate::error::Error;

pub(crate) fn run(config: EvenflowConfig) -> Result<(), Error> {
    println!("Evenflow is running! Edge file is {}", config.edge_file.display());
    Ok(())
}