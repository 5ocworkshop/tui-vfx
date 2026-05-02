// <FILE>crates/tui-vfx-compost/src/render/fnc_build_shadow_config.rs</FILE> - <DESC>Map v3.1 scene-element shadow specs to runtime shadow configuration</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Surface shadow rendering adapts the mature tui-vfx-shadow configuration model to canonical v3.1 scene-element data.</WCTX>
// <CLOG>0.1.0: INIT — translate typed v3.1 shadow geometry into ShadowConfig.</CLOG>

use tui_vfx_contract::{ShadowEdge, ShadowInset, ShadowSpec};
use tui_vfx_shadow::{ShadowConfig, ShadowEdges, ShadowStyle};

pub(crate) fn build_shadow_config(shadow: &ShadowSpec) -> ShadowConfig {
    let mut config = ShadowConfig::new(shadow.shadow_color)
        .with_offset(
            clamp_i16_to_i8(shadow.offset.x),
            clamp_i16_to_i8(shadow.offset.y),
        )
        .with_edges(shadow_edges(&shadow.edges))
        .with_style(shadow_style(shadow.soft_edges))
        .with_soft_edges(shadow.soft_edges);

    if let Some(inset) = shadow.inset {
        config = apply_inset(config, inset, &shadow.edges);
    }
    if let Some(falloff) = shadow.falloff {
        config = config.with_falloff(clamp_u16_to_u8(falloff.x), clamp_u16_to_u8(falloff.y));
    }

    config
}

fn shadow_style(soft_edges: bool) -> ShadowStyle {
    if soft_edges {
        ShadowStyle::HalfBlock
    } else {
        ShadowStyle::Solid
    }
}

fn apply_inset(config: ShadowConfig, inset: ShadowInset, edges: &[ShadowEdge]) -> ShadowConfig {
    let start = clamp_u16_to_u8(inset.start);
    let end = clamp_u16_to_u8(inset.end);
    let has_horizontal = edges
        .iter()
        .any(|edge| matches!(edge, ShadowEdge::Top | ShadowEdge::Bottom));
    let has_vertical = edges
        .iter()
        .any(|edge| matches!(edge, ShadowEdge::Left | ShadowEdge::Right));

    match (has_horizontal, has_vertical) {
        (true, true) => config.with_inset(start, start).with_inset_end(end, end),
        (true, false) => config.with_inset(start, 0).with_inset_end(end, 0),
        (false, true) => config.with_inset(0, start).with_inset_end(0, end),
        (false, false) => config,
    }
}

fn shadow_edges(edges: &[ShadowEdge]) -> ShadowEdges {
    edges.iter().fold(ShadowEdges::empty(), |acc, edge| {
        acc | match edge {
            ShadowEdge::Top => ShadowEdges::TOP,
            ShadowEdge::Right => ShadowEdges::RIGHT,
            ShadowEdge::Bottom => ShadowEdges::BOTTOM,
            ShadowEdge::Left => ShadowEdges::LEFT,
        }
    })
}

fn clamp_i16_to_i8(value: i16) -> i8 {
    value.clamp(i16::from(i8::MIN), i16::from(i8::MAX)) as i8
}

fn clamp_u16_to_u8(value: u16) -> u8 {
    value.min(u16::from(u8::MAX)) as u8
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_build_shadow_config.rs</FILE> - <DESC>Map v3.1 scene-element shadow specs to runtime shadow configuration</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
