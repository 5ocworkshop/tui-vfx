// <FILE>crates/tui-vfx-contract/tests/test_shadow_contract.rs</FILE> - <DESC>Scene shadow contract tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Scene shadow contract: prove explicit author intent for viewport edge crossing behavior.</WCTX>
// <CLOG>0.1.0: INIT — cover shadow edge-crossing policy wire format and round-trip.</CLOG>

use tui_vfx_contract::{
    ShadowBlendMode, ShadowCompositeMode, ShadowEdge, ShadowEdgeCrossingPolicy, ShadowFalloff,
    ShadowInset, ShadowOffset, ShadowSpec,
};
use tui_vfx_types::Color;

#[test]
fn shadow_edge_crossing_policy_round_trips_as_explicit_author_intent() {
    let shadow = ShadowSpec {
        source_region: None,
        edges: vec![ShadowEdge::Bottom],
        offset: ShadowOffset { x: 1, y: 1 },
        inset: Some(ShadowInset { start: 1, end: 2 }),
        falloff: Some(ShadowFalloff { x: 2, y: 0 }),
        shadow_color: Color::new(0, 0, 0, 128),
        soft_edges: true,
        composite_mode: ShadowCompositeMode::Under,
        blend_mode: ShadowBlendMode::Multiply,
        edge_crossing_policy: Some(ShadowEdgeCrossingPolicy::Preserve),
        glyph_material: None,
        paint_outset: None,
    };

    let json = serde_json::to_value(&shadow).expect("shadow serializes");
    assert_eq!(json["edgeCrossingPolicy"], serde_json::json!("preserve"));

    let restored: ShadowSpec = serde_json::from_value(json).expect("shadow deserializes");
    assert_eq!(
        restored.edge_crossing_policy,
        Some(ShadowEdgeCrossingPolicy::Preserve)
    );
}

// <FILE>crates/tui-vfx-contract/tests/test_shadow_contract.rs</FILE> - <DESC>Scene shadow contract tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
