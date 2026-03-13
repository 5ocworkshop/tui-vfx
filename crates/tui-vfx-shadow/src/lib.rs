// <FILE>crates/tui-vfx-shadow/src/lib.rs</FILE> - <DESC>Shadow rendering effects for TUI applications</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Phase 0 dramatic color-shadow rollout: re-export compositing mode and grade config</WCTX>
// <CLOG>Add ShadowCompositeMode and ShadowGradeConfig to crate-root re-exports and update crate docs</CLOG>

//! # TUI VFX Shadow
//!
//! Shadow rendering effects for terminal user interfaces.
//!
//! This crate provides theme-aware shadow rendering with multiple styles,
//! configurable offsets, edge selection, and animation support.
//!
//! ## Features
//!
//! - **Multiple shadow styles**: HalfBlock, Braille, MediumShade, Solid, and Gradient
//! - **Sub-cell precision**: Half-block characters provide smooth shadow edges
//! - **Configurable offsets**: Position shadows in any direction (±x, ±y)
//! - **Edge selection**: Render shadows on specific edges (top, bottom, left, right)
//! - **Animation support**: Animate shadows in/out with progress parameter
//! - **Theme integration**: Works with any color from your theme system
//!
//! ## Architecture & Pipeline Position
//!
//! Shadows are rendered **before** the element they belong to, following the
//! standard rendering order:
//!
//! ```text
//! 1. Background layer (surface color)
//! 2. Shadow layer        ← render_shadow() goes here
//! 3. Element layer       ← your UI element
//! 4. Compositor effects  ← wipes, dissolves, fades, etc.
//! ```
//!
//! This ensures shadows appear "behind" elements and can be animated
//! independently using the compositor pipeline.
//!
//! ## Quick Start
//!
//! ```
//! use tui_vfx_shadow::{render_shadow, ShadowConfig, ShadowEdges, ShadowStyle};
//! use tui_vfx_types::{Color, OwnedGrid, Rect};
//!
//! // Create a grid to render into
//! let mut grid = OwnedGrid::new(40, 20);
//!
//! // Define the element that casts the shadow
//! let element_rect = Rect::new(10, 5, 15, 8);
//!
//! // Configure the shadow
//! let config = ShadowConfig::new(Color::BLACK.with_alpha(128))
//!     .with_offset(2, 1)
//!     .with_style(ShadowStyle::HalfBlock)
//!     .with_edges(ShadowEdges::BOTTOM_RIGHT)
//!     .with_soft_edges(true);
//!
//! // Render the shadow (progress=1.0 for full shadow)
//! render_shadow(&mut grid, element_rect, &config, 1.0);
//! ```
//!
//! ## Shadow Styles
//!
//! ### Style Comparison
//!
//! | Style | Characters | Sub-cell | Compatibility | Best For |
//! |-------|------------|----------|---------------|----------|
//! | HalfBlock | ▐▄▌▀ | Yes (2x) | Good | Default, most UIs |
//! | Braille | ⣿⡇⠿ | Yes (2x4) | Variable | Dithered effects |
//! | MediumShade | ▒ | No | Good | Textured full-cell shade |
//! | Solid | (space) | No | Excellent | Maximum compat |
//! | Gradient | (space) | No | Excellent | Soft drop shadows |
//!
//! ### HalfBlock (Default)
//!
//! Uses Unicode half-block characters for sub-cell precision. The shadow
//! edge uses half-block characters with `fg=shadow_color` and `bg=surface_color`,
//! while the shadow interior uses solid background color.
//!
//! ```text
//! ┌──────────┐
//! │  Modal   │▐░░
//! │  Window  │▐░░
//! └──────────┘▐░░
//!  ▄▄▄▄▄▄▄▄▄▄▄░░
//!
//! ▐ = RIGHT_HALF (soft edge)
//! ▄ = LOWER_HALF (soft edge)
//! ░ = solid shadow cells
//! ```
//!
//! ```
//! use tui_vfx_shadow::{ShadowConfig, ShadowStyle};
//! use tui_vfx_types::Color;
//!
//! let config = ShadowConfig::new(Color::BLACK.with_alpha(128))
//!     .with_style(ShadowStyle::HalfBlock)
//!     .with_soft_edges(true)   // Use half-blocks at shadow boundary
//!     .with_surface_color(Color::rgb(40, 40, 50));  // Background for blending
//! ```
//!
//! ### Braille
//!
//! Uses braille patterns (U+2800-U+28FF) for dithered shadows. Each character
//! provides a 2x4 subpixel grid (8 dots), allowing fine-grained density control.
//!
//! ```text
//! Density mapping:
//! 0.0 → ⠀ (empty)
//! 0.25 → ⠃
//! 0.5  → ⡇
//! 0.75 → ⣷
//! 1.0  → ⣿ (full)
//! ```
//!
//! ```
//! use tui_vfx_shadow::{ShadowConfig, ShadowStyle};
//! use tui_vfx_types::Color;
//!
//! let config = ShadowConfig::new(Color::BLACK.with_alpha(200))
//!     .with_style(ShadowStyle::braille(0.7)); // 70% density
//! ```
//!
//! **Note:** Braille rendering depends on terminal font support. Some fonts
//! may not render braille characters at the expected size.
//!
//! ### MediumShade
//!
//! Uses the Unicode medium shade character (`▒`) for a textured full-cell shadow.
//! This style avoids braille dot patterns while retaining visual grain.
//!
//! ```
//! use tui_vfx_shadow::{ShadowConfig, ShadowStyle};
//! use tui_vfx_types::Color;
//!
//! let config = ShadowConfig::new(Color::BLACK.with_alpha(180))
//!     .with_style(ShadowStyle::MediumShade);
//! ```
//!
//! ### Solid
//!
//! The simplest style - fills shadow cells with solid background color.
//! Maximum compatibility but no sub-cell precision.
//!
//! ```
//! use tui_vfx_shadow::{ShadowConfig, ShadowStyle};
//! use tui_vfx_types::Color;
//!
//! let config = ShadowConfig::new(Color::BLACK.with_alpha(100))
//!     .with_style(ShadowStyle::Solid);
//! ```
//!
//! ### Gradient
//!
//! Multi-layer shadows with progressively lighter colors. Each layer extends
//! further from the element with decreasing intensity.
//!
//! ```text
//! Layer 1 (closest):  alpha = base_alpha * 1.0
//! Layer 2:            alpha = base_alpha * 0.67
//! Layer 3 (furthest): alpha = base_alpha * 0.33
//! ```
//!
//! ```
//! use tui_vfx_shadow::{ShadowConfig, ShadowStyle};
//! use tui_vfx_types::Color;
//!
//! let config = ShadowConfig::new(Color::BLACK.with_alpha(200))
//!     .with_offset(1, 1)
//!     .with_style(ShadowStyle::gradient(3)); // 3 gradient layers
//! ```
//!
//! ## Edge Configuration
//!
//! Control which edges of the element cast shadows using [`ShadowEdges`]:
//!
//! ```
//! use tui_vfx_shadow::ShadowEdges;
//!
//! // Common configurations
//! let drop_shadow = ShadowEdges::BOTTOM_RIGHT;  // Standard drop shadow
//! let all_sides = ShadowEdges::ALL;             // Shadow on all edges
//! let custom = ShadowEdges::RIGHT | ShadowEdges::BOTTOM | ShadowEdges::TOP;
//! ```
//!
//! Edge rendering depends on offset direction:
//! - **RIGHT** edge: only rendered when `offset_x > 0`
//! - **BOTTOM** edge: only rendered when `offset_y > 0`
//! - **LEFT** edge: only rendered when `offset_x < 0`
//! - **TOP** edge: only rendered when `offset_y < 0`
//!
//! ## Offset Configuration
//!
//! The offset determines shadow position relative to the element:
//!
//! ```
//! use tui_vfx_shadow::ShadowConfig;
//! use tui_vfx_types::Color;
//!
//! // Standard drop shadow (down-right)
//! let config = ShadowConfig::new(Color::BLACK.with_alpha(128))
//!     .with_offset(2, 1);  // 2 cells right, 1 cell down
//!
//! // Inverted shadow (up-left)
//! let config = ShadowConfig::new(Color::BLACK.with_alpha(128))
//!     .with_offset(-1, -1);  // 1 cell left, 1 cell up
//!
//! // Horizontal-only shadow
//! let config = ShadowConfig::new(Color::BLACK.with_alpha(128))
//!     .with_offset(2, 0);  // 2 cells right, no vertical offset
//! ```
//!
//! ## Animation Support
//!
//! The `progress` parameter (0.0 to 1.0) controls shadow intensity, enabling
//! smooth fade-in/fade-out animations:
//!
//! ```
//! use tui_vfx_shadow::{render_shadow, ShadowConfig};
//! use tui_vfx_types::{Color, OwnedGrid, Rect};
//!
//! let mut grid = OwnedGrid::new(40, 20);
//! let rect = Rect::new(10, 5, 15, 8);
//! let config = ShadowConfig::new(Color::BLACK.with_alpha(200));
//!
//! // Animate shadow from 0% to 100% over time
//! for frame in 0..60 {
//!     let progress = frame as f64 / 60.0;
//!     render_shadow(&mut grid, rect, &config, progress);
//! }
//! ```
//!
//! The progress value interpolates the shadow color's alpha channel:
//! - `progress = 0.0`: shadow alpha = 0 (invisible)
//! - `progress = 0.5`: shadow alpha = base_alpha * 0.5
//! - `progress = 1.0`: shadow alpha = base_alpha (full)
//!
//! ## Integration Patterns
//!
//! ### With the TUI-VFX Compositor
//!
//! Shadows render directly to the grid before compositor effects are applied:
//!
//! ```ignore
//! use tui_vfx::prelude::*;
//!
//! // 1. Clear or prepare your destination grid
//! dest.fill(Cell::default());
//!
//! // 2. Render shadow first
//! render_shadow(&mut dest, modal_rect, &shadow_config, animation_progress);
//!
//! // 3. Render your element (modal, popup, etc.)
//! render_modal(&mut dest, modal_rect, &theme);
//!
//! // 4. Apply compositor effects (wipe reveal, dissolve, etc.)
//! render_pipeline(&source, &mut dest, width, height, 0, 0, compositor_options, None);
//! ```
//!
//! ### With Theme Systems
//!
//! Create shadow configs from theme definitions:
//!
//! ```ignore
//! // Example theme-driven shadow configuration
//! fn shadow_from_theme(theme: &Theme) -> ShadowConfig {
//!     ShadowConfig::new(theme.shadow.color)
//!         .with_offset(theme.shadow.offset_x, theme.shadow.offset_y)
//!         .with_surface_color(theme.surface.background)
//!         .with_style(match theme.shadow.style {
//!             "braille" => ShadowStyle::braille(theme.shadow.density),
//!             "medium_shade" => ShadowStyle::MediumShade,
//!             "gradient" => ShadowStyle::gradient(theme.shadow.layers),
//!             "solid" => ShadowStyle::Solid,
//!             _ => ShadowStyle::HalfBlock,
//!         })
//! }
//! ```
//!
//! ### Synchronized with Modal Animations
//!
//! For modals with wipe animations, sync shadow progress with wipe progress:
//!
//! ```ignore
//! fn render_modal_with_shadow(
//!     grid: &mut impl Grid,
//!     modal_rect: Rect,
//!     shadow_config: &ShadowConfig,
//!     wipe_progress: f64,  // 0.0 = closed, 1.0 = fully open
//! ) {
//!     // Shadow fades in with the modal
//!     render_shadow(grid, modal_rect, shadow_config, wipe_progress);
//!
//!     // Modal content rendered separately with wipe mask...
//! }
//! ```
//!
//! ## Performance Considerations
//!
//! - **HalfBlock** and **Solid**: O(shadow_area) - linear in cells rendered
//! - **Braille**: Same as HalfBlock, plus character lookup
//! - **Gradient**: O(shadow_area × layers) - renders multiple overlapping layers
//!
//! For large shadows or performance-critical rendering:
//! 1. Use **Solid** style for maximum speed
//! 2. Limit **Gradient** layers to 2-3
//! 3. Consider caching shadow renders for static elements
//!
//! ## Shadow Geometry vs Compositing
//!
//! Shadow geometry (style, offset, edges) and shadow compositing mode are
//! independently configurable. Geometry controls where and how shadow cells
//! are rendered; compositing mode controls how those cells are applied onto
//! destination content. See [`ShadowCompositeMode`] for available modes.
//!
//! ## Module Structure
//!
//! - [`render_shadow`] - Main entry point for shadow rendering
//! - [`render_shadow_simple`] - Convenience function with defaults
//! - [`ShadowConfig`] - Configuration struct with builder pattern
//! - [`ShadowStyle`] - Enum of rendering styles
//! - [`ShadowCompositeMode`] - Compositing mode (glyph overlay vs grade underlying)
//! - [`ShadowGradeConfig`] - Color grading parameters for grade-underlying mode
//! - [`ShadowEdges`] - Bitflags for edge selection
//! - [`renderers`] - Direct access to individual renderer implementations

mod fnc_render_shadow;
pub mod renderers;
pub mod types;

// Re-export main functions
pub use fnc_render_shadow::{render_shadow, render_shadow_gradient_colors, render_shadow_simple};

// Re-export types at crate root for convenience
pub use types::{ShadowCompositeMode, ShadowConfig, ShadowEdges, ShadowGradeConfig, ShadowStyle};

// Re-export renderers for direct access if needed
pub use renderers::{
    BrailleRenderer, GradientRenderer, HalfBlockRenderer, MediumShadeRenderer, SolidRenderer,
};

// <FILE>crates/tui-vfx-shadow/src/lib.rs</FILE> - <DESC>Shadow rendering effects for TUI applications</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
