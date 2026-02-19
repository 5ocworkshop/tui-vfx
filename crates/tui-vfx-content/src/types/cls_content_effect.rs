// <FILE>tui-vfx-content/src/types/cls_content_effect.rs</FILE> - <DESC>ContentEffect enum with all content transformations</DESC>
// <VERS>VERSION: 2.9.1</VERS>
// <WCTX>Rustfmt normalization for content effect docs</WCTX>
// <CLOG>Apply formatting updates after clippy run</CLOG>

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
//! ## Signal-Driven Parameters
//!
//! Many parameters use [`SignalOrFloat`] for animation-driven values:
//!
//! ```json
//! { "type": "typewriter", "speed_variance": 0.2 }            // Static
//! { "type": "typewriter", "speed_variance": { "signal": "t" }} // Animated
//! ```

use super::cls_dissolve_config::{DissolveDirection, DissolvePattern, DissolveReplacement};
use super::cls_mirror_axis::MirrorAxis;
use super::cls_morph_config::{MorphDirection, MorphProgression};
use super::cls_scramble_charset::ScrambleCharset;
use super::cls_slide_shift_flow_mode::SlideShiftFlowMode;
use super::cls_slide_shift_line_mode::SlideShiftLineMode;
use super::cls_typewriter_cursor::TypewriterCursor;
use mixed_signals::prelude::SignalOrFloat;

fn default_shift_width() -> u16 {
    1
}

/// Content effects that transform text before rendering.
///
/// Content effects operate on character data, modifying what text is displayed
/// rather than how it appears (style) or where it appears (filters/samplers).
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
        /// Speed variation for organic typing feel (0.0 = steady, higher = more variable).
        #[serde(default)]
        speed_variance: SignalOrFloat,
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
        /// How quickly characters resolve (higher = faster stabilization).
        #[serde(default)]
        resolve_pace: SignalOrFloat,
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
        /// Progress value when glitch starts (0.0-1.0).
        #[serde(default)]
        glitch_start: SignalOrFloat,
        /// Progress value when glitch ends (0.0-1.0).
        #[serde(default)]
        glitch_end: SignalOrFloat,
        /// Seed for deterministic behavior.
        seed: u64,
    },

    /// Combined scramble and glitch shift effect.
    ///
    /// Scrambles text progressively while adding a brief horizontal shift
    /// glitch. Combines the aesthetics of both effects.
    ScrambleGlitchShift {
        /// How quickly characters resolve.
        #[serde(default)]
        resolve_pace: SignalOrFloat,
        /// Character set for scrambled display.
        charset: ScrambleCharset,
        /// Seed for scramble pattern.
        scramble_seed: u64,
        /// Number of characters to shift right.
        shift_amount: u8,
        /// Progress value when glitch starts.
        #[serde(default)]
        glitch_start: SignalOrFloat,
        /// Progress value when glitch ends.
        #[serde(default)]
        glitch_end: SignalOrFloat,
    },

    /// Airport/train station split-flap display.
    ///
    /// Characters flip through the alphabet like mechanical departure boards.
    /// Creates a satisfying retro-mechanical aesthetic.
    SplitFlap {
        /// Flip animation speed.
        #[serde(default)]
        speed: SignalOrFloat,
        /// Cascade delay between characters (0 = simultaneous, higher = wave).
        #[serde(default)]
        cascade: SignalOrFloat,
    },

    /// Vertical scrolling digit counter.
    ///
    /// Numbers scroll vertically like a mechanical odometer or slot machine.
    Odometer,

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
        /// Scroll speed.
        #[serde(default)]
        speed: SignalOrFloat,
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
            ContentEffect::SplitFlap { .. } => "SplitFlap",
            ContentEffect::Odometer => "Odometer",
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
            ContentEffect::SplitFlap { .. } => "Airport/train station split-flap display",
            ContentEffect::Odometer => "Vertical scrolling digit counter",
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
            ContentEffect::SplitFlap { speed, cascade } => vec![
                ("speed", format!("{:?}", speed)),
                ("cascade", format!("{:?}", cascade)),
            ],
            ContentEffect::Odometer => vec![],
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

// <FILE>tui-vfx-content/src/types/cls_content_effect.rs</FILE> - <DESC>ContentEffect enum with all content transformations</DESC>
// <VERS>END OF VERSION: 2.9.1</VERS>
