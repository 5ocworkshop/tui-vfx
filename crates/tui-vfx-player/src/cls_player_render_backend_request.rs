// <FILE>crates/tui-vfx-player/src/cls_player_render_backend_request.rs</FILE> - <DESC>Backend-neutral render request carrying recipe context for adapters</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Native compositor source isolation: carry both post-effect and source-only IR so adapters can choose the honest source substrate.</WCTX>
// <CLOG>0.2.0: MINOR — add source-only IR alongside post-effect IR for native backend source isolation.
// 0.1.0: INIT — add PlayerRenderBackendRequest with IR, recipe, catalog, sample, and options.</CLOG>

use tui_vfx_contract::{DescriptorCatalog, RecipeDocument};

use crate::{PlayerRenderBackendOptions, PlayerRenderIrReport, PlayerSampleRequest};

/// Complete player-owned request passed to backend adapters that need recipe context.
#[derive(Clone, Debug, PartialEq)]
pub struct PlayerRenderBackendRequest {
    /// Post-effect player IR for compatibility and IR-resolved fallback rendering.
    pub ir: PlayerRenderIrReport,
    /// Source-only player IR before recipe-level graph effects.
    pub source_ir: PlayerRenderIrReport,
    /// Validated canonical recipe document that produced the IR.
    pub recipe: RecipeDocument,
    /// Descriptor catalog used to validate and render the recipe.
    pub descriptor_catalog: DescriptorCatalog,
    /// Runtime sample request, including phase timing and host signals.
    pub sample: PlayerSampleRequest,
    /// Backend-neutral execution options.
    pub backend_options: PlayerRenderBackendOptions,
}

// <FILE>crates/tui-vfx-player/src/cls_player_render_backend_request.rs</FILE> - <DESC>Backend-neutral render request carrying recipe context for adapters</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
