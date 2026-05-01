// <FILE>crates/tui-vfx-compositor-next/src/lib.rs</FILE> - <DESC>Copied compositor-next baseline library entry point</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Compositor-next Phase 2 — copied baseline crate starts from hardened compositor behavior.</WCTX>
// <CLOG>0.1.0: INIT add copy-first compositor-next crate docs and exports.</CLOG>

//! Copied compositor-next baseline.
//!
//! This crate starts as a mechanical copy of `tui-vfx-compositor` so the
//! v3.1 schema-boundary runtime can preserve hardened compositor behavior
//! before any primitive-by-primitive alignment work begins. Do not rewrite
//! behavior in the baseline copy; prove parity first, then change behavior
//! only inside signed vertical primitive slices.

pub mod context;
pub(crate) mod filters;
pub(crate) mod masks;
pub mod pipeline;
pub(crate) mod samplers;
pub mod traits;
pub mod types;
pub mod utils;
pub mod widgets;

// <FILE>crates/tui-vfx-compositor-next/src/lib.rs</FILE> - <DESC>Copied compositor-next baseline library entry point</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
