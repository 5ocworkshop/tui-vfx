// <FILE>tui-vfx-geometry/tests/types/test_timeline.rs</FILE> - <DESC>Tests for Timeline</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-16T20:22:32Z</VERS>
// <WCTX>Turn 10 Completeness</WCTX>
// <CLOG>Initial tests</CLOG>

use approx::assert_relative_eq;
use tui_vfx_geometry::types::Timeline;
#[test]
fn test_progress() {
    let timeline = Timeline::new(1000, 500);
    // Before start
    assert_relative_eq!(timeline.progress(900), 0.0);
    // At start
    assert_relative_eq!(timeline.progress(1000), 0.0);
    // Halfway
    assert_relative_eq!(timeline.progress(1250), 0.5);
    // Finished
    assert_relative_eq!(timeline.progress(1500), 1.0);
    // After finish
    assert_relative_eq!(timeline.progress(2000), 1.0);
}
#[test]
fn test_is_finished() {
    let timeline = Timeline::new(1000, 500);
    assert!(!timeline.is_finished(1499));
    assert!(timeline.is_finished(1500));
    assert!(timeline.is_finished(1501));
}
#[test]
fn test_zero_duration() {
    let timeline = Timeline::new(1000, 0);
    assert_relative_eq!(timeline.progress(1000), 1.0);
    assert!(timeline.is_finished(1000));
}

// <FILE>tui-vfx-geometry/tests/types/test_timeline.rs</FILE> - <DESC>Tests for Timeline</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-16T20:22:32Z</VERS>
