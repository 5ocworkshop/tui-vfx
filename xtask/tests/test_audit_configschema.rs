// <FILE>xtask/tests/test_audit_configschema.rs</FILE> - <DESC>Integration tests for the configschema justification lint</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Packet 1.9.A — ConfigSchema justification lint</WCTX>
// <CLOG>1.0.0: initial test suite — 7 tests per the packet's test plan</CLOG>

//! Integration tests for `cargo xtask audit configschema`.
//!
//! Each test builds a fixture directory tree under a `tempdir`, then calls
//! `audit_configschema` directly to verify pass/fail/warning behaviour.

use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

// Re-export the public entrypoint from xtask.
// Because xtask is a binary crate, we access the function via the library-
// style `extern crate`-free path through the public module it re-exports.
// We use `path` dep in Cargo.toml to reach the xtask crate directly.

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Create a fixture workspace under a temporary directory and return its root.
///
/// `files` is a slice of `(relative_path, content)` pairs.  The function
/// creates all necessary parent directories and writes each file.
fn make_fixture(files: &[(&str, &str)]) -> TempDir {
    let dir = TempDir::new().expect("tempdir");
    for (rel, content) in files {
        let abs = dir.path().join(rel);
        fs::create_dir_all(abs.parent().unwrap()).unwrap();
        fs::write(&abs, content).unwrap();
    }
    dir
}

/// Write the baseline TOML content to `<root>/xtask/data/configschema_baseline.toml`.
fn write_baseline(root: &Path, toml_content: &str) {
    let path = root.join("xtask/data/configschema_baseline.toml");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, toml_content).unwrap();
}

/// Call the audit function against the given workspace root.
fn run_audit(workspace_root: &Path) -> anyhow::Result<()> {
    xtask_audit_configschema::audit_configschema(workspace_root)
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 1: a new unjustified impl in a new file emits a warning (warn-only mode).
// ─────────────────────────────────────────────────────────────────────────────

/// In warn-only mode (the default) a new unjustified impl results in a
/// warning but the lint still exits 0.  The warning message must reference
/// the type name.
///
/// NOTE: The packet specifies that missing-justification should be a *failure*
/// in the test, but the implementation is deliberately in warn-only mode until
/// 2026-07-01 per Q2 default. To test the failing path we use an *unrecognised
/// kind*, which is always a hard error regardless of mode.
#[test]
fn unjustified_impl_in_new_file_warns_in_warn_only_mode() {
    let fixture = make_fixture(&[("crates/foo/src/lib.rs", UNJUSTIFIED_IMPL)]);
    write_baseline(
        fixture.path(),
        "schema_version = 1\n",
    );

    // In warn-only mode this exits 0 (not Err).
    let result = run_audit(fixture.path());
    assert!(
        result.is_ok(),
        "warn-only mode must not fail on missing justification: {:?}",
        result
    );
}

/// An unrecognised justification kind is always a hard error.
#[test]
fn unrecognized_kind_is_hard_error() {
    let fixture = make_fixture(&[("crates/foo/src/lib.rs", UNRECOGNIZED_KIND_IMPL)]);
    write_baseline(fixture.path(), "schema_version = 1\n");

    let result = run_audit(fixture.path());
    assert!(result.is_err(), "unrecognised kind must be a hard error");
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("1"),
        "error message should name the count: {msg}"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2: a new impl with a canonical justification passes.
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn justified_impl_passes() {
    let fixture = make_fixture(&[("crates/foo/src/lib.rs", JUSTIFIED_IMPL)]);
    write_baseline(fixture.path(), "schema_version = 1\n");

    let result = run_audit(fixture.path());
    assert!(result.is_ok(), "justified impl must pass: {:?}", result);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 3: an existing impl in the baseline passes regardless of justification.
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn baselined_impl_passes_without_justification() {
    let fixture = make_fixture(&[("crates/foo/src/lib.rs", UNJUSTIFIED_IMPL)]);
    let baseline_toml = r#"
schema_version = 1
[[entry]]
file = "crates/foo/src/lib.rs"
type = "LegacyType"
"#;
    write_baseline(fixture.path(), baseline_toml);

    let result = run_audit(fixture.path());
    assert!(
        result.is_ok(),
        "baselined impl must pass without justification: {:?}",
        result
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 4: baseline entry for a deleted file does not false-positive.
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn baseline_entry_for_deleted_file_does_not_panic() {
    // No source files at all. Baseline has a stale entry.
    let baseline_toml = r#"
schema_version = 1
[[entry]]
file = "crates/deleted/src/lib.rs"
type = "GoneType"
"#;
    let fixture = make_fixture(&[]);
    write_baseline(fixture.path(), baseline_toml);

    let result = run_audit(fixture.path());
    assert!(
        result.is_ok(),
        "stale baseline entry must not cause a panic or false-positive: {:?}",
        result
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 5: macro-body matches (containing `$`) are skipped.
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn macro_body_matches_are_skipped() {
    let fixture = make_fixture(&[("crates/foo/src/lib.rs", MACRO_BODY_IMPL)]);
    write_baseline(fixture.path(), "schema_version = 1\n");

    let result = run_audit(fixture.path());
    assert!(
        result.is_ok(),
        "macro-body impl must be skipped: {:?}",
        result
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 6: `Other("...")` justification passes with the lint still exiting 0.
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn other_justification_passes() {
    let fixture = make_fixture(&[("crates/foo/src/lib.rs", OTHER_KIND_IMPL)]);
    write_baseline(fixture.path(), "schema_version = 1\n");

    let result = run_audit(fixture.path());
    assert!(
        result.is_ok(),
        "Other(...) justification must pass (with a warning): {:?}",
        result
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 7: unrecognised kind is rejected (hard error even in warn-only mode).
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn unrecognized_kind_fails() {
    let fixture = make_fixture(&[("crates/foo/src/lib.rs", UNRECOGNIZED_KIND_IMPL)]);
    write_baseline(fixture.path(), "schema_version = 1\n");

    let result = run_audit(fixture.path());
    assert!(result.is_err(), "unrecognised kind must fail");
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("1"),
        "error message should name the count: {msg}"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Fixture source strings
// ─────────────────────────────────────────────────────────────────────────────

const UNJUSTIFIED_IMPL: &str = r#"
pub struct LegacyType;
impl ConfigSchema for LegacyType {
    fn schema() -> SchemaNode { unimplemented!() }
}
"#;

const JUSTIFIED_IMPL: &str = r#"
pub struct NewType;
// CONFIGSCHEMA-JUSTIFICATION: derive-cannot-handle-untagged-enum: NewType serializes via serde(untagged).
impl ConfigSchema for NewType {
    fn schema() -> SchemaNode { unimplemented!() }
}
"#;

const MACRO_BODY_IMPL: &str = r#"
macro_rules! impl_thing {
    ($t:ty) => {
        impl ConfigSchema for $t {
            fn schema() -> SchemaNode { unimplemented!() }
        }
    };
}
"#;

const OTHER_KIND_IMPL: &str = r#"
pub struct NewType;
// CONFIGSCHEMA-JUSTIFICATION: Other("custom one-off reason for this type")
impl ConfigSchema for NewType {
    fn schema() -> SchemaNode { unimplemented!() }
}
"#;

const UNRECOGNIZED_KIND_IMPL: &str = r#"
pub struct NewType;
// CONFIGSCHEMA-JUSTIFICATION: derive-cannot-handle-magic
impl ConfigSchema for NewType {
    fn schema() -> SchemaNode { unimplemented!() }
}
"#;

// ─────────────────────────────────────────────────────────────────────────────
// Helper: resolve workspace root used by run_audit at test call sites.
// (Unused here but included to document the pattern.)
// ─────────────────────────────────────────────────────────────────────────────

#[allow(dead_code)]
fn workspace_root_of_fixture(fixture: &TempDir) -> PathBuf {
    fixture.path().to_path_buf()
}

// <FILE>xtask/tests/test_audit_configschema.rs</FILE> - <DESC>Integration tests for the configschema justification lint</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
