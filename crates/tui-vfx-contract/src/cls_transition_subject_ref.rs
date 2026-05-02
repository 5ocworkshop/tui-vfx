// <FILE>crates/tui-vfx-contract/src/cls_transition_subject_ref.rs</FILE> - <DESC>Transition subject reference DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase A of canonicalize completion: add previous/current variants so relation transitions can name the prior and next surface symbolically.</WCTX>
// <CLOG>0.2.0: MINOR — add previous and current variants for symbolic relation-transition subjects (Q16 in the v3.1 expansion table).</CLOG>

use crate::{ElementId, SceneId};
use tui_vfx_types::RoleTag;

/// Concrete subject participating in a transition envelope.
///
/// Subjects identify *what* a transition operates on. The `previous` and
/// `current` variants are symbolic — they defer concrete-id resolution to
/// the runtime, which knows the scene order. The remaining variants name
/// concrete recipe entities and resolve eagerly.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum TransitionSubjectRef {
    /// Absence of a prior or next subject, used for enter/exit.
    Empty,
    /// Symbolic reference to the surface that was active before the transition began.
    /// Relation transitions (crossfade/push/morph) use this in the `from` slot to
    /// defer concrete scene-id resolution to the runtime.
    Previous,
    /// Symbolic reference to the surface that becomes active after the transition completes.
    /// Relation transitions use this in the `to` slot for the same deferred-resolution reason.
    Current,
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
// <VERS>END OF VERSION: 0.2.0</VERS>
