use serde::{Deserialize, Serialize};
use crate::action::ActionType;
use crate::mode::RuntimeMode;

/// Full runtime state — rebuilt by replaying events through the reducer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeState {
    // Cycle tracking
    pub cycle_id:            u64,
    pub total_cycles:        u64,

    // Resource accounting
    pub resources:           f64,
    pub conserve_actions:    u64,

    // Safety counters (MUST stay at zero in production)
    pub unsafe_action_count:    u64,
    pub false_confidence_count: u64,
    pub repeated_mistakes:      u64,

    // Action history
    pub last_action_type:       Option<ActionType>,
    pub selected_action_type:   Option<ActionType>,
    pub last_candidate_action_type: Option<ActionType>,

    // Dialogue/world coherence
    pub contradiction_count:    u64,
    pub world_model_mismatch:   u64,
    pub self_report_invalid:    u64,

    // Mode
    pub current_mode:           RuntimeMode,

    // Memory health (0.0–1.0)
    pub memory_health:          f64,

    // Symbolic state
    pub symbolic_state_hash:    Option<u64>,

    // Score tracking (rolling)
    pub total_score_sum:        f64,
    pub total_score_count:      u64,
    pub last_total_score:       f64,

    // Per-component score sums (for audit)
    pub truth_score_sum:         f64,
    pub kindness_score_sum:      f64,
    pub social_score_sum:        f64,
    pub logic_score_sum:         f64,
    pub utility_score_sum:       f64,
    pub harm_score_sum:          f64,

    // Outcome tracking
    pub outcome_count:           u64,
    pub matched_expected_count:  u64,
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self {
            cycle_id:                 0,
            total_cycles:             0,
            resources:                1.0,
            conserve_actions:         0,
            unsafe_action_count:      0,
            false_confidence_count:   0,
            repeated_mistakes:        0,
            last_action_type:         None,
            selected_action_type:     None,
            last_candidate_action_type: None,
            contradiction_count:      0,
            world_model_mismatch:     0,
            self_report_invalid:      0,
            current_mode:             RuntimeMode::Normal,
            memory_health:            1.0,
            symbolic_state_hash:      None,
            total_score_sum:          0.0,
            total_score_count:        0,
            last_total_score:         0.0,
            truth_score_sum:          0.0,
            kindness_score_sum:       0.0,
            social_score_sum:         0.0,
            logic_score_sum:          0.0,
            utility_score_sum:        0.0,
            harm_score_sum:           0.0,
            outcome_count:            0,
            matched_expected_count:   0,
        }
    }
}
