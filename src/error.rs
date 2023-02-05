//! Contains the Result and Error types for server operations.
use std::{error::Error, fmt, io, result};

/// The result type for server operations.
pub type Result<T> = result::Result<T, SfsError>;

/// The error type which is returned from server operations.
#[derive(Debug)]
pub enum SfsError {
    /// The supplied path is not a valid directory.
    InvalidDirectory,
    /// An IO error occurred while trying to start the server.
    IoError(io::Error),
}

impl Error for SfsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidDirectory => None,
            Self::IoError(err) => Some(err),
        }
    }
}

impl fmt::Display for SfsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDirectory => write!(f, "Invalid directory"),
            Self::IoError(err) => err.fmt(f),
        }
    }
}

impl From<io::Error> for SfsError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}
