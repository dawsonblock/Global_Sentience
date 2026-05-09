//! Resonance scoring — how strongly a symbol "resonates" with current state.
//!
//! **CRITICAL**: Resonance scores are advisory only. They cannot override
//! or bypass critic rejection rules. If the critic has rejected a candidate,
//! high resonance does not make it acceptable.

use serde::{Deserialize, Serialize};

/// Measure of how well a symbol aligns with current context and internal state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceScore {
    pub symbol_id: String,
    pub score: f64, // 0.0–1.0
    pub cycle_id: u64,
}

impl ResonanceScore {
    pub fn new(symbol_id: String, cycle_id: u64) -> Self {
        Self {
            symbol_id,
            score: 0.5,
            cycle_id,
        }
    }
}
