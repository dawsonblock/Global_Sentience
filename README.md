# GlobalWorkspaceRuntime

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
- Architecture integrity checker: scripts/check_integrity.py validates trace, shortlist, scratchpad, self-report, memory, stream, and state influence invariants.


## Memory and creativity upgrade

Version 0.4 adds a portable long-term archive boundary inspired by Memvid-style use cases. The built-in `MemvidArchive` writes append-only JSONL frames to a `.mv2` file and supports lexical query plus rewind inspection. It is dependency-free by default; a real Memvid backend can replace this boundary later.

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

## Run tests

```bash
cd global_workspace_runtime
python -m pytest -q
```

## Run the demo

```bash
cd global_workspace_runtime
python scripts/run_demo.py
```

The demo prints internal state values, resonance tags, candidate budget, stream candidates, bridge metrics, workspace shortlist, selected candidate, rejected self-report claims, and memory write summary.

## Run ablations

```bash
cd global_workspace_runtime
python scripts/run_ablation.py
python scripts/check_integrity.py
python scripts/analyze_traces.py
```

Ablation artifacts are saved under `artifacts/ablation/`.

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

- `simworld/`: deterministic closed-world events, simulated users, bounded
  actions and outcome scoring.
- `modulation/somatic.py`: a 16-dimensional operational pressure map used as a
  repeatable "gut-signal" style telemetry vector.
- `core/background_processor.py`: bounded idle work for archive recall,
  unresolved scratchpad review and principle abstraction.
- `cognition/meta_critic.py`: advisory review of traces for conservatism,
  repeated rejection and novelty starvation.
- `cognition/predictive_goal_model.py`: deterministic surprise/outcome
  estimator for candidate actions.
- `scripts/run_simworld.py`: runs long-term engagement trials and writes JSONL
  artifacts.

Run SimWorld:

```bash
python scripts/run_simworld.py --cycles 25 --seed 7
```

The world tracks truth score, kindness score, social harmony, trust deltas,
resource stability, uncertainty resolution, repair success and cold-optimization
penalties.


## Corrected merged build: v0.6 Action-Grounded SimWorld Repair

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

Proof from the corrected package:

```bash
python -m pytest -q global_workspace_runtime/tests
# 26 passed
```

A seeded 25-cycle SimWorld run with semantic cache disabled produced 25/25 expected action matches in the included proof artifact. This is still a deterministic research scaffold, not evidence of agency or subjective experience.
