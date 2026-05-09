//! Symbolic graph — relationships between symbols.

use crate::symbol::SymbolId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A directed edge in the symbolic graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolEdge {
    pub from_id: SymbolId,
    pub to_id: SymbolId,
    pub relation: String, // e.g. "implies", "contradicts", "supports"
    pub weight: f64,      // 0.0–1.0, strength of the relationship
}

/// The graph of symbols and their relationships.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SymbolGraph {
    pub nodes: HashMap<SymbolId, crate::symbol::Symbol>,
    pub edges: Vec<SymbolEdge>,
}

impl SymbolGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_symbol(&mut self, symbol: crate::symbol::Symbol) {
        self.nodes.insert(symbol.id.clone(), symbol);
    }

    pub fn add_edge(&mut self, edge: SymbolEdge) {
        self.edges.push(edge);
    }

    pub fn neighbors(&self, symbol_id: &SymbolId) -> Vec<&SymbolEdge> {
        self.edges
            .iter()
            .filter(|e| &e.from_id == symbol_id)
            .collect()
    }
}
