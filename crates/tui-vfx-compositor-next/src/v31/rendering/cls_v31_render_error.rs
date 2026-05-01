// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/cls_v31_render_error.rs</FILE> - <DESC>Direct v3.1 render error type</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Keep render error ownership separate from render orchestration.</WCTX>
// <CLOG>0.1.0: INIT — extract V31RenderError.</CLOG>

/// Error returned by direct v3.1 compositor-next rendering.
#[derive(Clone, Debug, PartialEq)]
pub enum V31RenderError {
    /// The direct v3.1 lane does not yet support the requested shape.
    Unsupported(String),
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/cls_v31_render_error.rs</FILE> - <DESC>Direct v3.1 render error type</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
