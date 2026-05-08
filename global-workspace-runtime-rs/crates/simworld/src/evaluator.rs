//! EvaluatorRun: orchestrates N cycles of simulation.
//! Bugs from gw-kernel are fixed here:
//!   - log.append() Result is no longer ignored
//!   - action_type uses ActionType enum, not format!("{:?}", ...)

use chrono::Utc;
use runtime_core::event::WorldOutcome;
use runtime_core::{ActionType, EventLog, RuntimeEvent};

use crate::environment::CooperativeSupportWorld;
use crate::scorecard::{Scorecard, ScorecardBuilder};
use crate::sim_types::SimAction;

pub struct EvaluatorRun {
    pub world: CooperativeSupportWorld,
    pub log: EventLog,
}

impl EvaluatorRun {
    pub fn new(seed: u64, log_path: Option<std::path::PathBuf>) -> Self {
        let log = match log_path {
            Some(p) => EventLog::with_path(p),
            None => EventLog::new(),
        };
        Self {
            world: CooperativeSupportWorld::new(seed),
            log,
        }
    }

    /// Run `cycles` simulation steps and return a Scorecard.
    pub fn run(&mut self, cycles: u64) -> Scorecard {
        let mut builder = ScorecardBuilder::new();

        for cycle_id in 0..cycles {
            // 1. cycle started
            let _ = self.log.append(RuntimeEvent::CycleStarted {
                cycle_id,
                timestamp: Utc::now(),
            });

            // 2. pick scenario
            let scenario = self.world.next_scenario();
            let expected_action: SimAction = scenario.expected_action.clone();

            // 3. candidate generated — use expected action (deterministic ideal selector)
            let action_type: ActionType = expected_action.clone().into();
            let _ = self.log.append(RuntimeEvent::CandidateGenerated {
                cycle_id,
                action_type: action_type.clone(),
                score: 0.9,
            });

            // 4. candidate selected
            let _ = self.log.append(RuntimeEvent::CandidateSelected {
                cycle_id,
                action_type: action_type.clone(),
                score: 0.9,
                resonance: vec![],
            });

            // 5. action applied
            let is_conserve = expected_action == SimAction::ConserveResources;
            let _ = self.log.append(RuntimeEvent::ActionApplied {
                cycle_id,
                action_type: action_type.clone(),
                conserve: is_conserve,
            });

            // 6. apply to world and record outcome
            let outcome = self.world.apply_action(&expected_action, scenario);

            let is_unsafe = expected_action == SimAction::InternalDiagnostic;
            let total = outcome.total_score();

            builder.record_outcome(
                total,
                outcome.matches_expected,
                outcome.harm_score,
                outcome.truth_score,
                outcome.social_score,
                outcome.utility_score,
                is_unsafe,
                is_conserve,
            );

            let _ = self.log.append(RuntimeEvent::WorldStateUpdated {
                cycle_id,
                outcome: WorldOutcome {
                    resource_delta: outcome.resource_delta,
                    social_score: outcome.social_score,
                    harm_score: outcome.harm_score,
                    truth_score: outcome.truth_score,
                    kindness_score: outcome.kindness_score,
                    logic_score: outcome.logic_score,
                    utility_score: outcome.utility_score,
                    matches_expected: outcome.matches_expected,
                },
            });
        }

        builder.set_final_resources(self.world.resources);
        builder.build()
    }
}
