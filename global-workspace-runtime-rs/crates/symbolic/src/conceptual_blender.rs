//! Conceptual blending — combination of multiple symbol inputs into novel concepts.

use serde::{Deserialize, Serialize};

/// Result of conceptually blending multiple input symbols into a novel concept.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptBlend {
    pub blend_id: String,
    pub input_ids: Vec<String>,
    pub result_label: String,
    pub confidence: f64,
    /// Indicates whether this blend is speculative and requires further validation.
    pub validated: bool,
}

impl ConceptBlend {
    pub fn new(blend_id: String, input_ids: Vec<String>, result_label: String) -> Self {
        Self {
            blend_id,
            input_ids,
            result_label,
            confidence: 0.5,
            validated: false,
        }
    }
}
