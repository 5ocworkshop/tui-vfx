// <FILE>tui-vfx-content/src/types/cls_content_effect.rs</FILE> - <DESC>ContentEffect enum with all content transformations</DESC>
// <VERS>VERSION: 2.16.0</VERS>
// <WCTX>Packet 69-A: promote 11 rate-bearing fields from SignalOrFloat to VfxBindableValue so host applications can drive content-effect rates per frame via runtime bindings.</WCTX>
// <CLOG>2.16.0: MINOR — change 11 rate-bearing fields (Typewriter.speed_variance, Scramble.resolve_pace, GlitchShift.{glitch_start, glitch_end}, ScrambleGlitchShift.{resolve_pace, glitch_start, glitch_end}, SplitFlap.{speed, cascade, cycles}, Marquee.speed) from SignalOrFloat to VfxBindableValue. Wire format preserved via VfxBindableRepr's lenient deserializer; existing JSON parses byte-equivalently. Inherent doctest and apply_with_runtime doc reference updated.</CLOG>

//! # Content Effects
//!
//! Content effects transform the actual text content before rendering.
//! Unlike style effects (which modify colors/modifiers) or filters
//! (which transform coordinates), content effects operate on character data.
//!
//! ## Effect Categories
//!
//! | Category | Effects | Description |
//! |----------|---------|-------------|
//! | **Typing** | [`Typewriter`], [`SplitFlap`], [`Odometer`] | Character reveal animations |
//! | **Text Corruption** | [`Scramble`], [`GlitchShift`], [`ScrambleGlitchShift`] | Distortion and noise |
//! | **Transitions** | [`Dissolve`], [`Morph`], [`Mirror`] | Text transformation effects |
//! | **Movement** | [`Marquee`], [`SlideShift`] | Scrolling and sliding text |
//! | **Display** | [`Redact`], [`Numeric`], [`WrapIndicator`] | Text formatting effects |
//!
//! [`Typewriter`]: ContentEffect::Typewriter
//! [`SplitFlap`]: ContentEffect::SplitFlap
//! [`Odometer`]: ContentEffect::Odometer
//! [`Scramble`]: ContentEffect::Scramble
//! [`GlitchShift`]: ContentEffect::GlitchShift
//! [`ScrambleGlitchShift`]: ContentEffect::ScrambleGlitchShift
//! [`Dissolve`]: ContentEffect::Dissolve
//! [`Morph`]: ContentEffect::Morph
//! [`Mirror`]: ContentEffect::Mirror
//! [`Marquee`]: ContentEffect::Marquee
//! [`SlideShift`]: ContentEffect::SlideShift
//! [`Redact`]: ContentEffect::Redact
//! [`Numeric`]: ContentEffect::Numeric
//! [`WrapIndicator`]: ContentEffect::WrapIndicator
//!
//! ## Bindable Rate Parameters
//!
//! Rate-bearing parameters (`speed_variance`, `resolve_pace`, `glitch_start`,
//! `glitch_end`, `speed`, `cascade`, `cycles`) accept the three forms of
//! [`VfxBindableValue`]: a static literal, a host-supplied runtime binding,
//! or a signal expression evaluated per frame.
//!
//! ```json
//! { "type": "typewriter", "speed_variance": 0.2 }                          // literal
//! { "type": "typewriter", "speed_variance": { "binding": "typing_jitter" }} // runtime binding
//! { "type": "typewriter", "speed_variance": { "signal": { "sine": { "frequency_hz": 0.5 } } }} // signal
//! ```
//!
//! Bindings resolve against the host's `ShaderRuntimeParams` map; legacy
//! `{"static": N}` shapes continue to parse via the lenient bare-signal fallback.
//!
//! [`VfxBindableValue`]: tui_vfx_core::bindable::VfxBindableValue

use super::cls_dissolve_config::{DissolveDirection, DissolvePattern, DissolveReplacement};
use super::cls_glyph_cascade::{GlyphCascadeAlphabet, GlyphCascadeMode, GlyphCascadePattern};
use super::cls_mechanical_cycle_config::MechanicalCycleConfig;
use super::cls_mirror_axis::MirrorAxis;
use super::cls_morph_config::{MorphDirection, MorphProgression};
use super::cls_scramble_charset::ScrambleCharset;
use super::cls_slide_shift_flow_mode::SlideShiftFlowMode;
use super::cls_slide_shift_line_mode::SlideShiftLineMode;
use super::cls_typewriter_cursor::TypewriterCursor;
use tui_vfx_core::bindable::VfxBindableValue;

fn default_shift_width() -> u16 {
    1
}

fn default_tile_width() -> u16 {
    1
}

fn default_tile_height() -> u16 {
    1
}

/// Direction for mechanical Odometer tile rolling.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum OdometerDirection {
    #[default]
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

/// Travel distance policy for mechanical Odometer tile rolling.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum OdometerTravel {
    #[default]
    Axis,
    FullClear,
    Cells {
        cells: u16,
    },
}

/// Content effects that transform text before rendering.
///
/// Content effects operate on character data, modifying what text is displayed
/// rather than how it appears (style) or where it appears (filters/samplers).
///
/// # Applying an effect
///
/// The simplest way to drive a content effect is the inherent
/// [`apply`](Self::apply) method, which collapses the dispatcher +
/// [`SignalContext`](mixed_signals::prelude::SignalContext) +
/// [`Cow`](std::borrow::Cow) ceremony into one call:
///
/// ```
/// use tui_vfx_content::prelude::*;
/// use tui_vfx_core::bindable::VfxBindableValue;
///
/// let effect = ContentEffect::Typewriter {
///     speed_variance: VfxBindableValue::Literal(0.0),
///     cursor: None,
/// };
/// let revealed: String = effect.apply("Hello World", 0.5);
/// ```
///
/// For the borrowed-fast-path variant see
/// [`apply_to_borrowed`](Self::apply_to_borrowed); for signal- or
/// binding-driven pacing see
/// [`apply_with_runtime`](Self::apply_with_runtime).
///
/// # Categories
///
/// - **Typing Effects**: Reveal text character-by-character (Typewriter, SplitFlap, Odometer)
/// - **Corruption Effects**: Distort text with glitches and noise (Scramble, GlitchShift)
/// - **Transition Effects**: Transform between text states (Dissolve, Morph, Mirror)
/// - **Movement Effects**: Scroll or slide text (Marquee, SlideShift)
/// - **Display Effects**: Format text presentation (Redact, Numeric, WrapIndicator)
///
/// # JSON Configuration
///
/// ```json
/// { "type": "typewriter", "cursor": { "char": "_", "blink_rate": 2.0 } }
/// { "type": "scramble", "charset": "ascii", "seed": 42 }
/// { "type": "dissolve", "pattern": "random", "replacement": "space" }
/// ```
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
// The `Typewriter { cursor: Option<TypewriterCursor>, .. }` variant is
// substantially larger than the others because `TypewriterCursor` composes
// the general `Cursor` primitive via `#[serde(flatten)]` (which in turn
// carries `Wake { tint: ColorConfig, .. }`). Boxing `cursor` would be a
// breaking public-API change for every ContentEffect constructor; we accept
// the size delta since `ContentEffect` is rarely stored in large vectors.
#[allow(clippy::large_enum_variant)]
pub enum ContentEffect {
    /// Classic typewriter reveal effect.
    ///
    /// Characters appear one at a time with optional cursor. Great for
    /// text reveals, terminal aesthetics, or storytelling animations.
    ///
    /// # Parameters
    ///
    /// - `speed_variance`: Randomizes typing speed for organic feel
    /// - `cursor`: Optional blinking cursor at typing position
    Typewriter {
        /// Speed variation for organic typing feel (0.0 = steady, higher = more variable). Bindable.
        #[serde(default)]
        speed_variance: VfxBindableValue,
        /// Optional cursor displayed at the typing position.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cursor: Option<TypewriterCursor>,
    },

    /// Character scramble/decode effect.
    ///
    /// Displays random characters that progressively resolve into the target
    /// text. Creates a "hacking" or decryption aesthetic.
    ///
    /// # Character Sets
    ///
    /// Choose from predefined charsets (ASCII, digits, symbols) or custom.
    Scramble {
        /// How quickly characters resolve (higher = faster stabilization). Bindable.
        #[serde(default)]
        resolve_pace: VfxBindableValue,
        /// Character set for scrambled display.
        charset: ScrambleCharset,
        /// Seed for deterministic scramble patterns.
        seed: u64,
    },

    /// Horizontal shift glitch effect.
    ///
    /// Prepends spaces during a brief window to create a "shift right" glitch.
    /// Text clips at the border naturally without wrapping.
    GlitchShift {
        /// Number of characters to shift right (typically 4-6).
        shift_amount: u8,
        /// Progress value when glitch starts (0.0-1.0). Bindable.
        #[serde(default)]
        glitch_start: VfxBindableValue,
        /// Progress value when glitch ends (0.0-1.0). Bindable.
        #[serde(default)]
        glitch_end: VfxBindableValue,
        /// Seed for deterministic behavior.
        seed: u64,
    },

    /// Combined scramble and glitch shift effect.
    ///
    /// Scrambles text progressively while adding a brief horizontal shift
    /// glitch. Combines the aesthetics of both effects.
    ScrambleGlitchShift {
        /// How quickly characters resolve. Bindable.
        #[serde(default)]
        resolve_pace: VfxBindableValue,
        /// Character set for scrambled display.
        charset: ScrambleCharset,
        /// Seed for scramble pattern.
        scramble_seed: u64,
        /// Number of characters to shift right.
        shift_amount: u8,
        /// Progress value when glitch starts. Bindable.
        #[serde(default)]
        glitch_start: VfxBindableValue,
        /// Progress value when glitch ends. Bindable.
        #[serde(default)]
        glitch_end: VfxBindableValue,
    },

    /// Glyph-cascade / symbol-evolution effect.
    ///
    /// Transitions each character position through a configurable glyph alphabet
    /// (blocks, circles, braille, custom strings, etc.) according to a reveal order.
    /// This is a richer recipe-friendly evolve primitive: it can land on the target
    /// text, destabilize away from it, or stay in glyph-space for the full effect.
    GlyphCascade {
        /// Glyph alphabet used for the intermediate cascade.
        #[serde(default)]
        alphabet: GlyphCascadeAlphabet,
        /// Reveal ordering pattern across the text.
        #[serde(default)]
        pattern: GlyphCascadePattern,
        /// Direction for sequential patterns.
        #[serde(default)]
        direction: DissolveDirection,
        /// Seed for deterministic random ordering.
        #[serde(default)]
        seed: u64,
        /// How the glyph cascade interacts with the target text.
        #[serde(default)]
        mode: GlyphCascadeMode,
    },

    /// Airport/train station split-flap display.
    ///
    /// Characters flip through a character pool like mechanical departure
    /// boards (Solari, Alitalia, Frankfurt Hbf). The default `1x1` tile
    /// preserves legacy character flips; even `tile_height` values `2`, `4`,
    /// `6`, and `8` render larger center-hinged Solari cards.
    SplitFlap {
        /// Flip animation speed. Bindable.
        #[serde(default)]
        speed: VfxBindableValue,
        /// Cascade delay between characters (0 = simultaneous, higher = wave).
        /// Authentic Solari boards use cascade=0; distance-based staggering
        /// is provided by `authentic_timing` instead. Bindable.
        #[serde(default)]
        cascade: VfxBindableValue,
        /// Minimum full character-pool cycles each char walks before
        /// landing, so low-distance targets (like "A") still flip through
        /// a satisfying alphabet span. Bindable.
        #[serde(default)]
        cycles: VfxBindableValue,
        /// Per-character deterministic cascade-timing variance (0.0-1.0).
        /// Breaks lockstep landings; cells arrive in bursts like a real
        /// Solari board.
        #[serde(default)]
        jitter: f32,
        /// Character pool — Alpha (default, space+A-Z+digits+punctuation),
        /// Digits (for flight numbers / clocks), or Uppercase (for station
        /// names).
        #[serde(default)]
        charset: crate::transformers::SplitFlapCharset,
        /// Brief overshoot-and-bounce at the end of each char's
        /// progression — shows the target+1 character then settles.
        #[serde(default)]
        settle_overshoot: bool,
        /// Fraction of each char's progression spent cycling through
        /// █▓▒░ block density glyphs at the opening. Because █ fills
        /// the cell with the terminal fg color, this flashes a "full
        /// fg-color cursor" before any letters appear.
        #[serde(default)]
        leading_blocks: f32,
        /// Plays a 6-frame physical flap rotation at the end of each
        /// char's progression: █→▀→▔→—→▁→▄→letter. The upper-half (▀)
        /// and lower-half (▄) blocks and their one-eighth edge variants
        /// (▔/▁) simulate the top of an old card falling over a hinge
        /// and the bottom of a new card rising into place.
        #[serde(default)]
        settle_hinge: bool,
        /// Remap the hinge rotation through a DampedSpring curve so the
        /// card falls fast under gravity and bounces at the landing
        /// detent. Only applies when settle_hinge is true.
        #[serde(default)]
        spring_settle: bool,
        /// Physical-Solari arrival timing: all columns start rotating
        /// simultaneously at progress=0 from the blank position and each
        /// lands at a time proportional to its flap distance. The
        /// longest rotation lands at progress=1.0; shorter rotations
        /// land earlier. Matches real boards where staggered clacking
        /// comes from per-column distance, not sequential cascade.
        #[serde(default)]
        authentic_timing: bool,
        /// Message-to-message transition: each column rotates FROM the
        /// character at `from_message[i]` TO the character at `target[i]`,
        /// rotating through the shortest forward-only drum path. Columns
        /// whose character doesn't change do zero flaps and render
        /// instantly.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from_message: Option<String>,
        /// Replace the intermediate-letter walk with a continuous
        /// rolling-card animation. Each walk step plays through the
        /// 6-frame `█→▀→▔→—→▁→▄` hinge rotation, so the viewer sees
        /// cards physically tumbling over each other throughout the
        /// walk instead of watching the alphabet flicker past. The
        /// target letter is revealed only at progress=1.0. Best paired
        /// with low `cycles` (0.2–0.5) and longer animation durations
        /// so each rotation has enough frames to read as a flip.
        #[serde(default)]
        rolling_flip: bool,
        /// Substitute the `▔` hinge frame with the Unicode 180°-turned
        /// target glyph so viewers catch a brief upside-down preview of
        /// the arriving letter mid-flip. Requires `settle_hinge: true`.
        /// Falls back silently for targets without an honest turned form
        /// (Q, J, digits 1/2/4/5/7, non-ASCII). Default: false.
        #[serde(default)]
        flip_preview: bool,
        /// Replace the ordered hinge sequence with per-column-hashed
        /// random draws from a pool of block/hinge/letter glyphs,
        /// producing chaotic mechanical flicker (~30Hz). Requires
        /// `settle_hinge: true`. Opt-in — this is perceptible flicker
        /// and may be uncomfortable for motion-sensitive viewers.
        /// Default: false.
        #[serde(default)]
        flip_flicker: bool,
        /// Dispersion pattern — how per-column start delays are
        /// distributed across the board. `Legacy` (default) preserves
        /// pre-3.2.0 cascade+authentic_timing behavior; the other
        /// variants (cascade/authentic/simultaneous/random/center_out/
        /// edge_in/shuffled) override the old knobs with their own
        /// delay curve.
        #[serde(default)]
        dispersion: crate::transformers::SplitFlapDispersion,
        /// Tile width in character cells. `1` with `tile_height: 1` preserves legacy SplitFlap behavior.
        #[serde(default = "default_tile_width")]
        tile_width: u16,
        /// Tile height in character cells. `1` preserves legacy behavior; even heights 2/4/6/8 use center-hinged grid rendering.
        #[serde(default = "default_tile_height")]
        tile_height: u16,
    },

    /// Mechanical cell-grid rolling display.
    ///
    /// Odometer treats content as a fixed character-cell grid and rolls old
    /// cells out while target cells enter from the opposite edge. This is a
    /// mechanical board/window motion effect, not numeric interpolation: the
    /// previous vertical digit interpolation behavior was intentionally
    /// replaced and is not preserved.
    Odometer {
        /// Direction old content rolls toward while target content enters.
        direction: OdometerDirection,
        /// Distance policy for each roll.
        travel: OdometerTravel,
        /// Tile width in character cells; must be non-zero to animate.
        #[serde(default = "default_tile_width")]
        tile_width: u16,
        /// Tile height in character cells; must be non-zero to animate.
        #[serde(default = "default_tile_height")]
        tile_height: u16,
        /// Optional source message. Missing cells are padded with blanks by grid coordinate.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from_message: Option<String>,
        /// Optional ordered/circular cycle config. When absent (or set
        /// to the explicit-Pair default), Odometer uses the existing
        /// whole-grid pair-roll behavior byte-for-byte. When present
        /// with a non-Pair source, tiles are individually routed
        /// through the configured cycle, scheduled per the cascade
        /// policy, and settled per `MechanicalSettleConfig`. Cascade
        /// `NumericCarry` requires source/target numeric strings of
        /// matching tile count and a `decimal_digits` source preset.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        mechanical: Option<MechanicalCycleConfig>,
    },

    /// Text redaction/censorship effect.
    ///
    /// Replaces characters with a redaction symbol (typically █ or ▓).
    /// Progress controls how much text is revealed vs. redacted.
    Redact {
        /// Symbol used for redacted characters.
        symbol: char,
    },

    /// Numeric formatting effect.
    ///
    /// Formats numbers according to a format string. Useful for counters,
    /// statistics, or any numeric display.
    Numeric {
        /// Format string for number display.
        format: String,
    },

    /// Scrolling marquee text.
    ///
    /// Text scrolls horizontally through a fixed-width viewport.
    /// Classic for news tickers or limited-space displays.
    Marquee {
        /// Scroll speed. Bindable.
        #[serde(default)]
        speed: VfxBindableValue,
        /// Viewport width in characters.
        width: u16,
    },
    /// Sliding text with row jumps.
    ///
    /// Text slides horizontally and can jump to different rows when crossing
    /// a "shift barrier" column. Creates complex text movement patterns.
    ///
    /// # Use Cases
    ///
    /// - Menu item sliding animations
    /// - Text that wraps around corners
    /// - Multi-row reveal effects
    SlideShift {
        /// Starting column offset (in cells).
        start_col: i16,
        /// Ending column offset (in cells).
        end_col: i16,
        /// Base row offset (in lines).
        start_row: i16,
        /// Starting column of the shift barrier.
        shift_col: i16,
        /// Width of the shift barrier in cells (≥ 1).
        #[serde(default = "default_shift_width")]
        shift_width: u16,
        /// Row delta applied after crossing the barrier (negative = up).
        row_shift: i16,
        /// How horizontal shift applies across multi-line text.
        #[serde(default)]
        line_mode: SlideShiftLineMode,
        /// Whether to stay shifted or flow back after clearing the barrier.
        #[serde(default)]
        flow_mode: SlideShiftFlowMode,
    },

    /// Mirror/reverse text effect.
    ///
    /// Displays text mirrored along an axis during animation, returning
    /// to normal at completion. Useful for flip or rotation transitions.
    Mirror {
        /// Axis to mirror around (horizontal or vertical).
        axis: MirrorAxis,
    },

    /// Character-level dissolve effect.
    ///
    /// Progressively replaces characters with a replacement (space, block, etc.).
    /// Unlike mask dissolve which affects pixel visibility, this operates on
    /// actual character content.
    ///
    /// # Patterns
    ///
    /// - **Random**: Characters dissolve in random order
    /// - **Sequential**: Characters dissolve left-to-right or right-to-left
    /// - **Center-out**: Dissolve radiates from center
    Dissolve {
        /// What character to use for dissolved positions.
        #[serde(default)]
        replacement: DissolveReplacement,
        /// Pattern controlling dissolve order.
        #[serde(default)]
        pattern: DissolvePattern,
        /// Direction for sequential dissolve patterns.
        #[serde(default)]
        direction: DissolveDirection,
        /// Seed for random dissolve pattern.
        #[serde(default)]
        seed: u64,
    },

    /// Text morphing transition.
    ///
    /// Transitions characters from source text to target text. Characters
    /// change based on the progression pattern, creating smooth text
    /// transformations.
    ///
    /// # Example
    ///
    /// Morph "Hello" → "World" with characters transitioning individually.
    Morph {
        /// Source text to morph from (target is the normal content).
        source: String,
        /// How characters transition between states.
        #[serde(default)]
        progression: MorphProgression,
        /// Direction of the morph animation.
        #[serde(default)]
        direction: MorphDirection,
        /// Seed for scatter progression pattern.
        #[serde(default)]
        seed: u64,
    },

    /// Prefix/suffix wrapper for indicators.
    ///
    /// Wraps text with symbols based on animation progress.
    /// Perfect for hover indicators, selection markers, or attention callouts.
    ///
    /// # Example
    ///
    /// `"Menu Item"` → `"» Menu Item «"` when hovered.
    WrapIndicator {
        /// Prefix to prepend (e.g., "» ").
        prefix: String,
        /// Suffix to append (e.g., " «").
        suffix: String,
    },
}

impl ContentEffect {
    /// Returns the effect type name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            ContentEffect::Typewriter { .. } => "Typewriter",
            ContentEffect::Scramble { .. } => "Scramble",
            ContentEffect::GlitchShift { .. } => "GlitchShift",
            ContentEffect::ScrambleGlitchShift { .. } => "ScrambleGlitchShift",
            ContentEffect::GlyphCascade { .. } => "GlyphCascade",
            ContentEffect::SplitFlap { .. } => "SplitFlap",
            ContentEffect::Odometer { .. } => "Odometer",
            ContentEffect::Redact { .. } => "Redact",
            ContentEffect::Numeric { .. } => "Numeric",
            ContentEffect::Marquee { .. } => "Marquee",
            ContentEffect::SlideShift { .. } => "SlideShift",
            ContentEffect::Mirror { .. } => "Mirror",
            ContentEffect::Dissolve { .. } => "Dissolve",
            ContentEffect::Morph { .. } => "Morph",
            ContentEffect::WrapIndicator { .. } => "WrapIndicator",
        }
    }

    /// Returns a brief human-readable description of what this effect does.
    pub fn terse_description(&self) -> &'static str {
        match self {
            ContentEffect::Typewriter { .. } => "Classic typewriter reveal effect",
            ContentEffect::Scramble { .. } => "Character scramble/decode effect",
            ContentEffect::GlitchShift { .. } => "Horizontal shift glitch effect",
            ContentEffect::ScrambleGlitchShift { .. } => {
                "Combined scramble and glitch shift effect"
            }
            ContentEffect::GlyphCascade { .. } => "Glyph-cascade / symbol-evolution effect",
            ContentEffect::SplitFlap { .. } => "Airport/train station split-flap display",
            ContentEffect::Odometer { .. } => "Mechanical cell-grid rolling display",
            ContentEffect::Redact { .. } => "Text redaction/censorship effect",
            ContentEffect::Numeric { .. } => "Numeric formatting effect",
            ContentEffect::Marquee { .. } => "Scrolling marquee text",
            ContentEffect::SlideShift { .. } => "Sliding text with row jumps",
            ContentEffect::Mirror { .. } => "Mirror/reverse text effect",
            ContentEffect::Dissolve { .. } => "Character-level dissolve effect",
            ContentEffect::Morph { .. } => "Text morphing transition",
            ContentEffect::WrapIndicator { .. } => "Prefix/suffix wrapper for indicators",
        }
    }

    /// Returns key parameters of this effect for documentation purposes.
    pub fn key_parameters(&self) -> Vec<(&'static str, String)> {
        match self {
            ContentEffect::Typewriter {
                speed_variance,
                cursor,
            } => {
                let mut params = vec![("speed_variance", format!("{:?}", speed_variance))];
                if let Some(c) = cursor {
                    params.push(("cursor", format!("{:?}", c)));
                }
                params
            }
            ContentEffect::Scramble {
                resolve_pace,
                charset,
                seed,
            } => vec![
                ("resolve_pace", format!("{:?}", resolve_pace)),
                ("charset", format!("{:?}", charset)),
                ("seed", format!("{}", seed)),
            ],
            ContentEffect::GlitchShift {
                shift_amount,
                glitch_start,
                glitch_end,
                seed,
            } => vec![
                ("shift_amount", format!("{}", shift_amount)),
                ("glitch_start", format!("{:?}", glitch_start)),
                ("glitch_end", format!("{:?}", glitch_end)),
                ("seed", format!("{}", seed)),
            ],
            ContentEffect::ScrambleGlitchShift {
                resolve_pace,
                charset,
                scramble_seed,
                shift_amount,
                glitch_start,
                glitch_end,
            } => vec![
                ("resolve_pace", format!("{:?}", resolve_pace)),
                ("charset", format!("{:?}", charset)),
                ("scramble_seed", format!("{}", scramble_seed)),
                ("shift_amount", format!("{}", shift_amount)),
                ("glitch_start", format!("{:?}", glitch_start)),
                ("glitch_end", format!("{:?}", glitch_end)),
            ],
            ContentEffect::GlyphCascade {
                alphabet,
                pattern,
                direction,
                seed,
                mode,
            } => vec![
                ("alphabet", format!("{:?}", alphabet)),
                ("pattern", format!("{:?}", pattern)),
                ("direction", format!("{:?}", direction)),
                ("seed", format!("{}", seed)),
                ("mode", format!("{:?}", mode)),
            ],
            ContentEffect::SplitFlap {
                speed,
                cascade,
                cycles,
                jitter,
                charset,
                settle_overshoot,
                leading_blocks,
                settle_hinge,
                spring_settle,
                authentic_timing,
                from_message,
                rolling_flip,
                flip_preview,
                flip_flicker,
                dispersion,
                tile_width,
                tile_height,
            } => vec![
                ("speed", format!("{:?}", speed)),
                ("cascade", format!("{:?}", cascade)),
                ("cycles", format!("{:?}", cycles)),
                ("jitter", format!("{}", jitter)),
                ("charset", format!("{:?}", charset)),
                ("settle_overshoot", format!("{}", settle_overshoot)),
                ("leading_blocks", format!("{}", leading_blocks)),
                ("settle_hinge", format!("{}", settle_hinge)),
                ("spring_settle", format!("{}", spring_settle)),
                ("authentic_timing", format!("{}", authentic_timing)),
                (
                    "from_message",
                    from_message
                        .as_deref()
                        .map(|s| format!("\"{}\"", s))
                        .unwrap_or_else(|| "None".to_string()),
                ),
                ("rolling_flip", format!("{}", rolling_flip)),
                ("flip_preview", format!("{}", flip_preview)),
                ("flip_flicker", format!("{}", flip_flicker)),
                ("dispersion", format!("{:?}", dispersion)),
                ("tile_width", format!("{}", tile_width)),
                ("tile_height", format!("{}", tile_height)),
            ],
            ContentEffect::Odometer {
                direction,
                travel,
                tile_width,
                tile_height,
                from_message,
                mechanical,
            } => vec![
                ("direction", format!("{:?}", direction)),
                ("travel", format!("{:?}", travel)),
                ("tile_width", format!("{}", tile_width)),
                ("tile_height", format!("{}", tile_height)),
                ("from_message", format!("{:?}", from_message)),
                ("mechanical", format!("{:?}", mechanical)),
            ],
            ContentEffect::Redact { symbol } => vec![("symbol", format!("{}", symbol))],
            ContentEffect::Numeric { format } => vec![("format", format.clone())],
            ContentEffect::Marquee { speed, width } => vec![
                ("speed", format!("{:?}", speed)),
                ("width", format!("{}", width)),
            ],
            ContentEffect::SlideShift {
                start_col,
                end_col,
                start_row,
                shift_col,
                shift_width,
                row_shift,
                line_mode,
                flow_mode,
            } => vec![
                ("start_col", format!("{}", start_col)),
                ("end_col", format!("{}", end_col)),
                ("start_row", format!("{}", start_row)),
                ("shift_col", format!("{}", shift_col)),
                ("shift_width", format!("{}", shift_width)),
                ("row_shift", format!("{}", row_shift)),
                ("line_mode", format!("{:?}", line_mode)),
                ("flow_mode", format!("{:?}", flow_mode)),
            ],
            ContentEffect::Mirror { axis } => vec![("axis", format!("{:?}", axis))],
            ContentEffect::Dissolve {
                replacement,
                pattern,
                direction,
                seed,
            } => vec![
                ("replacement", format!("{:?}", replacement)),
                ("pattern", format!("{:?}", pattern)),
                ("direction", format!("{:?}", direction)),
                ("seed", format!("{}", seed)),
            ],
            ContentEffect::Morph {
                source,
                progression,
                direction,
                seed,
            } => vec![
                ("source", source.clone()),
                ("progression", format!("{:?}", progression)),
                ("direction", format!("{:?}", direction)),
                ("seed", format!("{}", seed)),
            ],
            ContentEffect::WrapIndicator { prefix, suffix } => {
                vec![("prefix", prefix.clone()), ("suffix", suffix.clone())]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mixed_signals::prelude::SignalContext;
    use tui_vfx_core::bindable::VfxBindable;
    use tui_vfx_style::traits::ShaderRuntimeParams;

    /// Packet 69-A wire-format parity guard. Pre-69-A recipes used two
    /// shapes for rate-bearing fields: bare numbers (SignalOrFloat::Static)
    /// and tagged signal-spec objects like `{"type":"sine",...}`
    /// (SignalOrFloat::Signal). Post-packet they must continue to parse
    /// through the lenient `VfxBindableRepr` deserializer.
    #[test]
    fn legacy_bare_number_parses_as_literal() {
        let effect: ContentEffect =
            serde_json::from_str(r#"{"type":"typewriter","speed_variance":0.7}"#).expect("parse");
        let ContentEffect::Typewriter { speed_variance, .. } = effect else {
            panic!("expected Typewriter");
        };
        // Bare numbers route to Literal — the canonical home for static values.
        assert!(matches!(speed_variance, VfxBindable::Literal(v) if (v - 0.7).abs() < 1e-6));
    }

    #[test]
    fn legacy_signal_spec_object_parses_via_bare_signal_arm() {
        // A tagged SignalSpec object — SignalOrFloat::Signal historically —
        // lands in VfxBindableRepr::BareSignal and becomes
        // Signal(SignalOrFloat::Signal{spec, ..}). At loop_t=0, a sine with
        // amplitude=0.5 + offset=0.5 evaluates near 0.5 (sin(0) = 0). Tests
        // packet §3's claim that existing JSON parses byte-equivalently.
        let json = r#"{"type":"typewriter","speed_variance":{"type":"sine","frequency":1.0,"amplitude":0.5,"offset":0.5}}"#;
        let effect: ContentEffect = serde_json::from_str(json).expect("parse");
        let ContentEffect::Typewriter { speed_variance, .. } = effect else {
            panic!("expected Typewriter");
        };
        assert!(
            matches!(speed_variance, VfxBindable::Signal(_)),
            "tagged SignalSpec must land in the Signal arm, got {speed_variance:?}"
        );
    }

    #[test]
    fn binding_arm_resolves_from_runtime_params() {
        let effect: ContentEffect =
            serde_json::from_str(r#"{"type":"typewriter","speed_variance":{"binding":"jitter"}}"#)
                .expect("parse");
        let ContentEffect::Typewriter { speed_variance, .. } = effect else {
            panic!("expected Typewriter");
        };
        assert!(matches!(speed_variance, VfxBindable::Binding(_)));
        let sig = SignalContext::default();
        let mut params = ShaderRuntimeParams::new();
        params.insert("jitter", 0.33_f32);
        let v = speed_variance
            .evaluate(0.0, &sig, &params)
            .expect("evaluate");
        assert!((v - 0.33).abs() < 1e-6);
    }

    #[test]
    fn split_flap_speed_round_trips_legacy_bare_number() {
        // SplitFlap has three rate-bearing fields; one round-trip there
        // suffices to prove the wire format flows through every variant.
        let json = r#"{"type":"split_flap","speed":1.5,"cascade":0.0,"cycles":0.5}"#;
        let effect: ContentEffect = serde_json::from_str(json).expect("parse");
        let ContentEffect::SplitFlap { speed, .. } = effect else {
            panic!("expected SplitFlap");
        };
        let sig = SignalContext::default();
        let params = ShaderRuntimeParams::new();
        let v = speed.evaluate(0.0, &sig, &params).expect("evaluate");
        assert!((v - 1.5).abs() < 1e-6);
    }

    #[test]
    fn marquee_speed_binding_resolves() {
        let json = r#"{"type":"marquee","speed":{"binding":"scroll_rate"},"width":10}"#;
        let effect: ContentEffect = serde_json::from_str(json).expect("parse");
        let ContentEffect::Marquee { speed, .. } = effect else {
            panic!("expected Marquee");
        };
        let sig = SignalContext::default();
        let mut params = ShaderRuntimeParams::new();
        params.insert("scroll_rate", 2.0_f32);
        let v = speed.evaluate(0.0, &sig, &params).expect("evaluate");
        assert!((v - 2.0).abs() < 1e-6);
    }
}

// <FILE>tui-vfx-content/src/types/cls_content_effect.rs</FILE> - <DESC>ContentEffect enum with all content transformations</DESC>
// <VERS>END OF VERSION: 2.16.0</VERS>
