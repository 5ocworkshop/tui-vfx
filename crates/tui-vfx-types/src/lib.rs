// <FILE>crates/tui-vfx-types/src/lib.rs</FILE> - <DESC>Foundation types for mixed-animations ecosystem</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>Glyph rendering framework Phase 3: add glyph module with GlyphEncoder and subcell sampling helpers</WCTX>
// <CLOG>0.5.0: add pub mod glyph exposing GlyphEncoder, sample_eight_subcells, sample_eight_subcells_with_slope, and SUBCELL_OFFSETS for Phase 3 of the glyph rendering framework.</CLOG>

//! # tui-vfx-types
//!
//! Framework-agnostic foundation types for the tui-vfx ecosystem.
//!
//! This crate provides the core type definitions that are shared across
//! the animation pipeline, independent of any specific TUI framework.
//!
//! ## Role-tagging and `SemanticScene`
//!
//! Since v0.4.0, `tui-vfx-types` owns the foundation primitives that
//! underpin the unified recipe scene composer (see the spec at
//! `docs/superpowers/specs/2026-04-20-recipe-scene-composer-design.md`
//! in the gt-design repo):
//!
//! - [`SemanticScene`] — a source surface (grid + per-cell role tags)
//!   that is equally produced by widget renders and by recipe-driven
//!   scene composers. Every per-cell pipeline stage (sampler, mask,
//!   shader, filter, shadow) runs against it identically.
//! - [`RoleMap`] — dense per-cell `RoleTag` storage backing a
//!   `SemanticScene`.
//! - [`RoleTag`] — 12 first-class semantic roles (Background, Text,
//!   Title, Caption, Border, Image, Icon, Indicator, Highlight, Shadow,
//!   Decoration, Procedural) plus `Custom(InternedRoleName)` for ad-hoc
//!   recipe-declared roles.
//! - [`RoleInterner`] / [`RoleId`] — compact numeric IDs with stable
//!   assignment (first-class 0–11, Custom starts at 12).
//! - [`LayerId`] / [`RecipeId`] — opaque interned newtypes consumed by
//!   trace selectors / inspection sinks without forcing downstream
//!   inspection code to depend on the recipe crate.
//! - [`InternedString`] — cheap-to-clone `Arc<str>` wrapper backing the
//!   opaque identifiers.
//!
//! ## Types
//!
//! ### Geometry
//! - [`Rect`] - Rectangle with position and dimensions
//! - [`Point`] - 2D coordinate point
//! - [`Size`] - Width and height dimensions
//!
//! ### Styling
//! - [`Color`] - RGBA color with alpha compositing
//! - [`Modifiers`] - Text modifiers (bold, italic, underline, etc.)
//! - [`Style`] - Combined foreground, background, and modifiers
//!
//! ### Display
//! - [`Cell`] - Single character with styling (char + style)
//! - [`Grid`] - Trait for 2D cell grids
//! - [`OwnedGrid`] - Simple owned grid implementation
//!
//! ### Braille
//! - [`braille`] - Utilities for Unicode braille patterns (U+2800–U+28FF)
//!
//! ### Color-Inert Glyph Detection
//! - [`color_inert`] - Detection for glyphs that ignore ANSI fg color (emoji, PUA/nerd fonts)
//!
//! ### Animation Timing
//! - [`rigid_shake_timing`] - Shared timing for RigidShake filter and style effects
//!
//! ## Design
//!
//! These types are designed to be:
//! - **Framework-agnostic**: No dependencies on ratatui, crossterm, etc.
//! - **Lightweight**: Minimal dependencies, fast compilation
//! - **Composable**: Types work together naturally
//! - **Serializable**: Optional serde support

pub mod braille;
mod cell;
mod color;
pub mod color_inert;
mod geometry;
pub mod glyph;
mod grid;
mod interned_string;
mod layer_id;
mod modifiers;
mod recipe_id;
pub mod rigid_shake_timing;
mod role_id;
mod role_interner;
mod role_map;
mod role_tag;
mod scene_metadata;
mod semantic_scene;
mod style;

pub use cell::Cell;
pub use color::Color;
pub use geometry::{Anchor, Point, Rect, Size};
pub use grid::{BoundaryMode, Grid, GridExt, OwnedGrid};
pub use interned_string::InternedString;
pub use layer_id::LayerId;
pub use modifiers::Modifiers;
pub use recipe_id::RecipeId;
pub use rigid_shake_timing::{RigidShakeState, RigidShakeTiming};
pub use role_id::RoleId;
pub use role_interner::RoleInterner;
pub use role_map::{RoleMap, RoleMapIter};
pub use role_tag::{InternedRoleName, RoleTag};
pub use scene_metadata::SceneMetadata;
pub use semantic_scene::SemanticScene;
pub use style::Style;

// <FILE>crates/tui-vfx-types/src/lib.rs</FILE> - <DESC>Foundation types for mixed-animations ecosystem</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
