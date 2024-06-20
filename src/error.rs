use thiserror::Error as ThisError;

/// Custom error type.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Error that may occur during I/O operations.
    #[error("IO error: `{0}`")]
    IoError(#[from] std::io::Error),
    /// Error that might occur while parsing TOML.
    #[error("TOML parsing error: `{0}`")]
    TomlError(#[from] toml::de::Error),
    /// Error that might occur while processing/sending requests.
    #[error("Request error: `{0}`")]
    RequestError(#[from] Box<ureq::Error>),
    /// Error that might occur when running on unsupported platforms.
    #[error("Unsupported platform.")]
    UnsupportedPlatformError,
    /// Error that might occur while serializing the configuration into TOML.
    #[error("TOML serialization error: `{0}`")]
    TomlSerializeError(#[from] toml::ser::Error),
    /// Error that might occur when tray to get help from an external provider.
    #[error("External help provider error: `{0}`")]
    ProviderError(String),
    /// Error that might occur during showing dialogues.
    #[error("Dialogue error: `{0}`")]
    DialogueError(#[from] dialoguer::Error),
    /// Error that might occur when the command times out.
    #[error("Command timed out after {0} seconds x_x")]
    TimeoutError(u64),
}

/// Type alias for the standard [`Result`] type.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::io::{Error as IoError, ErrorKind};

    #[test]
    fn test_error() {
        let message = "your computer is on fire!";
        let error = Error::from(IoError::new(ErrorKind::Other, message));
        assert_eq!(format!("IO error: `{message}`"), error.to_string());
        assert_eq!(
            format!("\"IO error: `{message}`\""),
            format!("{:?}", error.to_string())
        );
    }
}
