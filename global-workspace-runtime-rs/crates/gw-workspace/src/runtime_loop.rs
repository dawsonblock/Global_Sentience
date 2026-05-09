//! The 9-stage RuntimeLoop pipeline — the authoritative decision-making engine.
//!
//! Stages:
//! 1. Observation ingestion
//! 2. Memory retrieval
//! 3. Symbolic context activation
//! 4. Candidate generation (all ActionTypes with cost table)
//! 5. Critic evaluation via score_candidate
//! 6. Planner selection
//! 7. Action applied
//! 8. Internal state update
//! 9. Archive commit

use cognition::{CandidatePacket, Planner, ThoughtCandidate};
use memory::SemanticMemory;
use modulation::SomaticMap;
use runtime_core::{ActionType, InternalState, RuntimeAgent, RuntimeEvent};
use symbolic::SymbolGraph;

/// The 9-stage runtime loop that orchestrates decision-making.
pub struct RuntimeLoop {
    pub state: InternalState,
    pub somatic: SomaticMap,
    pub semantic_memory: SemanticMemory,
    pub symbol_graph: SymbolGraph,
    pub cycle_id: u64,
}

impl RuntimeLoop {
    /// Create a new RuntimeLoop with default/empty state.
    pub fn new() -> Self {
        Self {
            state: InternalState::default(),
            somatic: SomaticMap::default(),
            semantic_memory: SemanticMemory::new(),
            symbol_graph: SymbolGraph::new(),
            cycle_id: 0,
        }
    }

    /// Return all ActionType variants as a vector.
    fn action_types() -> Vec<ActionType> {
        vec![
            ActionType::Answer,
            ActionType::AskClarification,
            ActionType::RetrieveMemory,
            ActionType::WriteScratchpad,
            ActionType::Defer,
            ActionType::RefuseUngrounded,
            ActionType::Repair,
            ActionType::Summarize,
            ActionType::ConserveResources,
            ActionType::GeneratePrinciple,
            ActionType::InternalDiagnostic,
        ]
    }

    /// Execute one cycle of the 9-stage pipeline.
    fn step_inner(
        &mut self,
        observation: &str,
        world_resources: f64,
    ) -> (ActionType, Vec<RuntimeEvent>) {
        let mut events = Vec::new();
        let cid = self.cycle_id;

        // ========== Stage 1: Observation ==========
        events.push(RuntimeEvent::ObservationReceived {
            cycle_id: cid,
            observation_len: observation.len(),
            world_resources,
        });

        // ========== Stage 2: Memory Retrieval ==========
        events.push(RuntimeEvent::MemoryQueried {
            cycle_id: cid,
            query: observation.to_string(),
        });

        let memory_hits = self.semantic_memory.query(observation, 3);
        let hit_count = memory_hits.len();
        let top_text = memory_hits.first().map(|h| h.value.clone());
        events.push(RuntimeEvent::MemoryHitReturned {
            cycle_id: cid,
            hit_count,
            top_text,
        });

        // ========== Stage 3: Symbolic Context ==========
        for hit in &memory_hits {
            events.push(RuntimeEvent::SymbolActivated {
                cycle_id: cid,
                symbol_id: hit.key.clone(),
                kind: "memory_hit".to_string(),
            });
        }

        // ========== Stage 4: Candidate Generation ==========
        let mut packet = CandidatePacket::new(cid);

        // Generate candidates for all action types using cost table
        let cost_table = self.get_action_costs();
        for action_type in Self::action_types() {
            let cost = cost_table.get(&action_type).copied().unwrap_or(0.24);
            let candidate = ThoughtCandidate::new(action_type.clone(), cost);
            packet.push(candidate);
            events.push(RuntimeEvent::CandidateGenerated {
                cycle_id: cid,
                action_type: action_type.clone(),
                score: 0.0,
            });
        }

        // ========== Stage 5: Critic Evaluation ==========
        let score_ctx = cognition::CriticContext {
            state: self.state.clone(),
            world_resources,
            memory_consistency: 0.8,
            reversibility: 0.7,
            self_report_grounding: 0.9,
            resource_cost: 0.1,
            ungrounded_self_report: false,
        };

        for candidate in &mut packet.candidates {
            cognition::score_candidate(candidate, &score_ctx);
            if !candidate.passes_critic {
                events.push(RuntimeEvent::CandidateRejected {
                    cycle_id: cid,
                    candidate_id: format!("{:?}", candidate.action_type),
                    reason: "Critic rejection".to_string(),
                });
            }
        }

        // ========== Stage 6: Planner Selection ==========
        let allowed = Self::action_types();
        let selected_action = Planner::select(&self.state, &self.somatic, &packet, &allowed);
        events.push(RuntimeEvent::CandidateSelected {
            cycle_id: cid,
            action_type: selected_action.clone(),
            score: packet.best().map(|c| c.score).unwrap_or(0.0),
            resonance: Vec::new(),
        });

        // ========== Stage 7: Action Applied ==========
        let conserve = selected_action == ActionType::ConserveResources;
        events.push(RuntimeEvent::ActionApplied {
            cycle_id: cid,
            action_type: selected_action.clone(),
            conserve,
        });

        // ========== Stage 8: Internal State Update ==========
        self.state.control = (self.state.control + 0.1).clamp(0.0, 1.0);
        self.state.threat = (self.state.threat * 0.95).clamp(0.0, 1.0);
        self.state.uncertainty = (self.state.uncertainty * 0.9).clamp(0.0, 1.0);

        // ========== Stage 9: Archive Commit ==========
        let archive_frame_id = format!("frame-{}", cid);
        events.push(RuntimeEvent::ArchiveCommitted {
            cycle_id: cid,
            frame_id: archive_frame_id,
            archive_path: "artifacts/proof/runtime_trace.gwlog".to_string(),
        });

        (selected_action, events)
    }

    fn get_action_costs(&self) -> std::collections::HashMap<ActionType, f64> {
        use std::collections::HashMap;

        let mut costs = HashMap::new();
        costs.insert(ActionType::Answer, 0.15);
        costs.insert(ActionType::ConserveResources, 0.08);
        costs.insert(ActionType::AskClarification, 0.12);
        costs.insert(ActionType::RetrieveMemory, 0.18);
        costs.insert(ActionType::RefuseUngrounded, 0.10);
        costs.insert(ActionType::WriteScratchpad, 0.14);
        costs.insert(ActionType::Repair, 0.20);
        costs.insert(ActionType::Summarize, 0.16);
        costs.insert(ActionType::GeneratePrinciple, 0.24);
        costs.insert(ActionType::Defer, 0.09);
        costs.insert(ActionType::InternalDiagnostic, 0.05);
        costs
    }
}

impl Default for RuntimeLoop {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeAgent for RuntimeLoop {
    fn step(
        &mut self,
        observation: &str,
        world_resources: f64,
        cycle_id: u64,
    ) -> (ActionType, Vec<RuntimeEvent>) {
        self.cycle_id = cycle_id;
        let (action, mut events) = self.step_inner(observation, world_resources);

        // Update cycle tracking
        events.insert(
            0,
            RuntimeEvent::CycleStarted {
                cycle_id,
                timestamp: chrono::Utc::now(),
            },
        );

        (action, events)
    }
}
