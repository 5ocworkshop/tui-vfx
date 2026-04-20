// <FILE>crates/tui-vfx-debug/tests/test_cls_stage_mask.rs</FILE> - <DESC>Tests for StageMask bitflags + Send+Sync assertions on TraceSink</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — TDD red tests that StageMask is a composable bitmask with OR semantics, NONE/ALL sentinels, per-stage flags, and that TraceSink is Send + Sync so it can be shared across threads.</WCTX>
// <CLOG>0.1.0: initial red-phase tests covering bit OR, contains semantics, NONE short-circuit flag, ALL covers every stage, and static assert_send_sync on TraceSink.</CLOG>

use tui_vfx_debug::inspection::{StageMask, TraceFilter, TraceSink};

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn none_mask_contains_no_stages() {
    let m = StageMask::NONE;
    assert!(!m.contains(StageMask::LIFECYCLE));
    assert!(!m.contains(StageMask::RESOLUTION));
    assert!(!m.contains(StageMask::COMPOSITION));
    assert!(!m.contains(StageMask::PIPELINE));
    assert!(m.is_empty());
}

#[test]
fn all_mask_contains_every_stage() {
    let m = StageMask::ALL;
    assert!(m.contains(StageMask::LIFECYCLE));
    assert!(m.contains(StageMask::RESOLUTION));
    assert!(m.contains(StageMask::COMPOSITION));
    assert!(m.contains(StageMask::PIPELINE));
}

#[test]
fn or_combines_flags() {
    let m = StageMask::LIFECYCLE | StageMask::PIPELINE;
    assert!(m.contains(StageMask::LIFECYCLE));
    assert!(!m.contains(StageMask::RESOLUTION));
    assert!(!m.contains(StageMask::COMPOSITION));
    assert!(m.contains(StageMask::PIPELINE));
}

#[test]
fn or_assign_accumulates() {
    let mut m = StageMask::NONE;
    m |= StageMask::COMPOSITION;
    m |= StageMask::RESOLUTION;
    assert!(m.contains(StageMask::COMPOSITION));
    assert!(m.contains(StageMask::RESOLUTION));
    assert!(!m.contains(StageMask::LIFECYCLE));
    assert!(!m.contains(StageMask::PIPELINE));
}

#[test]
fn individual_flag_values_are_distinct_bits() {
    let life = StageMask::LIFECYCLE;
    let resl = StageMask::RESOLUTION;
    let comp = StageMask::COMPOSITION;
    let pipe = StageMask::PIPELINE;
    // Each pair should have empty intersection (distinct bits).
    assert!((life & resl).is_empty());
    assert!((life & comp).is_empty());
    assert!((life & pipe).is_empty());
    assert!((resl & comp).is_empty());
    assert!((resl & pipe).is_empty());
    assert!((comp & pipe).is_empty());
}

#[test]
fn trace_sink_is_send_and_sync() {
    // The compiler will fail this test if TraceSink loses Send or Sync.
    assert_send_sync::<TraceSink>();
    // The filter type must also be Send+Sync so the sink can hold it safely.
    assert_send_sync::<TraceFilter>();
}

// <FILE>crates/tui-vfx-debug/tests/test_cls_stage_mask.rs</FILE> - <DESC>Tests for StageMask + Send+Sync on TraceSink</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
