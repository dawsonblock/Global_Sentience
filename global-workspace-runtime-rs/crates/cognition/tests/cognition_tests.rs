//! Tests for candidate generation and planning.
//! Requirements: candidate_rejected_cannot_be_selected

use cognition::candidate::{ThoughtCandidate, CandidatePacket};
use runtime_core::ActionType;

#[test]
fn candidate_packet_best_selection() {
    // Create candidates with different scores
    let mut packet = CandidatePacket::new(1);

    let mut candidate1 = ThoughtCandidate::new(ActionType::Answer, 0.15);
    candidate1.score = 0.5;
    candidate1.passes_critic = true;
    packet.push(candidate1);

    let mut candidate2 = ThoughtCandidate::new(ActionType::ConserveResources, 0.08);
    candidate2.score = 0.8;
    candidate2.passes_critic = true;
    packet.push(candidate2);

    let mut candidate3 = ThoughtCandidate::new(ActionType::AskClarification, 0.12);
    candidate3.score = 0.3;
    candidate3.passes_critic = true;
    packet.push(candidate3);

    // best() should select the highest score (0.8)
    if let Some(best) = packet.best() {
        assert_eq!(best.action_type, ActionType::ConserveResources);
        assert!((best.score - 0.8).abs() < 0.001);
    } else {
        panic!("best() should return Some for non-empty passing packet");
    }
}

#[test]
fn candidate_packet_empty_returns_none() {
    let packet = CandidatePacket::new(1);

    assert!(packet.best().is_none(), "best() should return None for empty packet");
}

#[test]
fn candidate_packet_single_candidate() {
    let mut packet = CandidatePacket::new(1);

    let mut candidate = ThoughtCandidate::new(ActionType::Answer, 0.15);
    candidate.score = 0.9;
    candidate.passes_critic = true;
    packet.push(candidate);

    if let Some(best) = packet.best() {
        assert_eq!(best.action_type, ActionType::Answer);
        assert!((best.score - 0.9).abs() < 0.001);
    } else {
        panic!("best() should return Some for single passing candidate");
    }
}

#[test]
fn candidate_packet_filters_non_passing_critics() {
    // Verify best() only returns candidates that pass_critic
    let mut packet = CandidatePacket::new(1);

    let mut candidate1 = ThoughtCandidate::new(ActionType::Answer, 0.15);
    candidate1.score = 0.9; // High score but fails critic
    candidate1.passes_critic = false;
    packet.push(candidate1);

    let mut candidate2 = ThoughtCandidate::new(ActionType::ConserveResources, 0.08);
    candidate2.score = 0.5; // Lower score but passes critic
    candidate2.passes_critic = true;
    packet.push(candidate2);

    if let Some(best) = packet.best() {
        assert_eq!(best.action_type, ActionType::ConserveResources);
        assert!((best.score - 0.5).abs() < 0.001);
    } else {
        panic!("best() should return the passing candidate even with lower score");
    }
}

#[test]
fn all_action_types_can_be_candidates() {
    // Verify all 11 ActionType variants can be used as candidates
    let action_types = vec![
        ActionType::Answer,
        ActionType::ConserveResources,
        ActionType::AskClarification,
        ActionType::RetrieveMemory,
        ActionType::RefuseUngrounded,
        ActionType::WriteScratchpad,
        ActionType::Repair,
        ActionType::Summarize,
        ActionType::GeneratePrinciple,
        ActionType::Defer,
        ActionType::InternalDiagnostic,
    ];

    for action in action_types {
        let candidate = ThoughtCandidate::new(action.clone(), 0.5);
        let mut packet = CandidatePacket::new(1);
        packet.push(candidate);

        let best = packet
            .best()
            .expect("should have candidate for each action type");
        assert_eq!(best.action_type, action);
    }
}

#[test]
fn candidate_packet_cycle_id() {
    // Verify CandidatePacket tracks cycle_id correctly
    let packet1 = CandidatePacket::new(5);
    let packet2 = CandidatePacket::new(10);

    assert_eq!(packet1.cycle_id, 5);
    assert_eq!(packet2.cycle_id, 10);
    assert_ne!(packet1.cycle_id, packet2.cycle_id);
}

#[test]
fn thought_candidate_reversible_flag() {
    // Verify ThoughtCandidate tracks reversibility
    let candidate = ThoughtCandidate::new(ActionType::Answer, 0.15);
    assert!(candidate.reversible, "NewThoughtCandidate should start reversible");
}

#[test]
fn thought_candidate_reasoning_optional() {
    // Verify ThoughtCandidate reasoning field is optional
    let mut candidate = ThoughtCandidate::new(ActionType::Answer, 0.15);
    assert!(candidate.reasoning.is_none(), "Initial reasoning should be None");

    candidate.reasoning = Some("reasoning here".to_string());
    assert!(candidate.reasoning.is_some());
    assert_eq!(candidate.reasoning.unwrap(), "reasoning here");
}
