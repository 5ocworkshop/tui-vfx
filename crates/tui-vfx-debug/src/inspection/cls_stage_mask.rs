// <FILE>crates/tui-vfx-debug/src/inspection/cls_stage_mask.rs</FILE> - <DESC>Bitflags StageMask over the four inspection stages</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — inspection foundation. StageMask is the coarse bitmask gate used by TraceFilter at sink-time; it also serves as the fast short-circuit when NONE (no stage enabled) so emit sites can skip envelope construction entirely.</WCTX>
// <CLOG>0.1.0: initial StageMask bitflags with LIFECYCLE / RESOLUTION / COMPOSITION / PIPELINE + NONE / ALL sentinels; is_empty / contains / bitwise ops; serde round-trip as an integer bitmask.</CLOG>

//! Bitmask over the four inspection stages.
//!
//! `StageMask` is the coarse-grained stage gate used by the inspection
//! pipeline. It is semantically equivalent to a hand-rolled bitflags type
//! (we avoid taking a runtime dependency on the `bitflags` crate for a
//! four-bit mask): the four stage bits, a `NONE` zero sentinel for fast
//! short-circuiting, and an `ALL` convenience.
//!
//! # Stages
//!
//! - `LIFECYCLE` — recipe lifecycle / phase transitions
//! - `RESOLUTION` — asset / procedural / token / recipe-binding resolution
//! - `COMPOSITION` — scene composer / layer paint events
//! - `PIPELINE` — per-cell compositor pipeline events
//!
//! # Emit-site short-circuit contract
//!
//! A well-formed emitter calls [`StageMask::is_empty`] on the sink's mask
//! before building an envelope. An empty mask means "no stages of
//! interest" — the emitter drops the event without allocation.
//!
//! # Examples
//!
//! ```
//! use tui_vfx_debug::inspection::StageMask;
//!
//! let m = StageMask::LIFECYCLE | StageMask::PIPELINE;
//! assert!(m.contains(StageMask::LIFECYCLE));
//! assert!(!m.contains(StageMask::RESOLUTION));
//! assert!(!m.is_empty());
//! assert!(StageMask::NONE.is_empty());
//! assert!(StageMask::ALL.contains(StageMask::COMPOSITION));
//! ```

use serde::{Deserialize, Serialize};

/// Bitmask over the four inspection stages.
///
/// Stored as a `u8`; the four low bits are meaningful and correspond to
/// the four stages documented in the module-level docs. Higher bits are
/// reserved and must stay zero.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StageMask(u8);

impl StageMask {
    /// The empty mask — no stages enabled. Emit-site short-circuit value.
    pub const NONE: StageMask = StageMask(0);
    /// Recipe lifecycle / phase transitions.
    pub const LIFECYCLE: StageMask = StageMask(1 << 0);
    /// Asset / procedural / token / recipe-binding resolution.
    pub const RESOLUTION: StageMask = StageMask(1 << 1);
    /// Scene composer / layer paint events.
    pub const COMPOSITION: StageMask = StageMask(1 << 2);
    /// Per-cell compositor pipeline events.
    pub const PIPELINE: StageMask = StageMask(1 << 3);
    /// Convenience: every stage enabled.
    pub const ALL: StageMask = StageMask(
        StageMask::LIFECYCLE.0
            | StageMask::RESOLUTION.0
            | StageMask::COMPOSITION.0
            | StageMask::PIPELINE.0,
    );

    /// Return the raw bit representation.
    pub const fn bits(self) -> u8 {
        self.0
    }

    /// Construct a mask from raw bits. Bits outside the four defined
    /// stages are masked off.
    pub const fn from_bits_truncate(bits: u8) -> Self {
        StageMask(bits & StageMask::ALL.0)
    }

    /// True if no stage bits are set.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// True if every bit set in `other` is also set in `self`. For a
    /// single-bit `other`, equivalent to "is that flag enabled".
    pub const fn contains(self, other: StageMask) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Bitwise OR of two masks.
    pub const fn union(self, other: StageMask) -> Self {
        StageMask(self.0 | other.0)
    }

    /// Bitwise AND of two masks.
    pub const fn intersection(self, other: StageMask) -> Self {
        StageMask(self.0 & other.0)
    }
}

impl std::ops::BitOr for StageMask {
    type Output = StageMask;
    fn bitor(self, rhs: StageMask) -> StageMask {
        self.union(rhs)
    }
}

impl std::ops::BitOrAssign for StageMask {
    fn bitor_assign(&mut self, rhs: StageMask) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAnd for StageMask {
    type Output = StageMask;
    fn bitand(self, rhs: StageMask) -> StageMask {
        self.intersection(rhs)
    }
}

impl std::ops::BitAndAssign for StageMask {
    fn bitand_assign(&mut self, rhs: StageMask) {
        self.0 &= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_and_all_are_disjoint() {
        assert!(StageMask::NONE.is_empty());
        assert!(!StageMask::ALL.is_empty());
    }

    #[test]
    fn from_bits_truncate_ignores_reserved_bits() {
        let m = StageMask::from_bits_truncate(0xFF);
        assert_eq!(m, StageMask::ALL);
    }
}

// <FILE>crates/tui-vfx-debug/src/inspection/cls_stage_mask.rs</FILE> - <DESC>Bitflags StageMask</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
