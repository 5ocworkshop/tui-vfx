// <FILE>crates/tui-vfx-content/src/lib.rs</FILE> - <DESC>Library entry point</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>TUI VFX extraction - text manipulation primitives</WCTX>
// <CLOG>Removed sixel/media module (framework-specific, excluded from library)</CLOG>

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
//! ## Usage
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

pub mod prelude;
pub mod traits;
pub mod transformers;
pub mod types;
pub mod utils;

// <FILE>crates/tui-vfx-content/src/lib.rs</FILE> - <DESC>Library entry point</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
