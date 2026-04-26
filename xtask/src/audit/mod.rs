// <FILE>xtask/src/audit/mod.rs</FILE> - <DESC>Audit subcommand group — validation gates for workspace policy enforcement</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Packet 1.9.A — ConfigSchema justification lint</WCTX>
// <CLOG>1.0.0: initial audit module with configschema subcommand</CLOG>

mod fnc_audit_configschema;
mod fnc_find_justification;
mod fnc_load_baseline;
mod fnc_scan_file_for_impls;

pub use fnc_audit_configschema::audit_configschema;

// <FILE>xtask/src/audit/mod.rs</FILE> - <DESC>Audit subcommand group — validation gates for workspace policy enforcement</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
