// <FILE>crates/tui-vfx-player/src/fnc_apply_styled_primitive.rs</FILE> - <DESC>Route styled primitive adapters to player styled grids</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Primitive adapter work: keep styled adapter routing small and explicit.</WCTX>
// <CLOG>0.4.0: PATCH — split style and shader adapter bodies into focused modules.</CLOG>

use tui_vfx_contract::NodeSpec;

use crate::{
    PlayerSampleRequest, PlayerStyledGrid, fnc_apply_shader_primitive::apply_shader_primitive,
    fnc_apply_style_primitive::apply_style_primitive,
};

/// Apply a supported styled primitive effect to the styled grid.
pub(crate) fn apply_styled_primitive(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    styled_grid: &mut PlayerStyledGrid,
) -> bool {
    apply_style_primitive(node, request, styled_grid)
        || apply_shader_primitive(node, request, styled_grid)
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_styled_primitive.rs</FILE> - <DESC>Route styled primitive adapters to player styled grids</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
