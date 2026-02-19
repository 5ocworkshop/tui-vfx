// <FILE>tui-vfx-compositor/src/types/cls_mask_spec.rs</FILE> - <DESC>MaskSpec enum with full parameters</DESC>
// <VERS>VERSION: 2.3.1</VERS>
// <WCTX>Fix doctest imports for mask example</WCTX>
// <CLOG>Add missing WipeDirection and IrisShape imports to docs</CLOG>

//! # Mask Specifications
//!
//! Masks control the visibility of content during transitions. All masks operate
//! on a progress value `t` in the range `0.0..=1.0`, where:
//!
//! - `t = 0.0`: Content is fully hidden (or fully visible for "hide" masks)
//! - `t = 1.0`: Content is fully visible (or fully hidden for "hide" masks)
//!
//! ## Available Masks
//!
//! | Mask | Description | Best For |
//! |------|-------------|----------|
//! | [`MaskSpec::Wipe`] | Linear edge reveal | Classic film transitions |
//! | [`MaskSpec::Dissolve`] | Random pixel fade | Dream sequences, magical effects |
//! | [`MaskSpec::Iris`] | Spotlight from center | Dramatic reveals |
//! | [`MaskSpec::Radial`] | Circular expansion | Impact effects |
//! | [`MaskSpec::Blinds`] | Venetian blinds | Presentation style |
//! | [`MaskSpec::Checkers`] | Checkerboard pattern | Retro/game aesthetic |
//! | [`MaskSpec::Cellular`] | Organic patterns | Creative/artistic transitions |
//!
//! ## Soft Edges
//!
//! Many masks support a `soft_edge` parameter that adds gradient blending at
//! the transition boundary. This creates smoother, more polished transitions
//! compared to hard pixel boundaries.
//!
//! ## Example Usage
//!
//! ```rust
//! use tui_vfx_compositor::types::{IrisShape, MaskSpec, WipeDirection};
//!
//! // Classic left-to-right wipe reveal
//! let wipe = MaskSpec::Wipe {
//!     reveal: Some(WipeDirection::LeftToRight),
//!     hide: None,
//!     direction: None,
//!     soft_edge: true,
//! };
//!
//! // Dramatic iris reveal
//! let iris = MaskSpec::Iris {
//!     shape: IrisShape::Circle,
//!     soft_edge: true,
//! };
//! ```

use crate::masks::cls_cellular::CellularPattern;
use crate::masks::cls_path_reveal::RevealPathType;
use crate::masks::cls_radial::RadialOrigin;
use serde::{Deserialize, Serialize};

/// Direction for wipe mask transitions.
///
/// Wipe directions define how content is revealed or hidden across the screen.
/// Multiple naming conventions are supported for flexibility:
///
/// ## Cardinal Directions (4)
/// - [`LeftToRight`](Self::LeftToRight), [`RightToLeft`](Self::RightToLeft)
/// - [`TopToBottom`](Self::TopToBottom), [`BottomToTop`](Self::BottomToTop)
///
/// ## Diagonal Directions (4)
/// - [`TopLeftToBottomRight`](Self::TopLeftToBottomRight), [`TopRightToBottomLeft`](Self::TopRightToBottomLeft)
/// - [`BottomLeftToTopRight`](Self::BottomLeftToTopRight), [`BottomRightToTopLeft`](Self::BottomRightToTopLeft)
///
/// ## Barn Door / Curtain Effects (4)
/// - [`HorizontalCenterOut`](Self::HorizontalCenterOut) — Curtains opening horizontally
/// - [`VerticalCenterOut`](Self::VerticalCenterOut) — Curtains opening vertically
/// - [`HorizontalEdgesIn`](Self::HorizontalEdgesIn) — Curtains closing horizontally
/// - [`VerticalEdgesIn`](Self::VerticalEdgesIn) — Curtains closing vertically
///
/// ## Source-Based Aliases (4)
/// Convenience aliases that read naturally: `FromLeft`, `FromRight`, `FromTop`, `FromBottom`
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum WipeDirection {
    // ─────────────────────────────────────────────────────────────────
    // Cardinal directions
    // ─────────────────────────────────────────────────────────────────
    /// Wipe from left edge to right edge.
    ///
    /// The most common wipe direction, matching natural reading order
    /// in left-to-right languages.
    #[default]
    LeftToRight,

    /// Wipe from right edge to left edge.
    RightToLeft,

    /// Wipe from top edge to bottom edge.
    TopToBottom,

    /// Wipe from bottom edge to top edge.
    BottomToTop,

    // ─────────────────────────────────────────────────────────────────
    // Diagonal directions
    // ─────────────────────────────────────────────────────────────────
    /// Diagonal wipe from top-left corner to bottom-right corner.
    ///
    /// Creates a dynamic, angular reveal effect.
    TopLeftToBottomRight,

    /// Diagonal wipe from top-right corner to bottom-left corner.
    TopRightToBottomLeft,

    /// Diagonal wipe from bottom-left corner to top-right corner.
    BottomLeftToTopRight,

    /// Diagonal wipe from bottom-right corner to top-left corner.
    BottomRightToTopLeft,

    // ─────────────────────────────────────────────────────────────────
    // Source-based aliases (convenience)
    // ─────────────────────────────────────────────────────────────────
    /// Alias for [`LeftToRight`](Self::LeftToRight).
    ///
    /// Reads naturally: "wipe from left"
    #[serde(alias = "FromLeft")]
    FromLeft,

    /// Alias for [`RightToLeft`](Self::RightToLeft).
    #[serde(alias = "FromRight")]
    FromRight,

    /// Alias for [`TopToBottom`](Self::TopToBottom).
    #[serde(alias = "FromTop")]
    FromTop,

    /// Alias for [`BottomToTop`](Self::BottomToTop).
    #[serde(alias = "FromBottom")]
    FromBottom,

    // ─────────────────────────────────────────────────────────────────
    // Center-out / Barn door effects
    // ─────────────────────────────────────────────────────────────────
    /// Horizontal wipe from center outward to left and right edges.
    ///
    /// Creates a "curtains opening" effect. Great for dramatic reveals.
    #[serde(alias = "center_out_horizontal", alias = "barn_door_horizontal")]
    HorizontalCenterOut,

    /// Vertical wipe from center outward to top and bottom edges.
    ///
    /// Vertical variant of the curtain opening effect.
    #[serde(alias = "center_out_vertical", alias = "barn_door_vertical")]
    VerticalCenterOut,

    /// Horizontal wipe from edges inward to center.
    ///
    /// Creates a "curtains closing" effect.
    #[serde(alias = "edges_in_horizontal", alias = "barn_door_close_horizontal")]
    HorizontalEdgesIn,

    /// Vertical wipe from edges inward to center.
    #[serde(alias = "edges_in_vertical", alias = "barn_door_close_vertical")]
    VerticalEdgesIn,
}

/// Orientation for blinds and similar directional masks.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum Orientation {
    /// Horizontal blinds (slats run left-right, open top-to-bottom).
    #[default]
    Horizontal,

    /// Vertical blinds (slats run top-bottom, open left-to-right).
    Vertical,
}

/// Shape for iris/spotlight masks.
///
/// The iris shape determines the geometry of the expanding/contracting
/// reveal area centered on the screen.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum IrisShape {
    /// Circular iris (classic spotlight effect).
    ///
    /// Creates a smooth, organic reveal from the center.
    #[default]
    Circle,

    /// Diamond-shaped iris (rotated square).
    ///
    /// Creates a more geometric, angular reveal.
    Diamond,

    /// Rectangular/box-shaped iris.
    ///
    /// Useful for UI elements that are rectangular in nature.
    Box,
}

/// Dither matrix size for noise dither masks.
///
/// Larger matrices produce more gradual dithering patterns
/// but require more computation.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum DitherMatrix {
    /// 4×4 Bayer dithering matrix.
    ///
    /// Faster computation, more visible pattern. Good for retro aesthetics.
    #[default]
    Bayer4,

    /// 8×8 Bayer dithering matrix.
    ///
    /// Smoother gradients, less visible pattern. Better for subtle transitions.
    Bayer8,
}

/// Complete mask specification with all parameters.
///
/// This enum provides full configuration for each mask type,
/// allowing masks to be serialized/deserialized and customized.
///
/// # Schema Design Philosophy
///
/// Field names are chosen to read as plain English. For example, wipe masks use
/// `reveal` and `hide` fields rather than a generic `direction` + `invert` because:
///
/// ```json
/// { "type": "wipe", "reveal": "left_to_right" }  // "wipe that reveals left to right"
/// { "type": "wipe", "hide": "left_to_right" }    // "wipe that hides left to right"
/// ```
///
/// This self-documenting approach makes themes readable without technical knowledge.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[derive(Default)]
pub enum MaskSpec {
    /// No mask applied — content is fully visible.
    ///
    /// Use this when you want to disable masking without removing the mask
    /// configuration entirely.
    #[default]
    None,

    /// Linear wipe reveal/hide from one edge to another.
    ///
    /// The classic film-style transition. Content is progressively revealed
    /// or hidden as a line sweeps across the screen.
    ///
    /// # Parameters
    ///
    /// Provide `reveal` to specify direction where content appears, or `hide` to specify
    /// direction where content disappears. If both are set, `hide` takes priority.
    /// If neither is set, defaults to reveal left-to-right.
    ///
    /// - `reveal`: Direction content appears (e.g., `left_to_right`)
    /// - `hide`: Direction content disappears (takes priority over `reveal`)
    /// - `soft_edge`: Enable gradient blending at the wipe boundary
    ///
    /// # Direction Variants
    ///
    /// - **Cardinal**: `left_to_right`, `right_to_left`, `top_to_bottom`, `bottom_to_top`
    /// - **Diagonal**: `top_left_to_bottom_right`, `top_right_to_bottom_left`, etc.
    /// - **Barn Door**: `horizontal_center_out`, `vertical_center_out` (curtains opening)
    ///
    /// # Examples
    ///
    /// ```json
    /// { "type": "wipe", "reveal": "left_to_right" }
    /// { "type": "wipe", "hide": "left_to_right", "soft_edge": true }
    /// { "type": "wipe", "reveal": "horizontal_center_out" }
    /// ```
    Wipe {
        /// Direction for reveal — content appears traveling in this direction.
        ///
        /// Mutually exclusive with `hide`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reveal: Option<WipeDirection>,

        /// Direction for hide — content disappears traveling in this direction.
        ///
        /// Mutually exclusive with `reveal`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        hide: Option<WipeDirection>,

        /// Legacy field — alias for `reveal`. Prefer using `reveal` for clarity.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        direction: Option<WipeDirection>,

        /// Enable soft edge blending at the wipe boundary.
        ///
        /// When `true`, adds a gradient at the transition edge for a smoother look.
        /// Recommended value: use `true` for polished transitions.
        #[serde(default)]
        soft_edge: bool,
    },

    /// Random pixel dissolve effect.
    ///
    /// Pixels are randomly revealed/hidden, creating a film dissolve or
    /// "materializing" effect. Great for dream sequences, magical effects,
    /// or retro aesthetics.
    ///
    /// # Parameters
    ///
    /// - `seed`: Random seed for reproducible patterns
    /// - `chunk_size`: Size of dissolve units (1 = fine grain, 4+ = blocky/retro)
    ///
    /// # Tips
    ///
    /// - `chunk_size: 1-2` — Subtle shimmer, elegant transitions
    /// - `chunk_size: 4-8` — Retro/pixelated feel, visible blocks
    Dissolve {
        /// Random seed for deterministic, reproducible dissolve patterns.
        seed: u64,

        /// Size of dissolve chunks in cells.
        ///
        /// - `1`: Single-pixel dissolve (finest grain)
        /// - `2-3`: Subtle chunking
        /// - `4+`: Visible blocks (retro/pixelated aesthetic)
        chunk_size: u8,
    },

    /// Checkerboard pattern reveal.
    ///
    /// Content is revealed in alternating checker cells, creating a
    /// classic retro TV or game aesthetic.
    ///
    /// # Parameters
    ///
    /// - `cell_size`: Size of each checker cell in terminal cells
    Checkers {
        /// Size of each checker cell in terminal cells.
        ///
        /// Larger values create bigger, more visible checkers.
        cell_size: u16,
    },

    /// Venetian blinds effect.
    ///
    /// Multiple horizontal or vertical slats open progressively,
    /// creating a window blinds effect. Professional presentation style.
    ///
    /// # Parameters
    ///
    /// - `orientation`: Horizontal (slats run left-right) or Vertical
    /// - `count`: Number of blinds/slats
    Blinds {
        /// Orientation of the blinds.
        ///
        /// - `Horizontal`: Slats run left-right, open top-to-bottom
        /// - `Vertical`: Slats run top-bottom, open left-to-right
        orientation: Orientation,

        /// Number of blind slats.
        ///
        /// More blinds = thinner slats, faster-feeling transition.
        count: u16,
    },

    /// Iris/spotlight reveal from center.
    ///
    /// A shape expands from (or contracts to) the center of the screen,
    /// like a camera iris or spotlight. Classic dramatic reveal effect.
    ///
    /// # Parameters
    ///
    /// - `shape`: Geometry of the iris (Circle, Diamond, Box)
    /// - `soft_edge`: Enable gradient blending at the iris boundary
    ///
    /// # Shapes
    ///
    /// - `Circle`: Smooth, organic spotlight (most common)
    /// - `Diamond`: Angular, geometric feel
    /// - `Box`: Rectangular reveal, good for UI elements
    Iris {
        /// Shape of the iris aperture.
        shape: IrisShape,

        /// Enable soft edge blending at the iris boundary.
        soft_edge: bool,
    },

    /// Diamond-shaped expand from center.
    ///
    /// A diamond shape expands outward from the center. Similar to
    /// [`Iris`](Self::Iris) with `Diamond` shape but optimized for
    /// this specific pattern.
    Diamond {
        /// Enable soft edge blending at the diamond boundary.
        soft_edge: bool,
    },

    /// Dithered noise pattern reveal.
    ///
    /// Uses Bayer dithering matrices to create ordered noise patterns.
    /// Creates a subtle, technical feel compared to random dissolve.
    ///
    /// # Parameters
    ///
    /// - `seed`: Random seed for pattern variation
    /// - `matrix`: Dither matrix size (Bayer4 or Bayer8)
    ///
    /// # Matrix Selection
    ///
    /// - `Bayer4`: 4×4 matrix, more visible pattern, retro aesthetic
    /// - `Bayer8`: 8×8 matrix, smoother gradients, subtler effect
    NoiseDither {
        /// Random seed for pattern variation.
        seed: u64,

        /// Dither matrix size.
        ///
        /// Larger matrices produce smoother but less distinctive patterns.
        matrix: DitherMatrix,
    },

    /// Path-based reveal (spiral, radial sweep, etc.).
    ///
    /// Content is revealed following a geometric path pattern.
    /// Creates dynamic, animated-feeling transitions.
    ///
    /// # Parameters
    ///
    /// - `path`: The path pattern (Spiral, Radial sweep, etc.)
    /// - `soft_edge`: Enable gradient blending
    ///
    /// # Path Types
    ///
    /// - `Spiral`: Outward spiral from center (playful, dynamic)
    /// - `RadialSweep`: Clock-hand sweep (scanning, loading feel)
    PathReveal {
        /// The path pattern defining the reveal animation.
        path: RevealPathType,

        /// Enable soft edge blending along the path.
        soft_edge: bool,
    },

    /// Radial reveal expanding from a configurable origin.
    ///
    /// A circle expands outward from a specified origin point.
    /// More flexible than Iris since the origin can be any point.
    ///
    /// # Parameters
    ///
    /// - `origin`: Center point for the radial expansion
    /// - `soft_edge`: Enable gradient blending
    ///
    /// # Origin Options
    ///
    /// - `Center`: Screen center (default)
    /// - `TopLeft`, `TopRight`, `BottomLeft`, `BottomRight`: Corners
    /// - `Custom { x, y }`: Arbitrary normalized coordinates (0.0-1.0)
    Radial {
        /// Origin point for the radial expansion.
        origin: RadialOrigin,

        /// Enable soft edge blending at the radial boundary.
        #[serde(default)]
        soft_edge: bool,
    },

    /// Cellular/organic pattern reveal.
    ///
    /// Creates organic, natural-looking transition patterns using
    /// cellular algorithms. Great for creative, artistic transitions.
    ///
    /// # Parameters
    ///
    /// - `pattern`: The cellular pattern type
    /// - `seed`: Random seed for reproducibility
    /// - `cell_count`: Number of cells (affects granularity)
    ///
    /// # Pattern Types
    ///
    /// - `Voronoi`: Organic, natural cell boundaries
    /// - `Hexagonal`: Regular hexagonal grid (tech/futuristic)
    /// - `Organic`: Flowing, amoeba-like shapes
    Cellular {
        /// The cellular pattern algorithm.
        pattern: CellularPattern,

        /// Random seed for reproducible patterns.
        seed: u64,

        /// Number of cells in the pattern.
        ///
        /// Higher values = smaller cells, finer granularity.
        /// Default: 16
        #[serde(default = "default_cell_count")]
        cell_count: u16,
    },
}

fn default_cell_count() -> u16 {
    16
}

/// Resolved wipe configuration with direction and invert flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedWipe {
    /// The wipe direction
    pub direction: WipeDirection,
    /// Whether to invert mask values (true for hide, false for reveal)
    pub invert: bool,
}

impl MaskSpec {
    /// For Wipe masks, resolve the reveal/hide/direction fields to a canonical form.
    ///
    /// Returns `Some(ResolvedWipe)` for Wipe masks, `None` for other mask types.
    ///
    /// # Resolution Rules
    ///
    /// - `reveal` field → direction with `invert: false`
    /// - `hide` field → direction with `invert: true`
    /// - `direction` field → treated as reveal (backwards compatibility)
    /// - If multiple fields set, priority: `hide` > `reveal` > `direction`
    /// - If no fields set, defaults to `LeftToRight` reveal
    pub fn resolve_wipe(&self) -> Option<ResolvedWipe> {
        match self {
            MaskSpec::Wipe {
                reveal,
                hide,
                direction,
                ..
            } => {
                // Priority: hide > reveal > direction > default
                if let Some(dir) = hide {
                    Some(ResolvedWipe {
                        direction: *dir,
                        invert: true,
                    })
                } else if let Some(dir) = reveal {
                    Some(ResolvedWipe {
                        direction: *dir,
                        invert: false,
                    })
                } else if let Some(dir) = direction {
                    Some(ResolvedWipe {
                        direction: *dir,
                        invert: false,
                    })
                } else {
                    // Default: reveal left to right
                    Some(ResolvedWipe {
                        direction: WipeDirection::default(),
                        invert: false,
                    })
                }
            }
            _ => None,
        }
    }

    /// Check if this mask should invert its values (i.e., is a "hide" mask).
    ///
    /// Returns `true` for Wipe masks with `hide` set, `false` otherwise.
    pub fn should_invert(&self) -> bool {
        self.resolve_wipe().map(|r| r.invert).unwrap_or(false)
    }

    /// Returns the mask type name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            MaskSpec::None => "None",
            MaskSpec::Wipe { .. } => "Wipe",
            MaskSpec::Dissolve { .. } => "Dissolve",
            MaskSpec::Checkers { .. } => "Checkers",
            MaskSpec::Blinds { .. } => "Blinds",
            MaskSpec::Iris { .. } => "Iris",
            MaskSpec::Diamond { .. } => "Diamond",
            MaskSpec::NoiseDither { .. } => "NoiseDither",
            MaskSpec::PathReveal { .. } => "PathReveal",
            MaskSpec::Radial { .. } => "Radial",
            MaskSpec::Cellular { .. } => "Cellular",
        }
    }

    /// Returns a brief human-readable description of what this mask does.
    pub fn terse_description(&self) -> &'static str {
        match self {
            MaskSpec::None => "No mask applied — content is fully visible",
            MaskSpec::Wipe { .. } => "Linear wipe reveal/hide from one edge to another",
            MaskSpec::Dissolve { .. } => "Random pixel dissolve effect",
            MaskSpec::Checkers { .. } => "Checkerboard pattern reveal",
            MaskSpec::Blinds { .. } => "Venetian blinds effect",
            MaskSpec::Iris { .. } => "Iris/spotlight reveal from center",
            MaskSpec::Diamond { .. } => "Diamond-shaped expand from center",
            MaskSpec::NoiseDither { .. } => "Dithered noise pattern reveal",
            MaskSpec::PathReveal { .. } => "Path-based reveal (spiral, radial sweep)",
            MaskSpec::Radial { .. } => "Radial reveal expanding from configurable origin",
            MaskSpec::Cellular { .. } => "Cellular/organic pattern reveal",
        }
    }

    /// Returns key parameters of this mask for documentation purposes.
    pub fn key_parameters(&self) -> Vec<(&'static str, String)> {
        match self {
            MaskSpec::None => vec![],
            MaskSpec::Wipe {
                reveal,
                hide,
                soft_edge,
                ..
            } => {
                let mut params = vec![];
                if let Some(dir) = reveal {
                    params.push(("reveal", format!("{:?}", dir)));
                }
                if let Some(dir) = hide {
                    params.push(("hide", format!("{:?}", dir)));
                }
                params.push(("soft_edge", format!("{}", soft_edge)));
                params
            }
            MaskSpec::Dissolve { seed, chunk_size } => vec![
                ("seed", format!("{}", seed)),
                ("chunk_size", format!("{}", chunk_size)),
            ],
            MaskSpec::Checkers { cell_size } => {
                vec![("cell_size", format!("{}", cell_size))]
            }
            MaskSpec::Blinds { orientation, count } => vec![
                ("orientation", format!("{:?}", orientation)),
                ("count", format!("{}", count)),
            ],
            MaskSpec::Iris { shape, soft_edge } => vec![
                ("shape", format!("{:?}", shape)),
                ("soft_edge", format!("{}", soft_edge)),
            ],
            MaskSpec::Diamond { soft_edge } => {
                vec![("soft_edge", format!("{}", soft_edge))]
            }
            MaskSpec::NoiseDither { seed, matrix } => vec![
                ("seed", format!("{}", seed)),
                ("matrix", format!("{:?}", matrix)),
            ],
            MaskSpec::PathReveal { path, soft_edge } => vec![
                ("path", format!("{:?}", path)),
                ("soft_edge", format!("{}", soft_edge)),
            ],
            MaskSpec::Radial { origin, soft_edge } => vec![
                ("origin", format!("{:?}", origin)),
                ("soft_edge", format!("{}", soft_edge)),
            ],
            MaskSpec::Cellular {
                pattern,
                seed,
                cell_count,
            } => vec![
                ("pattern", format!("{:?}", pattern)),
                ("seed", format!("{}", seed)),
                ("cell_count", format!("{}", cell_count)),
            ],
        }
    }
}

// <FILE>tui-vfx-compositor/src/types/cls_mask_spec.rs</FILE> - <DESC>MaskSpec enum with full parameters</DESC>
// <VERS>END OF VERSION: 2.3.1</VERS>
