// <FILE>crates/tui-vfx-contract/src/cls_transition_subject_ref.rs</FILE> - <DESC>Transition subject reference DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition schema: make from/to/empty/canvas subjects explicit.</WCTX>
// <CLOG>0.1.0: INIT — add subject reference union.</CLOG>

use crate::{ElementId, SceneId};
use tui_vfx_types::RoleTag;

/// Concrete subject participating in a transition envelope.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum TransitionSubjectRef {
    /// Absence of a prior or next subject, used for enter/exit.
    Empty,
    /// Explicit recipe scene subject.
    Scene {
        /// Referenced scene id.
        id: SceneId,
    },
    /// Explicit scene element subject.
    Element {
        /// Referenced element id.
        id: ElementId,
    },
    /// Role-scoped subject.
    Role {
        /// Semantic role tag.
        role: RoleTag,
    },
    /// Background or full canvas subject.
    Canvas,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_subject_ref.rs</FILE> - <DESC>Transition subject reference DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
