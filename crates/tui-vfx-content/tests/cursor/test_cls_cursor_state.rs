// <FILE>tui-vfx-content/tests/cursor/test_cls_cursor_state.rs</FILE> - <DESC>Tests for CursorState</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: CursorState tests</WCTX>
// <CLOG>Initial tests</CLOG>

use tui_vfx_content::cursor::{CursorState, GrowInPhase};

#[test]
fn default_state_has_no_position_and_empty_history() {
    let s = CursorState::default();
    assert!(s.position.is_none());
    assert!(s.history.is_empty());
    assert_eq!(s.grow_in_phase, GrowInPhase::Hidden);
    assert_eq!(s.last_effective_visibility, 0.0);
    assert!(!s.grow_in_has_fired_once);
}

#[test]
fn grow_in_phase_variants_exist() {
    let _ = GrowInPhase::Hidden;
    let _ = GrowInPhase::GrowingIn { elapsed_ms: 0.0 };
    let _ = GrowInPhase::Visible;
    let _ = GrowInPhase::GrowingOut { elapsed_ms: 0.0 };
}

// <FILE>tui-vfx-content/tests/cursor/test_cls_cursor_state.rs</FILE> - <DESC>Tests for CursorState</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
