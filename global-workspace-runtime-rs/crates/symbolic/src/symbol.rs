//! Core symbol types and representations.

use serde::{Deserialize, Serialize};

/// Unique identifier for a symbol in the symbolic graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SymbolId(pub String);

impl std::fmt::Display for SymbolId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Classification of a symbol's semantic role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymbolKind {
    Concept,
    Action,
    Agent,
    Constraint,
    Goal,
    Value,
    Abstraction,
    Principle,
}

/// Represents a single symbol in the symbolic workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub id: SymbolId,
    pub label: String,
    pub kind: SymbolKind,
    pub confidence: f64, // 0.0–1.0, reflects epistemic credibility
    pub activated_count: u64,
}

impl Symbol {
    pub fn new(id: SymbolId, label: String, kind: SymbolKind) -> Self {
        Self {
            id,
            label,
            kind,
            confidence: 0.5,
            activated_count: 0,
        }
    }
}

/// Event representing a symbol's activation in the workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolActivation {
    pub symbol_id: SymbolId,
    pub activation_strength: f64, // 0.0–1.0
    pub source: String,           // e.g. "memory_hit", "inference"
}
