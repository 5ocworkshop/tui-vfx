// <FILE>crates/tui-vfx-contract-cli/tests/support/mod.rs</FILE> - <DESC>Shared contract CLI integration-test helpers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J2 deslop: keep CLI integration tests OFPF-sized.</WCTX>
// <CLOG>0.1.0: INIT — extract shared command, fixture, and temp-file helpers.</CLOG>

use std::{fs, path::PathBuf, process::Command};

use serde_json::Value;

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
    let path = recipe_root().join(relative_path);
    serde_json::from_str(&fs::read_to_string(path).expect("read canonical recipe"))
        .expect("parse canonical recipe")
}

pub fn recipe_path(relative_path: &str) -> PathBuf {
    recipe_root().join(relative_path)
}

pub fn recipe_root() -> PathBuf {
    recipe_repo_root().join("recipes/v3.1/debug_recipes")
}

#[allow(dead_code)]
pub fn descriptor_pack_path() -> PathBuf {
    workspace_root().join("descriptors/v3.1/packs/primitive.json")
}

#[allow(dead_code)]
pub fn descriptor_pack_dir() -> PathBuf {
    workspace_root().join("descriptors/v3.1/packs")
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

fn recipe_repo_root() -> PathBuf {
    std::env::var_os("RECIPE_REPO")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            workspace_root()
                .parent()
                .expect("workspace has a parent")
                .join("tui-vfx-recipes")
        })
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("crate lives under workspace crates directory")
        .to_path_buf()
}

// <FILE>crates/tui-vfx-contract-cli/tests/support/mod.rs</FILE> - <DESC>Shared contract CLI integration-test helpers</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
