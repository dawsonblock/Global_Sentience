//! Symbolic crate: conceptual blending stubs used by gw-workspace and runtime-cli.

use runtime_core::ActionType;
use serde::{Deserialize, Serialize};

// --- Core symbolic types ---
pub mod symbol;
pub mod symbol_graph;
pub mod symbolic_frame;
pub mod symbolic_trace;

// --- Symbolic streams ---
pub mod analytic_stream;
pub mod associative_stream;
pub mod conceptual_blender;
pub mod creative_stream;

// --- Symbolic operations ---
pub mod abstraction;
pub mod compression;
pub mod glyph;
pub mod principle;
pub mod resonance;
pub mod symbolic_memory;

// Re-export key types
pub use abstraction::Abstraction;
pub use analytic_stream::AnalyticStream;
pub use associative_stream::AssociativeStream;
pub use compression::{CompressionStats, SymbolicCompression};
pub use conceptual_blender::ConceptBlend;
pub use creative_stream::CreativeStream;
pub use glyph::Glyph;
pub use principle::Principle;
pub use resonance::ResonanceScore;
pub use symbol::{Symbol, SymbolActivation, SymbolId, SymbolKind};
pub use symbol_graph::SymbolGraph;
pub use symbolic_frame::SymbolicFrame;
pub use symbolic_memory::SymbolicMemoryTag;
pub use symbolic_trace::SymbolicTrace;

/// A blended thought produced by combining memory context with the current problem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlendedCandidate {
    pub id: String,
    pub action_type: ActionType,
    pub resource_cost: f64,
    pub reversible: bool,
    pub reasoning: String,
}

impl BlendedCandidate {
    /// Produce a generic blend placeholder (no LLM required).
    pub fn blend(current_problem: &str, action_type: ActionType) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        current_problem.hash(&mut h);
        let digest = format!("{:x}", h.finish());
        BlendedCandidate {
            id: format!("blend-{}", &digest[..8]),
            action_type,
            resource_cost: 0.24,
            reversible: true,
            reasoning: format!(
                "Conceptual blend: apply prior principle to '{}' with a reversible, \
                 kind, evidence-aware next step.",
                &current_problem[..current_problem.len().min(80)]
            ),
        }
    }
}

/// Hash a symbolic state vector to a short digest for the runtime state.
pub fn hash_symbolic_state(entries: &[&str]) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    for e in entries {
        e.hash(&mut h);
    }
    h.finish()
}
