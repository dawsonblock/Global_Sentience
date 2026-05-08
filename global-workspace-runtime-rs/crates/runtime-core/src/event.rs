use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::action::ActionType;
use crate::types::ResonanceEntry;

/// Structured outcome from the simworld after an action is applied.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldOutcome {
    pub resource_delta:     f64,
    pub social_score:       f64,
    pub harm_score:         f64,
    pub truth_score:        f64,
    pub kindness_score:     f64,
    pub logic_score:        f64,
    pub utility_score:      f64,
    pub matches_expected:   bool,
}

/// All events that can be appended to the event log.
/// Variants are tagged in JSONL as `{"type": "...", "payload": {...}}`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum RuntimeEvent {
    CycleStarted {
        cycle_id:  u64,
        timestamp: DateTime<Utc>,
    },
    ObservationReceived {
        cycle_id:        u64,
        observation_len: usize,
        world_resources: f64,
    },
    CandidateGenerated {
        cycle_id:    u64,
        action_type: ActionType,
        score:       f64,
    },
    CandidateSelected {
        cycle_id:    u64,
        action_type: ActionType,
        score:       f64,
        resonance:   Vec<ResonanceEntry>,
    },
    ActionApplied {
        cycle_id:    u64,
        action_type: ActionType,
        conserve:    bool,
    },
    WorldStateUpdated {
        cycle_id: u64,
        outcome:  WorldOutcome,
    },
    RuntimeModeChanged {
        from: String,
        to:   String,
    },
    MemoryWritten {
        cycle_id: u64,
        key:      String,
    },
    ScratchpadUpdated {
        cycle_id: u64,
        entry_count: usize,
    },
    ErrorOccurred {
        cycle_id: u64,
        message:  String,
    },
    // --- symbolic events (stubbed) ---
    SymbolicBlendEmitted {
        cycle_id: u64,
        glyph:    String,
    },
    ConceptCompressed {
        cycle_id:   u64,
        source_len: usize,
        hash:       u64,
    },
}
