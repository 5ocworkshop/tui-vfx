// <FILE>crates/tui-vfx-contract/src/cls_lifecycle_phase.rs</FILE> - <DESC>Recipe lifecycle phase enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase I0: name high-level recipe lifecycle intervals.</WCTX>
// <CLOG>0.1.0: INIT — add enter/dwell/exit phase vocabulary.</CLOG>

/// Named high-level interval in a recipe lifecycle.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum LifecyclePhase {
    /// Initial appearance or setup interval.
    Enter,
    /// Sustained interval that may be fixed or trigger-terminated.
    Dwell,
    /// Final departure or teardown interval.
    Exit,
}

// <FILE>crates/tui-vfx-contract/src/cls_lifecycle_phase.rs</FILE> - <DESC>Recipe lifecycle phase enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
