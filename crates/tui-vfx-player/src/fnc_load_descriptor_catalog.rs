// <FILE>crates/tui-vfx-player/src/fnc_load_descriptor_catalog.rs</FILE> - <DESC>Load descriptor packs for player runs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K0: share descriptor catalog loading with player CLI.</WCTX>
// <CLOG>0.1.0: INIT — add explicit/default descriptor pack loading without CLI dependencies.</CLOG>

use std::{collections::BTreeMap, path::PathBuf};

use tui_vfx_contract::{DescriptorCatalog, DescriptorPack, DescriptorPackId};

/// Descriptor pack loaded for one player invocation.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptorPackReport {
    /// Loaded descriptor pack id.
    pub id: String,
    /// Filesystem path that supplied the pack.
    pub path: String,
}

/// Loaded descriptor catalog plus report metadata.
#[derive(Clone, Debug)]
pub struct LoadedDescriptorCatalog {
    /// Catalog passed to RecipePlayer.
    pub catalog: DescriptorCatalog,
    /// Machine-readable loaded pack metadata.
    pub reports: Vec<DescriptorPackReport>,
}

/// Load descriptor packs from files, directories, or the repo default primitive pack.
pub fn load_descriptor_catalog(
    files: &[String],
    dirs: &[String],
) -> Result<LoadedDescriptorCatalog, String> {
    let mut paths = collect_pack_paths(files, dirs)?;
    if paths.is_empty()
        && let Some(path) = default_primitive_pack_path()
    {
        paths.push(path);
    }
    let mut packs = BTreeMap::<DescriptorPackId, DescriptorPack>::new();
    let mut reports = Vec::new();
    for path in paths {
        let pack = read_pack(&path)?;
        if packs.contains_key(&pack.id) {
            return Err(format!(
                "duplicate descriptor pack id `{}`",
                pack.id.as_str()
            ));
        }
        pack.validate().map_err(|error| {
            format!(
                "descriptor pack `{}` failed validation: {error:?}",
                path.display()
            )
        })?;
        reports.push(DescriptorPackReport {
            id: pack.id.as_str().to_string(),
            path: path.display().to_string(),
        });
        packs.insert(pack.id.clone(), pack);
    }
    Ok(LoadedDescriptorCatalog {
        catalog: DescriptorCatalog { packs },
        reports,
    })
}

fn collect_pack_paths(files: &[String], dirs: &[String]) -> Result<Vec<PathBuf>, String> {
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

fn collect_pack_dir(path: &PathBuf, collected: &mut Vec<PathBuf>) -> Result<(), String> {
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

fn default_primitive_pack_path() -> Option<PathBuf> {
    let path = PathBuf::from("descriptors/v3.1/packs/primitive.json");
    path.is_file().then_some(path)
}

fn read_pack(path: &PathBuf) -> Result<DescriptorPack, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("read descriptor pack `{}` failed: {error}", path.display()))?;
    serde_json::from_str(&text)
        .map_err(|error| format!("parse descriptor pack `{}` failed: {error}", path.display()))
}

// <FILE>crates/tui-vfx-player/src/fnc_load_descriptor_catalog.rs</FILE> - <DESC>Load descriptor packs for player runs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
