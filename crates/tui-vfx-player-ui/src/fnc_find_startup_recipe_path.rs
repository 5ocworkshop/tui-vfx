// <FILE>crates/tui-vfx-player-ui/src/fnc_find_startup_recipe_path.rs</FILE> - <DESC>Find startup recipe path for UI roots</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Player UI de-slop: accept Path borrows without PathBuf coupling.</WCTX>
// <CLOG>0.1.1: PATCH — narrow helper arguments from PathBuf to Path borrows.</CLOG>

use std::path::{Path, PathBuf};

/// Resolve a startup recipe path from an optional recipe browser root.
pub fn find_startup_recipe_path(recipes_root: Option<&Path>) -> Result<PathBuf, String> {
    let Some(root) = recipes_root else {
        return Err("missing recipe path or --recipes-root".to_string());
    };
    let mut paths = Vec::new();
    collect_json_paths(root, &mut paths)?;
    paths.sort();
    paths
        .into_iter()
        .next()
        .ok_or_else(|| format!("no JSON recipes found under `{}`", root.display()))
}

fn collect_json_paths(path: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    if path.is_file() {
        collect_file(path, paths);
        return Ok(());
    }
    if !path.is_dir() {
        return Err(format!(
            "recipes root `{}` is not a directory",
            path.display()
        ));
    }
    collect_dir(path, paths)
}

fn collect_dir(path: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in std::fs::read_dir(path).map_err(|error| error.to_string())? {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.is_dir() {
            collect_json_paths(&path, paths)?;
        } else {
            collect_file(&path, paths);
        }
    }
    Ok(())
}

fn collect_file(path: &Path, paths: &mut Vec<PathBuf>) {
    if path
        .extension()
        .is_some_and(|extension| extension == "json")
    {
        paths.push(path.to_path_buf());
    }
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_find_startup_recipe_path.rs</FILE> - <DESC>Find startup recipe path for UI roots</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
