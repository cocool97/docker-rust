use thiserror::Error;

/// Internal result type.
pub type Result<T> = std::result::Result<T, DockerError>;

/// Internal error type.
#[derive(Error, Debug)]
pub enum DockerError {
    /// Input/output errors.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    /// Serialization/Deserialization error
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}
