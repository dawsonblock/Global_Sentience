//! Symbolic memory tagging — associations between symbols and memory indices.

use serde::{Deserialize, Serialize};

/// A tag linking a symbol to a memory entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolicMemoryTag {
    pub key: String,
    pub symbol_id: String,
    pub confidence: f64,
}

impl SymbolicMemoryTag {
    pub fn new(key: String, symbol_id: String) -> Self {
        Self {
            key,
            symbol_id,
            confidence: 0.5,
        }
    }
}
