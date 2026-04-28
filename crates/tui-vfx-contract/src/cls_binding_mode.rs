// <FILE>crates/tui-vfx-contract/src/cls_binding_mode.rs</FILE> - <DESC>Declarative binding mode vocabulary</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase F2: declare minimal binding mode vocabulary.</WCTX>
// <CLOG>0.1.0: INIT — add replace-only binding mode for declarative parameter bindings.</CLOG>

/// Declarative mode for applying a binding source to a target.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum BindingMode {
    /// Source replaces the target value.
    Replace,
}

// <FILE>crates/tui-vfx-contract/src/cls_binding_mode.rs</FILE> - <DESC>Declarative binding mode vocabulary</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
