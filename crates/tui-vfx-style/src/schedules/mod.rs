// <FILE>tui-vfx-style/src/schedules/mod.rs</FILE> - <DESC>Per-cell trigger-time schedule generators for one-shot per-cell effects (TTE Beams cadence, eased wavefronts, future custom schedulers); produce Vec<f64> consumed by GlyphTimeline::TimelineTrigger::PerCellSchedule</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>TTE effects port — extract schedule generators from utils/ into a dedicated discoverable module so future schedulers (eased wavefront, custom-fn, easing-driven) cluster here rather than diffusing through utils.</WCTX>
// <CLOG>0.1.0: NEW module — initial home for poisson_burst_schedule (TTE Beams stochastic cadence). Sibling generators land here as they earn their place.</CLOG>

//! Per-cell trigger-time schedule generators.
//!
//! A "schedule" is a `Vec<f64>` of length `width × height` indexed
//! `out[y * width + x]`, where each entry is the time in seconds at
//! which cell `(x, y)` should "fire." Schedules are consumed by
//! [`tui_vfx_compositor::filters::cls_glyph_timeline::TimelineTrigger::PerCellSchedule`]
//! via O(1) lookup.
//!
//! # Why a dedicated module
//!
//! Different effects want different per-cell trigger fields. TTE
//! Beams wants a stochastic batch-cadence schedule
//! ([`poisson_burst_schedule`]). TTE Sweep wants a smooth eased
//! wavefront. A toast cascade wants explicit `(time, cells)` events.
//! Each is a small composition of `mixed_signals` primitives;
//! clustering them here makes the abstraction discoverable and gives
//! authors a single place to look for "how do I drive a per-cell
//! timeline."
//!
//! # Composition over invention
//!
//! All schedule generators in this module compose existing primitives
//! from `mixed-signals` (deterministic hashing, easing curves, signal
//! graphs). When a generator wants a new substrate, the substrate
//! goes upstream into `mixed-signals` — not into this module. Per
//! Intention 9.
//!
//! # Future generators (none yet earned a place)
//!
//! Per Intention 23 (rule of three), no `Schedule` trait exists today
//! — there's only one concrete generator. When a third concrete
//! schedule shape lands, lift the common shape into a trait. Until
//! then, free functions returning `Vec<f64>` keep the surface small.
//!
//! Likely future generators (in order of plausibility):
//!
//! - `eased_wavefront_schedule(axis, easing, total_duration_seconds)`
//!   — pre-bakes [`TimelineTrigger::Wavefront`] (currently inline in
//!   the filter) so authors can compose it with other transforms.
//! - `keyframes_schedule(events: &[(f64, &[(u16, u16)])])` — explicit
//!   per-cell `(time, cell)` event list for hand-authored cascades.
//! - `custom_schedule<F: Fn(u16, u16) -> f64>(f, width, height)` —
//!   author-supplied closure baked into a Vec.

mod fnc_poisson_burst_schedule;

pub use fnc_poisson_burst_schedule::{
    LaneAxis, PoissonBurstScheduleConfig, poisson_burst_schedule,
};

// <FILE>tui-vfx-style/src/schedules/mod.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
