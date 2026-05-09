//! Memvid backend stub — not implemented.
//!
//! A real Memvid archive backend would integrate with the memvid-main
//! library to store and retrieve symbolic traces in Memvid format.
//! For now, all operations return NotImplemented.

use crate::archive::{ArchiveBackend, ArchiveError};
use std::path::PathBuf;

/// Stub backend that rejects all operations.
pub struct MemvidBackend {
    path: PathBuf,
}

impl MemvidBackend {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl ArchiveBackend for MemvidBackend {
    fn append(&mut self, _frame_data: &[u8]) -> Result<(), ArchiveError> {
        Err(ArchiveError::NotImplemented)
    }

    fn load_all(&mut self) -> Result<Vec<Vec<u8>>, ArchiveError> {
        Err(ArchiveError::NotImplemented)
    }

    fn path_str(&self) -> String {
        self.path.display().to_string()
    }
}
