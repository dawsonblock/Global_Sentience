//! Memory claims and contradiction detection.

use serde::{Deserialize, Serialize};

/// Source of evidence for a memory claim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub source_id: String,
    pub timestamp: u64,
    pub confidence: f64,
}

/// Status of a memory claim in the knowledge base.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClaimStatus {
    Active,
    Superseded,
    Contradiction,
    Uncertain,
}

/// A single factual or computational claim in memory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryClaim {
    pub claim_id: String,
    pub content: String,
    pub status: ClaimStatus,
    pub confidence: f64,
    pub evidence: Vec<Evidence>,
}

impl MemoryClaim {
    pub fn new(claim_id: String, content: String) -> Self {
        Self {
            claim_id,
            content,
            status: ClaimStatus::Active,
            confidence: 0.5,
            evidence: Vec::new(),
        }
    }
}

/// Link between an evidence item and the claim it supports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimEvidenceLink {
    pub claim_id: String,
    pub evidence_id: String,
    pub support_strength: f64,
}

/// Status of the entire memory corpus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStatus {
    pub total_claims: u64,
    pub contradictions_found: u64,
    pub last_update_cycle: u64,
}

/// A detected contradiction between two memory claims.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contradiction {
    pub claim_a_id: String,
    pub claim_b_id: String,
    pub conflict_description: String,
    pub resolution_strategy: Option<String>,
}

/// Packaged result of a memory retrieval operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalPacket {
    pub query: String,
    pub hits: Vec<MemoryClaim>,
    pub contradictions: Vec<Contradiction>,
    pub confidence: f64,
}

impl RetrievalPacket {
    pub fn new(query: String) -> Self {
        Self {
            query,
            hits: Vec::new(),
            contradictions: Vec::new(),
            confidence: 1.0,
        }
    }
}
