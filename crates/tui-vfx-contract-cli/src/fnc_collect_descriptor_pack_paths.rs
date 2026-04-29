// <FILE>crates/tui-vfx-contract-cli/src/fnc_collect_descriptor_pack_paths.rs</FILE> - <DESC>Collect descriptor pack JSON paths</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J2: support explicit descriptor pack files and directories.</WCTX>
// <CLOG>0.1.0: INIT — add deterministic descriptor pack file collection.</CLOG>

use std::path::{Path, PathBuf};

/// Collect descriptor pack JSON files from explicit file and directory flags.
pub fn collect_descriptor_pack_paths(
    files: &[String],
    dirs: &[String],
) -> Result<Vec<PathBuf>, String> {
    let mut collected = Vec::new();
    for file in files {
        let path = PathBuf::from(file);
        if !path.is_file() {
            return Err(format!(
                "descriptor pack `{}` is not a file",
                path.display()
            ));
        }
        collected.push(path);
    }
    for dir in dirs {
        collect_pack_dir(&PathBuf::from(dir), &mut collected)?;
    }
    collected.sort();
    Ok(collected)
}

fn collect_pack_dir(path: &Path, collected: &mut Vec<PathBuf>) -> Result<(), String> {
    if !path.is_dir() {
        return Err(format!(
            "descriptor pack directory `{}` is not a directory",
            path.display()
        ));
    }
    for entry in std::fs::read_dir(path).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            collect_pack_dir(&path, collected)?;
        } else if path
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            collected.push(path);
        }
    }
    Ok(())
}

// <FILE>crates/tui-vfx-contract-cli/src/fnc_collect_descriptor_pack_paths.rs</FILE> - <DESC>Collect descriptor pack JSON paths</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
