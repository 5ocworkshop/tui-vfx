// <FILE>crates/tui-vfx-player/src/cls_player_render_backend_options.rs</FILE> - <DESC>Backend-neutral player render backend options</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Native compositor lowering: carry backend mode and fallback policy without exposing compositor internals.</WCTX>
// <CLOG>0.1.0: INIT — add composition mode and fail-on-fallback options.</CLOG>

use crate::PlayerRenderCompositionMode;

/// Options supplied to backend adapters from player CLI/UI callers.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRenderBackendOptions {
    /// Requested composition strategy.
    pub composition_mode: PlayerRenderCompositionMode,
    /// Whether callers should treat backend fallback as a command failure.
    pub fail_on_fallback: bool,
}

impl Default for PlayerRenderBackendOptions {
    fn default() -> Self {
        Self {
            composition_mode: PlayerRenderCompositionMode::IrResolved,
            fail_on_fallback: false,
        }
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_render_backend_options.rs</FILE> - <DESC>Backend-neutral player render backend options</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
