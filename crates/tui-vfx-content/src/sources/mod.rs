// <FILE>crates/tui-vfx-content/src/sources/mod.rs</FILE> - <DESC>Source primitives that produce tui-vfx Grid cells from external assets (rocketsplash images, font atlases). Composable with every downstream VFX primitive since all sources terminate in cell-level writes to a Grid.</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Stage 1 of the splash library + VFX integration plan.</WCTX>
// <CLOG>0.1.0: initial; ship RocketsplashImage + RocketsplashFont + shared blit helper.</CLOG>

//! External asset sources that feed tui-vfx grids.
//!
//! Rocketsplash is a co-designed sister project that produces
//! high-fidelity terminal art (static `.rss` splashes and dynamic `.rsf`
//! font rasterization). Both formats share a [`rocketsplash_rt::RenderBuffer`]
//! substrate, so the blit pipeline is written once in
//! [`fnc_blit_render_buffer_to_grid::blit_render_buffer_to_grid`] and reused
//! by both source types.
//!
//! Once on a [`tui_vfx_types::Grid`], source cells compose with every
//! downstream tui-vfx primitive: drop shadows, wipes, glisten sweeps,
//! filters, masks, and content transformers layered on top.

mod cls_rocketsplash_font;
mod cls_rocketsplash_image;
mod fnc_blit_render_buffer_to_grid;

pub use cls_rocketsplash_font::{FontRender, RocketsplashFont};
pub use cls_rocketsplash_image::RocketsplashImage;
pub use fnc_blit_render_buffer_to_grid::blit_render_buffer_to_grid;

// <FILE>crates/tui-vfx-content/src/sources/mod.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
