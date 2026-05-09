use std::process::Command;

#[test]
fn simworld_outputs_json_with_mode() {
    let output = Command::new(env!("CARGO_BIN_EXE_runtime-cli"))
        .args([
            "simworld", "--cycles", "3", "--seed", "5", "--mode", "oracle",
        ])
        .output()
        .expect("failed to run runtime-cli simworld");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("stdout should be valid json");
    assert_eq!(value["command"], "simworld");
    assert_eq!(value["mode"], "oracle");
    assert_eq!(value["scorecard"]["mode"], "oracle");
}

#[test]
fn check_action_schema_outputs_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_runtime-cli"))
        .arg("check-action-schema")
        .output()
        .expect("failed to run runtime-cli check-action-schema");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("stdout should be valid json");
    assert_eq!(value["command"], "check-action-schema");
    assert_eq!(value["all_ok"], true);
}
