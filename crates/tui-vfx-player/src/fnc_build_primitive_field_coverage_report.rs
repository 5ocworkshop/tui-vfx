// <FILE>crates/tui-vfx-player/src/fnc_build_primitive_field_coverage_report.rs</FILE> - <DESC>Build primitive field coverage reports</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Player evidence tooling: compare authored primitive inputs to descriptors and handled adapters.</WCTX>
// <CLOG>0.3.0: PATCH — delegate descriptor loading, recipe scanning, and summary aggregation.</CLOG>

use std::path::PathBuf;

use crate::{
    DescriptorPackReport, PlayerPrimitiveFieldCoverageReport,
    fnc_load_primitive_field_descriptor_coverage::load_primitive_field_descriptor_coverage,
    fnc_scan_primitive_field_recipe::scan_primitive_field_recipe,
    fnc_summarize_primitive_field_coverage::summarize_primitive_field_coverage,
};

/// Build primitive field coverage from recipe JSON and descriptor pack JSON.
pub fn build_primitive_field_coverage_report(
    root: String,
    descriptor_packs: Vec<DescriptorPackReport>,
    paths: &[PathBuf],
) -> Result<PlayerPrimitiveFieldCoverageReport, String> {
    let descriptors = load_primitive_field_descriptor_coverage(&descriptor_packs)?;
    let recipes = paths
        .iter()
        .map(|path| scan_primitive_field_recipe(path, &descriptors))
        .collect::<Result<Vec<_>, _>>()?;
    let summary = summarize_primitive_field_coverage(&recipes);
    Ok(PlayerPrimitiveFieldCoverageReport::new(
        root,
        descriptor_packs,
        summary,
        recipes,
    ))
}

// <FILE>crates/tui-vfx-player/src/fnc_build_primitive_field_coverage_report.rs</FILE> - <DESC>Build primitive field coverage reports</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
