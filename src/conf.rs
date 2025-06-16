use crate::error::Error;

pub(crate) enum Config {
    Evenflow
}

pub(crate) fn get_config() -> Result<Config, Error> {
    Ok(Config::Evenflow)
}
