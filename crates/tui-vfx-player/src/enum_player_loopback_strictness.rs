// <FILE>crates/tui-vfx-player/src/enum_player_loopback_strictness.rs</FILE> - <DESC>Player-local authored loopback strictness semantics</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 parity: adapt vetted authored loopback behavior to the player data model without mutating older recipe pathways.</WCTX>
// <CLOG>0.1.0: INIT — copy authored permissive/warn/strict/error merge policy for preview loopbacks.</CLOG>

/// Controls how the v3.1 player handles authored loopbacks when a host signal is absent.
///
/// host signals always win; permissive and warn insert
/// loopback values; strict and error suppress insertion and record
/// would-have-fired intent at the call site.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerLoopbackStrictness {
    /// Insert authored loopback values for missing host signals.
    #[default]
    Permissive,
    /// Insert authored loopbacks and allow callers to surface warnings.
    Warn,
    /// Do not insert loopbacks; missing host signals remain missing.
    Strict,
    /// Do not insert loopbacks and allow callers to escalate would-have-fired keys.
    Error,
}

impl PlayerLoopbackStrictness {
    /// True when this mode suppresses loopback insertion.
    pub const fn suppresses_merge(self) -> bool {
        matches!(self, Self::Strict | Self::Error)
    }

    /// True when this mode should escalate any missing host wiring.
    pub const fn errors_on_fire(self) -> bool {
        matches!(self, Self::Error)
    }
}

// <FILE>crates/tui-vfx-player/src/enum_player_loopback_strictness.rs</FILE> - <DESC>PlayerLoopbackStrictness</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
