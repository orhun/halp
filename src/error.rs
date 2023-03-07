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
}

/// Type alias for the standard [`Result`] type.
pub type Result<T> = std::result::Result<T, Error>;
