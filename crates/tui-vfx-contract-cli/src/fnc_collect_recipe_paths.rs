// <FILE>crates/tui-vfx-contract-cli/src/fnc_collect_recipe_paths.rs</FILE> - <DESC>Collect recipe JSON paths for validation</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J1: support recursive migration-smoke fixture validation.</WCTX>
// <CLOG>0.1.0: INIT — add recursive JSON file collection without external dependencies.</CLOG>

use std::path::{Path, PathBuf};

/// Collect files to validate from file or directory arguments.
pub fn collect_recipe_paths(paths: &[String], recursive: bool) -> Result<Vec<PathBuf>, String> {
    let mut collected = Vec::new();
    for path in paths {
        let path = PathBuf::from(path);
        collect_one_path(&path, recursive, &mut collected)?;
    }
    collected.sort();
    Ok(collected)
}

fn collect_one_path(
    path: &Path,
    recursive: bool,
    collected: &mut Vec<PathBuf>,
) -> Result<(), String> {
    if path.is_file() {
        collected.push(path.to_path_buf());
        return Ok(());
    }
    if path.is_dir() && recursive {
        collect_json_files(path, collected)?;
        return Ok(());
    }
    if path.is_dir() {
        return Err(format!(
            "directory `{}` requires --recursive",
            path.display()
        ));
    }
    Err(format!("path `{}` does not exist", path.display()))
}

fn collect_json_files(path: &Path, collected: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in std::fs::read_dir(path).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            collect_json_files(&path, collected)?;
        } else if path
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            collected.push(path);
        }
    }
    Ok(())
}

// <FILE>crates/tui-vfx-contract-cli/src/fnc_collect_recipe_paths.rs</FILE> - <DESC>Collect recipe JSON paths for validation</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
