//! Principles — stable, high-confidence generalizations extracted from experience.

use serde::{Deserialize, Serialize};

/// A general principle or rule extracted from consistent patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principle {
    pub principle_id: String,
    pub statement: String,
    pub confidence: f64,
    pub evidence_ids: Vec<String>,
}

impl Principle {
    pub fn new(principle_id: String, statement: String) -> Self {
        Self {
            principle_id,
            statement,
            confidence: 0.5,
            evidence_ids: Vec::new(),
        }
    }
}
