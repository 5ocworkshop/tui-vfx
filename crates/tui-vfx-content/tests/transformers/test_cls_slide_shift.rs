// <FILE>tui-vfx-content/tests/transformers/test_cls_slide_shift.rs</FILE> - <DESC>Tests for SlideShift</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>SlideShift barrier span support</WCTX>
// <CLOG>Cover shift_width behavior and update constructors</CLOG>

use mixed_signals::prelude::SignalContext;
use tui_vfx_content::traits::TextTransformer;
use tui_vfx_content::transformers::SlideShift;
use tui_vfx_content::types::{SlideShiftFlowMode, SlideShiftLineMode};

// Helper for creating test SignalContext
fn test_signal_ctx() -> SignalContext {
    SignalContext {
        frame: 0,
        seed: 0,
        width: 80,
        height: 24,
        phase: None,
        phase_t: None,
        loop_t: None,
        absolute_t: None,
        char_index: None,
    }
}

#[test]
fn test_before_shift_threshold() {
    let tx = SlideShift::new(
        0,
        20,
        2,
        10,
        1,
        -1,
        SlideShiftLineMode::Block,
        SlideShiftFlowMode::StayShifted,
    );
    let out = tx.transform("Hello", 0.4, &test_signal_ctx());
    assert_eq!(out, "\n\n        Hello");
}

#[test]
fn test_after_shift_threshold() {
    let tx = SlideShift::new(
        0,
        20,
        2,
        10,
        1,
        -1,
        SlideShiftLineMode::Block,
        SlideShiftFlowMode::StayShifted,
    );
    let out = tx.transform("Hello", 0.6, &test_signal_ctx());
    assert_eq!(out, "\n            Hello");
}

#[test]
fn test_line_mode_block() {
    let tx = SlideShift::new(
        2,
        2,
        0,
        10,
        1,
        0,
        SlideShiftLineMode::Block,
        SlideShiftFlowMode::StayShifted,
    );
    let out = tx.transform("A\nB", 0.0, &test_signal_ctx());
    assert_eq!(out, "  A\n  B");
}

#[test]
fn test_line_mode_first_line_only() {
    let tx = SlideShift::new(
        2,
        2,
        0,
        10,
        1,
        0,
        SlideShiftLineMode::FirstLineOnly,
        SlideShiftFlowMode::StayShifted,
    );
    let out = tx.transform("A\nB", 0.0, &test_signal_ctx());
    assert_eq!(out, "  A\nB");
}

#[test]
fn test_clamps_negative_offsets() {
    let tx = SlideShift::new(
        -4,
        -4,
        -3,
        10,
        1,
        -2,
        SlideShiftLineMode::Block,
        SlideShiftFlowMode::StayShifted,
    );
    let out = tx.transform("Hello", 0.0, &test_signal_ctx());
    assert_eq!(out, "Hello");
}

#[test]
fn test_flow_back_clears_after_barrier() {
    let tx = SlideShift::new(
        0,
        20,
        0,
        6,
        1,
        1,
        SlideShiftLineMode::Block,
        SlideShiftFlowMode::FlowBack,
    );
    let out_overlap = tx.transform("Hello", 0.2, &test_signal_ctx());
    assert_eq!(out_overlap, "\n    Hello");
    let out_clear = tx.transform("Hello", 0.6, &test_signal_ctx());
    assert_eq!(out_clear, "            Hello");
}

#[test]
fn test_shift_width_extends_barrier() {
    let tx = SlideShift::new(
        0,
        20,
        0,
        10,
        3,
        1,
        SlideShiftLineMode::Block,
        SlideShiftFlowMode::StayShifted,
    );
    let out_before = tx.transform("Hi", 0.55, &test_signal_ctx());
    assert_eq!(out_before, "           Hi");
    let out_after = tx.transform("Hi", 0.65, &test_signal_ctx());
    assert_eq!(out_after, "\n             Hi");
}

// <FILE>tui-vfx-content/tests/transformers/test_cls_slide_shift.rs</FILE> - <DESC>Tests for SlideShift</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
