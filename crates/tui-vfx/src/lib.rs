// <FILE>crates/tui-vfx/src/lib.rs</FILE> - <DESC>Meta-crate re-exporting all tui-vfx components</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>Phase 2 dramatic color-shadow rollout: docs, examples, and quality closure</WCTX>
// <CLOG>Add ShadowCompositeMode and ShadowGradeConfig to prelude exports</CLOG>

//! # TUI VFX
//!
//! Cell-based visual effects for terminal UIs.
//!
//! `tui-vfx` is a framework-agnostic library for applying visual effects to
//! terminal cells. It provides masks, filters, samplers, shaders, shadows, and
//! content transformers that work with any terminal rendering framework through
//! the `Grid` trait.
//!
//! ## Architecture
//!
//! The library is organized into several sub-crates:
//!
//! | Crate | Purpose |
//! |-------|---------|
//! | [`types`] | Foundation types (Color, Style, Cell, Grid trait) |
//! | [`core`] | Schema and introspection primitives |
//! | [`geometry`] | Math, layout, and motion primitives |
//! | [`style`] | Color interpolation and style effects |
//! | [`content`] | Text manipulation primitives |
//! | [`shadow`] | Shadow rendering with multiple styles |
//! | [`compositor`] | Pipeline and compositing effects |
//!
//! ## Shadow Rendering
//!
//! Shadows can be rendered in two ways:
//!
//! 1. **Compositor integration** (recommended for animated elements) - Shadow
//!    wipes/dissolves in sync with the element
//! 2. **Direct rendering** (for static elements) - Simple, no extra allocation
//!
//! ## Quick Start: Compositor with Shadow
//!
//! For animated modals with wipe effects, use the compositor integration:
//!
//! ```rust,ignore
//! use tui_vfx::prelude::*;
//!
//! let modal_source = OwnedGrid::new(30, 12);  // Your modal content
//! let mut dest = OwnedGrid::new(80, 24);
//!
//! // Shadow config - specify offset and color
//! let shadow = ShadowSpec::new(
//!     ShadowConfig::new(Color::BLACK.with_alpha(150))
//!         .with_offset(2, 1)
//! );
//!
//! // Render with compositor - shadow wipes with the modal!
//! render_pipeline(
//!     &modal_source,
//!     &mut dest,
//!     30,     // Element width (NOT including shadow)
//!     12,     // Element height (NOT including shadow)
//!     10,     // offset_x
//!     5,      // offset_y
//!     CompositionOptions {
//!         t: 0.5,  // animation progress
//!         shadow: Some(shadow),
//!         masks: vec![MaskSpec::Wipe {
//!             reveal: Some(WipeDirection::TopToBottom),
//!             ..Default::default()
//!         }].into(),
//!         ..Default::default()
//!     },
//!     None,
//! );
//! // Note: Total rendered area is 32x13 (element + shadow offset)
//! ```
//!
//! ## Quick Start: Direct Shadow Rendering
//!
//! For static elements or simple fades:
//!
//! ```rust,ignore
//! use tui_vfx::prelude::*;
//!
//! let mut grid = OwnedGrid::new(80, 24);
//! let modal_rect = Rect::new(20, 5, 40, 14);
//!
//! // Configure shadow
//! let shadow_config = ShadowConfig::new(Color::BLACK.with_alpha(128))
//!     .with_offset(2, 1)
//!     .with_edges(ShadowEdges::BOTTOM_RIGHT);
//!
//! // Render shadow BEFORE element
//! render_shadow(&mut grid, modal_rect, &shadow_config, 1.0);
//! // ... render modal content ...
//! ```
//!
//! ## Framework Adapters
//!
//! This library is framework-agnostic. Implement the `Grid` trait for your
//! terminal framework's buffer type to use `tui-vfx`:
//!
//! ```rust,ignore
//! impl Grid for MyFrameworkBuffer {
//!     fn width(&self) -> usize { self.cols }
//!     fn height(&self) -> usize { self.rows }
//!     fn get(&self, x: usize, y: usize) -> Option<&Cell> { ... }
//!     fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> { ... }
//!     fn set(&mut self, x: usize, y: usize, cell: Cell) { ... }
//! }
//! ```
//!
//! ## Shadow Styles
//!
//! The [`shadow`] module provides four rendering styles:
//!
//! | Style | Description | Best For |
//! |-------|-------------|----------|
//! | `HalfBlock` | Sub-cell precision with ▐▄▌▀ | Default, most UIs |
//! | `Braille` | Dithered with ⣿ patterns | Variable density |
//! | `Solid` | Simple background fill | Maximum compatibility |
//! | `Gradient` | Multi-layer falloff | Soft drop shadows |
//!
//! See [`shadow`] module documentation for detailed usage.
//!
//! ## Prelude
//!
//! The [`prelude`] module re-exports the most commonly used types:
//!
//! ```rust,ignore
//! use tui_vfx::prelude::*;
//!
//! // Now you have access to:
//! // - Grid types: OwnedGrid, Cell, Color, Rect, etc.
//! // - Compositor: render_pipeline, CompositionOptions, MaskSpec, etc.
//! // - Shadow: render_shadow, ShadowConfig, ShadowStyle, ShadowEdges
//! // - Style: ColorConfig, FadeEffect, BlendMode, etc.
//! // - Content: text transformers
//! ```

// Re-export all sub-crates
pub use tui_vfx_compositor as compositor;
pub use tui_vfx_content as content;
pub use tui_vfx_core as core;
pub use tui_vfx_geometry as geometry;
pub use tui_vfx_shadow as shadow;
pub use tui_vfx_style as style;
pub use tui_vfx_types as types;

/// Prelude module for convenient imports
pub mod prelude {
    // Types - foundation
    pub use tui_vfx_types::{
        Anchor, BoundaryMode, Cell, Color, Grid, GridExt, Modifiers, OwnedGrid, Point, Rect, Size,
        Style,
    };

    // Core - schema
    pub use tui_vfx_core::{ConfigSchema, FieldMeta, Range, ScalarValue, SchemaField, SchemaNode};

    // Geometry - easing and motion
    pub use tui_vfx_geometry::easing::EasingType;

    // Compositor - Pipeline API
    pub use tui_vfx_compositor::pipeline::{
        CompositionOptions, CompositionSpec, ShaderLayerSpec, ShaderWithRegion, ShadowSpec,
        render_pipeline, render_pipeline_with_spec, render_pipeline_with_spec_area,
    };

    // Compositor - Effect specs
    pub use tui_vfx_compositor::types::{
        ApplyTo, Axis, DitherMatrix, FilterSpec, HoverBarPosition, IrisShape, MaskCombineMode,
        MaskSpec, Orientation, RippleCenter, SamplerSpec, WipeDirection,
    };

    // Style - models and effects
    pub use tui_vfx_style::models::{
        BlendMode, ColorConfig, ColorRamp, ColorSpace, FadeDirection, FadeEffect, FadeSpec,
        Gradient, StyleConfig, StyleEffect, StyleLayer, StyleTransition,
    };

    // Content - text transformers
    pub use tui_vfx_content::prelude::*;

    // Shadow - shadow rendering
    pub use tui_vfx_shadow::{
        ShadowCompositeMode, ShadowConfig, ShadowEdges, ShadowGradeConfig, ShadowStyle,
        render_shadow, render_shadow_simple,
    };
}

// <FILE>crates/tui-vfx/src/lib.rs</FILE> - <DESC>Meta-crate re-exporting all tui-vfx components</DESC>
// <VERS>END OF VERSION: 0.6.0</VERS>
