// <FILE>crates/tui-vfx-types/tests/test_interned_string.rs</FILE> - <DESC>Tests for InternedString newtype</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.1 — foundation primitive tests</WCTX>
// <CLOG>0.1.0: TDD red tests for InternedString equality-by-content, Send+Sync, round-trip.</CLOG>

use tui_vfx_types::InternedString;

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn same_str_input_produces_equal_instances() {
    let a = InternedString::new("hello");
    let b = InternedString::new("hello");
    assert_eq!(a, b);
    // Independent allocations are fine; contract is string equality, not identity.
    assert_eq!(a.as_str(), "hello");
    assert_eq!(b.as_str(), "hello");
}

#[test]
fn distinct_strings_are_not_equal() {
    let a = InternedString::new("foo");
    let b = InternedString::new("bar");
    assert_ne!(a, b);
}

#[test]
fn round_trip_to_string() {
    let original = "phase_transition_marker";
    let interned = InternedString::new(original);
    assert_eq!(interned.as_str(), original);
}

#[test]
fn empty_sentinel_exists_and_is_empty() {
    let empty = InternedString::empty();
    assert_eq!(empty.as_str(), "");
}

#[test]
fn clone_is_cheap_and_equal() {
    let original = InternedString::new("cheap_to_clone");
    let cloned = original.clone();
    assert_eq!(original, cloned);
    assert_eq!(original.as_str(), cloned.as_str());
}

#[test]
fn interned_string_is_send_and_sync() {
    assert_send_sync::<InternedString>();
}

use std::collections::HashSet;

#[test]
fn interned_string_hashes_by_content() {
    let mut set: HashSet<InternedString> = HashSet::new();
    set.insert(InternedString::new("a"));
    set.insert(InternedString::new("a"));
    set.insert(InternedString::new("b"));
    assert_eq!(set.len(), 2);
}

// <FILE>crates/tui-vfx-types/tests/test_interned_string.rs</FILE> - <DESC>InternedString tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
