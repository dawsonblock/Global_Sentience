//! Abstraction — generalized concepts pulled out of specific instances.

use serde::{Deserialize, Serialize};

/// A generalized concept extracted from multiple specific instances.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Abstraction {
    pub label: String,
    pub source_count: usize,
    pub confidence: f64,
}

impl Abstraction {
    pub fn new(label: String, source_count: usize) -> Self {
        Self {
            label,
            source_count,
            confidence: 0.5,
        }
    }
}
