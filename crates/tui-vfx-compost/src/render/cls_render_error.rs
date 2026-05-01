// <FILE>crates/tui-vfx-compost/src/render/cls_render_error.rs</FILE> - <DESC>Native render diagnostics</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Render errors stay native and avoid non-canonical construction vocabulary.</WCTX>
// <CLOG>0.1.1: PATCH — remove non-canonical migration vocabulary from metadata.
// 0.1.0: INIT — add render error type.</CLOG>

use std::error::Error;
use std::fmt;

/// Error returned while rendering a load-validated recipe.
#[derive(Debug)]
pub enum RenderError {
    /// The accepted recipe uses a shape the skeleton renderer does not yet support.
    Unsupported(String),
}

impl fmt::Display for RenderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported(message) => write!(formatter, "unsupported render shape: {message}"),
        }
    }
}

impl Error for RenderError {}

// <FILE>crates/tui-vfx-compost/src/render/cls_render_error.rs</FILE> - <DESC>Native render diagnostics</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
