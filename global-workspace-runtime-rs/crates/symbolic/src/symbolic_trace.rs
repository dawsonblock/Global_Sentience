//! Trace of symbolic reasoning steps across a single cycle.

use crate::symbolic_frame::SymbolicFrame;
use serde::{Deserialize, Serialize};

/// Complete symbolic trace for a single runtime cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolicTrace {
    pub cycle_id: u64,
    pub frames: Vec<SymbolicFrame>,
    /// Flag to indicate if this trace has been validated against runtime state.
    pub validated: bool,
}

impl SymbolicTrace {
    pub fn new(cycle_id: u64) -> Self {
        Self {
            cycle_id,
            frames: Vec::new(),
            validated: false,
        }
    }

    pub fn add_frame(&mut self, frame: SymbolicFrame) {
        self.frames.push(frame);
    }
}
