// <FILE>crates/tui-vfx-contract-cli/tests/support/mod.rs</FILE> - <DESC>Shared contract CLI integration-test helpers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J2 deslop: keep CLI integration tests OFPF-sized.</WCTX>
// <CLOG>0.1.0: INIT — extract shared command, fixture, and temp-file helpers.</CLOG>

use std::{fs, path::PathBuf, process::Command};

use serde_json::Value;

pub const RECIPE_ROOT: &str = "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes";

pub fn run_success(args: &[&str]) -> Value {
    let output = Command::new(contract_cli())
        .args(args)
        .output()
        .expect("run contract validation CLI");
    assert!(
        output.status.success(),
        "expected validation success; stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("parse validation report")
}

pub fn run_failure_args(args: &[&str]) -> Value {
    let output = Command::new(contract_cli())
        .args(args)
        .output()
        .expect("run contract validation CLI");
    assert_eq!(
        output.status.code(),
        Some(1),
        "expected validation failure; stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("parse validation report")
}

pub fn read_recipe(relative_path: &str) -> Value {
    let path = PathBuf::from(RECIPE_ROOT).join(relative_path);
    serde_json::from_str(&fs::read_to_string(path).expect("read canonical recipe"))
        .expect("parse canonical recipe")
}

pub fn write_json(path: &PathBuf, value: &Value) {
    fs::write(
        path,
        serde_json::to_string_pretty(value).expect("serialize JSON fixture"),
    )
    .expect("write JSON fixture");
}

pub fn mutated_recipe_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "tui-vfx-contract-cli-{label}-{}.json",
        std::process::id()
    ))
}

pub fn remove_temp(path: &PathBuf) {
    let _ = fs::remove_file(path);
}

fn contract_cli() -> &'static str {
    env!("CARGO_BIN_EXE_tui-vfx-contract-cli")
}

// <FILE>crates/tui-vfx-contract-cli/tests/support/mod.rs</FILE> - <DESC>Shared contract CLI integration-test helpers</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
