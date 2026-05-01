// <FILE>crates/tui-vfx-contract/src/cls_transition_spec.rs</FILE> - <DESC>Native v3.1 transition specification DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 native motion/compositing language: transition is a first-class state-change envelope.</WCTX>
// <CLOG>0.1.0: INIT — add native transition envelope with executable tracks.</CLOG>

use crate::{
    LifecyclePhase, ReducedMotionPolicy, ScopeSpec, TransitionId, TransitionIntent,
    TransitionInterruption, TransitionSubjects, TransitionTiming, TransitionTrack,
    TransitionVariant,
};

/// Native v3.1 state-change composition interval.
///
/// A transition coordinates subjects, timing, lifecycle phase participation,
/// interruption policy, accessibility fallback policy, conditional variants,
/// and executable tracks such as `visibility.iris`, `opacity.fade`,
/// `motion.slide`, and `relation.crossfade`. Preset intent may be retained as
/// metadata, but the canonical executable form is the track list.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransitionSpec {
    /// Stable transition identifier.
    pub id: TransitionId,
    /// Optional author intent preserved after shorthand canonicalization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<TransitionIntent>,
    /// Named subjects participating in this state-change interval.
    pub subjects: TransitionSubjects,
    /// Default timing inherited by tracks unless they override it.
    pub timing: TransitionTiming,
    /// Optional default scope inherited by tracks unless they override it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeSpec>,
    /// Lifecycle phases in which this transition participates; empty means the use site selects phases.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub active_phases: Vec<LifecyclePhase>,
    /// Executable canonical transition tracks.
    pub tracks: Vec<TransitionTrack>,
    /// Required policy for superseded interactive transitions.
    pub interruption: TransitionInterruption,
    /// Required accessibility behavior for reduced-motion contexts.
    pub reduced_motion: ReducedMotionPolicy,
    /// Optional generic variants for reduced-motion, capability fallback, or host-selected substitutions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<TransitionVariant>,
}

impl TransitionSpec {
    /// Return true when this transition has a valid id and at least one track.
    pub fn is_structurally_valid(&self) -> bool {
        self.id.is_valid() && !self.tracks.is_empty()
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_spec.rs</FILE> - <DESC>Native v3.1 transition specification DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
