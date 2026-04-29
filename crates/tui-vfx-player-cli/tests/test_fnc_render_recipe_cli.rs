// <FILE>crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs</FILE> - <DESC>Player CLI render-recipe regression tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K0: lock single and recursive JSON report behavior.</WCTX>
// <CLOG>0.1.0: INIT — add CLI smoke tests for frame and recursive run schemas.</CLOG>

use std::process::Command;

#[test]
fn test_fnc_cli_renders_single_recipe_frame_json() {
    let descriptor_pack = descriptor_pack();
    let output = Command::new(env!("CARGO_BIN_EXE_tui-vfx-player-cli"))
        .args([
            "render-recipe",
            "--recipe",
            "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json",
            "--descriptor-pack",
            descriptor_pack.to_str().expect("utf8 path"),
            "--json",
        ])
        .output()
        .expect("run player cli");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json report");
    assert_eq!(report["schemaVersion"], "v3.1.player.frame.1");
    assert_eq!(report["status"], "rendered");
    assert!(report["nonEmptyCells"].as_u64().expect("cell count") > 0);
}

#[test]
fn test_fnc_cli_renders_recursive_smoke_report_json() {
    let descriptor_pack = descriptor_pack();
    let output = Command::new(env!("CARGO_BIN_EXE_tui-vfx-player-cli"))
        .args([
            "render-recipe",
            "--json",
            "--recursive",
            "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes",
            "--descriptor-pack",
            descriptor_pack.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run recursive player cli");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json report");
    assert_eq!(report["schemaVersion"], "v3.1.player.run.1");
    assert_eq!(report["summary"]["total"], 16);
    assert!(report["summary"]["rendered"].as_u64().expect("rendered") > 0);
    assert!(
        report["summary"]["unsupported"]
            .as_u64()
            .expect("unsupported")
            > 0
    );
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn descriptor_pack() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root")
        .join("descriptors/v3.1/packs/primitive.json")
}

// <FILE>crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs</FILE> - <DESC>Player CLI render-recipe regression tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
