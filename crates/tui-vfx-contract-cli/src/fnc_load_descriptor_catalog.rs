// <FILE>crates/tui-vfx-contract-cli/src/fnc_load_descriptor_catalog.rs</FILE> - <DESC>Load descriptor packs into a catalog</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J2: resolve descriptor pack JSON before recipe validation.</WCTX>
// <CLOG>0.1.0: INIT — add descriptor pack deserialization, validation, and report metadata.</CLOG>

use std::{collections::BTreeMap, path::PathBuf};

use tui_vfx_contract::{DescriptorCatalog, DescriptorPack, DescriptorPackId};

use crate::{
    cls_descriptor_pack_load::DescriptorPackLoad, cls_descriptor_pack_report::DescriptorPackReport,
    fnc_collect_descriptor_pack_paths::collect_descriptor_pack_paths,
};

/// Load descriptor packs from CLI file and directory flags.
pub fn load_descriptor_catalog(
    files: &[String],
    dirs: &[String],
) -> Result<DescriptorPackLoad, String> {
    let paths = collect_descriptor_pack_paths(files, dirs)?;
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
    Ok(DescriptorPackLoad {
        catalog: DescriptorCatalog { packs },
        reports,
    })
}

fn read_pack(path: &PathBuf) -> Result<DescriptorPack, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("read descriptor pack `{}` failed: {error}", path.display()))?;
    serde_json::from_str(&text)
        .map_err(|error| format!("parse descriptor pack `{}` failed: {error}", path.display()))
}

// <FILE>crates/tui-vfx-contract-cli/src/fnc_load_descriptor_catalog.rs</FILE> - <DESC>Load descriptor packs into a catalog</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
