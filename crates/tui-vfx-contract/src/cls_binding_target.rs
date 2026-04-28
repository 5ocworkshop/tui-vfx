// <FILE>crates/tui-vfx-contract/src/cls_binding_target.rs</FILE> - <DESC>Declarative binding target DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase F2: target public parameters without introducing node graph identity.</WCTX>
// <CLOG>0.1.0: INIT — add parameter-only binding target vocabulary.</CLOG>

use crate::ParameterId;

/// Declarative target for a value binding.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum BindingTarget {
    /// Binding targets a declared public parameter.
    Parameter {
        /// Referenced target parameter id.
        id: ParameterId,
    },
}

// <FILE>crates/tui-vfx-contract/src/cls_binding_target.rs</FILE> - <DESC>Declarative binding target DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
