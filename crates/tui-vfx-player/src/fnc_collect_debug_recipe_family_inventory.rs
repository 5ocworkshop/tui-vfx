// <FILE>crates/tui-vfx-player/src/fnc_collect_debug_recipe_family_inventory.rs</FILE> - <DESC>Collect debug recipe path family inventory</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: inventory old/new debug recipe families without legacy runtime.</WCTX>
// <CLOG>0.2.0: PATCH — split path family classification into a focused helper.</CLOG>

use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

use tui_vfx_contract::RecipeDocument;

use crate::fnc_classify_debug_recipe_family::classify_debug_recipe_family;

/// Path-first inventory for one debug recipe family.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DebugRecipeFamilyInventory {
    /// Stable family bucket name.
    pub family: String,
    /// Recipe paths relative to the inspected root.
    pub recipe_paths: Vec<String>,
    /// Canonical v3.1 effect ids observed while parsing recipe documents.
    pub effect_ids: BTreeSet<String>,
}

impl DebugRecipeFamilyInventory {
    /// Number of recipe paths assigned to this family.
    pub fn count(&self) -> usize {
        self.recipe_paths.len()
    }
}

/// Collect JSON recipe paths under a debug recipe root and bucket them by family.
pub fn collect_debug_recipe_family_inventory(
    root: &Path,
    collect_effect_ids: bool,
) -> Result<BTreeMap<String, DebugRecipeFamilyInventory>, String> {
    let mut paths = Vec::new();
    collect_json_files(root, &mut paths)?;
    paths.sort();
    let mut families = BTreeMap::new();
    for path in paths {
        insert_recipe_path(root, &path, collect_effect_ids, &mut families)?;
    }
    Ok(families)
}

fn insert_recipe_path(
    root: &Path,
    path: &Path,
    collect_effect_ids: bool,
    families: &mut BTreeMap<String, DebugRecipeFamilyInventory>,
) -> Result<(), String> {
    let relative_path = path.strip_prefix(root).unwrap_or(path);
    let family = classify_debug_recipe_family(relative_path);
    let relative_label = normalize_path(relative_path);
    let entry = families
        .entry(family.clone())
        .or_insert_with(|| DebugRecipeFamilyInventory {
            family,
            recipe_paths: Vec::new(),
            effect_ids: BTreeSet::new(),
        });
    entry.recipe_paths.push(relative_label);
    if collect_effect_ids {
        entry.effect_ids.extend(read_v31_effect_ids(path)?);
    }
    Ok(())
}

fn collect_json_files(root: &Path, collected: &mut Vec<PathBuf>) -> Result<(), String> {
    if !root.is_dir() {
        return Err(format!("recipe root `{}` does not exist", root.display()));
    }
    for entry in std::fs::read_dir(root).map_err(|error| error.to_string())? {
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

fn read_v31_effect_ids(path: &Path) -> Result<BTreeSet<String>, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read `{}`: {error}", path.display()))?;
    let recipe: RecipeDocument = serde_json::from_str(&text).map_err(|error| {
        format!(
            "failed to parse `{}` as v3.1 recipe: {error}",
            path.display()
        )
    })?;
    Ok(recipe
        .graph
        .nodes
        .values()
        .map(|node| node.effect.as_str().to_string())
        .collect())
}

fn normalize_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
}

// <FILE>crates/tui-vfx-player/src/fnc_collect_debug_recipe_family_inventory.rs</FILE> - <DESC>Collect debug recipe path family inventory</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
