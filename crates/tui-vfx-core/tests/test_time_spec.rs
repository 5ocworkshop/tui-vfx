// <FILE>tui-vfx-core/tests/test_time_spec.rs</FILE>
// <DESC>Tests for TimeSpec (deterministic time contract)</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Batch E: shared TimeSpec behavior</WCTX>
// <CLOG>Added progress and clamping tests</CLOG>

use std::time::{Duration, Instant};

use tui_vfx_core::TimeSpec;

#[test]
fn test_progress_midpoint() {
    let start = Instant::now();
    let now = start + Duration::from_millis(500);

    let ts = TimeSpec {
        start,
        now,
        duration: Duration::from_millis(1000),
    };

    assert!((ts.progress() - 0.5).abs() < 1e-6);
}

#[test]
fn test_progress_clamps_to_one() {
    let start = Instant::now();
    let now = start + Duration::from_millis(2000);
    let ts = TimeSpec {
        start,
        now,
        duration: Duration::from_millis(1000),
    };

    assert_eq!(ts.progress(), 1.0);
}

#[test]
fn test_zero_duration_is_one() {
    let start = Instant::now();
    let ts = TimeSpec {
        start,
        now: start,
        duration: Duration::from_millis(0),
    };
    assert_eq!(ts.progress(), 1.0);
}

#[test]
fn test_now_before_start_saturates_to_zero() {
    let start = Instant::now();
    let now = start - Duration::from_millis(1);
    let ts = TimeSpec {
        start,
        now,
        duration: Duration::from_millis(1000),
    };
    assert_eq!(ts.progress(), 0.0);
}
