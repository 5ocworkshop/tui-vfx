// <FILE>tui-vfx-geometry/src/types/timeline.rs</FILE> - <DESC>Time tracking primitive</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-16T20:22:32Z</VERS>
// <WCTX>Turn 10 Completeness</WCTX>
// <CLOG>Initial implementation</CLOG>

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Timeline {
    pub start_ms: u64,
    pub duration_ms: u64,
}
impl Timeline {
    pub fn new(start_ms: u64, duration_ms: u64) -> Self {
        Self {
            start_ms,
            duration_ms,
        }
    }
    /// Returns the normalized progress (0.0 to 1.0) given the current time.
    /// Clamps to 1.0 if duration is exceeded.
    pub fn progress(&self, now_ms: u64) -> f32 {
        if now_ms < self.start_ms {
            return 0.0;
        }
        if self.duration_ms == 0 {
            return 1.0;
        }
        let elapsed = now_ms - self.start_ms;
        let t = elapsed as f32 / self.duration_ms as f32;
        t.min(1.0)
    }
    pub fn is_finished(&self, now_ms: u64) -> bool {
        now_ms >= self.start_ms + self.duration_ms
    }
}

// <FILE>tui-vfx-geometry/src/types/timeline.rs</FILE> - <DESC>Time tracking primitive</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-16T20:22:32Z</VERS>
