// <FILE>crates/tui-vfx-player/src/fnc_build_primitive_adapter_gap_report.rs</FILE> - <DESC>Build primitive adapter gap reports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Primitive adapter work: derive adapter gaps from inventory reports.</WCTX>
// <CLOG>0.1.0: INIT — add report builder over represented inventory effects.</CLOG>

use crate::{
    DescriptorPackReport, PlayerInventoryReport, PlayerPrimitiveAdapterGapReport,
    fnc_classify_primitive_adapter_gap::classify_primitive_adapter_gap,
    fnc_summarize_primitive_adapter_gaps::summarize_primitive_adapter_gaps,
};

/// Build a primitive adapter gap report from a player inventory report.
pub fn build_primitive_adapter_gap_report(
    root: String,
    descriptor_packs: Vec<DescriptorPackReport>,
    inventory: &PlayerInventoryReport,
) -> PlayerPrimitiveAdapterGapReport {
    let effects = inventory
        .effects
        .iter()
        .filter(|effect| effect.represented_by_recipes)
        .map(classify_primitive_adapter_gap)
        .collect::<Vec<_>>();
    let summary = summarize_primitive_adapter_gaps(&effects);
    PlayerPrimitiveAdapterGapReport::new(root, descriptor_packs, summary, effects)
}

// <FILE>crates/tui-vfx-player/src/fnc_build_primitive_adapter_gap_report.rs</FILE> - <DESC>Build primitive adapter gap reports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
