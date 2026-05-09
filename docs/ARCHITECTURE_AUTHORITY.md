# Architecture Authority

Global_Sentience is a deterministic, global-workspace-inspired runtime scaffold.
It does not claim sentience, consciousness, awareness, feelings, desires, or
real understanding.

## Authoritative Implementations

- Python under `src/global_workspace_runtime/` is the current authoritative
  research/runtime implementation.
- Rust under `global-workspace-runtime-rs/` is a deterministic kernel-in-progress.
  It is not the current end-to-end runtime authority.

## Legacy And Vendored Code

- `runtime/kernel/` is legacy/deprecated. Do not treat it as current runtime
  authority.
- `vendor/memvid-main/` is vendored upstream code. It is not integrated into
  the runtime.

## Memory Boundary

- JSONL is the active archive backend today.
- `JsonlArchive` is the current long-term archive implementation.
- Any Memvid or `RealMemvidBackend` surface is a stub/integration point unless
  a future implementation explicitly wires it into the runtime.

## Proof And Trace Boundaries

- Generated traces, proof outputs, and memory logs are runtime artifacts, not
  source-of-truth assets.
- Only sanitized fixtures under `tests/fixtures/` should be committed as proof
  examples.