pub mod action;
pub mod errors;
pub mod event;
pub mod event_log;
pub mod mode;
pub mod reducer;
pub mod replay;
pub mod runtime_state;
pub mod trace;
pub mod types;

pub use action::ActionType;
pub use event::RuntimeEvent;
pub use event_log::EventLog;
pub use mode::RuntimeMode;
pub use reducer::reduce;
pub use replay::{replay, replay_jsonl, replay_log};
pub use runtime_state::RuntimeState;
pub use types::{InternalState, Observation, ResonanceEntry, ResonanceTag};

/// Trait implemented by any agent that can be driven by the RuntimeLoop.
/// Kept in runtime-core so simworld (and tests) can use it without pulling
/// in the heavier gw-workspace crate.
pub trait RuntimeAgent {
    fn step(
        &mut self,
        observation: &str,
        world_resources: f64,
        cycle_id: u64,
    ) -> (ActionType, Vec<RuntimeEvent>);
}
