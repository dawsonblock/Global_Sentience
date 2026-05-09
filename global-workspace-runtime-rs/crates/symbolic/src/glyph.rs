//! Glyphs — compressed symbolic labels for efficient communication.
//!
//! A Glyph is a **computational abstraction only**. It does not represent
//! sensory experience, qualia, or phenomenal consciousness. Glyphs are tokens
//! used by the symbolic reasoning machinery to reference patterns compactly.

use serde::{Deserialize, Serialize};

/// A compressed, labeled encoding of a symbol or concept.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Glyph {
    pub glyph_id: String,
    /// Human-readable label (for debugging/documentation only).
    pub label: String,
    /// The symbol or concept this glyph references.
    pub origin_symbol: String,
}

impl Glyph {
    pub fn new(glyph_id: String, label: String, origin_symbol: String) -> Self {
        Self {
            glyph_id,
            label,
            origin_symbol,
        }
    }
}
