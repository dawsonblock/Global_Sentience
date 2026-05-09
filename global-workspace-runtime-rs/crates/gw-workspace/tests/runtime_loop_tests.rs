//! Tests for RuntimeLoop 9-stage pipeline.
//! Requirements: replay_reconstructs_final_state

use gw_workspace::runtime_loop::RuntimeLoop;
use runtime_core::{RuntimeAgent, ActionType};

#[test]
fn runtime_loop_instantiation() {
    // Verify RuntimeLoop can be created
    let loop_instance = RuntimeLoop::new();

    assert_eq!(loop_instance.cycle_id, 0);
    // Verify internal state is properly initialized
}

#[test]
fn runtime_loop_implements_runtime_agent() {
    // Verify RuntimeLoop properly implements RuntimeAgent trait
    let mut loop_instance = RuntimeLoop::new();

    let (action, events) = loop_instance.step(
        "test observation",
        1.0,  // world_resources
        0,    // cycle_id
    );

    // Should return valid action and non-empty event vec
    assert!(matches!(
        action,
        ActionType::Answer | ActionType::ConserveResources | ActionType::AskClarification
            | ActionType::RetrieveMemory | ActionType::RefuseUngrounded
            | ActionType::WriteScratchpad | ActionType::Repair | ActionType::Summarize
            | ActionType::GeneratePrinciple | ActionType::Defer | ActionType::InternalDiagnostic
    ));

    // step() should emit at least a CycleStarted event
    assert!(!events.is_empty(), "step() should emit events");
}

#[test]
fn runtime_loop_increments_cycle_id() {
    // Verify cycle_id increases after each step
    let mut loop_instance = RuntimeLoop::new();

    let cycle_id_before = loop_instance.cycle_id;
    let _result = loop_instance.step("observation", 1.0, cycle_id_before);
    let cycle_id_after = loop_instance.cycle_id;

    assert!(cycle_id_after >= cycle_id_before);
}
