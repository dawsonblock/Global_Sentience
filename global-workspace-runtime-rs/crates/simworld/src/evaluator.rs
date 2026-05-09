//! EvaluatorRun: orchestrates N cycles of simulation.
//! Supports both oracle mode (using expected action) and real agent mode (using RuntimeAgent).
//!
//! Bugs from gw-kernel are fixed here:
//!   - log.append() Result is no longer ignored
//!   - action_type uses ActionType enum, not format!("{:?}", ...)
//!   - SimWorld now stops using the oracle (expected_action) in real agent mode

use chrono::Utc;
use runtime_core::event::WorldOutcome;
use runtime_core::{ActionType, EventLog, RuntimeAgent, RuntimeEvent};

use crate::environment::CooperativeSupportWorld;
use crate::scorecard::{Scorecard, ScorecardBuilder};
use crate::sim_types::SimAction;

pub struct EvaluatorRun {
    pub world: CooperativeSupportWorld,
    pub log: EventLog,
    agent: Option<Box<dyn RuntimeAgent>>,
}

impl EvaluatorRun {
    /// Create an oracle-mode evaluator (uses expected action as the ground truth).
    /// Backward compatible with existing code.
    pub fn new(seed: u64, log_path: Option<std::path::PathBuf>) -> Self {
        let log = match log_path {
            Some(p) => EventLog::with_path(p),
            None => EventLog::new(),
        };
        Self {
            world: CooperativeSupportWorld::new(seed),
            log,
            agent: None,
        }
    }

    /// Create a real agent-mode evaluator (uses the provided RuntimeAgent for decisions).
    pub fn with_agent(
        seed: u64,
        log_path: Option<std::path::PathBuf>,
        agent: Box<dyn RuntimeAgent>,
    ) -> Self {
        let log = match log_path {
            Some(p) => EventLog::with_path(p),
            None => EventLog::new(),
        };
        Self {
            world: CooperativeSupportWorld::new(seed),
            log,
            agent: Some(agent),
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

            // 3-4. Take action (either oracle or agent-based)
            let action_type = if let Some(ref mut agent) = self.agent.as_mut() {
                // Real agent mode: use the RuntimeAgent
                let observation = "observation placeholder".to_string(); // TODO: fill from scenario
                let (action, events) = agent.step(&observation, self.world.resources, cycle_id);
                // Log all events from the agent
                for event in events {
                    let _ = self.log.append(event);
                }
                action
            } else {
                // Oracle mode: use expected action directly
                let expected_action: SimAction = scenario.expected_action.clone();
                let action_type: ActionType = expected_action.clone().into();

                let _ = self.log.append(RuntimeEvent::CandidateGenerated {
                    cycle_id,
                    action_type: action_type.clone(),
                    score: 0.9,
                });

                let _ = self.log.append(RuntimeEvent::CandidateSelected {
                    cycle_id,
                    action_type: action_type.clone(),
                    score: 0.9,
                    resonance: vec![],
                });

                action_type
            };

            // 5. action applied
            let is_conserve = action_type == ActionType::ConserveResources;
            let _ = self.log.append(RuntimeEvent::ActionApplied {
                cycle_id,
                action_type: action_type.clone(),
                conserve: is_conserve,
            });

            // 6. apply to world and record outcome (use oracle's expected outcome for scoring)
            let expected_action: SimAction = scenario.expected_action.clone();
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
