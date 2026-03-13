// <FILE>crates/tui-vfx-types/src/lib.rs</FILE> - <DESC>Foundation types for mixed-animations ecosystem</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Color-inert glyph detection for shadow grading replacement</WCTX>
// <CLOG>Add color_inert module with is_color_inert_glyph detection utility</CLOG>

//! # mixed-types
//!
//! Framework-agnostic foundation types for the mixed-animations ecosystem.
//!
//! This crate provides the core type definitions that are shared across
//! the animation pipeline, independent of any specific TUI framework.
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
pub mod color_inert;
mod color;
mod geometry;
mod grid;
mod modifiers;
pub mod rigid_shake_timing;
mod style;

pub use cell::Cell;
pub use color::Color;
pub use geometry::{Anchor, Point, Rect, Size};
pub use grid::{BoundaryMode, Grid, GridExt, OwnedGrid};
pub use modifiers::Modifiers;
pub use rigid_shake_timing::{RigidShakeState, RigidShakeTiming};
pub use style::Style;

// <FILE>crates/tui-vfx-types/src/lib.rs</FILE> - <DESC>Foundation types for mixed-animations ecosystem</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
