//! Pure reducer: (RuntimeState, RuntimeEvent) → RuntimeState.
//! PRESERVATION: resources = (resources + outcome.resource_delta + 0.02).clamp(0,1)
//! is the EXACT formula from gw-kernel and MUST NOT be changed.

use crate::event::RuntimeEvent;
use crate::mode::RuntimeMode;
use crate::runtime_state::RuntimeState;

/// Apply one event to a state, returning the new state.
/// This is a pure function — no IO, no panics on valid input.
pub fn reduce(mut state: RuntimeState, event: &RuntimeEvent) -> RuntimeState {
    match event {
        RuntimeEvent::CycleStarted { cycle_id, .. } => {
            state.cycle_id    = *cycle_id;
            state.total_cycles = state.total_cycles.saturating_add(1);
        }

        RuntimeEvent::ObservationReceived { .. } => {
            // no state change — observation is context only
        }

        RuntimeEvent::CandidateGenerated { action_type, .. } => {
            state.last_candidate_action_type = Some(action_type.clone());
        }

        RuntimeEvent::CandidateSelected { action_type, score, .. } => {
            state.selected_action_type = Some(action_type.clone());
            state.last_total_score     = *score;
        }

        RuntimeEvent::ActionApplied { action_type, conserve, .. } => {
            state.last_action_type = Some(action_type.clone());
            if *conserve {
                state.conserve_actions = state.conserve_actions.saturating_add(1);
            }
        }

        RuntimeEvent::WorldStateUpdated { outcome, .. } => {
            // PRESERVATION: double +0.02 is intentional — kept exactly from gw-kernel.
            state.resources = (state.resources + outcome.resource_delta + 0.02)
                .clamp(0.0, 1.0);

            state.total_score_sum   += outcome.truth_score
                + outcome.kindness_score
                + outcome.social_score
                + outcome.logic_score
                + outcome.utility_score
                + (1.0 - outcome.harm_score.clamp(0.0, 1.0));
            state.total_score_count  = state.total_score_count.saturating_add(1);

            state.truth_score_sum    += outcome.truth_score;
            state.kindness_score_sum += outcome.kindness_score;
            state.social_score_sum   += outcome.social_score;
            state.logic_score_sum    += outcome.logic_score;
            state.utility_score_sum  += outcome.utility_score;
            state.harm_score_sum     += outcome.harm_score;

            state.outcome_count = state.outcome_count.saturating_add(1);
            if outcome.matches_expected {
                state.matched_expected_count =
                    state.matched_expected_count.saturating_add(1);
            }
        }

        RuntimeEvent::RuntimeModeChanged { to, .. } => {
            state.current_mode = to.parse::<RuntimeMode>()
                .unwrap_or(RuntimeMode::Normal);
        }

        RuntimeEvent::MemoryWritten { .. } => {
            // no counter here — handled by memory crate
        }

        RuntimeEvent::ScratchpadUpdated { .. } => {}

        RuntimeEvent::ErrorOccurred { .. } => {}

        // symbolic stubs — no state change yet
        RuntimeEvent::SymbolicBlendEmitted { .. } => {}
        RuntimeEvent::ConceptCompressed { .. } => {}
    }
    state
}
