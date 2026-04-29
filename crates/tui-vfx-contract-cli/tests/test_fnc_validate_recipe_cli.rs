// <FILE>crates/tui-vfx-contract-cli/tests/test_fnc_validate_recipe_cli.rs</FILE> - <DESC>Validate canonical recipe CLI behavior</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J0: prove migrated v3.1 debug recipe fixtures pass contract validation.</WCTX>
// <CLOG>0.1.0: INIT — assert success for J0 fixture corpus and nonzero failure for invalid JSON.</CLOG>

use std::{fs, path::PathBuf, process::Command};

fn contract_cli() -> &'static str {
    env!("CARGO_BIN_EXE_tui-vfx-contract-cli")
}

fn j0_recipe_paths() -> [&'static str; 10] {
    [
        "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json",
        "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/event_driven_dwell/bool_binding_demo.json",
        "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_dim.json",
        "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_greyscale.json",
        "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_invert.json",
        "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_tint.json",
        "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_checkers.json",
        "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_none.json",
        "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_wipe.json",
        "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/samplers/sampler_sinewave.json",
    ]
}

#[test]
fn validates_j0_canonical_recipe_files() {
    let output = Command::new(contract_cli())
        .arg("validate-recipe")
        .args(j0_recipe_paths())
        .output()
        .expect("run contract validation CLI");

    assert!(
        output.status.success(),
        "expected fixtures to validate; stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.matches("\"valid\": true").count(), 10);
}

#[test]
fn rejects_invalid_recipe_json() {
    let path = invalid_recipe_path();
    fs::write(&path, "{ this is not strict JSON }").expect("write invalid recipe fixture");

    let output = Command::new(contract_cli())
        .arg("validate-recipe")
        .arg(&path)
        .output()
        .expect("run contract validation CLI");

    let _ = fs::remove_file(&path);

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"stage\": \"deserialize\""));
    assert!(stdout.contains("\"valid\": false"));
}

fn invalid_recipe_path() -> PathBuf {
    std::env::temp_dir().join(format!(
        "tui-vfx-contract-cli-invalid-{}.json",
        std::process::id()
    ))
}

// <FILE>crates/tui-vfx-contract-cli/tests/test_fnc_validate_recipe_cli.rs</FILE> - <DESC>Validate canonical recipe CLI behavior</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
