# LEGACY: gw-kernel

This crate (`runtime/kernel/`) was the first-generation Rust runtime kernel
used during the Phase 1 bootstrap.  It is **superseded** by the workspace at
`global-workspace-runtime-rs/` which is now the authoritative Rust runtime.

**Do not add new code here.**

The integration test suites below have been migrated to the new workspace:

| Legacy file | New location |
|---|---|
| `tests/replay_invariant.rs` | `crates/runtime-core/tests/` |
| `tests/scorecard.rs` | `crates/simworld/tests/` |
| `tests/adversarial.rs` | `crates/simworld/tests/` |
| `tests/property_tests.rs` | `crates/simworld/tests/` |

This directory is kept for historical reference and may be removed in a
future cleanup pass.
