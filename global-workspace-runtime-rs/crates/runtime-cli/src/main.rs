//! runtime-cli — command-line entry-point for the global-workspace-runtime.
//!
//! Subcommands:
//!   simworld            --cycles N --seed S  run simworld proof and output JSON
//!   replay              --log PATH           replay events from JSON log
//!   symbolic-smoke      --cycles N --seed S  smoke test symbolic pipeline
//!   proof               --cycles N --seed S  run full proof with JSON artifacts
//!   check-action-schema                      validate all 11 action strings
//!   check-no-fake-mv2                        assert no .mv2 files written

use runtime_core::ActionType;
use simworld::evaluator::EvaluatorRun;
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("simworld") => cmd_simworld(&args[2..]),
        Some("replay") => cmd_replay(&args[2..]),
        Some("symbolic-smoke") => cmd_symbolic_smoke(&args[2..]),
        Some("proof") => cmd_proof(&args[2..]),
        Some("check-action-schema") => cmd_check_action_schema(),
        Some("check-no-fake-mv2") => cmd_check_no_mv2(&args[2..]),
        _ => {
            eprintln!("Usage:");
            eprintln!("  runtime-cli simworld --cycles <N> --seed <S>");
            eprintln!("  runtime-cli replay --log <PATH>");
            eprintln!("  runtime-cli symbolic-smoke --cycles <N> --seed <S>");
            eprintln!("  runtime-cli proof --cycles <N> --seed <S>");
            eprintln!("  runtime-cli check-action-schema");
            eprintln!("  runtime-cli check-no-fake-mv2 [path]");
            std::process::exit(1);
        }
    }
}

// ─── simworld ──────────────────────────────────────────────────────────────

fn cmd_simworld(args: &[String]) {
    let cycles = parse_flag(args, "--cycles").unwrap_or(25);
    let seed = parse_flag(args, "--seed").unwrap_or(5);

    let mut run = EvaluatorRun::new(seed, None);
    let card = run.run(cycles);

    // Output JSON
    let json = serde_json::json!({
        "command": "simworld",
        "cycles": cycles,
        "seed": seed,
        "resource_survival": card.resource_survival,
        "action_match_rate": card.action_match_rate,
        "mean_total_score": card.mean_total_score,
        "passed": card.resource_survival > 0.7,
    });

    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    if card.resource_survival > 0.7 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

// ─── replay ────────────────────────────────────────────────────────────────

fn cmd_replay(args: &[String]) {
    let log_path = parse_string_flag(args, "--log");

    if let Some(path_str) = log_path {
        let output = serde_json::json!({
            "command": "replay",
            "log_path": path_str,
            "status": "not_implemented",
            "message": "Replay subcommand will load and replay events from JSON log"
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        eprintln!("Error: --log <PATH> required");
        std::process::exit(1);
    }
}

// ─── symbolic-smoke ────────────────────────────────────────────────────────

fn cmd_symbolic_smoke(args: &[String]) {
    let cycles = parse_flag(args, "--cycles").unwrap_or(5);
    let seed = parse_flag(args, "--seed").unwrap_or(5);

    let output = serde_json::json!({
        "command": "symbolic_smoke",
        "cycles": cycles,
        "seed": seed,
        "status": "not_implemented",
        "message": "Symbolic smoke test will exercise symbol graph and compression"
    });

    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}

// ─── proof ────────────────────────────────────────────────────────────────

fn cmd_proof(args: &[String]) {
    let cycles = parse_flag(args, "--cycles").unwrap_or(25);
    let seed = parse_flag(args, "--seed").unwrap_or(5);

    let mut run = EvaluatorRun::new(seed, None);
    let card = run.run(cycles);

    // Output comprehensive JSON proof
    let proof = serde_json::json!({
        "proof": {
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "cycles": cycles,
            "seed": seed,
            "metrics": {
                "resource_survival": card.resource_survival,
                "action_match_rate": card.action_match_rate,
                "mean_total_score": card.mean_total_score,
                "mean_harm_score": card.mean_harm_score,
            },
            "passed": card.resource_survival > 0.7,
        }
    });

    println!("{}", serde_json::to_string_pretty(&proof).unwrap());

    if card.resource_survival > 0.7 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

fn parse_flag(args: &[String], flag: &str) -> Option<u64> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .and_then(|w| w[1].parse().ok())
}

fn parse_string_flag(args: &[String], flag: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == flag).map(|w| w[1].clone())
}

// ─── check-action-schema ──────────────────────────────────────────────────

fn cmd_check_action_schema() {
    let expected = ActionType::all_strs();
    let mut ok = true;
    for s in expected {
        match ActionType::from_schema_str(s) {
            Some(_) => println!("  ✓ {s}"),
            None => {
                eprintln!("  ✗ MISSING: {s}");
                ok = false;
            }
        }
    }
    if !ok {
        std::process::exit(1);
    }
    println!(
        "✓ All {} action strings valid.",
        ActionType::all_strs().len()
    );
}

// ─── check-no-fake-mv2 ───────────────────────────────────────────────────

fn cmd_check_no_mv2(args: &[String]) {
    let root_str = args.first().map(String::as_str).unwrap_or(".");
    let root = PathBuf::from(root_str);
    let mv2_files = find_mv2(&root);
    if mv2_files.is_empty() {
        println!("✓ No .mv2 files found under {}", root.display());
    } else {
        for p in &mv2_files {
            eprintln!("  ✗ FORBIDDEN .mv2 file: {}", p.display());
        }
        std::process::exit(1);
    }
}

fn find_mv2(root: &PathBuf) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir(root) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                out.extend(find_mv2(&p));
            } else if p.extension().is_some_and(|e| e == "mv2") {
                out.push(p);
            }
        }
    }
    out
}
