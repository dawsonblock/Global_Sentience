//! Archive backend trait and error types.

use thiserror::Error;

/// Errors that can occur during archive operations.
#[derive(Debug, Error)]
pub enum ArchiveError {
    #[error("file extension rejected: expected .gwlog or .jsonl, got {0}")]
    ExtensionRejected(String),

    #[error("not implemented")]
    NotImplemented,

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Trait for persisting and retrieving symbolic frames and event logs.
pub trait ArchiveBackend: Send + Sync {
    /// Append a serialized frame to the archive.
    fn append(&mut self, frame_data: &[u8]) -> Result<(), ArchiveError>;

    /// Load all frames from the archive.
    fn load_all(&mut self) -> Result<Vec<Vec<u8>>, ArchiveError>;

    /// Return the archive path as a string.
    fn path_str(&self) -> String;
}
