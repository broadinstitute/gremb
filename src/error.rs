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
