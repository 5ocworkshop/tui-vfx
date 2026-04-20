// <FILE>crates/tui-vfx-types/tests/test_role_id.rs</FILE> - <DESC>Tests for RoleId newtype</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.1 — foundation primitive tests</WCTX>
// <CLOG>0.1.0: TDD red tests confirming Copy + Eq + Ord semantics and accessor.</CLOG>

use tui_vfx_types::RoleId;

fn takes_by_value<T: Copy>(_: T) {}

#[test]
fn role_id_is_copy() {
    let id = RoleId::new(7);
    takes_by_value(id);
    // still usable after move-by-copy
    assert_eq!(id.id(), 7);
}

#[test]
fn role_id_equality() {
    assert_eq!(RoleId::new(3), RoleId::new(3));
    assert_ne!(RoleId::new(3), RoleId::new(4));
}

#[test]
fn role_id_ord() {
    let mut ids = [RoleId::new(2), RoleId::new(5), RoleId::new(1)];
    ids.sort();
    assert_eq!(ids, [RoleId::new(1), RoleId::new(2), RoleId::new(5)]);
}

#[test]
fn role_id_accessor() {
    assert_eq!(RoleId::new(0).id(), 0);
    assert_eq!(RoleId::new(u16::MAX).id(), u16::MAX);
}

// <FILE>crates/tui-vfx-types/tests/test_role_id.rs</FILE> - <DESC>RoleId tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
