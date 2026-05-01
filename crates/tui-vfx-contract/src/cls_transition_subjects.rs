// <FILE>crates/tui-vfx-contract/src/cls_transition_subjects.rs</FILE> - <DESC>Transition subject set DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition schema: name from/to/shared participants before adding visual variants.</WCTX>
// <CLOG>0.1.0: INIT — add transition subject set.</CLOG>

use crate::TransitionSubjectRef;

/// Named subjects participating in a transition.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransitionSubjects {
    /// Prior surface or state.
    pub from: TransitionSubjectRef,
    /// Next surface or state.
    pub to: TransitionSubjectRef,
    /// Optional shared/matched subjects for shared-element or shared-axis motion.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shared: Vec<TransitionSubjectRef>,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_subjects.rs</FILE> - <DESC>Transition subject set DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
