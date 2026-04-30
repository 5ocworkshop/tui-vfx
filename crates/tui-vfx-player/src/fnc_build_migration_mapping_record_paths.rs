// <FILE>crates/tui-vfx-player/src/fnc_build_migration_mapping_record_paths.rs</FILE> - <DESC>Build path fields for migration mapping records</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.10 corpus mapping: keep path-derived record fields focused.</WCTX>
// <CLOG>0.1.0: INIT — add path normalization and candidate path helpers.</CLOG>

use std::path::Path;

/// Normalize a path relative to the legacy root into slash-separated report form.
pub(crate) fn normalize_migration_mapping_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
}

/// Derive a stable legacy family from a normalized legacy path.
pub(crate) fn migration_mapping_family_for(legacy_path: &str) -> String {
    legacy_path
        .split_once('/')
        .map(|(family, _)| family)
        .unwrap_or("other")
        .to_string()
}

/// Derive a candidate canonical path from a legacy path and recipe name.
pub(crate) fn migration_mapping_canonical_path_for(legacy_path: &str, recipe_name: &str) -> String {
    if recipe_name.starts_with("_DEPRECATED_") {
        return legacy_path.replacen("_DEPRECATED_", "", 1);
    }
    canonical_renamed_fixture_path(legacy_path)
        .unwrap_or(legacy_path)
        .to_string()
}

fn canonical_renamed_fixture_path(legacy_path: &str) -> Option<&'static str> {
    match legacy_path {
        "shaders/primitives/shader_reveal_wipe_corner_in_bottom_right.json" => {
            Some("shaders/primitives/shader_reveal_wipe_right_to_left.json")
        }
        _ => None,
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_build_migration_mapping_record_paths.rs</FILE> - <DESC>Build path fields for migration mapping records</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
