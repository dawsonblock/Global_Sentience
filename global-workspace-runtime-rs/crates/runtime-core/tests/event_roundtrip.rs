//! Core tests for runtime-core types and event handling.

use runtime_core::{ActionType, RuntimeEvent, RuntimeState};

#[test]
fn runtime_state_default_initialization() {
    let state = RuntimeState::default();
    assert_eq!(state.cycle_id, 0);
    assert_eq!(state.symbolic_activations, 0);
}

#[test]
fn action_type_variants_exist() {
    // Verify all 11 ActionType variants compile
    let _a = ActionType::Answer;
    let _b = ActionType::ConserveResources;
    let _c = ActionType::AskClarification;
    let _d = ActionType::RetrieveMemory;
    let _e = ActionType::RefuseUngrounded;
    let _f = ActionType::WriteScratchpad;
    let _g = ActionType::Repair;
    let _h = ActionType::Summarize;
    let _i = ActionType::GeneratePrinciple;
    let _j = ActionType::Defer;
    let _k = ActionType::InternalDiagnostic;
}

#[test]
fn runtime_event_cycle_started_creation() {
    let event = RuntimeEvent::CycleStarted {
        cycle_id: 42,
        timestamp: chrono::Utc::now(),
    };

    // Verify cycle_id method works
    assert_eq!(event.cycle_id(), Some(42));
}

#[test]
fn action_type_cloneable() {
    let action = ActionType::Answer;
    let cloned = action.clone();
    assert_eq!(action, cloned);
}
