// <FILE>src/recipe_schema/config.rs</FILE> - <DESC>V2 recipe config types</DESC>
// <VERS>VERSION: 2.8.0</VERS>
// <WCTX>Hotfix H2 — add an additive top-level config.shadow field to RaRecipeConfig so recipe-owned shadow specs parse upstream without breaking the back-compat corpus.</WCTX>
// <CLOG>2.8.0: add optional opaque RaRecipeConfig.shadow: Option<ShadowSpec> so recipe-owned shadows parse upstream and round-trip without expanding the shadow schema surface.
// 2.7.0: add optional RaRecipeConfig.scene and RaPipelineConfig.continuous blocks plus per-effect clock selectors on RaMaskConfig/RaSamplerConfig/RaFilterConfig/RaStylePipelineConfig.
// 2.6.0: RaLifecycleConfig.loop (bool, default false) for looping recipes; RaContentConfig gains text + text_pool/effect_pool/preset_pool + image_name/image_pool + font_name/font_pool (all Option, serde skip_if_none). Precedence documented inline.
// 2.5.0: Expose StyleEffect::RigidShakeStyle to the recipe schema.

use serde::{Deserialize, Deserializer, Serialize};
use tui_vfx_compositor::types::{FilterSpec, MaskCombineMode, MaskSpec, SamplerSpec, ShadowSpec};
use tui_vfx_content::pool::{EffectPool, FontPool, ImagePool, PresetPool, TextPool};
use tui_vfx_content::types::ContentEffect;
use tui_vfx_geometry::easing::EasingType;
use tui_vfx_geometry::types::{
    AnchorSpec, EasingCurve, MotionSpec, PathType, PlacementSpec, RectScaleSpec, SnappingStrategy,
    TransitionSpec,
};
use tui_vfx_style::models::{
    ColorConfig, ColorSpace, FadeApplyTo, FadeTarget, SpatialShaderType, StyleEffect, StyleLayer,
    StyleRegion,
};

use crate::inspect;
use crate::inspector::InspectorContext;
use crate::recipe_schema::{RaClock, RaContinuousConfig, RaSceneConfig};
use crate::types::{Animation, AnimationProfile, AutoDismiss, SlideBorderTrimPolicy};
use std::time::Duration;

// ============================================================================
// V2 Layout Config
// ============================================================================

/// Layout mode determines how the notification size is calculated.
///
/// Controls whether the notification has fixed dimensions or scales to fill the terminal.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RaLayoutMode {
    /// Fixed dimensions (width/height from config).
    ///
    /// The notification uses explicit width and height values.
    #[default]
    Fixed,
    /// Scale to fill the entire terminal.
    ///
    /// The notification expands to match terminal dimensions, ignoring width/height.
    Fullscreen,
}

/// Animation type for entering and exiting transitions.
///
/// Determines the visual behavior of how the notification appears and disappears.
/// Each type uses different parameters from the transition config.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RaAnimationType {
    /// Slide in/out from the anchor direction.
    ///
    /// Default animation. The notification slides from the edge specified by the anchor.
    /// Compatible with `from`/`to` placement specs for offscreen positioning.
    #[default]
    Slide,
    /// Expand/collapse using rect_scale.
    ///
    /// The notification grows from or shrinks to a specified scale.
    /// Requires `rect_scale` in the transition config.
    ExpandCollapse,
    /// Fade in/out (position unchanged).
    ///
    /// The notification appears/disappears at its final position with opacity transition.
    /// No motion, only style fading.
    Fade,
    /// Custom motion using from/via/to waypoints.
    ///
    /// The notification follows an explicit motion path with optional intermediate waypoints.
    /// Requires `from`/`to` placement specs, optional `via` for curved paths.
    CustomMotion,
    /// No animation (instant appear/disappear).
    ///
    /// The notification appears and disappears instantly without transition.
    None,
}

/// Configuration for notification layout and positioning.
///
/// Controls the size and screen placement of the notification.
/// In Fixed mode, width/height determine the notification size.
/// In Fullscreen mode, the notification expands to fill the terminal.
#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaLayoutConfig {
    /// Layout mode (Fixed or Fullscreen).
    ///
    /// Default: `Fixed`
    #[serde(default)]
    pub mode: RaLayoutMode,

    /// Notification width in columns (ignored in Fullscreen mode).
    ///
    /// Default: 30, Range: 1-200
    #[config(help = "Toast width", default = 30, min = 1, max = 200)]
    pub width: u16,

    /// Notification height in rows (ignored in Fullscreen mode).
    ///
    /// Default: 3, Range: 1-50
    #[config(help = "Toast height", default = 3, min = 1, max = 50)]
    pub height: u16,

    /// Screen position anchor and alignment.
    ///
    /// Determines where the notification appears (e.g., top_left, bottom_center).
    /// See `AnchorSpec` for detailed positioning options.
    pub anchor: AnchorSpec,

    /// Whether to wrap text that exceeds the width.
    ///
    /// If false, text will be truncated (ellipsized).
    /// Default: true
    #[serde(default = "default_true")]
    #[config(help = "Wrap text", default = true)]
    pub wrap: bool,
}

fn default_true() -> bool {
    true
}

impl Default for RaLayoutConfig {
    fn default() -> Self {
        Self {
            mode: RaLayoutMode::Fixed,
            width: 30,
            height: 3,
            anchor: AnchorSpec::default(),
            wrap: true,
        }
    }
}

// ============================================================================
// V2 Lifecycle Config
// ============================================================================

/// Configuration for notification lifecycle timing.
///
/// Controls how long the notification remains visible before automatically dismissing.
/// Set to 0 to disable auto-dismiss (notification stays until manually dismissed).
#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaLifecycleConfig {
    /// Duration before auto-dismiss, in milliseconds.
    ///
    /// This is the total visible time (enter + dwell + exit).
    /// Set to 0 to disable auto-dismiss.
    ///
    /// Default: 4000 (4 seconds), Range: 0-120000 (0-2 minutes)
    #[config(
        help = "Auto-dismiss after (ms)",
        default = 4000,
        min = 0,
        max = 120_000
    )]
    pub auto_dismiss_ms: u64,

    /// Loop the recipe indefinitely until the user interrupts.
    ///
    /// When `true`, the recipe replays from t=0 each time it completes
    /// its `auto_dismiss_ms` cycle. Useful for title screens, demo mode,
    /// and splash screens that double as idle ambient motion. The user
    /// interrupt (Space / Esc at the host level) is the only exit path.
    ///
    /// Default: false (one-shot).
    #[config(help = "Loop until interrupted", default = false)]
    #[serde(rename = "loop")]
    pub r#loop: bool,
}

impl Default for RaLifecycleConfig {
    fn default() -> Self {
        Self {
            auto_dismiss_ms: 4000,
            r#loop: false,
        }
    }
}

// ============================================================================
// V2 Border Config
// ============================================================================

/// Border rendering style.
///
/// Determines which character set is used for the border. Can be overridden by
/// `custom_chars` for artistic borders, or replaced entirely by `frame` content.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RaBorderType {
    /// Rounded corners (┌─┐│└─┘).
    ///
    /// Default border style with smooth corners.
    #[default]
    Rounded,

    /// Plain ASCII corners (+-+|+-+).
    ///
    /// Compatible with terminals that don't support box-drawing characters.
    Plain,

    /// Double-line borders (╔═╗║╚═╝).
    ///
    /// Heavier visual weight than rounded.
    Double,

    /// Thick borders (┏━┓┃┗━┛).
    ///
    /// Bold appearance for emphasis.
    Thick,

    /// No border.
    ///
    /// Content fills the entire notification area.
    /// Useful when using `frame` content instead.
    None,
}

/// Border trim effect for slide animations.
///
/// Controls whether border edges fade or clip during enter/exit transitions.
/// Only applies when using Slide animation type.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RaBorderTrim {
    /// No special trim behavior.
    ///
    /// Border appears and disappears with the notification. Opt-in when a
    /// hard silhouette is deliberate (glitch/corruption themes, CRT cabinets,
    /// any recipe where the border punching past the edge is part of the
    /// visual intent).
    None,

    /// Border edge vanishes at screen boundary during slide.
    ///
    /// Default since 0.2.0. The leading/trailing border edge fades out as it
    /// slides offscreen, creating a seamless transition with the screen edge.
    /// This is what most notifications, drawers, and sliding overlays want.
    #[default]
    VanishingEdge,
}

/// Title position on the border.
///
/// Determines which border edge displays the optional title text.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RaTitlePosition {
    /// Title on top border.
    ///
    /// Default position. Title appears horizontally on the top edge.
    #[default]
    Top,

    /// Title on bottom border.
    ///
    /// Title appears horizontally on the bottom edge.
    Bottom,

    /// Vertical title on left border (rendered top-to-bottom).
    ///
    /// Title text is rotated 90° clockwise on the left edge.
    Left,

    /// Vertical title on right border (rendered top-to-bottom).
    ///
    /// Title text is rotated 90° clockwise on the right edge.
    Right,
}

/// Title horizontal alignment within the border.
///
/// Controls how the title text is aligned on horizontal borders (Top/Bottom).
/// Has no effect on vertical title positions (Left/Right).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RaTitleAlignment {
    /// Align title to the left edge.
    ///
    /// Default alignment.
    #[default]
    Left,

    /// Center title horizontally.
    Center,

    /// Align title to the right edge.
    Right,
}

/// Inner padding (cells from border to content).
///
/// Specifies spacing between the border and the message text.
/// All values default to 0 (content directly adjacent to border).
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(default, deny_unknown_fields)]
pub struct RaPaddingConfig {
    /// Top padding in cells.
    ///
    /// Default: 0
    #[serde(default)]
    pub top: u16,

    /// Right padding in cells.
    ///
    /// Default: 0
    #[serde(default)]
    pub right: u16,

    /// Bottom padding in cells.
    ///
    /// Default: 0
    #[serde(default)]
    pub bottom: u16,

    /// Left padding in cells.
    ///
    /// Default: 0
    #[serde(default)]
    pub left: u16,
}

fn is_default_padding(v: &RaPaddingConfig) -> bool {
    *v == RaPaddingConfig::default()
}

/// Custom border characters for artistic borders (braille, blocks, etc.).
///
/// Overrides the standard border_type with custom single-character glyphs.
/// Uses ratatui's Block widget, so characters must be single Unicode codepoints.
/// For multi-character patterns or animated borders, use `frame` instead.
#[derive(Debug, Clone, PartialEq, Eq, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RaCustomBorderChars {
    /// Top-left corner character (required).
    ///
    /// Example: "╔", "⎧", or braille patterns
    pub top_left: String,

    /// Top-right corner character (required).
    ///
    /// Example: "╗", "⎫", or braille patterns
    pub top_right: String,

    /// Bottom-left corner character (required).
    ///
    /// Example: "╚", "⎩", or braille patterns
    pub bottom_left: String,

    /// Bottom-right corner character (required).
    ///
    /// Example: "╝", "⎭", or braille patterns
    pub bottom_right: String,

    /// Horizontal line character for top edge.
    ///
    /// Default: "─"
    #[serde(default = "default_horizontal")]
    pub horizontal_top: String,

    /// Horizontal line character for bottom edge.
    ///
    /// Default: "─"
    #[serde(default = "default_horizontal")]
    pub horizontal_bottom: String,

    /// Vertical line character for left edge.
    ///
    /// Default: "│"
    #[serde(default = "default_vertical")]
    pub vertical_left: String,

    /// Vertical line character for right edge.
    ///
    /// Default: "│"
    #[serde(default = "default_vertical")]
    pub vertical_right: String,
}

fn default_horizontal() -> String {
    "─".to_string()
}

fn default_vertical() -> String {
    "│".to_string()
}

fn is_none_custom_chars(v: &Option<RaCustomBorderChars>) -> bool {
    v.is_none()
}

/// Frame content for direct rendering (bypasses Block widget).
///
/// Unlike `custom_chars` (which uses ratatui's border system), frame content
/// is drawn directly to the buffer as renderable content. This enables:
/// - Multi-character edge patterns (repeated to fill available space)
/// - Full effect/shader support via StyleRegion::BorderOnly
/// - Animated borders through content effects
/// - Per-cell styling of frame elements
///
/// When using frame content, set `border_type` to `None` since the frame replaces the border.
#[derive(Debug, Clone, PartialEq, Eq, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RaFrameContent {
    /// Top-left corner (can be multi-character) (required).
    ///
    /// Rendered once at the top-left position.
    pub top_left: String,

    /// Top-right corner (can be multi-character) (required).
    ///
    /// Rendered once at the top-right position.
    pub top_right: String,

    /// Bottom-left corner (can be multi-character) (required).
    ///
    /// Rendered once at the bottom-left position.
    pub bottom_left: String,

    /// Bottom-right corner (can be multi-character) (required).
    ///
    /// Rendered once at the bottom-right position.
    pub bottom_right: String,

    /// Top edge pattern (repeated to fill).
    ///
    /// This pattern repeats across the top edge between corners.
    /// Default: "─"
    #[serde(default = "default_frame_horizontal")]
    pub top: String,

    /// Bottom edge pattern (repeated to fill).
    ///
    /// This pattern repeats across the bottom edge between corners.
    /// Default: "─"
    #[serde(default = "default_frame_horizontal")]
    pub bottom: String,

    /// Left edge pattern (repeated to fill).
    ///
    /// This pattern repeats down the left edge between corners.
    /// Default: "│"
    #[serde(default = "default_frame_vertical")]
    pub left: String,

    /// Right edge pattern (repeated to fill).
    ///
    /// This pattern repeats down the right edge between corners.
    /// Default: "│"
    #[serde(default = "default_frame_vertical")]
    pub right: String,
}

fn default_frame_horizontal() -> String {
    "─".to_string()
}

fn default_frame_vertical() -> String {
    "│".to_string()
}

fn is_none_frame(v: &Option<RaFrameContent>) -> bool {
    v.is_none()
}

/// Configuration for border rendering and styling.
///
/// Controls all aspects of the notification border, including:
/// - Border style (type, custom characters, or frame content)
/// - Padding and content centering
/// - Optional title display
/// - Trim effects for slide animations
#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaBorderConfig {
    /// Border rendering style.
    ///
    /// Determines which character set is used. Set to `None` when using `frame`.
    /// Default: `Rounded`
    #[serde(rename = "type")]
    pub border_type: RaBorderType,

    /// Border trim effect for slide animations.
    ///
    /// Controls whether border edges fade during enter/exit.
    /// Default: `VanishingEdge` (since 0.2.0). Set to `None` explicitly to
    /// preserve the hard silhouette for glitch/corruption/CRT recipes.
    pub trim: RaBorderTrim,

    /// Inner padding (cells from border to content).
    ///
    /// Spacing between border and message text. All sides default to 0.
    #[serde(skip_serializing_if = "is_default_padding")]
    pub padding: RaPaddingConfig,

    /// Auto-center content within the notification area.
    ///
    /// When true, calculates padding to center the message text vertically/horizontally.
    /// Particularly useful with Fullscreen mode to center content in the terminal.
    /// Default: false
    #[serde(default)]
    pub center_content: bool,

    /// Optional title text displayed on the border.
    ///
    /// The title appears on the edge specified by `title_position`.
    /// Default: None (no title)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Vertical position of the title.
    ///
    /// Determines which border edge displays the title.
    /// Default: `Top`
    #[serde(skip_serializing_if = "is_default_title_position")]
    pub title_position: RaTitlePosition,

    /// Horizontal alignment of the title.
    ///
    /// Controls title alignment on horizontal borders (Top/Bottom).
    /// No effect on vertical positions (Left/Right).
    /// Default: `Left`
    #[serde(skip_serializing_if = "is_default_title_alignment")]
    pub title_alignment: RaTitleAlignment,

    /// Custom border characters (braille, blocks, etc.).
    ///
    /// When set, overrides `border_type` with custom single-character glyphs.
    /// Uses ratatui's Block widget (single Unicode codepoints only).
    /// For multi-character patterns, use `frame` instead.
    /// Default: None
    #[serde(skip_serializing_if = "is_none_custom_chars")]
    pub custom_chars: Option<RaCustomBorderChars>,

    /// Frame content for direct rendering (bypasses Block widget).
    ///
    /// Unlike `custom_chars`, frame content is drawn directly to the buffer,
    /// enabling multi-character patterns, effects, and animations on the border.
    /// When set, `border_type` should be `None` (frame replaces system border).
    /// Default: None
    #[serde(skip_serializing_if = "is_none_frame")]
    pub frame: Option<RaFrameContent>,
}

fn is_default_title_position(v: &RaTitlePosition) -> bool {
    *v == RaTitlePosition::Top
}

fn is_default_title_alignment(v: &RaTitleAlignment) -> bool {
    *v == RaTitleAlignment::Left
}

impl Default for RaBorderConfig {
    fn default() -> Self {
        Self {
            border_type: RaBorderType::Rounded,
            trim: RaBorderTrim::VanishingEdge,
            padding: RaPaddingConfig::default(),
            center_content: false,
            title: None,
            title_position: RaTitlePosition::Top,
            title_alignment: RaTitleAlignment::Left,
            custom_chars: None,
            frame: None,
        }
    }
}

// ============================================================================
// V2 Content Config (optional text effects)
// ============================================================================

/// Content effect playback mode.
///
/// Determines when and how content effects (text animations) play.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RaContentMode {
    /// Effect plays once during enter phase.
    ///
    /// Default mode. The effect completes before the notification enters dwell phase.
    #[default]
    EnterOnly,

    /// Effect loops continuously.
    ///
    /// The effect repeats throughout the notification's lifetime.
    /// Loop period is controlled by `time.loop_period_ms`.
    Loop,
}

/// Configuration for content (text) effects.
///
/// Controls optional text animations like typewriter, wave, or glitch effects.
/// Content effects are separate from style effects (which manipulate colors/modifiers).
#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaContentConfig {
    /// Playback mode for the content effect.
    ///
    /// Default: `EnterOnly`
    pub mode: RaContentMode,

    /// The content effect to apply.
    ///
    /// See `ContentEffect` for available text animation types.
    /// Default: None (no content effect)
    pub effect: Option<ContentEffect>,

    /// Static text payload. Substitutions like `{{lab_name}}`,
    /// `{{version}}` in this string are resolved against the caller's
    /// `Substitutions` map at render time.
    ///
    /// Takes a back seat to `text_pool` when that field is non-empty,
    /// and to `preset_pool` when a picked preset provides text. See the
    /// precedence rule below.
    ///
    /// Default: None.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Pool of text candidates rotated per launch. See [`TextPool`].
    /// Typical use: splash taglines, NPC dialog variants, error-message
    /// variety.
    ///
    /// Default: None (no pool).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_pool: Option<TextPool>,

    /// Pool of [`ContentEffect`]s rotated per launch. See [`EffectPool`].
    /// Pairs with `text_pool` for N×M combinatorial variety, or with
    /// `preset_pool` for curated pairings.
    ///
    /// Default: None.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect_pool: Option<EffectPool>,

    /// Pool of curated bundles (text + effect + image_name + font_name
    /// pairings). See [`PresetPool`]. Takes precedence over the
    /// independent per-field pools when non-empty.
    ///
    /// Default: None.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preset_pool: Option<PresetPool>,

    /// Asset-map key for a static rocketsplash `.rss` image. The caller
    /// resolves this key to bytes via its `AssetMap`. Overridden by
    /// `image_pool` when that field has items, and by a picked preset's
    /// `image_name` when a preset is active.
    ///
    /// Default: None.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,

    /// Pool of rocketsplash `.rss` asset-map keys rotated per launch.
    /// See [`ImagePool`]. Pairs with `font_pool` for full brand
    /// rotation, or with `preset_pool` for curated logo + font
    /// pairings.
    ///
    /// Default: None.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_pool: Option<ImagePool>,

    /// Asset-map key for a static rocketsplash `.rsf` font atlas. The
    /// caller resolves this key to bytes via its `AssetMap`. Overridden
    /// by `font_pool` when that field has items.
    ///
    /// Default: None.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_name: Option<String>,

    /// Pool of rocketsplash `.rsf` asset-map keys rotated per launch.
    /// See [`FontPool`].
    ///
    /// Default: None.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_pool: Option<FontPool>,
}

/// Precedence for content resolution across the pool fields:
///
/// 1. If `preset_pool` is non-empty, pick a [`Preset`] and use its
///    `text`, `effect`, `image_name`, `font_name` where present.
///    Preset fields that are `None` fall through to rules 2–4.
/// 2. Else use `text_pool` (or static `text`) and `effect_pool` (or
///    static `effect`) independently — N×M combinatorial variety.
/// 3. Else use `image_pool` (or static `image_name`) / `font_pool`
///    (or static `font_name`) — asset fields resolve the same way.
/// 4. Else fall through to the host's default (no image, no font,
///    no text effect).
///
/// This precedence is implemented in the splash library runtime
/// (`gtd-ratatui::splash`), not on `RaContentConfig` itself — the
/// schema just declares all slots and leaves resolution policy to the
/// consumer. Other recipe consumers (toasts, banners, modals) may
/// apply different precedence rules if they need to.
impl Default for RaContentConfig {
    fn default() -> Self {
        Self {
            mode: RaContentMode::EnterOnly,
            effect: None,
            text: None,
            text_pool: None,
            effect_pool: None,
            preset_pool: None,
            image_name: None,
            image_pool: None,
            font_name: None,
            font_pool: None,
        }
    }
}

// ============================================================================
// V2 Time Config (optional looping)
// ============================================================================

/// Configuration for time-based looping effects.
///
/// Controls periodic repetition of style effects and content effects.
/// When enabled, effects loop with the specified period during the dwell phase.
#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaTimeConfig {
    /// Enable looping for effects.
    ///
    /// When true, effects repeat with the period specified by `loop_period_ms`.
    /// Default: false
    #[serde(rename = "loop")]
    pub is_loop: bool,

    /// Loop period in milliseconds.
    ///
    /// Duration of each loop cycle when `is_loop` is true.
    /// Default: 2000 (2 seconds)
    pub loop_period_ms: u64,
}

impl Default for RaTimeConfig {
    fn default() -> Self {
        Self {
            is_loop: false,
            loop_period_ms: 2000,
        }
    }
}

// ============================================================================
// V2 Transition Config (enter/exit with from/to placements)
// ============================================================================

/// Configuration for enter or exit transition animations.
///
/// Defines timing, easing, motion paths, and placement for a single transition phase.
/// Separate configs are used for enter and exit, allowing asymmetric animations.
#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaTransitionConfig {
    /// Semantic intent for this transition (std feature only).
    ///
    /// When set, the intent can be resolved against a theme to get the TimingSpec.
    /// Examples: "enter_major", "nav_peer", "exit_minor"
    /// Default: None
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,

    /// Transition duration in milliseconds.
    ///
    /// How long the enter or exit animation takes to complete.
    /// Default: 500, Range: 0-10000 (0-10 seconds)
    #[config(help = "Duration (ms)", default = 500, min = 0, max = 10_000)]
    pub duration_ms: u64,

    /// Easing curve for the transition.
    ///
    /// Controls acceleration/deceleration during the animation.
    /// Examples: `Linear`, `QuadOut`, `ElasticOut`
    /// Default: `Linear`
    pub easing: EasingCurve,

    /// Motion path type.
    ///
    /// Determines the trajectory between waypoints.
    /// Examples: `Linear`, `Arc`, `Bezier`
    /// Default: None (uses Linear)
    pub motion_path: Option<PathType>,

    /// Rectangle scaling spec for ExpandCollapse animations.
    ///
    /// Defines how the notification grows/shrinks.
    /// Required when `animation_type` is `ExpandCollapse`.
    /// Default: None
    pub rect_scale: Option<RectScaleSpec>,

    /// Coordinate snapping strategy.
    ///
    /// Determines how floating-point positions map to terminal cells.
    /// Default: `Round`
    pub snapping: SnappingStrategy,

    /// Starting placement (for enter) or ending placement (for exit).
    ///
    /// Defines an explicit waypoint. When set on enter, notification starts offscreen.
    /// When set on exit, notification moves to this position before disappearing.
    /// Default: None (uses anchor position)
    pub from: Option<PlacementSpec>,

    /// Target placement (for enter) or starting placement (for exit).
    ///
    /// The opposite endpoint of the transition. Typically the final resting position (enter)
    /// or the offscreen exit position (exit).
    /// Default: None (uses anchor position)
    pub to: Option<PlacementSpec>,

    /// Optional step quantization for pixel-art/retro animation style.
    ///
    /// When set, progress is snapped to discrete steps (e.g., 8 = 8 discrete frames).
    /// Creates a stepped, low-framerate aesthetic.
    /// Range: 2-60 steps
    /// Default: None (smooth animation)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[config(help = "Quantize steps for stepped animation", min = 2, max = 60)]
    pub quantize_steps: Option<u32>,
}

impl Default for RaTransitionConfig {
    fn default() -> Self {
        Self {
            intent: None,
            duration_ms: 500,
            easing: EasingCurve::Type(EasingType::Linear),
            motion_path: None,
            rect_scale: None,
            snapping: SnappingStrategy::Round,
            from: None,
            to: None,
            quantize_steps: None,
        }
    }
}

// ============================================================================
// V2 Mask Config (separate enter/exit masks with multiple mask support)
// ============================================================================

/// Deserializer that accepts either a single item or an array of items.
/// This enables backward-compatible JSON syntax:
/// - Single: `"enter": { "type": "Wipe", "direction": "Left" }`
/// - Array:  `"enter": [{ "type": "Wipe" }, { "type": "Dissolve" }]`
fn deserialize_mask_list<'de, D>(deserializer: D) -> Result<Vec<MaskSpec>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{self, SeqAccess, Visitor};

    struct MaskListVisitor;

    impl<'de> Visitor<'de> for MaskListVisitor {
        type Value = Vec<MaskSpec>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a mask spec or array of mask specs")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut masks = Vec::new();
            while let Some(mask) = seq.next_element()? {
                masks.push(mask);
            }
            Ok(masks)
        }

        fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            // Single mask object - wrap in Vec
            let mask = MaskSpec::deserialize(de::value::MapAccessDeserializer::new(map))?;
            if matches!(mask, MaskSpec::None) {
                Ok(Vec::new())
            } else {
                Ok(vec![mask])
            }
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Vec::new())
        }
    }

    deserializer.deserialize_any(MaskListVisitor)
}

/// Deserializer for filter lists - same pattern as mask lists.
fn deserialize_filter_list<'de, D>(deserializer: D) -> Result<Vec<FilterSpec>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{self, SeqAccess, Visitor};

    struct FilterListVisitor;

    impl<'de> Visitor<'de> for FilterListVisitor {
        type Value = Vec<FilterSpec>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a filter spec or array of filter specs")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut filters = Vec::new();
            while let Some(filter) = seq.next_element()? {
                filters.push(filter);
            }
            Ok(filters)
        }

        fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            // Single filter object - wrap in Vec
            let filter = FilterSpec::deserialize(de::value::MapAccessDeserializer::new(map))?;
            if matches!(filter, FilterSpec::None) {
                Ok(Vec::new())
            } else {
                Ok(vec![filter])
            }
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Vec::new())
        }
    }

    deserializer.deserialize_any(FilterListVisitor)
}

/// Default empty mask list
fn default_empty_mask_list() -> Vec<MaskSpec> {
    Vec::new()
}

/// Default empty filter list
fn default_empty_filter_list() -> Vec<FilterSpec> {
    Vec::new()
}

/// Configuration for alpha masks during different animation phases.
///
/// Masks control which parts of the notification are visible/transparent.
/// Multiple masks can be combined using the `combine_mode` strategy.
/// Separate mask lists for enter, dwell, and exit enable phase-specific reveals.
#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaMaskConfig {
    /// Masks applied during enter phase (combined via combine_mode).
    /// Reveals the notification progressively as it enters.
    /// Examples: wipe, dissolve, circular reveal.
    /// Can specify a single mask object or array of masks.
    /// Default: [] (no masks)
    #[serde(
        deserialize_with = "deserialize_mask_list",
        default = "default_empty_mask_list"
    )]
    pub enter: Vec<MaskSpec>,

    /// Masks applied during dwell phase (combined via combine_mode).
    /// Optional masks that animate while the notification is fully visible.
    /// Default: [] (no masks)
    #[serde(
        deserialize_with = "deserialize_mask_list",
        default = "default_empty_mask_list"
    )]
    pub dwell: Vec<MaskSpec>,

    /// Masks applied during exit phase (combined via combine_mode).
    /// Hides the notification progressively as it exits.
    /// Default: [] (no masks)
    #[serde(
        deserialize_with = "deserialize_mask_list",
        default = "default_empty_mask_list"
    )]
    pub exit: Vec<MaskSpec>,

    /// How to combine multiple masks when more than one is active.
    /// - `All` (AND): Pixel is visible only if all masks allow it
    /// - `Any` (OR): Pixel is visible if any mask allows it
    ///   Default: `All`
    #[serde(default)]
    pub combine_mode: MaskCombineMode,

    /// Optional clock override for every mask in this group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock: Option<RaClock>,
}

impl Default for RaMaskConfig {
    fn default() -> Self {
        Self {
            enter: Vec::new(),
            dwell: Vec::new(),
            exit: Vec::new(),
            combine_mode: MaskCombineMode::All,
            clock: None,
        }
    }
}

// ============================================================================
// V2 Sampler Config (separate enter/exit samplers)
// ============================================================================

/// Configuration for texture sampling during different animation phases.
///
/// Samplers apply texture or pattern overlays to the notification.
/// Examples: noise, gradients, dithering, halftone patterns.
/// Separate samplers for enter, dwell, and exit allow phase-specific texturing.
#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaSamplerConfig {
    /// Sampler applied during enter phase.
    ///
    /// Default: `None` (no sampling)
    pub enter: SamplerSpec,

    /// Sampler applied during dwell phase.
    ///
    /// Default: `None` (no sampling)
    pub dwell: SamplerSpec,

    /// Sampler applied during exit phase.
    ///
    /// Default: `None` (no sampling)
    pub exit: SamplerSpec,

    /// Optional clock override for this sampler group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock: Option<RaClock>,
}

impl Default for RaSamplerConfig {
    fn default() -> Self {
        Self {
            enter: SamplerSpec::None,
            dwell: SamplerSpec::None,
            exit: SamplerSpec::None,
            clock: None,
        }
    }
}

// ============================================================================
// V2 Filter Config (separate enter/exit filters with multiple filter support)
// ============================================================================

/// Configuration for post-processing filters during different animation phases.
///
/// Filters apply visual effects after all other rendering.
/// Examples: blur, pixelation, color grading, chromatic aberration.
/// Filters are applied in order, enabling complex filter chains.
#[derive(Debug, Clone, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaFilterConfig {
    /// Filters applied during enter phase (applied in order).
    /// Filter chain processes the notification as it enters.
    /// Can specify a single filter object or array of filters.
    /// Default: [] (no filters)
    #[serde(
        deserialize_with = "deserialize_filter_list",
        default = "default_empty_filter_list"
    )]
    pub enter: Vec<FilterSpec>,

    /// Filters applied during dwell phase (applied in order).
    /// Filter chain processes the notification while fully visible.
    /// Default: [] (no filters)
    #[serde(
        deserialize_with = "deserialize_filter_list",
        default = "default_empty_filter_list"
    )]
    pub dwell: Vec<FilterSpec>,

    /// Filters applied during exit phase (applied in order).
    /// Filter chain processes the notification as it exits.
    /// Default: [] (no filters)
    #[serde(
        deserialize_with = "deserialize_filter_list",
        default = "default_empty_filter_list"
    )]
    pub exit: Vec<FilterSpec>,

    /// Optional clock override for this filter group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock: Option<RaClock>,
}

// ============================================================================
// V2 Style Config (base style + enter/exit effects)
// ============================================================================

/// Color component selector for style effects.
///
/// Determines which color channels a style effect modifies.
/// Used by effects like FadeIn/FadeOut to target specific components.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RaApplyTo {
    /// Apply to both foreground and background colors.
    ///
    /// Default. Affects both text color and cell background.
    #[default]
    Both,

    /// Apply to foreground (text) color only.
    Foreground,

    /// Apply to background (cell) color only.
    Background,
}

fn to_fade_apply_to(apply_to: RaApplyTo) -> FadeApplyTo {
    match apply_to {
        RaApplyTo::Both => FadeApplyTo::Both,
        RaApplyTo::Foreground => FadeApplyTo::Foreground,
        RaApplyTo::Background => FadeApplyTo::Background,
    }
}

/// Style effect for color and modifier animations.
///
/// Style effects manipulate colors, text modifiers (bold, italic), and color shifts.
/// Each effect can target specific regions (All, TextOnly, BorderOnly) via the optional `region` field.
/// Effects run during specific phases (enter, dwell, exit) as configured in `RaStylePipelineConfig`.
#[derive(Debug, Clone, PartialEq, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RaStyleEffect {
    /// Fade in toward the base color from a configurable starting target.
    ///
    /// Default `from` is `black`, which historically rises out of darkness.
    /// **On a non-default canvas color this paints near-black over the canvas**
    /// during the first ~30% of the fade, which reads as the canvas being torn
    /// open. Set `from` to a `color` matching your canvas (or use a design-system
    /// template that injects it) so the widget appears to grow out of the
    /// canvas instead of out of black. For a fully custom color-to-color fade
    /// use `ColorFade` instead.
    FadeIn {
        /// Which color components to fade (Foreground, Background, or Both).
        ///
        /// Default: `Both`
        #[serde(default)]
        apply_to: RaApplyTo,

        /// Easing curve for the fade.
        ///
        /// Default: `Linear`
        #[serde(default)]
        easing: EasingCurve,

        /// Color to fade from. Default `black` for backward compatibility.
        ///
        /// Set to `{"type": "color", "color": {"r": 252, "g": 252, "b": 248}}`
        /// (or whatever your canvas color is) so the widget rises out of the
        /// canvas instead of out of black.
        #[serde(default)]
        from: FadeTarget,

        /// Optional region override (if not set, uses layer's region).
        ///
        /// Allows this effect to target a different region than the style layer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        region: Option<StyleRegion>,
    },

    /// Fade out from base color toward a configurable target.
    ///
    /// Default `to` is `black`. Set to a `color` matching your canvas to
    /// avoid the symmetric "black flash" issue on exit. For a fully custom
    /// color-to-color fade use `ColorFade` instead.
    FadeOut {
        /// Which color components to fade (Foreground, Background, or Both).
        ///
        /// Default: `Both`
        #[serde(default)]
        apply_to: RaApplyTo,

        /// Easing curve for the fade.
        ///
        /// Default: `Linear`
        #[serde(default)]
        easing: EasingCurve,

        /// Color to fade to. Default `black` for backward compatibility.
        #[serde(default)]
        to: FadeTarget,

        /// Optional region override (if not set, uses layer's region).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        region: Option<StyleRegion>,
    },

    /// Pulsing color effect.
    ///
    /// Rhythmically oscillates between base color and pulse_color.
    Pulse {
        /// Pulse frequency in Hz (cycles per second).
        frequency: f32,

        /// Target color for the pulse peak.
        #[serde(alias = "color")]
        pulse_color: ColorConfig,

        /// Optional region override (if not set, uses layer's region).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        region: Option<StyleRegion>,
    },

    /// Rainbow hue shift.
    ///
    /// Continuously rotates through the color spectrum.
    Rainbow {
        /// Rotation speed in degrees per second.
        ///
        /// Higher values = faster color cycling.
        #[serde(alias = "speed")]
        rotation_speed: f32,

        /// Optional region override (if not set, uses layer's region).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        region: Option<StyleRegion>,
    },

    /// Glitch effect with optional synchronized italic window.
    ///
    /// Randomly shifts colors and optionally forces italic during specific time windows.
    /// Useful for retro/cyberpunk aesthetics.
    Glitch {
        /// Random seed for glitch pattern.
        ///
        /// Different seeds produce different glitch patterns.
        seed: u64,

        /// Glitch intensity (0.0 = subtle, 1.0 = extreme).
        intensity: f32,

        /// Optional: force italic during this time window (syncs with content shift).
        ///
        /// Start time as normalized progress (0.0 = start of phase, 1.0 = end).
        #[serde(default)]
        italic_start: Option<f32>,

        /// End time for italic window (normalized progress).
        #[serde(default)]
        italic_end: Option<f32>,

        /// Optional region override (if not set, uses layer's region).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        region: Option<StyleRegion>,
    },

    /// Neon flicker effect.
    ///
    /// Simulates unstable neon lighting with random brightness fluctuations.
    NeonFlicker {
        /// Stability factor (0.0 = very flickery, 1.0 = stable).
        ///
        /// Lower values produce more chaotic flicker.
        stability: f32,

        /// Optional region override (if not set, uses layer's region).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        region: Option<StyleRegion>,
    },

    /// Spatial shader effect.
    ///
    /// Applies position-dependent color/style transformations.
    /// See `SpatialShaderType` for available shader types.
    Spatial {
        /// The spatial shader to apply.
        shader: SpatialShaderType,

        /// Optional region override (if not set, uses layer's region).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        region: Option<StyleRegion>,
    },

    /// Italic modifier during a time window.
    ///
    /// Forces italic styling during a specific time range.
    /// Useful for synchronized effects with glitch or other animations.
    ItalicWindow {
        /// Start time as normalized progress (0.0 = start of phase, 1.0 = end).
        start: f32,

        /// End time as normalized progress.
        end: f32,

        /// Optional region override (if not set, uses layer's region).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        region: Option<StyleRegion>,
    },

    /// Shift HSL color values over time.
    ///
    /// Applies static hue, saturation, and lightness adjustments.
    /// For animated shifts, combine with looping or use in dwell phase.
    ColorShift {
        /// Hue shift in degrees (-180 to 180).
        ///
        /// Rotates the color wheel. 180° inverts colors.
        hue_shift: f32,

        /// Saturation adjustment (-1.0 to 1.0).
        ///
        /// Negative values desaturate (toward grayscale), positive values saturate.
        saturation_shift: f32,

        /// Lightness adjustment (-1.0 to 1.0).
        ///
        /// Negative values darken, positive values lighten.
        lightness_shift: f32,

        /// Optional region override (if not set, uses layer's region).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        region: Option<StyleRegion>,
    },

    /// Fade toward a configurable target color.
    ///
    /// Interpolates from base color to target_color over the phase duration.
    /// More flexible than FadeIn/FadeOut which always use black.
    ColorFade {
        /// Target color to fade toward.
        #[serde(alias = "target")]
        target_color: ColorConfig,

        /// Color space for interpolation.
        ///
        /// Different color spaces produce different interpolation paths.
        /// Default: RGB (direct linear interpolation)
        #[serde(default)]
        color_space: ColorSpace,

        /// Optional region override (if not set, uses layer's region).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        region: Option<StyleRegion>,
    },

    /// Italic modifier synchronized with the RigidShake filter.
    ///
    /// Applies italic text styling when the rigid-shake oscillation is
    /// moving right. Used to keep text styling in sync with the physical
    /// edge vibration that `FilterSpec::RigidShake` produces, so that a
    /// complete "ketchup bottle shake" notification has coordinated
    /// positional motion AND text styling.
    ///
    /// For the effect to be synchronized with the shake filter, the
    /// timing parameters here must match the matching `RigidShake` filter
    /// exactly. If you use this style effect alone (without the filter),
    /// the italic alternation is still visible on its own clock.
    RigidShakeStyle {
        /// Duration of one back-and-forth shake in seconds.
        ///
        /// Default: 0.29 (matches FilterSpec::RigidShake default).
        #[serde(default = "default_rigid_shake_period")]
        shake_period: f32,

        /// Number of shakes before pause. Max 8.
        ///
        /// Default: 4 (matches FilterSpec::RigidShake default).
        #[serde(default = "default_rigid_num_shakes")]
        num_shakes: u8,

        /// Duration of pause between shake cycles in seconds.
        ///
        /// Default: 0.52 (matches FilterSpec::RigidShake default).
        #[serde(default = "default_rigid_pause_duration")]
        pause_duration: f32,

        /// Optional region override (if not set, uses layer's region).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        region: Option<StyleRegion>,
    },
}

fn default_rigid_shake_period() -> f32 {
    0.29
}

fn default_rigid_num_shakes() -> u8 {
    4
}

fn default_rigid_pause_duration() -> f32 {
    0.52
}

impl Default for RaStyleEffect {
    fn default() -> Self {
        Self::FadeIn {
            apply_to: RaApplyTo::Both,
            easing: EasingCurve::Type(EasingType::Linear),
            from: FadeTarget::Black,
            region: None,
        }
    }
}

impl RaStyleEffect {
    /// Get the optional region override for this effect.
    pub fn region(&self) -> Option<&StyleRegion> {
        match self {
            RaStyleEffect::FadeIn { region, .. } => region.as_ref(),
            RaStyleEffect::FadeOut { region, .. } => region.as_ref(),
            RaStyleEffect::Pulse { region, .. } => region.as_ref(),
            RaStyleEffect::Rainbow { region, .. } => region.as_ref(),
            RaStyleEffect::Glitch { region, .. } => region.as_ref(),
            RaStyleEffect::NeonFlicker { region, .. } => region.as_ref(),
            RaStyleEffect::Spatial { region, .. } => region.as_ref(),
            RaStyleEffect::ItalicWindow { region, .. } => region.as_ref(),
            RaStyleEffect::ColorShift { region, .. } => region.as_ref(),
            RaStyleEffect::ColorFade { region, .. } => region.as_ref(),
            RaStyleEffect::RigidShakeStyle { region, .. } => region.as_ref(),
        }
    }
}

impl RaStyleEffect {
    /// Convert to the runtime StyleEffect type (region is handled separately)
    pub fn to_style_effect(&self) -> StyleEffect {
        match self {
            RaStyleEffect::FadeIn {
                apply_to,
                easing,
                from,
                ..
            } => StyleEffect::FadeIn {
                apply_to: to_fade_apply_to(*apply_to),
                ease: *easing,
                from: *from,
            },
            RaStyleEffect::FadeOut {
                apply_to,
                easing,
                to,
                ..
            } => StyleEffect::FadeOut {
                apply_to: to_fade_apply_to(*apply_to),
                ease: *easing,
                to: *to,
            },
            RaStyleEffect::Pulse {
                frequency,
                pulse_color,
                ..
            } => StyleEffect::Pulse {
                frequency: *frequency,
                color: (*pulse_color).into(),
            },
            RaStyleEffect::Rainbow { rotation_speed, .. } => StyleEffect::Rainbow {
                speed: *rotation_speed,
            },
            RaStyleEffect::Glitch {
                seed,
                intensity,
                italic_start,
                italic_end,
                ..
            } => StyleEffect::Glitch {
                seed: *seed,
                intensity: *intensity,
                italic_start: *italic_start,
                italic_end: *italic_end,
            },
            RaStyleEffect::NeonFlicker { stability, .. } => StyleEffect::NeonFlicker {
                stability: *stability,
            },
            RaStyleEffect::Spatial { shader, .. } => StyleEffect::Spatial {
                shader: shader.clone(),
            },
            RaStyleEffect::ItalicWindow { start, end, .. } => StyleEffect::ItalicWindow {
                start: *start,
                end: *end,
            },
            RaStyleEffect::ColorShift {
                hue_shift,
                saturation_shift,
                lightness_shift,
                ..
            } => StyleEffect::ColorShift {
                hue_shift: *hue_shift,
                saturation_shift: *saturation_shift,
                lightness_shift: *lightness_shift,
            },
            RaStyleEffect::ColorFade {
                target_color,
                color_space,
                ..
            } => StyleEffect::ColorFade {
                target: (*target_color).into(),
                color_space: *color_space,
            },
            RaStyleEffect::RigidShakeStyle {
                shake_period,
                num_shakes,
                pause_duration,
                ..
            } => StyleEffect::RigidShakeStyle {
                shake_period: *shake_period,
                num_shakes: *num_shakes,
                pause_duration: *pause_duration,
            },
        }
    }
}

/// Base style configuration (colors and text modifiers).
///
/// Defines the initial appearance before any effects are applied.
/// This is the "resting state" that effects animate from/to.
#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaBaseStyle {
    /// Foreground (text) color.
    ///
    /// Default: `White`
    pub foreground: Option<ColorConfig>,

    /// Background (cell) color.
    ///
    /// Default: `Black`
    pub background: Option<ColorConfig>,

    /// Text modifiers to enable.
    ///
    /// Examples: "BOLD", "ITALIC", "UNDERLINED"
    /// Default: [] (no modifiers)
    #[serde(default)]
    pub added_modifiers: Vec<String>,

    /// Text modifiers to disable.
    ///
    /// Used to remove inherited modifiers.
    /// Default: [] (no removals)
    #[serde(default)]
    pub removed_modifiers: Vec<String>,
}

impl Default for RaBaseStyle {
    fn default() -> Self {
        Self {
            foreground: Some(ColorConfig::White),
            background: Some(ColorConfig::Black),
            added_modifiers: vec![],
            removed_modifiers: vec![],
        }
    }
}

/// Style layer configuration (base style + phase-specific effects).
///
/// A style layer defines:
/// - The region it targets (All, TextOnly, BorderOnly)
/// - Base colors and modifiers
/// - Optional effects for enter, dwell, and exit phases
///
/// Multiple style layers can be combined to target different regions simultaneously.
/// For example, one layer for text effects and another for border effects.
#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaStylePipelineConfig {
    /// The region this style layer applies to.
    ///
    /// - `All`: Affects entire notification (text + border + background)
    /// - `TextOnly`: Affects only the message text
    /// - `BorderOnly`: Affects only the border characters
    ///   Default: `All`
    pub region: StyleRegion,

    /// Base style (colors and modifiers before effects).
    ///
    /// The starting appearance that effects animate from/to.
    pub base_style: RaBaseStyle,

    /// Style effect during enter phase.
    ///
    /// Runs as the notification enters. Common: FadeIn, ColorFade.
    /// Default: None
    pub enter_effect: Option<RaStyleEffect>,

    /// Style effect during dwell phase.
    ///
    /// Runs while the notification is fully visible. Common: Pulse, Rainbow, Spatial shaders.
    /// Default: None
    pub dwell_effect: Option<RaStyleEffect>,

    /// Style effect during exit phase.
    ///
    /// Runs as the notification exits. Common: FadeOut, ColorFade.
    /// Default: None
    pub exit_effect: Option<RaStyleEffect>,

    /// Deprecated: Use `dwell_effect` with `Spatial` variant instead.
    ///
    /// This field is kept for backwards compatibility.
    pub spatial_shader: Option<SpatialShaderType>,

    /// Interactive state styling as a list of state entries (hover, focus, active, etc.).
    ///
    /// Each entry contains a state name and its style configuration.
    /// State names: "hover", "focus", "active", "disabled", "selected", "loading", "error", "success"
    /// Used with L3 framework adapters to provide interactive feedback.
    /// Default: Empty vec
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub interaction_states: Vec<crate::recipe_schema::interactions::StateStyleEntry>,

    /// Interactive element configuration.
    ///
    /// Specifies element ID, transition timing, and accessibility settings.
    /// Required if interaction_states is non-empty.
    /// Default: None
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub interaction_config: Option<crate::recipe_schema::interactions::InteractionConfig>,

    /// Optional clock override for this style layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock: Option<RaClock>,
}

impl Default for RaStylePipelineConfig {
    fn default() -> Self {
        Self {
            region: StyleRegion::All,
            base_style: RaBaseStyle::default(),
            enter_effect: None,
            dwell_effect: None,
            exit_effect: None,
            spatial_shader: None,
            interaction_states: Vec::new(),
            interaction_config: None,
            clock: None,
        }
    }
}

// ============================================================================
// V2 Pipeline Config (combines all pipeline stages)
// ============================================================================

/// Configuration for the entire visual effects pipeline.
///
/// The pipeline combines all rendering stages:
/// - Motion (enter/exit transitions)
/// - Masks (alpha/visibility control)
/// - Samplers (texture overlays)
/// - Filters (post-processing effects)
/// - Styles (color and modifier effects)
///
/// Each stage processes the notification in sequence during each phase (enter, dwell, exit).
#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize)]
pub struct RaPipelineConfig {
    /// Animation type (Slide, Fade, ExpandCollapse, CustomMotion, None).
    ///
    /// Determines the primary animation behavior. Can be overridden by specific
    /// transition configs (e.g., setting `rect_scale` implies ExpandCollapse).
    /// Default: `Slide`
    #[serde(default)]
    pub animation_type: RaAnimationType,

    /// Enter transition configuration.
    ///
    /// Controls timing, easing, and motion for the entry animation.
    /// Default: 500ms with QuadOut easing
    pub enter: RaTransitionConfig,

    /// Exit transition configuration.
    ///
    /// Controls timing, easing, and motion for the exit animation.
    /// Default: 750ms with QuadIn easing
    pub exit: RaTransitionConfig,

    /// Mask configuration for all phases.
    ///
    /// Controls alpha/visibility masks during enter, dwell, and exit.
    /// Default: No masks
    pub mask: RaMaskConfig,

    /// Sampler configuration for all phases.
    ///
    /// Controls texture overlays during enter, dwell, and exit.
    /// Default: No samplers
    pub sampler: RaSamplerConfig,

    /// Filter configuration for all phases.
    ///
    /// Controls post-processing filters during enter, dwell, and exit.
    /// Default: No filters
    pub filter: RaFilterConfig,

    /// Style layers - supports both single `style` and array `styles` in JSON.
    /// Multiple layers are processed in order; each can target different regions.
    /// Example: One layer for text (TextOnly) and another for border (BorderOnly).
    /// Default: Single layer with All region, white on black
    #[serde(rename = "styles")]
    pub styles: Vec<RaStylePipelineConfig>,

    /// Optional cross-phase continuous effect block.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuous: Option<RaContinuousConfig>,
}

impl Default for RaPipelineConfig {
    fn default() -> Self {
        Self {
            animation_type: RaAnimationType::default(),
            enter: RaTransitionConfig {
                duration_ms: 500,
                easing: EasingCurve::Type(EasingType::QuadOut),
                ..Default::default()
            },
            exit: RaTransitionConfig {
                duration_ms: 750,
                easing: EasingCurve::Type(EasingType::QuadIn),
                ..Default::default()
            },
            mask: RaMaskConfig::default(),
            sampler: RaSamplerConfig::default(),
            filter: RaFilterConfig::default(),
            styles: vec![RaStylePipelineConfig::default()],
            continuous: None,
        }
    }
}

fn style_layers_from_value(value: serde_json::Value) -> Result<Vec<RaStylePipelineConfig>, String> {
    if value.is_null() {
        return Ok(vec![RaStylePipelineConfig::default()]);
    }
    if let Ok(list) = serde_json::from_value::<Vec<RaStylePipelineConfig>>(value.clone()) {
        return Ok(list);
    }
    serde_json::from_value::<RaStylePipelineConfig>(value)
        .map(|style| vec![style])
        .map_err(|error| error.to_string())
}

impl<'de> Deserialize<'de> for RaPipelineConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error as _;

        let value = serde_json::Value::deserialize(deserializer)?;
        let object = value
            .as_object()
            .ok_or_else(|| D::Error::custom("RaPipelineConfig expects a JSON object"))?;
        let mut config = RaPipelineConfig::default();
        let mut saw_styles = false;

        for (key, raw_value) in object {
            match key.as_str() {
                "animation_type" => {
                    config.animation_type =
                        serde_json::from_value(raw_value.clone()).map_err(D::Error::custom)?;
                }
                "enter" => {
                    config.enter =
                        serde_json::from_value(raw_value.clone()).map_err(D::Error::custom)?;
                }
                "exit" => {
                    config.exit =
                        serde_json::from_value(raw_value.clone()).map_err(D::Error::custom)?;
                }
                "mask" => {
                    config.mask =
                        serde_json::from_value(raw_value.clone()).map_err(D::Error::custom)?;
                }
                "sampler" => {
                    config.sampler =
                        serde_json::from_value(raw_value.clone()).map_err(D::Error::custom)?;
                }
                "filter" => {
                    config.filter =
                        serde_json::from_value(raw_value.clone()).map_err(D::Error::custom)?;
                }
                "styles" => {
                    config.styles =
                        style_layers_from_value(raw_value.clone()).map_err(D::Error::custom)?;
                    saw_styles = true;
                }
                "style" => {
                    if !saw_styles {
                        config.styles =
                            style_layers_from_value(raw_value.clone()).map_err(D::Error::custom)?;
                    }
                }
                "continuous" => {
                    config.continuous = if raw_value.is_null() {
                        None
                    } else {
                        Some(serde_json::from_value(raw_value.clone()).map_err(D::Error::custom)?)
                    };
                }
                other => {
                    return Err(D::Error::unknown_field(
                        other,
                        &[
                            "animation_type",
                            "enter",
                            "exit",
                            "mask",
                            "sampler",
                            "filter",
                            "style",
                            "styles",
                            "continuous",
                        ],
                    ));
                }
            }
        }

        Ok(config)
    }
}

// ============================================================================
// V2 Notification Config (top-level)
// ============================================================================

/// Complete configuration for a V2 notification.
///
/// This is the root type for all V2 notification recipes. It combines:
/// - **message**: The text to display
/// - **layout**: Size and positioning
/// - **lifecycle**: Auto-dismiss timing
/// - **border**: Border style, padding, and optional title
/// - **content**: Optional text effects (typewriter, wave, etc.)
/// - **time**: Optional looping for effects
/// - **pipeline**: The complete visual effects pipeline (motion, masks, filters, styles)
///
/// # Example
///
/// ```json
/// {
///   "message": "Hello, world!",
///   "layout": {
///     "width": 40,
///     "height": 5,
///     "anchor": { "position": "top_center", "offset": { "x": 0, "y": 1 } }
///   },
///   "lifecycle": { "auto_dismiss_ms": 3000 },
///   "border": { "type": "rounded", "padding": { "left": 2, "right": 2 } },
///   "pipeline": {
///     "animation_type": "slide",
///     "enter": { "duration_ms": 500, "easing": "quad_out" },
///     "exit": { "duration_ms": 750, "easing": "quad_in" },
///     "styles": {
///       "region": "all",
///       "base_style": { "foreground": "cyan", "background": "black" },
///       "enter_effect": { "type": "fade_in" },
///       "exit_effect": { "type": "fade_out" }
///     }
///   }
/// }
/// ```
#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaRecipeConfig {
    /// Motion theme to use for resolving intents (std feature only).
    ///
    /// When set, intent strings in transitions can be resolved against this theme
    /// to get the appropriate TimingSpec. Can reference built-in themes by name
    /// (e.g., "material3", "carbon", "fluent") or custom theme files.
    /// Default: None
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,

    /// The notification message text.
    ///
    /// Supports multiline text with `\n` or actual line breaks in JSON.
    /// Default: "Hello"
    pub message: String,

    /// Layout and positioning configuration.
    ///
    /// Controls notification size (width/height) and screen anchor.
    pub layout: RaLayoutConfig,

    /// Lifecycle timing configuration.
    ///
    /// Controls auto-dismiss duration (total visible time).
    pub lifecycle: RaLifecycleConfig,

    /// Border rendering and styling.
    ///
    /// Controls border type, padding, title, and optional custom characters.
    pub border: RaBorderConfig,

    /// Optional content (text) effects.
    ///
    /// Enables text animations like typewriter, wave, or glitch.
    /// Default: None (no content effects)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<RaContentConfig>,

    /// Optional time-based looping.
    ///
    /// Enables periodic effect repetition during the dwell phase.
    /// Default: None (no looping)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<RaTimeConfig>,

    /// Optional compositor-integrated shadow for the entire recipe surface.
    ///
    /// Uses the upstream `ShadowSpec` serde shape directly so recipe JSON can own
    /// shadow behavior (including `source_region`) without duplicating the shadow
    /// schema inside `tui-vfx-recipes`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[config(opaque)]
    pub shadow: Option<ShadowSpec>,

    /// Visual effects pipeline configuration.
    ///
    /// Combines all rendering stages: motion, masks, samplers, filters, and styles.
    /// This is where the majority of visual customization happens.
    pub pipeline: RaPipelineConfig,

    /// Optional scene-composer block for authored semantic layers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scene: Option<RaSceneConfig>,

    /// Required compositor primitives (internal use).
    ///
    /// Lists compositor features this notification depends on.
    /// Used for capability checking and graceful degradation.
    /// Default: [] (no special requirements)
    #[serde(default)]
    pub requires_primitives: Vec<String>,
}

impl Default for RaRecipeConfig {
    fn default() -> Self {
        Self {
            theme: None,
            message: "Hello".to_string(),
            layout: RaLayoutConfig::default(),
            lifecycle: RaLifecycleConfig::default(),
            border: RaBorderConfig::default(),
            content: None,
            time: None,
            shadow: None,
            pipeline: RaPipelineConfig::default(),
            scene: None,
            requires_primitives: vec![],
        }
    }
}

impl RaRecipeConfig {
    /// Build an AnimationProfile from this V2 config with inspection hooks.
    ///
    /// Use this variant when you need to trace the pipeline for debugging.
    /// Calls inspector methods at key points:
    /// - `on_config_parsed` after config is available
    /// - `on_profile_built` after AnimationProfile is constructed
    /// - `on_style_layers_extracted` after extracting effective style layers
    pub fn to_animation_profile_with_inspector(
        &self,
        ctx: &mut InspectorContext,
    ) -> AnimationProfile {
        inspect!(ctx, on_config_parsed, self);

        let profile = self.to_animation_profile();

        inspect!(ctx, on_profile_built, &profile);

        let layers = profile.effective_style_layers();
        inspect!(ctx, on_style_layers_extracted, &layers);

        profile
    }

    /// Build an AnimationProfile from this V2 config.
    pub fn to_animation_profile(&self) -> AnimationProfile {
        let enter_motion = Some(MotionSpec {
            duration_ms: self.pipeline.enter.duration_ms,
            ease: self.pipeline.enter.easing,
            path: self
                .pipeline
                .enter
                .motion_path
                .clone()
                .unwrap_or(PathType::Linear),
            snap: self.pipeline.enter.snapping.clone(),
            from: self.pipeline.enter.from,
            via: None,
            to: self.pipeline.enter.to,
        });

        let exit_motion = Some(MotionSpec {
            duration_ms: self.pipeline.exit.duration_ms,
            ease: self.pipeline.exit.easing,
            path: self
                .pipeline
                .exit
                .motion_path
                .clone()
                .unwrap_or(PathType::Linear),
            snap: self.pipeline.exit.snapping.clone(),
            from: self.pipeline.exit.from,
            via: None,
            to: self.pipeline.exit.to,
        });

        let loop_period = self.time.as_ref().and_then(|time| {
            if time.is_loop && time.loop_period_ms > 0 {
                Some(Duration::from_millis(time.loop_period_ms))
            } else {
                None
            }
        });

        // Build style layers from V2 styles array
        let style_layers: Vec<StyleLayer> = self
            .pipeline
            .styles
            .iter()
            .map(|v2_layer| {
                // Get dwell effect, with spatial_shader as fallback
                let dwell_effect = v2_layer
                    .dwell_effect
                    .as_ref()
                    .map(|e| e.to_style_effect())
                    .or_else(|| {
                        v2_layer
                            .spatial_shader
                            .as_ref()
                            .map(|s| StyleEffect::Spatial { shader: s.clone() })
                    });

                StyleLayer {
                    region: v2_layer.region.clone(),
                    enter_effect: v2_layer.enter_effect.as_ref().map(|e| e.to_style_effect()),
                    enter_region: v2_layer
                        .enter_effect
                        .as_ref()
                        .and_then(|e| e.region().cloned()),
                    dwell_effect,
                    dwell_region: v2_layer
                        .dwell_effect
                        .as_ref()
                        .and_then(|e| e.region().cloned()),
                    exit_effect: v2_layer.exit_effect.as_ref().map(|e| e.to_style_effect()),
                    exit_region: v2_layer
                        .exit_effect
                        .as_ref()
                        .and_then(|e| e.region().cloned()),
                }
            })
            .collect();

        // For backwards compatibility, also populate legacy fields from first layer
        let first_layer = self.pipeline.styles.first();
        let legacy_enter =
            first_layer.and_then(|l| l.enter_effect.as_ref().map(|e| e.to_style_effect()));
        let legacy_dwell = first_layer.and_then(|l| {
            l.dwell_effect
                .as_ref()
                .map(|e| e.to_style_effect())
                .or_else(|| {
                    l.spatial_shader
                        .as_ref()
                        .map(|s| StyleEffect::Spatial { shader: s.clone() })
                })
        });
        let legacy_exit =
            first_layer.and_then(|l| l.exit_effect.as_ref().map(|e| e.to_style_effect()));
        let legacy_region = first_layer
            .map(|l| l.region.clone())
            .unwrap_or(StyleRegion::All);

        AnimationProfile {
            enter: TransitionSpec {
                duration_ms: self.pipeline.enter.duration_ms,
                ease: self.pipeline.enter.easing,
                path: self
                    .pipeline
                    .enter
                    .motion_path
                    .clone()
                    .unwrap_or(PathType::Linear),
                snap: self.pipeline.enter.snapping.clone(),
                quantize_steps: self.pipeline.enter.quantize_steps,
            },
            exit: TransitionSpec {
                duration_ms: self.pipeline.exit.duration_ms,
                ease: self.pipeline.exit.easing,
                path: self
                    .pipeline
                    .exit
                    .motion_path
                    .clone()
                    .unwrap_or(PathType::Linear),
                snap: self.pipeline.exit.snapping.clone(),
                quantize_steps: self.pipeline.exit.quantize_steps,
            },
            enter_motion,
            exit_motion,
            style_layers,
            // Legacy fields for backwards compatibility
            enter_style: legacy_enter,
            dwell_style: legacy_dwell,
            exit_style: legacy_exit,
            shader_region: legacy_region,
            loop_period,
        }
    }

    /// Determine animation type from config.
    /// Explicit animation_type field takes precedence, otherwise inferred from enter config.
    pub fn animation_type(&self) -> Animation {
        // Explicit animation type takes precedence
        match self.pipeline.animation_type {
            RaAnimationType::Slide => {
                // Slide is the default - check if we should infer something else
                if self.pipeline.enter.rect_scale.is_some() {
                    Animation::ExpandCollapse
                } else if self.pipeline.enter.from.is_some() {
                    Animation::Motion
                } else {
                    Animation::Slide
                }
            }
            RaAnimationType::ExpandCollapse => Animation::ExpandCollapse,
            RaAnimationType::Fade => Animation::Fade,
            RaAnimationType::CustomMotion => Animation::Motion,
            RaAnimationType::None => Animation::None,
        }
    }

    /// Get auto-dismiss duration.
    pub fn auto_dismiss(&self) -> AutoDismiss {
        AutoDismiss::After(Duration::from_millis(self.lifecycle.auto_dismiss_ms))
    }

    /// Get border trim policy.
    pub fn border_trim_policy(&self) -> SlideBorderTrimPolicy {
        match self.border.trim {
            RaBorderTrim::VanishingEdge => SlideBorderTrimPolicy::VanishingEdge,
            RaBorderTrim::None => SlideBorderTrimPolicy::None,
        }
    }

    /// Extract the slide direction from the recipe's `pipeline.enter.from` field.
    /// Returns `SlideDirection::Default` when `from` is absent or not an
    /// `Offscreen` placement (e.g. Anchor, FramePermille, Absolute).
    /// Consumers use this to align vanishing-edge trim with the actual axis of
    /// motion rather than the anchor-derived diagonal fallback.
    pub fn slide_entry_direction(&self) -> tui_vfx_geometry::types::SlideDirection {
        use tui_vfx_geometry::types::{PlacementSpec, SlideDirection};
        match &self.pipeline.enter.from {
            Some(PlacementSpec::Offscreen { direction, .. }) => *direction,
            _ => SlideDirection::Default,
        }
    }

    /// Whether entry should be offscreen.
    pub fn slide_entry_offscreen(&self) -> bool {
        self.pipeline.enter.from.is_some()
    }

    /// Whether exit should be offscreen.
    pub fn slide_exit_offscreen(&self) -> bool {
        self.pipeline.exit.to.is_some()
    }
}

// <FILE>src/recipe_schema/config.rs</FILE> - <DESC>V2 recipe config types</DESC>
// <VERS>END OF VERSION: 2.8.0</VERS>
