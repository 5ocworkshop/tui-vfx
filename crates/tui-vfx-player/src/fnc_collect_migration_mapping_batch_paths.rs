// <FILE>crates/tui-vfx-player/src/fnc_collect_migration_mapping_batch_paths.rs</FILE> - <DESC>Collect legacy recipe paths for migration mapping</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.9 migration mapping: keep path discovery separate from record classification.</WCTX>
// <CLOG>0.1.0: INIT — add focused path collection helper.</CLOG>

use std::path::{Path, PathBuf};

/// Collect JSON recipe paths for one requested migration batch.
pub(crate) fn collect_migration_mapping_batch_paths(
    legacy_root: &Path,
    family: Option<&str>,
    recursive: bool,
) -> Result<Vec<PathBuf>, String> {
    let root = match family {
        Some(family) => legacy_root.join(family),
        None if recursive => legacy_root.to_path_buf(),
        None => return Err("migration-mapping-batch requires --family or --recursive".to_string()),
    };
    let mut paths = Vec::new();
    collect_json_files(&root, &mut paths)?;
    paths.sort();
    Ok(paths)
}

fn collect_json_files(root: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    if !root.is_dir() {
        return Err(format!("recipe root `{}` does not exist", root.display()));
    }
    for entry in std::fs::read_dir(root).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            collect_json_files(&path, paths)?;
        } else if path
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            paths.push(path);
        }
    }
    Ok(())
}

// <FILE>crates/tui-vfx-player/src/fnc_collect_migration_mapping_batch_paths.rs</FILE> - <DESC>Collect legacy recipe paths for migration mapping</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
