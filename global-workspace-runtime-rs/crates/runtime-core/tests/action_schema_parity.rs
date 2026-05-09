use std::fs;
use std::path::PathBuf;

use runtime_core::ActionType;

#[test]
fn action_schema_matches_json() {
    let schema_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join("schemas/action_types.json");
    let schema_text = fs::read_to_string(&schema_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", schema_path.display()));
    let schema_json: serde_json::Value = serde_json::from_str(&schema_text)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", schema_path.display()));

    let schema_values: Vec<String> = schema_json["enum"]
        .as_array()
        .expect("schema enum must be an array")
        .iter()
        .map(|value| value.as_str().expect("enum values must be strings").to_string())
        .collect();
    let rust_values: Vec<String> = ActionType::all_strs().iter().map(|value| (*value).to_string()).collect();

    assert_eq!(rust_values, schema_values, "Rust ActionType must match schemas/action_types.json exactly");
}