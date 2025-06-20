use std::fmt::{Debug, Display, Formatter};

pub struct Error {
    message: String,
    source: Option<Box<dyn std::error::Error>>,
}

impl Error {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Error {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source<S, E>(message: S, source: E) -> Self
    where
        S: Into<String>,
        E: std::error::Error + 'static,
    {
        Error {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }
    pub fn wrap<S, E>(message: S, error: E) -> Self
    where
        S: Into<String>,
        E: std::error::Error + 'static,
    {
        Error::with_source(message, error)
    }
}

pub trait ResultWrapErr<T, E: std::error::Error + 'static> {
    fn wrap_err<S>(self, message: S) -> Result<T, Error> where S: Into<String>;
}

impl<T, E: std::error::Error + 'static> ResultWrapErr<T, E> for Result<T, E> {
    fn wrap_err<S>(self, message: S) -> Result<T, Error>
    where S: Into<String> { self.map_err(|e| Error::wrap(message, e)) }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(source) = &self.source {
            write!(f, ": {}", source)?;
        }
        Ok(())
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_deref()
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self { Error::new(message) }
}

impl From<String> for Error {
    fn from(message: String) -> Self { Error::new(message) }
}

impl From<csv::Error> for Error {
    fn from(error: csv::Error) -> Self {
        Error::with_source("CSV error", error)
    }
}