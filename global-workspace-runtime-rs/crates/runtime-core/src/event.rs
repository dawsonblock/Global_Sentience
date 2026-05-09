use crate::action::ActionType;
use crate::types::ResonanceEntry;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Structured outcome from the simworld after an action is applied.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldOutcome {
    pub resource_delta: f64,
    pub social_score: f64,
    pub harm_score: f64,
    pub truth_score: f64,
    pub kindness_score: f64,
    pub logic_score: f64,
    pub utility_score: f64,
    pub matches_expected: bool,
}

/// All events that can be appended to the event log.
/// Variants are tagged in JSONL as `{"type": "...", "payload": {...}}`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum RuntimeEvent {
    CycleStarted {
        cycle_id: u64,
        timestamp: DateTime<Utc>,
    },
    ObservationReceived {
        cycle_id: u64,
        observation_len: usize,
        world_resources: f64,
    },
    CandidateGenerated {
        cycle_id: u64,
        action_type: ActionType,
        score: f64,
    },
    CandidateSelected {
        cycle_id: u64,
        action_type: ActionType,
        score: f64,
        resonance: Vec<ResonanceEntry>,
    },
    ActionApplied {
        cycle_id: u64,
        action_type: ActionType,
        conserve: bool,
    },
    WorldStateUpdated {
        cycle_id: u64,
        outcome: WorldOutcome,
    },
    RuntimeModeChanged {
        from: String,
        to: String,
    },
    MemoryWritten {
        cycle_id: u64,
        key: String,
    },
    ScratchpadUpdated {
        cycle_id: u64,
        entry_count: usize,
    },
    ErrorOccurred {
        cycle_id: u64,
        message: String,
    },
    // --- memory events ---
    MemoryQueried {
        cycle_id: u64,
        query: String,
    },
    MemoryHitReturned {
        cycle_id: u64,
        hit_count: usize,
        top_text: Option<String>,
    },
    // --- candidate lifecycle ---
    CandidateRejected {
        cycle_id: u64,
        candidate_id: String,
        reason: String,
    },
    // --- archive events ---
    ArchiveCommitted {
        cycle_id: u64,
        frame_id: String,
        archive_path: String,
    },
    // --- contradiction tracking ---
    ContradictionDetected {
        cycle_id: u64,
        claim_a: String,
        claim_b: String,
        subject: String,
    },
    ContradictionResolved {
        cycle_id: u64,
        superseded_claim: String,
        active_claim: String,
        resolution: String,
    },
    // --- symbolic events ---
    SymbolActivated {
        cycle_id: u64,
        symbol_id: String,
        kind: String,
    },
    SymbolLinked {
        cycle_id: u64,
        from_id: String,
        to_id: String,
        relation: String,
    },
    SymbolicTraceRecorded {
        cycle_id: u64,
        frame_id: String,
        source: String,
    },
    ConceptBlendGenerated {
        cycle_id: u64,
        blend_id: String,
        input_count: usize,
    },
    PrincipleExtracted {
        cycle_id: u64,
        principle_id: String,
        confidence: f64,
    },
    SymbolicCompressionApplied {
        cycle_id: u64,
        original_len: usize,
        compressed_len: usize,
    },
    ResonanceScoreComputed {
        cycle_id: u64,
        symbol_id: String,
        score: f64,
    },
}

impl RuntimeEvent {
    /// Return the cycle_id carried by this event, if any.
    pub fn cycle_id(&self) -> Option<u64> {
        match self {
            RuntimeEvent::CycleStarted { cycle_id, .. }
            | RuntimeEvent::ObservationReceived { cycle_id, .. }
            | RuntimeEvent::CandidateGenerated { cycle_id, .. }
            | RuntimeEvent::CandidateSelected { cycle_id, .. }
            | RuntimeEvent::ActionApplied { cycle_id, .. }
            | RuntimeEvent::WorldStateUpdated { cycle_id, .. }
            | RuntimeEvent::MemoryWritten { cycle_id, .. }
            | RuntimeEvent::ScratchpadUpdated { cycle_id, .. }
            | RuntimeEvent::ErrorOccurred { cycle_id, .. }
            | RuntimeEvent::MemoryQueried { cycle_id, .. }
            | RuntimeEvent::MemoryHitReturned { cycle_id, .. }
            | RuntimeEvent::CandidateRejected { cycle_id, .. }
            | RuntimeEvent::ArchiveCommitted { cycle_id, .. }
            | RuntimeEvent::ContradictionDetected { cycle_id, .. }
            | RuntimeEvent::ContradictionResolved { cycle_id, .. }
            | RuntimeEvent::SymbolActivated { cycle_id, .. }
            | RuntimeEvent::SymbolLinked { cycle_id, .. }
            | RuntimeEvent::SymbolicTraceRecorded { cycle_id, .. }
            | RuntimeEvent::ConceptBlendGenerated { cycle_id, .. }
            | RuntimeEvent::PrincipleExtracted { cycle_id, .. }
            | RuntimeEvent::SymbolicCompressionApplied { cycle_id, .. }
            | RuntimeEvent::ResonanceScoreComputed { cycle_id, .. } => Some(*cycle_id),
            RuntimeEvent::RuntimeModeChanged { .. } => None,
        }
    }
}
