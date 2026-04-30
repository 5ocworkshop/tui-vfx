// <FILE>crates/tui-vfx-player-backend-compositor/src/lib.rs</FILE> - <DESC>Compositor backend adapter exports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 player backend playback: expose a player-backend implementation that calls the compositor.</WCTX>
// <CLOG>0.1.0: INIT — export compositor backend, IR lowering, and bounded graph-to-spec lowering helpers.</CLOG>

//! Compositor-backed rendering for player-owned render IR.
//!
//! This crate is intentionally separate from `tui-vfx-player` and `tui-vfx-player-ui`:
//! player core owns the backend-neutral IR, this adapter owns compositor/types lowering,
//! and the UI consumes only backend output.

pub mod fnc_lower_player_ir_to_semantic_scene;
pub mod fnc_lower_recipe_graph_to_composition_spec;
pub mod fnc_render_compositor_backend;

pub use fnc_lower_player_ir_to_semantic_scene::{
    LoweredPlayerRenderIr, lower_player_ir_to_semantic_scene,
};
pub use fnc_lower_recipe_graph_to_composition_spec::lower_player_ir_to_composition_spec;
pub use fnc_render_compositor_backend::{
    CompositorRenderBackend, render_compositor_backend, render_compositor_backend_request,
};

// <FILE>crates/tui-vfx-player-backend-compositor/src/lib.rs</FILE> - <DESC>Compositor backend adapter exports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
