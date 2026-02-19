// <FILE>tui-vfx-core/src/time_spec.rs</FILE>
// <DESC>Deterministic time contract shared across the ecosystem</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Batch E: single shared TimeSpec definition</WCTX>
// <CLOG>Moved TimeSpec into tui-motion for cross-crate reuse</CLOG>

use std::time::{Duration, Instant};

/// Normalized time contract.
///
/// This is a runtime primitive (not configuration) and is intentionally small and copyable.
#[derive(Debug, Clone, Copy)]
pub struct TimeSpec {
    pub start: Instant,
    pub now: Instant,
    pub duration: Duration,
}

impl TimeSpec {
    /// Returns normalized progress clamped to [0.0, 1.0].
    #[inline]
    pub fn progress(&self) -> f64 {
        if self.duration.is_zero() {
            return 1.0;
        }

        let elapsed = self.now.saturating_duration_since(self.start);
        (elapsed.as_secs_f64() / self.duration.as_secs_f64()).clamp(0.0, 1.0)
    }
}

// <FILE>tui-vfx-core/src/time_spec.rs</FILE>
// <DESC>Deterministic time contract shared across the ecosystem</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
