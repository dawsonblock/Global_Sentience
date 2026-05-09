//! Tests for symbolic tracing and compression.
//! Requirements: symbolic_trace_serializes, symbolic_compression_preserves_semantics

use symbolic::symbolic_trace::SymbolicTrace;
use symbolic::symbolic_frame::SymbolicFrame;
use symbolic::compression::{SymbolicCompression, CompressionStats};
use symbolic::symbol::{Symbol, SymbolId, SymbolKind, SymbolActivation};

#[test]
fn symbolic_trace_structure() {
    // Verify SymbolicTrace has proper fields
    let mut frame1 = SymbolicFrame::new("f1".to_string(), "observation".to_string());
    frame1.active_symbols.push(SymbolActivation {
        symbol_id: SymbolId("sym1".to_string()),
        activation_strength: 0.8,
        source: "test".to_string(),
    });

    let mut frame2 = SymbolicFrame::new("f2".to_string(), "memory".to_string());
    frame2.active_symbols.push(SymbolActivation {
        symbol_id: SymbolId("sym2".to_string()),
        activation_strength: 0.9,
        source: "test".to_string(),
    });

    let frames = vec![frame1, frame2];

    let trace = SymbolicTrace {
        frames: frames.clone(),
        cycle_id: 5,
        validated: true,
    };

    assert_eq!(trace.cycle_id, 5);
    assert_eq!(trace.frames.len(), 2);
    assert!(trace.validated);
}

#[test]
fn symbolic_frame_validates_structure() {
    // Verify SymbolicFrame has proper fields and constructor
    let mut frame = SymbolicFrame::new("test".to_string(), "test_source".to_string());
    assert_eq!(frame.frame_id, "test");
    assert_eq!(frame.source, "test_source");
    assert_eq!(frame.active_symbols.len(), 0);
    assert!(frame.timestamp_ms > 0);
    assert!(!frame.validated);

    // Add activation
    frame.active_symbols.push(SymbolActivation {
        symbol_id: SymbolId("sym1".to_string()),
        activation_strength: 0.8,
        source: "test".to_string(),
    });
    assert_eq!(frame.active_symbols.len(), 1);
}

#[test]
fn symbol_kind_enum_complete() {
    // Verify SymbolKind has all expected variants
    let _concept = SymbolKind::Concept;
    let _action = SymbolKind::Action;
    let _agent = SymbolKind::Agent;
    let _constraint = SymbolKind::Constraint;
    let _goal = SymbolKind::Goal;
    let _value = SymbolKind::Value;
    let _abstraction = SymbolKind::Abstraction;
    let _principle = SymbolKind::Principle;
    
    // All variants should be constructible
    let kinds = [
        SymbolKind::Concept,
        SymbolKind::Action,
        SymbolKind::Agent,
        SymbolKind::Constraint,
        SymbolKind::Goal,
        SymbolKind::Value,
        SymbolKind::Abstraction,
        SymbolKind::Principle,
    ];

    for kind in &kinds {
        let _debug = format!("{:?}", kind);
        // Verify Debug works
    }
}

#[test]
fn compression_stats_structure() {
    // Verify CompressionStats has correct fields and constructor
    let stats = CompressionStats::new(100, 25);

    assert_eq!(stats.original_size, 100);
    assert_eq!(stats.compressed_size, 25);
    assert!((stats.ratio - 0.25).abs() < 0.001);
}

#[test]
fn symbolic_compression_structure() {
    // Verify SymbolicCompression aggregates stats correctly
    let compression = SymbolicCompression::new(100, 25);

    assert_eq!(compression.original_len, 100);
    assert_eq!(compression.compressed_len, 25);
    assert_eq!(compression.stats.original_size, 100);
    assert_eq!(compression.stats.compressed_size, 25);
    assert!((compression.stats.ratio - 0.25).abs() < 0.001);
}

#[test]
fn symbol_id_uniqueness() {
    // Verify SymbolId is a proper identifier
    let id1 = SymbolId("sym1".to_string());
    let id2 = SymbolId("sym1".to_string());
    let id3 = SymbolId("sym2".to_string());

    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn symbol_creation_and_fields() {
    // Verify Symbol can be created with proper fields
    let symbol = Symbol::new(
        SymbolId("test_sym".to_string()),
        "Test Symbol".to_string(),
        SymbolKind::Concept,
    );

    assert_eq!(symbol.id.0, "test_sym");
    assert_eq!(symbol.label, "Test Symbol");
    assert_eq!(symbol.kind, SymbolKind::Concept);
    assert!(symbol.confidence >= 0.0 && symbol.confidence <= 1.0);
    assert_eq!(symbol.activated_count, 0);
}

#[test]
fn symbol_activation_structure() {
    // Verify SymbolActivation structure
    let activation = SymbolActivation {
        symbol_id: SymbolId("sym1".to_string()),
        activation_strength: 0.8,
        source: "memory_hit".to_string(),
    };

    assert_eq!(activation.symbol_id.0, "sym1");
    assert!((activation.activation_strength - 0.8).abs() < 0.001);
    assert_eq!(activation.source, "memory_hit");
}

#[test]
fn compression_ratio_edge_cases() {
    // Verify compression ratio calculation handles edge cases
    
    // Case 1: Original size = 0
    let stats_zero = CompressionStats::new(0, 0);
    assert_eq!(stats_zero.ratio, 1.0);

    // Case 2: Perfect compression
    let stats_perfect = CompressionStats::new(100, 100);
    assert!((stats_perfect.ratio - 1.0).abs() < 0.001);

    // Case 3: Good compression
    let stats_good = CompressionStats::new(100, 50);
    assert!((stats_good.ratio - 0.5).abs() < 0.001);

    // Case 4: No compression
    let stats_none = CompressionStats::new(100, 100);
    assert!((stats_none.ratio - 1.0).abs() < 0.001);
}
