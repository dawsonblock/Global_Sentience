//! runtime-cli — command-line entry-point for the global-workspace-runtime.
//!
//! Subcommands:
//!   simworld            --cycles N --seed S  run simworld proof
//!   check-action-schema                       validate all 11 action strings
//!   check-no-fake-mv2                         assert no .mv2 files written

use runtime_core::ActionType;
use simworld::evaluator::EvaluatorRun;
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("simworld") => cmd_simworld(&args[2..]),
        Some("check-action-schema") => cmd_check_action_schema(),
        Some("check-no-fake-mv2") => cmd_check_no_mv2(&args[2..]),
        _ => {
            eprintln!("Usage:");
            eprintln!("  runtime-cli simworld --cycles <N> --seed <S>");
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

    println!("Running SimWorld: cycles={cycles}, seed={seed}");
    let mut run = EvaluatorRun::new(seed, None);
    let card = run.run(cycles);
    card.print_report();
    card.assert_spec();
    println!("✓ PROOF PASSED");
}

fn parse_flag(args: &[String], flag: &str) -> Option<u64> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .and_then(|w| w[1].parse().ok())
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
