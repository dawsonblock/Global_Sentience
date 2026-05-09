//! JSONL-based archive backend (never accepts .mv2 files).

use crate::archive::{ArchiveBackend, ArchiveError};
use std::path::PathBuf;

/// Archive that stores frames as JSONL (one JSON object per line).
/// Explicitly rejects .mv2 file extensions.
pub struct JsonlArchiveBackend {
    path: PathBuf,
}

impl JsonlArchiveBackend {
    /// Create a new JSONL archive, validating the file extension.
    pub fn new(path: PathBuf) -> Result<Self, ArchiveError> {
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if ext == "mv2" {
            return Err(ArchiveError::ExtensionRejected(
                ".mv2 not allowed; use .gwlog or .jsonl instead".to_string(),
            ));
        }
        if ext != "gwlog" && ext != "jsonl" {
            return Err(ArchiveError::ExtensionRejected(format!(
                "expected .gwlog or .jsonl, got .{}",
                ext
            )));
        }
        Ok(Self { path })
    }
}

impl ArchiveBackend for JsonlArchiveBackend {
    fn append(&mut self, frame_data: &[u8]) -> Result<(), ArchiveError> {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        f.write_all(frame_data)?;
        f.write_all(b"\n")?;
        Ok(())
    }

    fn load_all(&mut self) -> Result<Vec<Vec<u8>>, ArchiveError> {
        use std::fs;
        use std::io::{BufRead, BufReader};

        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let f = fs::File::open(&self.path)?;
        let reader = BufReader::new(f);
        let mut frames = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if !line.is_empty() {
                frames.push(line.as_bytes().to_vec());
            }
        }

        Ok(frames)
    }

    fn path_str(&self) -> String {
        self.path.display().to_string()
    }
}
