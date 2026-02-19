// <FILE>tui-vfx-compositor/src/types/cls_filter_spec.rs</FILE> - <DESC>FilterSpec enum with signal-driven parameters</DESC>
// <VERS>VERSION: 3.4.0</VERS>
// <WCTX>Add boost_separator_bg for continuous powerline backgrounds</WCTX>
// <CLOG>Add boost_separator_bg to KittScanner and GlistenSweep for powerlines with non-terminal backgrounds</CLOG>

//! # Filter Specifications
//!
//! Filters apply post-processing effects to rendered output. They modify colors,
//! add visual textures, and create animated feedback effects.
//!
//! ## Filter Categories
//!
//! ### Basic Adjustments
//! | Filter | Description |
//! |--------|-------------|
//! | [`FilterSpec::Dim`] | Darken output (0.0 = black, 1.0 = unchanged) |
//! | [`FilterSpec::Invert`] | Color inversion |
//! | [`FilterSpec::Tint`] | Apply color overlay |
//! | [`FilterSpec::Greyscale`] | Desaturate using BT.601 luminance |
//!
//! ### Ambient Textures
//! | Filter | Description |
//! |--------|-------------|
//! | [`FilterSpec::Vignette`] | Edge darkening for focus |
//! | [`FilterSpec::PatternFill`] | Background texture patterns |
//! | [`FilterSpec::BrailleDust`] | Animated braille particle dust |
//!
//! ### Retro/CRT Effects
//! | Filter | Description |
//! |--------|-------------|
//! | [`FilterSpec::Crt`] | CRT monitor scanlines and glow |
//! | [`FilterSpec::InterlaceCurtain`] | Scanline/interlace dimming |
//! | [`FilterSpec::MotionBlur`] | Directional blur trail |
//!
//! ### Hover/Focus Indicators
//! | Filter | Description |
//! |--------|-------------|
//! | [`FilterSpec::HoverBar`] | Progress-driven partial bar indicator |
//! | [`FilterSpec::UnderlineWipe`] | Horizontal underline wipe-in |
//! | [`FilterSpec::BracketEmphasis`] | Fade-in brackets around content |
//! | [`FilterSpec::DotIndicator`] | Simple dot/bullet marker |
//!
//! ### Tactile Feedback
//! | Filter | Description |
//! |--------|-------------|
//! | [`FilterSpec::SubCellShake`] | Edge vibration (error/rejection) |
//! | [`FilterSpec::RigidShake`] | Ketchup bottle damped shake |
//!
//! ## Signal-Driven Parameters
//!
//! Many filters use [`SignalOrFloat`] for parameters, allowing them to be
//! driven by animation signals or set to static values:
//!
//! ```json
//! // Static value
//! { "type": "dim", "factor": 0.5 }
//!
//! // Signal-driven (animated)
//! { "type": "dim", "factor": { "signal": "hover_progress" } }
//! ```
//!
//! ## The `apply_to` Pattern
//!
//! Filters that modify colors often have an `apply_to` field:
//! - `foreground` — Only affect text color
//! - `background` — Only affect cell background
//! - `both` — Affect both (default)

use super::cls_hover_bar_position::HoverBarPosition;
use super::cls_mask_spec::WipeDirection;
use mixed_signals::types::SignalOrFloat;
use serde::{Deserialize, Serialize};
use tui_vfx_style::models::ColorConfig;

/// Pattern types for filling cells.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PatternType {
    /// Single repeating character (e.g., '~' for water, '.' for sand)
    Single {
        /// The character to repeat
        char: char,
    },

    /// Checkerboard pattern alternating between two characters
    Checkerboard {
        /// Character for (x+y) % 2 == 0 positions
        char_a: char,
        /// Character for (x+y) % 2 == 1 positions
        char_b: char,
    },

    /// Horizontal line pattern (rows at regular intervals)
    HorizontalLines {
        /// Character for the lines
        line_char: char,
        /// Spacing between lines (line appears every N rows)
        spacing: u16,
    },

    /// Vertical line pattern (columns at regular intervals)
    VerticalLines {
        /// Character for the lines
        line_char: char,
        /// Spacing between lines (line appears every N columns)
        spacing: u16,
    },
}

impl Default for PatternType {
    fn default() -> Self {
        PatternType::Single { char: '.' }
    }
}

/// Braille dot pattern complexity for BrailleDust filter.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum BraillePatternType {
    /// Single dots only (⠁ ⠂ ⠄) - most subtle
    #[default]
    SingleDot,
    /// 1-2 vertical dots - subtle
    OneToTwoDots,
    /// 1-3 vertical dots - moderate
    OneToThreeDots,
    /// 1-4 dots using both columns - more visible
    OneToFourDots,
}

/// Target for filter effects - which color component to affect.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ApplyTo {
    /// Apply to foreground color only
    #[serde(alias = "fg")]
    #[serde(alias = "Fg")]
    Foreground,
    /// Apply to background color only
    #[serde(alias = "bg")]
    #[serde(alias = "Bg")]
    Background,
    /// Apply to both foreground and background
    #[serde(alias = "Both")]
    #[default]
    Both,
}

/// Complete filter specification with all parameters.
///
/// Filters are post-processing effects applied after content is rendered.
/// They modify colors, add visual textures, and provide animated feedback.
///
/// # Categories
///
/// - **Basic Adjustments**: Dim, Invert, Tint, Greyscale
/// - **Ambient Textures**: Vignette, PatternFill, BrailleDust
/// - **Retro/CRT**: Crt, InterlaceCurtain, MotionBlur
/// - **Hover Indicators**: HoverBar, UnderlineWipe, BracketEmphasis, DotIndicator
/// - **Tactile Feedback**: SubCellShake, RigidShake
///
/// # Signal-Driven Parameters
///
/// Many parameters use `SignalOrFloat` allowing static values or animation signals:
/// ```json
/// { "type": "dim", "factor": 0.5 }                    // Static
/// { "type": "dim", "factor": { "signal": "t" } }      // Animated
/// ```
///
/// # Filter Stacking
///
/// Multiple filters can be applied in sequence. Each filter processes
/// the output of the previous one, allowing complex effect combinations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[derive(Default)]
pub enum FilterSpec {
    /// No filter effect
    #[default]
    None,
    /// Dim/darken the output
    Dim {
        /// Dimming factor (0.0 = black, 1.0 = no change), can be static or signal-driven
        #[serde(default = "default_dim_factor")]
        factor: SignalOrFloat,
        /// Which color component to dim
        apply_to: ApplyTo,
    },
    /// Invert colors
    Invert {
        /// Which color component to invert
        apply_to: ApplyTo,
    },
    /// Apply a color tint
    Tint {
        /// The tint color
        color: ColorConfig,
        /// Strength of the tint (0.0 = no effect, 1.0 = full replacement), can be signal-driven
        #[serde(default = "default_tint_strength")]
        strength: SignalOrFloat,
        /// Which color component to tint
        apply_to: ApplyTo,
    },
    /// Vignette darkening around edges
    Vignette {
        /// Strength of the vignette effect, can be signal-driven
        #[serde(default = "default_vignette_strength")]
        strength: SignalOrFloat,
        /// Radius where vignette starts (0.0 = center, 1.0 = edges), can be signal-driven
        #[serde(default = "default_vignette_radius")]
        radius: SignalOrFloat,
    },
    /// CRT monitor post-processing effect
    Crt {
        /// Strength of scanline effect, can be signal-driven
        #[serde(default = "default_crt_scanline")]
        scanline_strength: SignalOrFloat,
        /// Phosphor glow amount, can be signal-driven
        #[serde(default = "default_crt_glow")]
        glow: SignalOrFloat,
    },
    /// Pattern fill effect for background textures
    PatternFill {
        /// The pattern type to apply
        pattern: PatternType,
        /// Optional color for pattern characters
        #[serde(default)]
        color: Option<ColorConfig>,
        /// If true, only fill cells that are empty (whitespace)
        #[serde(default)]
        only_empty: bool,
    },
    /// Greyscale/desaturate filter using BT.601 luminance
    ///
    /// Converts colors to greyscale for "ghost" effects, commonly used
    /// for modal backdrops to draw focus to the modal content.
    Greyscale {
        /// Strength of the greyscale effect (0.0 = no effect, 1.0 = full greyscale)
        #[serde(default = "default_greyscale_strength")]
        strength: SignalOrFloat,
        /// Which color component to desaturate
        #[serde(default)]
        apply_to: ApplyTo,
    },
    /// Stochastic braille dust for frosted glass / film grain texture
    ///
    /// Places small braille dot patterns in empty cells at random positions,
    /// creating a subtle animated "dust motes" effect. Only affects whitespace.
    BrailleDust {
        /// Fraction of empty cells to fill (0.0 - 1.0)
        #[serde(default = "default_braille_density")]
        density: f32,
        /// Pattern changes per second (1.0 = once/sec, 8.0 = 8 times/sec)
        #[serde(default = "default_braille_hz")]
        hz: f32,
        /// Random seed for deterministic patterns
        #[serde(default = "default_braille_seed")]
        seed: u64,
        /// Braille pattern complexity
        #[serde(default)]
        pattern: BraillePatternType,
        /// Optional foreground color for dust particles
        #[serde(default)]
        color: Option<ColorConfig>,
    },
    /// Scanline/interlace effect for backdrop dimming
    ///
    /// Creates horizontal stripe patterns by dimming rows at regular intervals.
    InterlaceCurtain {
        /// Controls row spacing (1.0 = every other row, 0.5 = every 4th row)
        #[serde(default = "default_interlace_density")]
        density: f32,
        /// Dimming factor applied to affected rows (0.0 = no dimming, 1.0 = black)
        #[serde(default = "default_interlace_dim")]
        dim_factor: f32,
        /// Scroll speed for animation (0.0 = static)
        #[serde(default)]
        scroll_speed: f32,
    },
    /// Motion blur trail effect with directional dimming
    ///
    /// Applies graduated dimming in a specified direction to simulate motion blur.
    MotionBlur {
        /// Length of the blur trail as fraction of dimension (0.0 - 1.0)
        #[serde(default = "default_motion_trail")]
        trail_length: f32,
        /// Opacity decay exponent (higher = sharper falloff)
        #[serde(default = "default_motion_decay")]
        opacity_decay: f32,
        /// Direction of motion blur trail
        #[serde(default)]
        direction: MotionBlurDirection,
    },
    /// Color-bridged shade for smooth opacity rendering
    ///
    /// Maps opacity to shade characters (░▒▓█) with smooth color bridging.
    ColorBridgedShade {
        /// Target opacity (0.0 = transparent, 1.0 = opaque)
        #[serde(default = "default_shade_opacity")]
        opacity: f32,
        /// Foreground/fill color
        #[serde(default = "default_shade_fg")]
        fg_color: ColorConfig,
        /// Background color (shows through at low opacity)
        #[serde(default = "default_shade_bg")]
        bg_color: ColorConfig,
    },
    /// Sub-pixel progress bar with 8x resolution
    ///
    /// Uses partial block characters (▏▎▍▌▋▊▉█ for horizontal, ▁▂▃▄▅▆▇█ for vertical)
    /// to render progress bars with 8 times the resolution of cell-by-cell filling.
    SubPixelBar {
        /// Progress value (0.0 = empty, 1.0 = full)
        #[serde(default = "default_bar_progress")]
        progress: f32,
        /// Fill direction
        #[serde(default)]
        direction: SubPixelBarDirection,
        /// Color of the filled portion
        #[serde(default = "default_bar_filled")]
        filled_color: ColorConfig,
        /// Color of the unfilled portion
        #[serde(default = "default_bar_unfilled")]
        unfilled_color: ColorConfig,
        /// If true, animate the progress using t parameter (0-1 cycle)
        #[serde(default)]
        animated: bool,
    },
    /// Sub-cell shake using partial vertical blocks
    ///
    /// Creates physical-feeling vibration by oscillating edges using partial blocks
    /// (▏▎▍▌▋▊▉) to shift the visual center of mass without changing grid coordinates.
    /// This is the "incorrect password" or "tactile click" effect from IDEAS.md.
    SubCellShake {
        /// Maximum offset in eighths of a cell (1-4 recommended)
        #[serde(default = "default_shake_amplitude")]
        amplitude: u8,
        /// Shake frequency (cycles per second)
        #[serde(default = "default_shake_frequency")]
        frequency: f32,
        /// Random seed for pattern variation
        #[serde(default = "default_shake_seed")]
        seed: u64,
        /// If true, only shake edge cells (left and right borders)
        #[serde(default)]
        edge_only: bool,
        /// Filled color (foreground of partial blocks)
        #[serde(default = "default_shake_filled")]
        filled_color: ColorConfig,
        /// Background color (shows through partial blocks)
        #[serde(default = "default_shake_bg")]
        bg_color: ColorConfig,
    },
    /// Rigid body shake filter with damped oscillation pattern
    ///
    /// Creates a "ketchup bottle" shake effect where the entire element appears to
    /// shift left and right as a rigid body. Uses partial block characters to draw
    /// extensions and gaps in margin cells outside the widget area.
    ///
    /// The effect consists of multiple damped oscillations followed by a pause:
    /// - Each oscillation is a full sine wave (right → center → left → center)
    /// - Amplitude decreases with each successive shake (damping)
    /// - A base extension is always visible so the effect doesn't appear from nothing
    ///
    /// IMPORTANT: Apply this filter to an area that includes margin cells on each side.
    RigidShake {
        /// Duration of one back-and-forth shake in seconds
        #[serde(default = "default_rigid_shake_period")]
        shake_period: f32,
        /// Number of shakes before pause (max 8)
        #[serde(default = "default_rigid_num_shakes")]
        num_shakes: u8,
        /// Duration of pause between shake cycles in seconds
        #[serde(default = "default_rigid_pause_duration")]
        pause_duration: f32,
        /// Maximum extension in eighths of a cell (max 16 for 2 cells)
        #[serde(default = "default_rigid_max_eighths")]
        max_eighths: u8,
        /// Base extension always visible at rest (creates natural appearance)
        #[serde(default = "default_rigid_base_eighths")]
        base_eighths: u8,
        /// Amplitude multipliers for each shake (damping curve, up to 8 values)
        #[serde(default = "default_rigid_damping")]
        damping: Vec<f32>,
        /// Color of the element being shaken
        #[serde(default = "default_rigid_element_color")]
        element_color: ColorConfig,
        /// Background color (shows in gaps)
        #[serde(default = "default_rigid_bg_color")]
        bg_color: ColorConfig,
        /// Width of the inner content area (excluding margins)
        #[serde(default = "default_rigid_inner_width")]
        inner_width: u16,
        /// Margin width on each side (default 2, max 4)
        #[serde(default = "default_rigid_margin_width")]
        margin_width: u8,
    },
    /// Progress-driven partial bar indicator for hover/focus states
    ///
    /// Uses partial block characters (▏▎▍▌▋▊▉█) to render a bar that expands
    /// from `base_eighths` to `max_eighths` based on animation progress.
    ///
    /// IMPORTANT: Apply to an area that includes margin cells for the indicator.
    HoverBar {
        /// Base width at rest (0.0 progress), in eighths (0-8)
        #[serde(default = "default_hover_base")]
        base_eighths: u8,
        /// Maximum width when fully active (1.0 progress), in eighths (0-16)
        #[serde(default = "default_hover_max")]
        max_eighths: u8,
        /// Position relative to content
        #[serde(default)]
        position: HoverBarPosition,
        /// Bar color
        #[serde(default = "default_hover_bar_color")]
        bar_color: ColorConfig,
        /// Background color (for inversion)
        #[serde(default = "default_hover_bg_color")]
        bg_color: ColorConfig,
        /// Animation progress (0.0 = rest, 1.0 = fully active)
        #[serde(default)]
        progress: f32,
        /// Margin width on the active side (1-2 cells)
        #[serde(default = "default_hover_margin_width")]
        margin_width: u8,
    },
    /// Horizontal underline that wipes in based on progress
    ///
    /// Draws a line character at the bottom of the content area,
    /// progressively revealed based on animation progress and wipe direction.
    UnderlineWipe {
        /// Wipe direction
        #[serde(default)]
        direction: WipeDirection,
        /// Line color
        #[serde(default = "default_underline_color")]
        color: ColorConfig,
        /// Background color (for gradient)
        #[serde(default = "default_underline_bg_color")]
        bg_color: ColorConfig,
        /// Character for the line (default: ▁ lower one-eighth)
        #[serde(default = "default_underline_char")]
        line_char: char,
        /// Row offset from bottom (0 = last row)
        #[serde(default)]
        row_offset: u16,
        /// Progress (0.0 = none, 1.0 = full width)
        #[serde(default)]
        progress: f32,
        /// Enable gradient from bg_color to color along wipe direction
        #[serde(default = "default_true")]
        gradient: bool,
        /// Enable glisten/shimmer effect on the line
        #[serde(default = "default_true")]
        glisten: bool,
    },
    /// Brackets that appear around content based on progress
    ///
    /// Draws bracket characters at the left and right edges of the content,
    /// with color fading in based on animation progress.
    BracketEmphasis {
        /// Left bracket character
        #[serde(default = "default_left_bracket")]
        left: char,
        /// Right bracket character
        #[serde(default = "default_right_bracket")]
        right: char,
        /// Bracket color
        #[serde(default = "default_bracket_color")]
        color: ColorConfig,
        /// Background color for blending
        #[serde(default = "default_bracket_bg_color")]
        bg_color: ColorConfig,
        /// Progress (0.0 = invisible, 1.0 = fully visible)
        #[serde(default)]
        progress: f32,
    },
    /// Simple dot/bullet indicator that appears adjacent to content
    ///
    /// Draws a single indicator character at the edge of the content,
    /// fading in based on animation progress.
    DotIndicator {
        /// Indicator character (default: •)
        #[serde(default = "default_dot_char")]
        indicator_char: char,
        /// Position (Left or Right)
        #[serde(default)]
        position: HoverBarPosition,
        /// Indicator color
        #[serde(default = "default_dot_color")]
        color: ColorConfig,
        /// Background color for blending
        #[serde(default = "default_dot_bg_color")]
        bg_color: ColorConfig,
        /// Progress (0.0 = invisible, 1.0 = fully visible)
        #[serde(default)]
        progress: f32,
    },
    /// Pill-shaped button with gradient edges
    ///
    /// Creates a soft, rounded button appearance using horizontal gradients
    /// on the left and right edges. Supports glisten effect on hover.
    PillButton {
        /// Button/fill color
        #[serde(default = "default_pill_button_color")]
        button_color: ColorConfig,
        /// Background color (for gradient edges)
        #[serde(default = "default_pill_bg_color")]
        bg_color: ColorConfig,
        /// Width of gradient edge in cells
        #[serde(default = "default_pill_edge_width")]
        edge_width: u16,
        /// Enable glisten effect on hover
        #[serde(default = "default_true")]
        glisten: bool,
        /// Hover progress (0.0 = not hovered, 1.0 = fully hovered)
        #[serde(default)]
        progress: f32,
    },
    /// Diagonal glisten sweep effect
    ///
    /// Creates a 45-degree highlight band that sweeps across content,
    /// providing a polished shine effect for buttons and interactive elements.
    GlistenSweep {
        /// Highlight color boost (added to existing colors)
        #[serde(default = "default_glisten_boost")]
        boost: u8,
        /// Width of the glisten band (0.0-1.0, relative to diagonal)
        #[serde(default = "default_glisten_band_width")]
        band_width: f32,
        /// Animation speed (0 = use progress only)
        #[serde(default = "default_glisten_speed")]
        speed: f32,
        /// Hover progress (0.0 = not hovered, 1.0 = fully hovered)
        #[serde(default)]
        progress: f32,
        /// Smart powerline mode: bg on text, fg only on separator glyphs
        #[serde(default)]
        powerline_mode: bool,
        /// When true AND powerline_mode is true, also boost separator backgrounds.
        /// Use when powerline has continuous background (not terminal bg).
        #[serde(default)]
        boost_separator_bg: bool,
    },
    /// Horizontal ping-pong scanner effect (KITT/Larson scanner)
    ///
    /// Creates a horizontal band of brightness that sweeps left-to-right,
    /// then right-to-left in a continuous ping-pong pattern.
    KittScanner {
        /// Brightness boost added to cells under the scanner
        #[serde(default = "default_kitt_boost")]
        boost: u8,
        /// Width of the scanner band (0.0-0.5 of total width)
        #[serde(default = "default_kitt_band_width")]
        band_width: f32,
        /// Beats per second for ping-pong cycle
        #[serde(default = "default_kitt_bps")]
        bps: f32,
        /// Animation progress (0.0 = inactive, 1.0 = fully active)
        #[serde(default)]
        progress: f32,
        /// Which color component to boost (ignored if powerline_mode is true)
        #[serde(default = "default_kitt_apply_to")]
        apply_to: ApplyTo,
        /// Smart powerline mode: bg on text, fg only on separator glyphs
        #[serde(default)]
        powerline_mode: bool,
        /// When true AND powerline_mode is true, also boost separator backgrounds.
        /// Use when powerline has continuous background (not terminal bg).
        #[serde(default)]
        boost_separator_bg: bool,
    },
    /// Ping-pong scanner that dims text with light shade overlay
    ///
    /// As the scanner sweeps right, text gets overlaid with a light shade
    /// character, creating a dimming effect. Returns left to reveal.
    ShadeScanner {
        /// Shade color (the dimming overlay color)
        #[serde(default = "default_shade_scanner_color")]
        shade_color: ColorConfig,
        /// Beats per second for ping-pong cycle
        #[serde(default = "default_shade_scanner_bps")]
        bps: f32,
        /// Animation progress (0.0 = inactive, 1.0 = fully active)
        #[serde(default)]
        progress: f32,
    },
}

/// Direction of motion blur trail.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum MotionBlurDirection {
    /// Trail extends to the left (motion toward right)
    #[default]
    Left,
    /// Trail extends to the right (motion toward left)
    Right,
    /// Trail extends upward (motion toward bottom)
    Up,
    /// Trail extends downward (motion toward top)
    Down,
}

/// Direction for sub-pixel progress bar rendering.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum SubPixelBarDirection {
    /// Fill from left to right using vertical partial blocks (▏▎▍▌▋▊▉█)
    #[default]
    Horizontal,
    /// Fill from bottom to top using horizontal partial blocks (▁▂▃▄▅▆▇█)
    Vertical,
}

// Default functions for signal-or-float fields
fn default_dim_factor() -> SignalOrFloat {
    SignalOrFloat::Static(0.5)
}

fn default_tint_strength() -> SignalOrFloat {
    SignalOrFloat::Static(0.5)
}

fn default_vignette_strength() -> SignalOrFloat {
    SignalOrFloat::Static(0.5)
}

fn default_vignette_radius() -> SignalOrFloat {
    SignalOrFloat::Static(0.5)
}

fn default_crt_scanline() -> SignalOrFloat {
    SignalOrFloat::Static(0.5)
}

fn default_crt_glow() -> SignalOrFloat {
    SignalOrFloat::Static(0.5)
}

fn default_greyscale_strength() -> SignalOrFloat {
    SignalOrFloat::Static(1.0)
}

fn default_braille_density() -> f32 {
    0.03
}

fn default_braille_hz() -> f32 {
    8.0
}

fn default_braille_seed() -> u64 {
    42
}

fn default_interlace_density() -> f32 {
    1.0
}

fn default_interlace_dim() -> f32 {
    0.3
}

fn default_motion_trail() -> f32 {
    0.5
}

fn default_motion_decay() -> f32 {
    1.5
}

fn default_shade_opacity() -> f32 {
    0.5
}

fn default_shade_fg() -> ColorConfig {
    ColorConfig::White
}

fn default_shade_bg() -> ColorConfig {
    ColorConfig::Black
}

fn default_bar_progress() -> f32 {
    0.5
}

fn default_bar_filled() -> ColorConfig {
    ColorConfig::Rgb {
        r: 100,
        g: 200,
        b: 100,
    }
}

fn default_bar_unfilled() -> ColorConfig {
    ColorConfig::Rgb {
        r: 50,
        g: 50,
        b: 50,
    }
}

fn default_shake_amplitude() -> u8 {
    2
}

fn default_shake_frequency() -> f32 {
    8.0
}

fn default_shake_seed() -> u64 {
    42
}

fn default_shake_filled() -> ColorConfig {
    ColorConfig::Rgb {
        r: 100,
        g: 150,
        b: 200,
    }
}

fn default_shake_bg() -> ColorConfig {
    ColorConfig::Rgb {
        r: 30,
        g: 30,
        b: 30,
    }
}

// RigidShake defaults
fn default_rigid_shake_period() -> f32 {
    0.29
}

fn default_rigid_num_shakes() -> u8 {
    4
}

fn default_rigid_pause_duration() -> f32 {
    0.52
}

fn default_rigid_max_eighths() -> u8 {
    12
}

fn default_rigid_base_eighths() -> u8 {
    3
}

fn default_rigid_damping() -> Vec<f32> {
    vec![1.0, 0.7, 0.45, 0.25, 0.15, 0.1, 0.05, 0.0]
}

fn default_rigid_element_color() -> ColorConfig {
    ColorConfig::Rgb {
        r: 100,
        g: 100,
        b: 100,
    }
}

fn default_rigid_bg_color() -> ColorConfig {
    ColorConfig::Rgb {
        r: 30,
        g: 30,
        b: 30,
    }
}

fn default_rigid_inner_width() -> u16 {
    10
}

fn default_rigid_margin_width() -> u8 {
    2
}

// HoverBar defaults
fn default_hover_base() -> u8 {
    4
}

fn default_hover_max() -> u8 {
    12
}

fn default_hover_bar_color() -> ColorConfig {
    ColorConfig::Rgb {
        r: 100,
        g: 150,
        b: 200,
    }
}

fn default_hover_bg_color() -> ColorConfig {
    ColorConfig::Rgb {
        r: 30,
        g: 30,
        b: 30,
    }
}

fn default_hover_margin_width() -> u8 {
    2
}

// UnderlineWipe defaults
fn default_underline_color() -> ColorConfig {
    ColorConfig::Rgb {
        r: 100,
        g: 150,
        b: 200,
    }
}

fn default_underline_char() -> char {
    '—'
}

fn default_underline_bg_color() -> ColorConfig {
    ColorConfig::Rgb {
        r: 30,
        g: 30,
        b: 30,
    }
}

fn default_true() -> bool {
    true
}

// BracketEmphasis defaults
fn default_left_bracket() -> char {
    '['
}

fn default_right_bracket() -> char {
    ']'
}

fn default_bracket_color() -> ColorConfig {
    ColorConfig::Rgb {
        r: 100,
        g: 150,
        b: 200,
    }
}

fn default_bracket_bg_color() -> ColorConfig {
    ColorConfig::Rgb {
        r: 30,
        g: 30,
        b: 30,
    }
}

// DotIndicator defaults
fn default_dot_char() -> char {
    '•'
}

fn default_dot_color() -> ColorConfig {
    ColorConfig::Rgb {
        r: 100,
        g: 150,
        b: 200,
    }
}

fn default_dot_bg_color() -> ColorConfig {
    ColorConfig::Rgb {
        r: 30,
        g: 30,
        b: 30,
    }
}

// PillButton defaults
fn default_pill_button_color() -> ColorConfig {
    ColorConfig::Rgb {
        r: 80,
        g: 120,
        b: 180,
    }
}

fn default_pill_bg_color() -> ColorConfig {
    ColorConfig::Rgb {
        r: 30,
        g: 30,
        b: 35,
    }
}

fn default_pill_edge_width() -> u16 {
    3
}

fn default_glisten_boost() -> u8 {
    40
}

fn default_glisten_band_width() -> f32 {
    0.2
}

fn default_glisten_speed() -> f32 {
    0.5
}

// KittScanner defaults
fn default_kitt_boost() -> u8 {
    50
}

fn default_kitt_band_width() -> f32 {
    0.15
}

fn default_kitt_bps() -> f32 {
    1.0
}

fn default_kitt_apply_to() -> ApplyTo {
    ApplyTo::Both // Both fg and bg for full effect
}

// ShadeScanner defaults
fn default_shade_scanner_color() -> ColorConfig {
    ColorConfig::Rgb {
        r: 40,
        g: 40,
        b: 45,
    }
}

fn default_shade_scanner_bps() -> f32 {
    1.0
}

impl FilterSpec {
    /// Returns the filter type name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            FilterSpec::None => "None",
            FilterSpec::Dim { .. } => "Dim",
            FilterSpec::Invert { .. } => "Invert",
            FilterSpec::Tint { .. } => "Tint",
            FilterSpec::Vignette { .. } => "Vignette",
            FilterSpec::Crt { .. } => "Crt",
            FilterSpec::PatternFill { .. } => "PatternFill",
            FilterSpec::Greyscale { .. } => "Greyscale",
            FilterSpec::BrailleDust { .. } => "BrailleDust",
            FilterSpec::InterlaceCurtain { .. } => "InterlaceCurtain",
            FilterSpec::MotionBlur { .. } => "MotionBlur",
            FilterSpec::ColorBridgedShade { .. } => "ColorBridgedShade",
            FilterSpec::SubPixelBar { .. } => "SubPixelBar",
            FilterSpec::SubCellShake { .. } => "SubCellShake",
            FilterSpec::RigidShake { .. } => "RigidShake",
            FilterSpec::HoverBar { .. } => "HoverBar",
            FilterSpec::UnderlineWipe { .. } => "UnderlineWipe",
            FilterSpec::BracketEmphasis { .. } => "BracketEmphasis",
            FilterSpec::DotIndicator { .. } => "DotIndicator",
            FilterSpec::PillButton { .. } => "PillButton",
            FilterSpec::GlistenSweep { .. } => "GlistenSweep",
            FilterSpec::KittScanner { .. } => "KittScanner",
            FilterSpec::ShadeScanner { .. } => "ShadeScanner",
        }
    }

    /// Returns a brief human-readable description of what this filter does.
    pub fn terse_description(&self) -> &'static str {
        match self {
            FilterSpec::None => "No filter effect",
            FilterSpec::Dim { .. } => "Dim/darken the output",
            FilterSpec::Invert { .. } => "Invert colors",
            FilterSpec::Tint { .. } => "Apply a color tint",
            FilterSpec::Vignette { .. } => "Vignette darkening around edges",
            FilterSpec::Crt { .. } => "CRT monitor post-processing effect",
            FilterSpec::PatternFill { .. } => "Pattern fill effect for background textures",
            FilterSpec::Greyscale { .. } => "Greyscale/desaturate filter using BT.601 luminance",
            FilterSpec::BrailleDust { .. } => "Stochastic braille dust for frosted glass texture",
            FilterSpec::InterlaceCurtain { .. } => "Scanline/interlace effect for backdrop dimming",
            FilterSpec::MotionBlur { .. } => "Motion blur trail effect with directional dimming",
            FilterSpec::ColorBridgedShade { .. } => {
                "Color-bridged shade for smooth opacity rendering"
            }
            FilterSpec::SubPixelBar { .. } => "Sub-pixel progress bar with 8x resolution",
            FilterSpec::SubCellShake { .. } => "Sub-cell shake using partial vertical blocks",
            FilterSpec::RigidShake { .. } => "Rigid body shake filter with damped oscillation",
            FilterSpec::HoverBar { .. } => "Progress-driven partial bar indicator for hover states",
            FilterSpec::UnderlineWipe { .. } => {
                "Horizontal underline that wipes in based on progress"
            }
            FilterSpec::BracketEmphasis { .. } => {
                "Brackets that appear around content based on progress"
            }
            FilterSpec::DotIndicator { .. } => "Simple dot/bullet indicator adjacent to content",
            FilterSpec::PillButton { .. } => "Pill-shaped button with gradient edges",
            FilterSpec::GlistenSweep { .. } => "Diagonal glisten sweep effect",
            FilterSpec::KittScanner { .. } => "Horizontal ping-pong scanner effect (KITT/Larson)",
            FilterSpec::ShadeScanner { .. } => {
                "Ping-pong scanner that dims text with shade overlay"
            }
        }
    }

    /// Returns key parameters of this filter for documentation purposes.
    pub fn key_parameters(&self) -> Vec<(&'static str, String)> {
        match self {
            FilterSpec::None => vec![],
            FilterSpec::Dim { factor, apply_to } => vec![
                ("factor", format!("{:?}", factor)),
                ("apply_to", format!("{:?}", apply_to)),
            ],
            FilterSpec::Invert { apply_to } => vec![("apply_to", format!("{:?}", apply_to))],
            FilterSpec::Tint {
                color,
                strength,
                apply_to,
            } => vec![
                ("color", format!("{:?}", color)),
                ("strength", format!("{:?}", strength)),
                ("apply_to", format!("{:?}", apply_to)),
            ],
            FilterSpec::Vignette { strength, radius } => vec![
                ("strength", format!("{:?}", strength)),
                ("radius", format!("{:?}", radius)),
            ],
            FilterSpec::Crt {
                scanline_strength,
                glow,
            } => vec![
                ("scanline_strength", format!("{:?}", scanline_strength)),
                ("glow", format!("{:?}", glow)),
            ],
            FilterSpec::PatternFill {
                pattern,
                color,
                only_empty,
            } => vec![
                ("pattern", format!("{:?}", pattern)),
                ("color", format!("{:?}", color)),
                ("only_empty", format!("{}", only_empty)),
            ],
            FilterSpec::Greyscale { strength, apply_to } => vec![
                ("strength", format!("{:?}", strength)),
                ("apply_to", format!("{:?}", apply_to)),
            ],
            FilterSpec::BrailleDust {
                density, hz, seed, ..
            } => vec![
                ("density", format!("{}", density)),
                ("hz", format!("{}", hz)),
                ("seed", format!("{}", seed)),
            ],
            FilterSpec::InterlaceCurtain {
                density,
                dim_factor,
                scroll_speed,
            } => vec![
                ("density", format!("{}", density)),
                ("dim_factor", format!("{}", dim_factor)),
                ("scroll_speed", format!("{}", scroll_speed)),
            ],
            FilterSpec::MotionBlur {
                trail_length,
                opacity_decay,
                direction,
            } => vec![
                ("trail_length", format!("{}", trail_length)),
                ("opacity_decay", format!("{}", opacity_decay)),
                ("direction", format!("{:?}", direction)),
            ],
            FilterSpec::ColorBridgedShade {
                opacity,
                fg_color,
                bg_color,
            } => vec![
                ("opacity", format!("{}", opacity)),
                ("fg_color", format!("{:?}", fg_color)),
                ("bg_color", format!("{:?}", bg_color)),
            ],
            FilterSpec::SubPixelBar {
                progress,
                direction,
                ..
            } => vec![
                ("progress", format!("{}", progress)),
                ("direction", format!("{:?}", direction)),
            ],
            FilterSpec::SubCellShake {
                amplitude,
                frequency,
                seed,
                edge_only,
                ..
            } => vec![
                ("amplitude", format!("{}", amplitude)),
                ("frequency", format!("{}", frequency)),
                ("seed", format!("{}", seed)),
                ("edge_only", format!("{}", edge_only)),
            ],
            FilterSpec::RigidShake {
                shake_period,
                num_shakes,
                pause_duration,
                max_eighths,
                ..
            } => vec![
                ("shake_period", format!("{}s", shake_period)),
                ("num_shakes", format!("{}", num_shakes)),
                ("pause_duration", format!("{}s", pause_duration)),
                ("max_eighths", format!("{}", max_eighths)),
            ],
            FilterSpec::HoverBar {
                base_eighths,
                max_eighths,
                position,
                progress,
                ..
            } => vec![
                ("base_eighths", format!("{}", base_eighths)),
                ("max_eighths", format!("{}", max_eighths)),
                ("position", format!("{:?}", position)),
                ("progress", format!("{}", progress)),
            ],
            FilterSpec::UnderlineWipe {
                direction,
                progress,
                gradient,
                glisten,
                ..
            } => vec![
                ("direction", format!("{:?}", direction)),
                ("progress", format!("{}", progress)),
                ("gradient", format!("{}", gradient)),
                ("glisten", format!("{}", glisten)),
            ],
            FilterSpec::BracketEmphasis {
                left,
                right,
                progress,
                ..
            } => vec![
                ("left", format!("{}", left)),
                ("right", format!("{}", right)),
                ("progress", format!("{}", progress)),
            ],
            FilterSpec::DotIndicator {
                indicator_char,
                position,
                progress,
                ..
            } => vec![
                ("indicator_char", format!("{}", indicator_char)),
                ("position", format!("{:?}", position)),
                ("progress", format!("{}", progress)),
            ],
            FilterSpec::PillButton {
                edge_width,
                glisten,
                progress,
                ..
            } => vec![
                ("edge_width", format!("{}", edge_width)),
                ("glisten", format!("{}", glisten)),
                ("progress", format!("{}", progress)),
            ],
            FilterSpec::GlistenSweep {
                boost,
                band_width,
                speed,
                progress,
                powerline_mode,
                boost_separator_bg,
            } => vec![
                ("boost", format!("{}", boost)),
                ("band_width", format!("{}", band_width)),
                ("speed", format!("{}", speed)),
                ("progress", format!("{}", progress)),
                ("powerline_mode", format!("{}", powerline_mode)),
                ("boost_separator_bg", format!("{}", boost_separator_bg)),
            ],
            FilterSpec::KittScanner {
                boost,
                band_width,
                bps,
                progress,
                apply_to,
                powerline_mode,
                boost_separator_bg,
            } => vec![
                ("boost", format!("{}", boost)),
                ("band_width", format!("{}", band_width)),
                ("bps", format!("{} Hz", bps)),
                ("progress", format!("{}", progress)),
                ("apply_to", format!("{:?}", apply_to)),
                ("powerline_mode", format!("{}", powerline_mode)),
                ("boost_separator_bg", format!("{}", boost_separator_bg)),
            ],
            FilterSpec::ShadeScanner {
                shade_color,
                bps,
                progress,
            } => vec![
                ("shade_color", format!("{:?}", shade_color)),
                ("bps", format!("{} Hz", bps)),
                ("progress", format!("{}", progress)),
            ],
        }
    }
}

// <FILE>tui-vfx-compositor/src/types/cls_filter_spec.rs</FILE> - <DESC>FilterSpec enum with signal-driven parameters</DESC>
// <VERS>END OF VERSION: 3.4.0</VERS>
