// <FILE>crates/tui-vfx-content/src/lib.rs</FILE> - <DESC>Library entry point</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>feat/cursor-primitive: register cursor module</WCTX>
// <CLOG>Add pub mod cursor</CLOG>

//! # TUI VFX Content
//!
//! `tui-vfx-content` provides text manipulation primitives for the tui-vfx ecosystem.
//! It focuses on strict Unicode safety (grapheme clusters) and deterministic effects.
//!
//! ## Key Features
//! * **Typewriter**: Character-by-character reveal.
//! * **Scramble**: Matrix-style character resolving.
//! * **Redact**: Masking text with symbols.
//! * **Numeric**: Interpolating numbers.
//! * **Marquee**: Scrolling text windows.
//!
//! ## Quick start
//!
//! For the common case where you have a `progress: f64` from an animation
//! loop and want the transformed text back, use the inherent
//! [`ContentEffect::apply`](crate::types::ContentEffect::apply) method:
//!
//! ```rust
//! use tui_vfx_content::prelude::*;
//!
//! let effect = ContentEffect::Typewriter {
//!     speed_variance: SignalOrFloat::Static(0.0),
//!     cursor: None,
//! };
//! let revealed: String = effect.apply("Hello World", 0.5);
//! ```
//!
//! Need the borrowed fast path?
//! [`ContentEffect::apply_to_borrowed`](crate::types::ContentEffect::apply_to_borrowed)
//! returns a [`std::borrow::Cow`] that borrows from the target when the
//! transformer produced no allocation. Need signal-driven pacing?
//! [`ContentEffect::apply_with_context`](crate::types::ContentEffect::apply_with_context)
//! is the advanced entry point.
//!
//! ## Trait-level usage
//!
//! The full transformer trait remains the canonical advanced API:
//!
//! ```rust
//! use tui_vfx_content::prelude::*;
//! use mixed_signals::prelude::SignalContext;
//!
//! let tx = Typewriter::default();
//! let signal_ctx = SignalContext::default();
//! let output = tx.transform("Hello World", 0.5, &signal_ctx);
//! assert_eq!(output, "Hello");
//! ```
//!
//! ## Static vs signal-driven parameters
//!
//! Every per-effect configuration field (cursor timing, scramble resolve
//! pace, marquee speed, and so on) is typed as
//! [`SignalOrFloat`](mixed_signals::prelude::SignalOrFloat). This enables
//! advanced consumers to drive parameters from procedural signals — frame
//! counters, spatial noise, phase-aware clocks — on a per-frame basis.
//!
//! **The common case is `SignalOrFloat::Static(value)`.** Most effects are
//! driven by a single external progress value (an animation timer, a
//! transition lifecycle, a scroll position) while the per-effect parameters
//! are constants. For that case wrap the raw number in
//! `SignalOrFloat::Static(n)` and the effect behaves as a pure function of
//! `(target, progress)`:
//!
//! ```rust
//! use tui_vfx_content::prelude::*;
//!
//! let effect = ContentEffect::Scramble {
//!     resolve_pace: SignalOrFloat::Static(1.0),
//!     charset: ScrambleCharset::Alphanumeric,
//!     seed: 42,
//! };
//! let result = effect.apply("SYSTEM ONLINE", 1.0);
//! assert_eq!(result, "SYSTEM ONLINE");
//! ```
//!
//! For signal-driven pacing — dynamic blink rates, breathing cursors,
//! parameters that vary with procedural noise — see the `mixed_signals`
//! crate and
//! [`ContentEffect::apply_with_context`](crate::types::ContentEffect::apply_with_context).
//!
//! ## Cursor presets
//!
//! [`TypewriterCursor`](crate::types::TypewriterCursor) ships with
//! one-line constructors for the canonical terminal cursor glyphs:
//!
//! ```rust
//! use tui_vfx_content::types::TypewriterCursor;
//!
//! let block = TypewriterCursor::block();        // █
//! let underscore = TypewriterCursor::underscore(); // _
//! let pipe = TypewriterCursor::pipe();          // |
//! let caret = TypewriterCursor::caret();        // ▌
//! let custom = TypewriterCursor::simple('◆');   // any single glyph
//! ```

pub mod cursor;
pub mod prelude;
pub mod traits;
pub mod transformers;
pub mod types;
pub mod utils;

// <FILE>crates/tui-vfx-content/src/lib.rs</FILE> - <DESC>Library entry point</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
