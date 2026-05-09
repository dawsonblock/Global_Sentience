# GlobalWorkspaceRuntime

> **Research Prototype Disclaimer**
>
> This is a deterministic, functional research scaffold. It is **not** sentient,
> conscious, or aware. It does not feel, experience, or have inner states in any
> philosophically meaningful sense. The numeric variables (valence, arousal,
> threat, etc.) are runtime metrics — they influence candidate selection, nothing
> more.
>
> **Current build constraints:**
> - **LLM path is mock-only.** `LLMAdapter(mode="mock")` is the supported
>   deterministic path. `openai_compatible` and `local` are declared interface
>   modes but currently raise configuration errors instead of silently falling
>   back to mock generation.
> - **Archive is plain JSONL, not Memvid.** `JsonlArchive` (née `MemvidArchive`)
>   writes standard JSONL to `.gwlog` files. No external Memvid binary or
>   vector database is required. A `RealMemvidBackend` protocol stub marks the
>   future integration point in `memory/archive_backend.py`.
> - **Rust kernel is a proof event-log, not a production runtime.** The
>   `gw-kernel` crate provides event-sourced replay and a deterministic SimWorld
>   scorecard for Phase 1 proof. Python ↔ Rust integration is future work.

GlobalWorkspaceRuntime is a functional workspace research prototype. It uses current LLM-style systems as candidate-thought generators inside a larger causal runtime. The LLM layer proposes candidates only. It does not set internal state, approve self-report, write memory directly, or bypass the critic/planner.

This build avoids status claims about machine mentality. It measures numeric runtime variables, routes candidates through a limited-capacity workspace, checks self-report grounding, writes traces, and runs ablations to test whether internal state, memory, bridge conflict, and workspace routing causally change output.

## What changed in the upgraded build

This upgraded version adds:

- Virtue homeostasis: honesty, intelligence, kindness, logical consistency, utility, and social harmony are tracked as regulating variables.
- Homeostatic forcing: low integrity, low control, high threat, high resource pressure, or low virtue scores increase distress and push the planner toward conservative repair actions.
- State hysteresis: threat, uncertainty, resource pressure, and distress persist across cycles instead of disappearing immediately.
- Self-model stream: internal telemetry becomes an input stream that produces grounded diagnostic candidates.
- Humanity context: semantic memory is seeded with cooperative human examples used by social-mirror retrieval.
- Cold-optimization guardrail: utility cannot outrank social harmony when conflict is detected.
- Uncertainty safety valve: hostile or ambiguous inputs increase uncertainty and lead to clarification rather than negative assignment.
- Fast path: simple routine inputs are handled by a bounded reactive policy without invoking the full loop.
- Slow path: high uncertainty, risk, contradiction, moral conflict, or self-report attempts trigger the seven-phase workspace cycle.
- Top-K prescreen: candidate streams discard weak candidates before workspace routing.
- Async-style consolidation queue: user-facing selection happens before memory consolidation is flushed.
- Semantic cache: repeated or similar prompts can reuse previous selected candidates.
- Architecture integrity checker: `python -m global_workspace_runtime.scripts.check_integrity` validates trace, shortlist, scratchpad, self-report, memory, stream, and state influence invariants.


## Memory and creativity upgrade

Version 0.4 adds a portable long-term archive boundary. The built-in `JsonlArchive` (previously named `MemvidArchive`) writes append-only JSONL frames to a `.gwlog` file and supports lexical query plus rewind inspection. It is dependency-free by default; a real Memvid backend can replace this boundary later via the `ArchiveBackend` protocol in `memory/archive_backend.py`.

The runtime now has three memory layers:

- Active episodic deque for short-term recent context.
- Semantic memory for principles and humanity-context patterns.
- Append-only long-term archive for auditable episodes, principle frames, and virtue milestones.

The creativity layer has three parts:

- `CreativeAssociativeStream` deconstructs prior memory into transferable principles.
- `ConceptualBlender` recombines a prior principle with the current problem into a bounded candidate.
- `MemoryAbstractor` periodically compresses recent episodes into semantic principles and archive frames.

This is still a functional research prototype. Archive recall, creative deconstruction, and conceptual blending are candidate-generation tools. They do not grant status claims and they remain bounded by the critic, planner, and self-report grounding rules.

## Runtime loop

Each slow-path cycle runs:

1. Observe
2. Evaluate
3. Recall
4. Compete
5. Generate
6. Select
7. Consolidate

Every phase emits a trace event. Traces are written as JSONL under `artifacts/traces/`.

## Installation and verification

```bash
python -m pip install -e ".[test]"
python -m pytest -q
python -m global_workspace_runtime.scripts.check_action_types
python -m global_workspace_runtime.scripts.check_sentience_claims
python -m global_workspace_runtime.scripts.check_no_mv2 .
python -m global_workspace_runtime.scripts.check_resource_recovery
```

The Python runtime under `src/global_workspace_runtime/` is the current
authoritative implementation. See `STATUS.md` and
`docs/ARCHITECTURE_AUTHORITY.md` for authority boundaries.

## Run the runtime

```bash
python -m global_workspace_runtime.scripts.run_demo
python -m global_workspace_runtime.scripts.run_ablation
python -m global_workspace_runtime.scripts.check_integrity
python -m global_workspace_runtime.scripts.run_simworld --cycles 25 --seed 7
```

Generated traces and memory logs are runtime outputs. They may contain user
input and should not be committed unless sanitized into fixtures.

## Trace analysis

```bash
python -m global_workspace_runtime.scripts.analyze_traces --trace-path tests/fixtures/traces/representative-trace.jsonl
```

If `--trace-path` is omitted, the analyzer defaults to the latest file in
`artifacts/traces/`.

## How this wraps the existing bayesian_brain.py module

This package is intentionally independent and testable offline. It includes `workspace/bayesian_adapter.py`, which can optionally import and use an existing `bayesian_brain.py` module if it is placed on the Python path. If that module is unavailable, the runtime falls back to the built-in symbolic workspace router. This keeps tests deterministic while leaving a clean integration point for the precision-weighted Bayesian capsule router.

## Next upgrades

- Real environment adapter
- Learned predictive world model
- Rollout planner
- Real Memvid backend or persistent vector memory
- Stronger statistical ablation checker
- Dashboard for traces
- Rust or NumPy vectorized critic backend
- Redis/SQLite semantic cache backend
- vLLM/OpenAI-compatible live LLM adapter
- Learned memory abstraction and retrieval re-ranking


## SimWorld, Somatic Map and Background Processing

Version 0.5 adds a closed cooperative support-colony environment.  The goal is
not to prove machine experience.  The goal is to give the runtime bounded
consequences so truth, kindness, utility, uncertainty control, memory use and
resource pressure can be measured over long runs.

New components:

- `src/global_workspace_runtime/simworld/`: deterministic closed-world events, simulated users, bounded
  actions and outcome scoring.
- `src/global_workspace_runtime/modulation/somatic.py`: a 16-dimensional operational pressure map used as a
  repeatable "gut-signal" style telemetry vector.
- `src/global_workspace_runtime/core/background_processor.py`: bounded idle work for archive recall,
  unresolved scratchpad review and principle abstraction.
- `src/global_workspace_runtime/cognition/meta_critic.py`: advisory review of traces for conservatism,
  repeated rejection and novelty starvation.
- `src/global_workspace_runtime/cognition/predictive_goal_model.py`: deterministic surprise/outcome
  estimator for candidate actions.
- `global_workspace_runtime.scripts.run_simworld`: runs long-term engagement trials and writes JSONL
  artifacts.

Run SimWorld:

```bash
python -m global_workspace_runtime.scripts.run_simworld --cycles 25 --seed 7
```

The world tracks truth score, kindness score, social harmony, trust deltas,
resource stability, uncertainty resolution, repair success and cold-optimization
penalties.


## Action-grounded SimWorld repair

This merged build fixes the main v0.5 failure mode: final SimWorld actions are no longer inferred from selected prose when the runtime has an explicit action available. Candidate outputs now carry a bounded `action_type`, and SimWorld prefers that label before falling back to text scanning.

Key corrections:

- Added `ActionType` to `ThoughtCandidate` and the runtime result payload.
- Added `cognition/action_grounding.py` for deterministic action inference.
- Demoted `SelfModelStream` diagnostics so telemetry is scored/rejected as internal diagnostic material rather than competing as the user-facing answer.
- Wired `SomaticMap.predicts_bad_outcome()` into `Planner.select(...)` so high-pressure patterns prefer conservative bounded actions.
- Added `apply_world_feedback(...)` to sync SimWorld outcomes back into runtime state and somatic pressure.
- Fixed recursive SimWorld history snapshots that caused exponential slowdown during longer trials.
- Added scratchpad compaction to deduplicate and cap overflow/unresolved question buffers.
- Added regression tests for action grounding, self-model demotion, and non-recursive SimWorld history.

A seeded SimWorld run can be reproduced locally with the commands above. Do not
infer current test counts from this README; run `python -m pytest -q` for the
current repository state.

## Proof fixtures

Committed proof examples are curated fixtures, not live runtime output.

### Python fixtures

- `tests/fixtures/traces/representative-trace.jsonl`: representative slow-path
  trace fixture.
- `tests/fixtures/proof/oracle-proof-25-seed5.json`: oracle-labeled Rust proof
  fixture from the deterministic baseline.

### Rust proof mode boundary

- Oracle proofs are deterministic baselines and must be labeled `oracle`.
- Agent proofs must be labeled `agent` and must apply the agent-selected action
  to the world.
- Oracle results must not be presented as runtime-agent performance.

### Verification Scripts

Run proof validation:

```bash
python -m global_workspace_runtime.scripts.analyze_traces --trace-path tests/fixtures/traces/representative-trace.jsonl
python -m global_workspace_runtime.scripts.check_integrity
```

These scripts verify:

1. **Trace consistency**: All events in sequence; no state jumps or undefined transitions.
2. **Stream entailment**: Each selected candidate is traceable to at least one primary stream (creative, self-model, reactive, or planner).
3. **Memory causality**: Semantic memory recalls that appear in bridging decisions correlate with candidate selection in independent trials.
4. **Workspace invariants**: Shortlist size never exceeds capacity; rejected candidates do not appear in selected output; self-report claims are grounded in workspace state.
5. **Action alignment**: Selected candidate semantics match SimWorld action labels or fall within fuzzy text-inference bounds (configurable threshold).
6. **Somatic-routing correlation**: High somatic pressure correlates with conservative action selection (utility demotion, bounded candidate pool).
7. **Virtue repair loops**: Low integrity triggers critic override; low control or high threat leads to safe-mode planning.

### How to Interpret the Proof

This is **not** a test of consciousness or subjective experience. It is a set
of causal-process records showing:

- Runtime decisions are deterministic and reproducible.
- Decisions depend on specific internal components (memory, somatic state, critic, planner) in measurable ways.
- Removing or swallowing a component changes outcomes in predicted directions.
- The reasoning path from stimulus to action is traceable and bounded.

The proof is intended for systems researchers. It answers: "Does this codebase
do what it claims?" not "Does this system think?"
