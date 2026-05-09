from __future__ import annotations
import argparse
import json, sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[2]))


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Analyze a runtime trace JSONL file.")
    parser.add_argument(
        "--trace-path",
        type=Path,
        default=None,
        help="Explicit path to a trace JSONL file. Defaults to the latest file in artifacts/traces/.",
    )
    return parser.parse_args()


def resolve_trace_path(trace_path: Path | None) -> Path:
    if trace_path is not None:
        if not trace_path.exists():
            raise SystemExit(f"Trace file does not exist: {trace_path}")
        if trace_path.is_dir():
            raise SystemExit(f"Trace path must be a file, not a directory: {trace_path}")
        return trace_path

    trace_dir = Path("artifacts/traces")
    files = sorted(trace_dir.glob("*.jsonl"))
    if not files:
        raise SystemExit("No traces found in artifacts/traces. Pass --trace-path to analyze a specific file.")
    return files[-1]


def main() -> None:
    args = parse_args()
    trace_path = resolve_trace_path(args.trace_path)
    events = [json.loads(line) for line in trace_path.read_text().splitlines() if line.strip()]
    phases = sorted({e["phase"] for e in events})
    print("=== Trace Analysis ===")
    print("file:", trace_path)
    print("events:", len(events))
    print("phases:", phases)
    print("last_event:", json.dumps(events[-1], indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
