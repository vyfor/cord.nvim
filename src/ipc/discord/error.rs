use std::{fmt, io};

#[derive(Debug)]
pub enum DiscordError {
    Io(io::Error),
    InvalidClientId(String),
    ConnectionClosed,
    PipeNotFound,
    Custom(String),
}

impl fmt::Display for DiscordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiscordError::Io(err) => write!(f, "IO error: {}", err),
            DiscordError::InvalidClientId(id) => {
                write!(f, "'{}' is not a valid client ID", id)
            }
            DiscordError::ConnectionClosed => {
                write!(f, "The connection was forcibly closed")
            }
            DiscordError::PipeNotFound => {
                write!(f, "Discord IPC pipe not found")
            }
            DiscordError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for DiscordError {}

impl From<io::Error> for DiscordError {
    fn from(err: io::Error) -> Self {
        DiscordError::Io(err)
    }
}

impl From<&str> for DiscordError {
    fn from(err: &str) -> Self {
        DiscordError::Custom(err.to_string())
    }
}

impl From<String> for DiscordError {
    fn from(err: String) -> Self {
        DiscordError::Custom(err)
    }
}
