//! Single frame of symbolic reasoning within a trace.

use crate::symbol::SymbolActivation;
use serde::{Deserialize, Serialize};

/// A single reasoning frame (e.g., a memory hit, a symbolic blend, a principle extraction).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolicFrame {
    pub frame_id: String,
    pub source: String, // e.g. "associative_stream", "conceptual_blender", "critic"
    pub active_symbols: Vec<SymbolActivation>,
    pub timestamp_ms: u64,
    /// Flag to indicate if the content of this frame is substantive or speculative.
    pub validated: bool,
}

impl SymbolicFrame {
    pub fn new(frame_id: String, source: String) -> Self {
        Self {
            frame_id,
            source,
            active_symbols: Vec::new(),
            timestamp_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            validated: false,
        }
    }
}
