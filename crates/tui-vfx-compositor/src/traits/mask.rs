// <FILE>tui-vfx-compositor/src/traits/mask.rs</FILE>
// <DESC>Trait for visibility testing</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>Slice 6.6 §F.3 — migrate Mask trait to take &VfxCellContext</WCTX>
// <CLOG>2.0.0: BREAKING — is_visible(x, y, w, h, progress) → is_visible(&VfxCellContext). ctx.t carries the legacy progress clock. Impls that do not read all fields may use ctx.local_x / ctx.width / etc. selectively; ignored fields can be left unused at the call site by accessing only the relevant ones.</CLOG>

use tui_vfx_types::VfxCellContext;

pub trait Mask {
    /// Determines if a cell is visible given its spatial context.
    ///
    /// The `ctx` bundle carries all per-cell fields: `local_x`, `local_y`,
    /// `width`, `height`, `screen_x`, `screen_y`, and `t`. The `t` field
    /// is the animation progress clock (same `f64` semantics as the legacy
    /// `progress` parameter). Impls that do not read every field should
    /// access only the ones they need; leaving `ctx` underscore-prefixed
    /// (`_ctx`) is appropriate only when the impl reads zero fields.
    fn is_visible(&self, ctx: &VfxCellContext) -> bool;
}

// <FILE>tui-vfx-compositor/src/traits/mask.rs</FILE>
// <DESC>Trait for visibility testing</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
