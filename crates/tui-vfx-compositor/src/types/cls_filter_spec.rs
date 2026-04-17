// <FILE>tui-vfx-compositor/src/types/cls_filter_spec.rs</FILE> - <DESC>FilterSpec enum with signal-driven parameters</DESC>
// <VERS>VERSION: 3.9.0</VERS>
// <WCTX>Phase 0 P0.C followup — add damping_scale_binding on RigidShake so severity can also scale the shake decay curve alongside num_shakes_binding, plus FadeToCanvas.canvas_color_binding for live terminal-background tracking</WCTX>
// <CLOG>Add FilterSpec::RigidShake.damping_scale_binding: Option<String> resolved to f32 at prepare time, clamped 0.1..=10.0, and multiplied element-wise into the 8-element damping array. Missing or out-of-range bindings fall back to the unscaled static damping curve. Also adds FilterSpec::FadeToCanvas.canvas_color_binding from the sibling O-P0.B commit

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

use super::cls_bindable_value::BindableValue;
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

/// A single stop in a CharsetNoise vertical gradient.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct CharsetNoiseGradientStop {
    /// Normalized vertical position (0.0 = top, 1.0 = bottom).
    pub at: f32,
    /// Pool of characters at this position.
    pub chars: String,
}

/// Controls which cells CharsetNoise affects.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum CharsetNoiseAffect {
    /// Replace all cells (including whitespace).
    All,
    /// Replace only non-whitespace cells (default).
    #[default]
    NonEmpty,
}

/// Controls which cells MatrixRain affects.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum MatrixRainAffect {
    /// Replace every cell in the target region.
    #[default]
    All,
    /// Replace only blank/whitespace cells.
    OnlyBlank,
}

/// Built-in glyph alphabets for MatrixRain.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum MatrixRainCharsetPreset {
    /// Half-width katakana + digits/symbols.
    #[default]
    Matrix,
    /// Binary digits only.
    Binary,
    /// Hexadecimal characters only.
    Hex,
    /// Uppercase ASCII letters and digits.
    Ascii,
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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ScannerMotionMode {
    #[default]
    PingPong,
    ForwardWrap,
    ReverseWrap,
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
/// - **Hover Indicators**: HoverBar, UnderlineWipe, BracketEmphasis, DotIndicator, EdgeGrow
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
    /// Canvas-aware exit fade that blends cells toward a declared canvas
    /// color. Use this in place of `tint(black, ...)` on exit animations so
    /// the widget doesn't flash dark on light terminal backgrounds: set
    /// `canvas_color` to match the terminal background and drive `strength`
    /// from the exit phase progress (static 1.0 for instant fade, signal
    /// for smooth animation, binding for runtime-driven control).
    ///
    /// Defaults to black for drop-in compatibility with the old tint hack.
    /// Add `canvas_color_binding` to swap the static canvas color for a
    /// runtime-provided RGB triple when the terminal background changes
    /// (theme mode flip, live palette adjustment, etc.) — the binding
    /// resolves once at prepare time from a
    /// `ShaderRuntimeParamValue::Rgb` entry in the composition's runtime
    /// params map, and falls back to `canvas_color` when the binding is
    /// missing or not an Rgb value.
    FadeToCanvas {
        /// The canvas color to fade into — should match the terminal
        /// background the recipe will run against.
        #[serde(default = "default_fade_to_canvas_color")]
        canvas_color: ColorConfig,
        /// Optional runtime parameter key that overrides `canvas_color`
        /// per frame. Must resolve to a `ShaderRuntimeParamValue::Rgb`
        /// (JSON shape `{"r": <u8>, "g": <u8>, "b": <u8>}`) to take
        /// effect; any other kind or a missing binding falls back to
        /// the static `canvas_color`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        canvas_color_binding: Option<String>,
        /// Fade strength (0.0 = untouched, 1.0 = fully replaced with
        /// canvas_color). Uses `BindableValue` so the P0.1 signal/binding
        /// surface applies uniformly across filter parameters.
        #[serde(default)]
        strength: BindableValue,
        /// Which color component(s) to fade.
        #[serde(default = "default_fade_to_canvas_apply_to")]
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
        /// Which edge(s) the darkening originates from. Empty/default keeps
        /// the classic all-sides radial vignette.
        #[serde(default)]
        sides: Vec<VignetteEdge>,
        /// Optional low-amplitude spatial dither to reduce visible contouring.
        #[serde(default)]
        dither_amount: f32,
        /// Optional temporal rate for the dither pattern in Hz. 0.0 = static.
        #[serde(default)]
        temporal_dither_hz: f32,
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
        /// Drift in cells per step lifecycle. Positive = downward (gravity),
        /// negative = upward (sparks rising). Default 0.0 (no drift).
        #[serde(default)]
        drift: f32,
    },
    /// Non-converging time-varying character replacement for living textures.
    ///
    /// Replaces cell characters from a position-aware charset gradient that
    /// changes over time. Unlike content transformers (which resolve toward
    /// a target), CharsetNoise cycles indefinitely — producing living textures
    /// like fire, rain, smoke, or static noise.
    ///
    /// Supports a vertical gradient of charsets: sparse characters at the top,
    /// dense at the bottom. Including empty characters (like `⠀`) in sparse
    /// pools makes the shape boundary itself fluctuate — cells flicker between
    /// visible and invisible.
    ///
    /// Chains naturally with other filters: run `charset_noise` first to mutate
    /// characters, then `braille_dust` to fill gaps with particles, then `tint`
    /// for color warmth.
    ///
    /// # Parameters
    ///
    /// - `hz`: Pattern changes per second (8.0 = organic fire flicker)
    /// - `seed`: Deterministic base for reproducible patterns
    /// - `jitter`: Per-cell random offset to gradient position (0.0–1.0)
    /// - `affect`: Which cells to replace (`"all"` or `"non_empty"`, default: `"non_empty"`)
    /// - `chars`: Flat charset (all cells use the same pool)
    /// - `gradient`: Position-aware charsets (overrides `chars` if present)
    ///
    /// # JSON Examples
    ///
    /// Flat charset (all cells use the same pool):
    /// ```json
    /// { "type": "charset_noise", "chars": "⣿⣷⣾⣯⣻⣽", "hz": 8.0, "seed": 42 }
    /// ```
    ///
    /// Vertical gradient (sparse flickering tips, solid dense base):
    /// ```json
    /// { "type": "charset_noise", "hz": 8.0, "seed": 42, "jitter": 0.15,
    ///   "gradient": [
    ///     { "at": 0.0, "chars": "⠀⠀⠀⠁⠂⠈" },
    ///     { "at": 0.5, "chars": "⡖⣂⢒⡒⣄⢆" },
    ///     { "at": 1.0, "chars": "⣿⣷⣾⣯⣻⣽" }
    ///   ]
    /// }
    /// ```
    CharsetNoise {
        /// Pattern changes per second (default 8.0).
        #[serde(default = "default_charset_noise_hz")]
        hz: f32,
        /// Deterministic seed for reproducible patterns.
        #[serde(default)]
        seed: u64,
        /// Per-cell random offset to gradient position (0.0 = none, 1.0 = full range).
        #[serde(default)]
        jitter: f32,
        /// Which cells to affect: "all" or "non_empty" (default).
        #[serde(default)]
        affect: CharsetNoiseAffect,
        /// Flat charset — all cells use this pool. Ignored if `gradient` is present.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        chars: Option<String>,
        /// Position-aware charset gradient. Overrides `chars` if present.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        gradient: Option<Vec<CharsetNoiseGradientStop>>,
    },
    /// Deterministic procedural digital-rain field.
    ///
    /// Synthesizes coherent per-column falling streams directly in the
    /// compositor filter path. Each column derives stream presence, speed,
    /// trail length, glyph churn, and head position from `(x, t, seed)`,
    /// avoiding widget-owned persistent state while still reading as Matrix
    /// rain instead of generic noise.
    ///
    /// `density` and `speed_multiplier` use `BindableValue`, making them
    /// suitable first-class runtime controls for dynamic recipes.
    MatrixRain {
        /// Fraction of columns that should be active (0.0 - 1.0).
        #[serde(default = "default_matrix_rain_density")]
        density: BindableValue,
        /// Scales the per-column authored speed range at runtime.
        #[serde(default = "default_matrix_rain_speed_multiplier")]
        speed_multiplier: BindableValue,
        /// Minimum normalized fall speed across the column population.
        #[serde(default = "default_matrix_rain_speed_min")]
        speed_min: f32,
        /// Maximum normalized fall speed across the column population.
        #[serde(default = "default_matrix_rain_speed_max")]
        speed_max: f32,
        /// Minimum trail length in cells.
        #[serde(default = "default_matrix_rain_trail_min")]
        trail_min: u16,
        /// Maximum trail length in cells.
        #[serde(default = "default_matrix_rain_trail_max")]
        trail_max: u16,
        /// Glyph churn cadence.
        #[serde(default = "default_matrix_rain_glyph_change_hz")]
        glyph_change_hz: f32,
        /// Deterministic seed.
        #[serde(default = "default_matrix_rain_seed")]
        seed: u64,
        /// Which cells the filter may overwrite.
        #[serde(default)]
        affect: MatrixRainAffect,
        /// Built-in glyph alphabet preset.
        #[serde(default)]
        preset: MatrixRainCharsetPreset,
        /// Optional custom glyph alphabet overriding the preset.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        chars: Option<String>,
        /// Bright head color.
        #[serde(default = "default_matrix_rain_head_color")]
        head_color: ColorConfig,
        /// Dim tail color.
        #[serde(default = "default_matrix_rain_tail_color")]
        tail_color: ColorConfig,
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
        /// Progress value (0.0 = empty, 1.0 = full). Accepts a raw number,
        /// a signal spec, or a runtime binding (`{"binding": "scroll_ratio"}`).
        #[serde(default = "default_bar_progress")]
        progress: BindableValue,
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
    /// Sub-cell light renderer for blank shell-owned cells.
    ///
    /// Interprets an existing color field (usually background light from a
    /// shader) and re-renders it into partial-block or braille glyphs. This is
    /// useful when a light field feels too square or too obviously cell-based:
    /// the shader defines the light distribution, while this filter provides a
    /// finer terminal-native rasterization in blank cells.
    ///
    /// This filter is usually best paired with background-heavy light shaders
    /// such as `ConcealedLight`, `Diffusion`, or `FocusField`.
    SubcellLight {
        /// The fully lit color used in the sub-cell glyph.
        #[serde(default = "default_subcell_light_lit")]
        lit_color: ColorConfig,
        /// The unlit/background color shown through the unfilled portion.
        #[serde(default = "default_subcell_light_unlit")]
        unlit_color: ColorConfig,
        /// Render strategy (braille, horizontal eighths, or vertical eighths).
        #[serde(default)]
        render_mode: SubcellLightRenderMode,
        /// Which existing color field to sample (`foreground` or `background`).
        #[serde(default)]
        sample_from: LightSampleFrom,
        /// Minimum normalized intensity required before a cell is converted.
        #[serde(default = "default_subcell_light_threshold")]
        threshold: f32,
        /// Optional low-rate temporal dither in Hz for braille mode. 0 = static.
        #[serde(default)]
        temporal_dither_hz: f32,
        /// If true (default), only convert blank cells.
        #[serde(default = "default_true")]
        only_blank: bool,
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
        /// Optional runtime parameter key used to resolve the shake count
        /// at render time. The resolved u16 is clamped to u8 range
        /// (0-255) and the downstream filter clamps further to 0-8.
        /// Missing bindings fall back to the static `num_shakes`.
        ///
        /// Use this to drive shake count from severity/error-level state:
        /// warning => 1 shake, error => 4 shakes, critical => 8 shakes.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        num_shakes_binding: Option<String>,
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
        /// Optional runtime parameter key that scales the entire `damping`
        /// curve uniformly at render time. Resolved as an `f32` and clamped
        /// to `0.1..=10.0` to prevent runaway decay (< 0.1 would make the
        /// shake decay slower than one full cycle, > 10.0 would stall the
        /// shake within the first fraction). The resolved scale multiplies
        /// every element of the 8-entry damping array before the filter
        /// constructs its `RigidShakeTiming`, so a runtime value of 2.0
        /// doubles the decay rate of every shake while 0.5 halves it.
        /// Missing bindings fall back to the static `damping` curve
        /// unchanged.
        ///
        /// Use this together with `num_shakes_binding` to drive both shake
        /// count AND shake intensity from the same severity state: warning
        /// might pair (1, 0.5) for a gentle single-cycle nudge while
        /// critical pairs (8, 2.0) for a tight, rapidly-decaying cluster.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        damping_scale_binding: Option<String>,
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
        /// Animation progress (0.0 = rest, 1.0 = fully active). Accepts a
        /// raw number, signal spec, or runtime binding.
        #[serde(default)]
        progress: BindableValue,
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
        /// Progress (0.0 = none, 1.0 = full width). Accepts a raw number,
        /// signal spec, or runtime binding.
        #[serde(default)]
        progress: BindableValue,
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
        /// Progress (0.0 = invisible, 1.0 = fully visible). Accepts a raw
        /// number, signal spec, or runtime binding.
        #[serde(default)]
        progress: BindableValue,
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
        /// Progress (0.0 = invisible, 1.0 = fully visible). Accepts a raw
        /// number, signal spec, or runtime binding.
        #[serde(default)]
        progress: BindableValue,
    },
    /// Generalized edge growth / stretch indicator using sub-cell blocks.
    ///
    /// A richer successor to hover bars: supports all four edges, arbitrary
    /// margin widths, and larger growth ranges while keeping the same terminal-native
    /// partial-block vocabulary. Ideal for hover rails, bottom tabs, expanding pills,
    /// and container-edge emphasis.
    EdgeGrow {
        /// Width at rest in eighths of a cell.
        #[serde(default = "default_edge_grow_rest_eighths")]
        rest_eighths: u8,
        /// Width at full activation in eighths.
        #[serde(default = "default_edge_grow_peak_eighths")]
        peak_eighths: u8,
        /// Which edge grows outward from the content.
        #[serde(default)]
        edge: HoverBarPosition,
        /// Fill/accent color for the grown edge.
        #[serde(default = "default_edge_grow_fill")]
        fill_color: ColorConfig,
        /// Background color behind the grown edge.
        #[serde(default = "default_edge_grow_bg")]
        bg_color: ColorConfig,
        /// Progress (0.0-1.0) as raw number, signal, or runtime binding.
        #[serde(default = "default_edge_grow_progress")]
        progress: BindableValue,
        /// Available margin width on the active side.
        #[serde(default = "default_edge_grow_margin_width")]
        margin_width: u8,
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
        /// Hover progress (0.0 = not hovered, 1.0 = fully hovered). Accepts
        /// a raw number, signal spec, or runtime binding.
        #[serde(default)]
        progress: BindableValue,
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
        /// Hover progress (0.0 = not hovered, 1.0 = fully hovered). Accepts
        /// a raw number, signal spec, or runtime binding.
        #[serde(default)]
        progress: BindableValue,
        /// Smart powerline mode: bg on text, fg only on separator glyphs
        #[serde(default)]
        powerline_mode: bool,
        /// When true AND powerline_mode is true, also boost separator backgrounds.
        /// Use when powerline has continuous background (not terminal bg).
        #[serde(default)]
        boost_separator_bg: bool,
    },
    /// Horizontal scanner effect (KITT/Larson scanner or one-way wrap sweep)
    ///
    /// Creates a horizontal band of brightness that either sweeps in a
    /// continuous ping-pong pattern or wraps one-way for lighthouse-style beams.
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
        /// Animation progress (0.0 = inactive, 1.0 = fully active).
        ///
        /// Accepts a raw number (`0.5`), a signal spec, or a runtime binding
        /// (`{"binding": "scroll_progress"}`). When set to a binding, the
        /// value is resolved at render time from `ShaderRuntimeParams`.
        #[serde(default)]
        progress: BindableValue,
        /// Motion mode for the scanner sweep
        #[serde(default)]
        motion_mode: ScannerMotionMode,
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
        /// Animation progress (0.0 = inactive, 1.0 = fully active). Accepts
        /// a raw number, signal spec, or runtime binding.
        #[serde(default)]
        progress: BindableValue,
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

/// Which existing cell color channel to interpret as the light field.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum LightSampleFrom {
    Foreground,
    #[default]
    Background,
}

/// Glyph strategy for sub-cell light rendering.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum SubcellLightRenderMode {
    #[default]
    Braille,
    Horizontal,
    Vertical,
}

/// Which edge a directional vignette darkens from.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VignetteEdge {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

// Default functions for signal-or-float fields
fn default_dim_factor() -> SignalOrFloat {
    SignalOrFloat::Static(0.5)
}

fn default_tint_strength() -> SignalOrFloat {
    SignalOrFloat::Static(0.5)
}

fn default_fade_to_canvas_color() -> ColorConfig {
    ColorConfig::Rgb { r: 0, g: 0, b: 0 }
}

fn default_fade_to_canvas_apply_to() -> ApplyTo {
    ApplyTo::Both
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

fn default_charset_noise_hz() -> f32 {
    8.0
}

fn default_matrix_rain_density() -> BindableValue {
    BindableValue::static_f32(0.5)
}

fn default_matrix_rain_speed_multiplier() -> BindableValue {
    BindableValue::static_f32(1.0)
}

fn default_matrix_rain_speed_min() -> f32 {
    5.0
}

fn default_matrix_rain_speed_max() -> f32 {
    15.0
}

fn default_matrix_rain_trail_min() -> u16 {
    8
}

fn default_matrix_rain_trail_max() -> u16 {
    20
}

fn default_matrix_rain_glyph_change_hz() -> f32 {
    8.0
}

fn default_matrix_rain_seed() -> u64 {
    42
}

fn default_matrix_rain_head_color() -> ColorConfig {
    ColorConfig::Rgb {
        r: 220,
        g: 255,
        b: 220,
    }
}

fn default_matrix_rain_tail_color() -> ColorConfig {
    ColorConfig::Rgb { r: 0, g: 160, b: 0 }
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

fn default_bar_progress() -> BindableValue {
    BindableValue::static_f32(0.5)
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

fn default_subcell_light_lit() -> ColorConfig {
    ColorConfig::Rgb {
        r: 220,
        g: 220,
        b: 220,
    }
}

fn default_subcell_light_unlit() -> ColorConfig {
    ColorConfig::Rgb {
        r: 24,
        g: 24,
        b: 24,
    }
}

fn default_subcell_light_threshold() -> f32 {
    0.06
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

// EdgeGrow defaults
fn default_edge_grow_rest_eighths() -> u8 {
    2
}

fn default_edge_grow_peak_eighths() -> u8 {
    12
}

fn default_edge_grow_fill() -> ColorConfig {
    ColorConfig::Rgb {
        r: 100,
        g: 150,
        b: 200,
    }
}

fn default_edge_grow_bg() -> ColorConfig {
    ColorConfig::Rgb {
        r: 30,
        g: 30,
        b: 30,
    }
}

fn default_edge_grow_progress() -> BindableValue {
    BindableValue::static_f32(0.0)
}

fn default_edge_grow_margin_width() -> u8 {
    2
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
            FilterSpec::FadeToCanvas { .. } => "FadeToCanvas",
            FilterSpec::Vignette { .. } => "Vignette",
            FilterSpec::Crt { .. } => "Crt",
            FilterSpec::PatternFill { .. } => "PatternFill",
            FilterSpec::Greyscale { .. } => "Greyscale",
            FilterSpec::BrailleDust { .. } => "BrailleDust",
            FilterSpec::CharsetNoise { .. } => "CharsetNoise",
            FilterSpec::MatrixRain { .. } => "MatrixRain",
            FilterSpec::InterlaceCurtain { .. } => "InterlaceCurtain",
            FilterSpec::MotionBlur { .. } => "MotionBlur",
            FilterSpec::ColorBridgedShade { .. } => "ColorBridgedShade",
            FilterSpec::SubPixelBar { .. } => "SubPixelBar",
            FilterSpec::SubcellLight { .. } => "SubcellLight",
            FilterSpec::SubCellShake { .. } => "SubCellShake",
            FilterSpec::RigidShake { .. } => "RigidShake",
            FilterSpec::HoverBar { .. } => "HoverBar",
            FilterSpec::UnderlineWipe { .. } => "UnderlineWipe",
            FilterSpec::BracketEmphasis { .. } => "BracketEmphasis",
            FilterSpec::DotIndicator { .. } => "DotIndicator",
            FilterSpec::EdgeGrow { .. } => "EdgeGrow",
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
            FilterSpec::FadeToCanvas { .. } => {
                "Canvas-aware exit fade: blend cells toward a declared canvas color"
            }
            FilterSpec::Vignette { .. } => "Vignette darkening around edges",
            FilterSpec::Crt { .. } => "CRT monitor post-processing effect",
            FilterSpec::PatternFill { .. } => "Pattern fill effect for background textures",
            FilterSpec::Greyscale { .. } => "Greyscale/desaturate filter using BT.601 luminance",
            FilterSpec::BrailleDust { .. } => "Stochastic braille dust for frosted glass texture",
            FilterSpec::CharsetNoise { .. } => {
                "Non-converging time-varying character replacement for living textures"
            }
            FilterSpec::MatrixRain { .. } => {
                "Deterministic procedural digital-rain field with coherent falling streams"
            }
            FilterSpec::InterlaceCurtain { .. } => "Scanline/interlace effect for backdrop dimming",
            FilterSpec::MotionBlur { .. } => "Motion blur trail effect with directional dimming",
            FilterSpec::ColorBridgedShade { .. } => {
                "Color-bridged shade for smooth opacity rendering"
            }
            FilterSpec::SubPixelBar { .. } => "Sub-pixel progress bar with 8x resolution",
            FilterSpec::SubcellLight { .. } => {
                "Sub-cell light renderer for blank shell-owned cells"
            }
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
            FilterSpec::EdgeGrow { .. } => {
                "Generalized edge growth / stretch indicator using sub-cell blocks"
            }
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
            FilterSpec::FadeToCanvas {
                canvas_color,
                canvas_color_binding,
                strength,
                apply_to,
            } => vec![
                ("canvas_color", format!("{:?}", canvas_color)),
                (
                    "canvas_color_binding",
                    format!("{:?}", canvas_color_binding),
                ),
                ("strength", format!("{:?}", strength)),
                ("apply_to", format!("{:?}", apply_to)),
            ],
            FilterSpec::Vignette {
                strength,
                radius,
                sides,
                dither_amount,
                temporal_dither_hz,
            } => vec![
                ("strength", format!("{:?}", strength)),
                ("radius", format!("{:?}", radius)),
                ("sides", format!("{:?}", sides)),
                ("dither_amount", format!("{}", dither_amount)),
                ("temporal_dither_hz", format!("{}", temporal_dither_hz)),
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
            FilterSpec::CharsetNoise {
                hz, seed, jitter, ..
            } => vec![
                ("hz", format!("{}", hz)),
                ("seed", format!("{}", seed)),
                ("jitter", format!("{}", jitter)),
            ],
            FilterSpec::MatrixRain {
                density,
                speed_multiplier,
                speed_min,
                speed_max,
                trail_min,
                trail_max,
                glyph_change_hz,
                seed,
                preset,
                ..
            } => vec![
                ("density", format!("{:?}", density)),
                ("speed_multiplier", format!("{:?}", speed_multiplier)),
                ("speed_min", format!("{}", speed_min)),
                ("speed_max", format!("{}", speed_max)),
                ("trail_min", format!("{}", trail_min)),
                ("trail_max", format!("{}", trail_max)),
                ("glyph_change_hz", format!("{}", glyph_change_hz)),
                ("seed", format!("{}", seed)),
                ("preset", format!("{:?}", preset)),
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
                ("progress", format!("{:?}", progress)),
                ("direction", format!("{:?}", direction)),
            ],
            FilterSpec::SubcellLight {
                lit_color,
                unlit_color,
                render_mode,
                sample_from,
                threshold,
                temporal_dither_hz,
                only_blank,
            } => vec![
                ("lit_color", format!("{:?}", lit_color)),
                ("unlit_color", format!("{:?}", unlit_color)),
                ("render_mode", format!("{:?}", render_mode)),
                ("sample_from", format!("{:?}", sample_from)),
                ("threshold", format!("{}", threshold)),
                ("temporal_dither_hz", format!("{}", temporal_dither_hz)),
                ("only_blank", format!("{}", only_blank)),
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
                ("progress", format!("{:?}", progress)),
            ],
            FilterSpec::UnderlineWipe {
                direction,
                progress,
                gradient,
                glisten,
                ..
            } => vec![
                ("direction", format!("{:?}", direction)),
                ("progress", format!("{:?}", progress)),
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
                ("progress", format!("{:?}", progress)),
            ],
            FilterSpec::DotIndicator {
                indicator_char,
                position,
                progress,
                ..
            } => vec![
                ("indicator_char", format!("{}", indicator_char)),
                ("position", format!("{:?}", position)),
                ("progress", format!("{:?}", progress)),
            ],
            FilterSpec::EdgeGrow {
                rest_eighths,
                peak_eighths,
                edge,
                progress,
                margin_width,
                ..
            } => vec![
                ("rest_eighths", format!("{}", rest_eighths)),
                ("peak_eighths", format!("{}", peak_eighths)),
                ("edge", format!("{:?}", edge)),
                ("progress", format!("{:?}", progress)),
                ("margin_width", format!("{}", margin_width)),
            ],
            FilterSpec::PillButton {
                edge_width,
                glisten,
                progress,
                ..
            } => vec![
                ("edge_width", format!("{}", edge_width)),
                ("glisten", format!("{}", glisten)),
                ("progress", format!("{:?}", progress)),
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
                ("progress", format!("{:?}", progress)),
                ("powerline_mode", format!("{}", powerline_mode)),
                ("boost_separator_bg", format!("{}", boost_separator_bg)),
            ],
            FilterSpec::KittScanner {
                boost,
                band_width,
                bps,
                progress,
                motion_mode,
                apply_to,
                powerline_mode,
                boost_separator_bg,
            } => vec![
                ("boost", format!("{}", boost)),
                ("band_width", format!("{}", band_width)),
                ("bps", format!("{} Hz", bps)),
                ("progress", format!("{:?}", progress)),
                ("motion_mode", format!("{:?}", motion_mode)),
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
                ("progress", format!("{:?}", progress)),
            ],
        }
    }
}

// <FILE>tui-vfx-compositor/src/types/cls_filter_spec.rs</FILE> - <DESC>FilterSpec enum with signal-driven parameters</DESC>
// <VERS>END OF VERSION: 3.8.0</VERS>
