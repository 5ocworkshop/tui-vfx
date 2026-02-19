// <FILE>tui-vfx-geometry/src/transitions/types.rs</FILE>
// <DESC>Transition model types</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>OFPF slicing</WCTX>
// <CLOG>Extracted SlidePhase/SlidePath/ExpandPhase</CLOG>

use crate::types::SignedRect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlidePhase {
    SlidingIn,
    SlidingOut,
}

/// Three-point slide path: start → dwell/target → end.
///
/// This allows callers to model entry, dwell, and exit positions explicitly.
///
/// Default “toast” behavior is represented by setting `start == dwell == end`.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    tui_vfx_core::ConfigSchema,
)]
pub struct SlidePath {
    pub start: SignedRect,
    pub dwell: SignedRect,
    pub end: SignedRect,
}

impl SlidePath {
    /// Default toast semantics: no movement (start=end=dwell).
    pub const fn toast(dwell: SignedRect) -> Self {
        Self {
            start: dwell,
            dwell,
            end: dwell,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpandPhase {
    Expanding,
    Collapsing,
}

// <FILE>tui-vfx-geometry/src/transitions/types.rs</FILE>
// <DESC>Transition model types</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
