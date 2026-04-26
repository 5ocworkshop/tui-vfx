// <FILE>xtask/src/lib.rs</FILE> - <DESC>Library facade exposing the audit module for integration tests</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Packet 1.9.A — ConfigSchema justification lint</WCTX>
// <CLOG>1.0.0: initial — re-exports audit_configschema for xtask integration tests</CLOG>

//! Thin library facade so integration tests under `xtask/tests/` can call
//! `xtask_audit_configschema::audit_configschema(workspace_root)` directly,
//! without going through the CLI binary.

mod audit;

pub use audit::audit_configschema;

// <FILE>xtask/src/lib.rs</FILE> - <DESC>Library facade exposing the audit module for integration tests</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
