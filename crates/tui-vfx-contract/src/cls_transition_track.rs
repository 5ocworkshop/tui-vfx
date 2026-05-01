// <FILE>crates/tui-vfx-contract/src/cls_transition_track.rs</FILE> - <DESC>Native v3.1 transition track union</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>V3.1 native motion/compositing language: executable transition tracks, not effect chains.</WCTX>
// <CLOG>0.3.0: MINOR — add color-fade, materialize, and structured wipe geometry track shapes.
// 0.2.0: MINOR — add recipe-oracle content, style, blinds, and path track shapes.
// 0.1.0: INIT — add visibility, opacity, motion, and relation track families.</CLOG>

use crate::{
    SceneAnchor, ScopeSpec, StyleColorSource, TransitionBlindsOrientation, TransitionCascadeOrder,
    TransitionEdge, TransitionFocal, TransitionMaterializePattern, TransitionMotionPath,
    TransitionMotionSampling, TransitionRevealDirection, TransitionTextCursor, TransitionTiming,
    TransitionTrackSubject, TransitionTravelDirection, TransitionVisibilityGeometry, ValueSource,
    VisibilityIrisShape,
};

/// Executable canonical V3.1 transition track.
///
/// Tracks express one animated concern inside a transition envelope. They are
/// executed directly by a V3.1 compositor; they are not converted to a legacy
/// DTO or interpreted through an effect-chain translation layer.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum TransitionTrack {
    /// `visibility.wipe` controls coverage/reveal with wipe geometry.
    #[serde(rename = "visibility.wipe")]
    VisibilityWipe {
        /// Subject whose visibility is controlled by this track.
        subject: TransitionTrackSubject,
        /// Qualified reveal direction; use `angle` with `angleDegrees` for diagonal/custom wipes.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reveal_direction: Option<TransitionRevealDirection>,
        /// Optional structured geometry for non-cardinal wipes.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        geometry: Option<TransitionVisibilityGeometry>,
        /// Optional custom reveal angle in degrees for diagonal wipes.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        angle_degrees: Option<ValueSource>,
        /// Optional visibility boundary treatment.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        edge: Option<TransitionEdge>,
        /// Optional per-track timing override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timing: Option<TransitionTiming>,
        /// Optional per-track scope override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scope: Option<ScopeSpec>,
        /// Optional normalized 0..1 transition progress source overriding time-derived progress.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transition_progress: Option<ValueSource>,
    },
    /// `visibility.iris` controls aperture reveal around a focal point.
    #[serde(rename = "visibility.iris")]
    VisibilityIris {
        /// Subject whose visibility is controlled by this track.
        subject: TransitionTrackSubject,
        /// Aperture shape.
        shape: VisibilityIrisShape,
        /// Optional focal point.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        focal: Option<TransitionFocal>,
        /// Optional visibility boundary treatment.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        edge: Option<TransitionEdge>,
        /// Optional per-track timing override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timing: Option<TransitionTiming>,
        /// Optional per-track scope override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scope: Option<ScopeSpec>,
        /// Optional normalized 0..1 transition progress source overriding time-derived progress.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transition_progress: Option<ValueSource>,
    },
    /// `visibility.dissolve` controls per-cell visibility reveal.
    #[serde(rename = "visibility.dissolve")]
    VisibilityDissolve {
        /// Subject whose visibility is controlled by this track.
        subject: TransitionTrackSubject,
        /// Optional deterministic seed source.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        seed: Option<ValueSource>,
        /// Optional per-track timing override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timing: Option<TransitionTiming>,
        /// Optional per-track scope override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scope: Option<ScopeSpec>,
        /// Optional normalized 0..1 transition progress source overriding time-derived progress.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transition_progress: Option<ValueSource>,
    },
    /// `visibility.stippled` controls stipple-pattern visibility reveal.
    #[serde(rename = "visibility.stippled")]
    VisibilityStippled {
        /// Subject whose visibility is controlled by this track.
        subject: TransitionTrackSubject,
        /// Optional normalized 0..1 transition progress source overriding time-derived progress.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transition_progress: Option<ValueSource>,
    },
    /// `visibility.braille` controls braille-pattern visibility reveal.
    #[serde(rename = "visibility.braille")]
    VisibilityBraille {
        /// Subject whose visibility is controlled by this track.
        subject: TransitionTrackSubject,
        /// Optional normalized 0..1 transition progress source overriding time-derived progress.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transition_progress: Option<ValueSource>,
    },
    /// `visibility.blinds` controls banded blind-style visibility reveal.
    #[serde(rename = "visibility.blinds")]
    VisibilityBlinds {
        /// Subject whose visibility is controlled by this track.
        subject: TransitionTrackSubject,
        /// Blind band orientation.
        orientation: TransitionBlindsOrientation,
        /// Number of blinds/bands as a bindable source.
        count: ValueSource,
        /// Optional per-track timing override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timing: Option<TransitionTiming>,
        /// Optional per-track scope override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scope: Option<ScopeSpec>,
        /// Optional normalized 0..1 transition progress source overriding time-derived progress.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transition_progress: Option<ValueSource>,
    },
    /// `visibility.materialize` reveals a subject through deterministic materialization.
    #[serde(rename = "visibility.materialize")]
    VisibilityMaterialize {
        /// Subject whose visibility is controlled by this track.
        subject: TransitionTrackSubject,
        /// Optional anchor from which materialization begins.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from_anchor: Option<SceneAnchor>,
        /// Materialization pattern.
        pattern: TransitionMaterializePattern,
        /// Optional deterministic seed source.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        seed: Option<ValueSource>,
        /// Optional chunk size source.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        chunk_size: Option<ValueSource>,
        /// Optional noise amount source.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        noise_amount: Option<ValueSource>,
        /// Optional visibility boundary treatment.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        edge: Option<TransitionEdge>,
        /// Optional per-track timing override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timing: Option<TransitionTiming>,
        /// Optional per-track scope override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scope: Option<ScopeSpec>,
        /// Optional normalized 0..1 transition progress source overriding time-derived progress.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transition_progress: Option<ValueSource>,
    },
    /// `opacity.fade` changes one subject's opacity.
    #[serde(rename = "opacity.fade")]
    OpacityFade {
        /// Subject whose opacity changes.
        subject: TransitionTrackSubject,
        /// Optional starting opacity source.
        #[serde(default, skip_serializing_if = "Option::is_none", rename = "from")]
        opacity_from: Option<ValueSource>,
        /// Optional ending opacity source.
        #[serde(default, skip_serializing_if = "Option::is_none", rename = "to")]
        opacity_to: Option<ValueSource>,
        /// Optional per-track timing override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timing: Option<TransitionTiming>,
        /// Optional per-track scope override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scope: Option<ScopeSpec>,
        /// Optional normalized 0..1 transition progress source overriding time-derived progress.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transition_progress: Option<ValueSource>,
    },
    /// `motion.slide` moves a single subject.
    #[serde(rename = "motion.slide")]
    MotionSlide {
        /// Subject that travels.
        subject: TransitionTrackSubject,
        /// Qualified movement direction.
        travel_direction: TransitionTravelDirection,
        /// Optional per-track timing override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timing: Option<TransitionTiming>,
        /// Optional normalized 0..1 transition progress source overriding time-derived progress.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transition_progress: Option<ValueSource>,
    },
    /// `motion.path` moves one subject along a grid-native path during a state-change interval.
    #[serde(rename = "motion.path")]
    MotionPath {
        /// Subject that travels.
        subject: TransitionTrackSubject,
        /// Path shape to follow.
        path: TransitionMotionPath,
        /// Optional grid quantization/sampling policy.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sampling: Option<TransitionMotionSampling>,
        /// Optional per-track timing override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timing: Option<TransitionTiming>,
        /// Optional normalized 0..1 transition progress source overriding time-derived progress.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transition_progress: Option<ValueSource>,
    },
    /// `relation.crossfade` blends from/to surfaces in relation to each other.
    #[serde(rename = "relation.crossfade")]
    RelationCrossfade {
        /// Optional normalized 0..1 transition progress source overriding time-derived progress.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transition_progress: Option<ValueSource>,
    },
    /// `relation.push` coordinates from/to surface displacement.
    #[serde(rename = "relation.push")]
    RelationPush {
        /// Qualified push travel direction.
        travel_direction: TransitionTravelDirection,
        /// Optional normalized 0..1 transition progress source overriding time-derived progress.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transition_progress: Option<ValueSource>,
    },
    /// `relation.morph` represents between-surface correspondence transform intent.
    #[serde(rename = "relation.morph")]
    RelationMorph {
        /// Optional normalized 0..1 transition progress source overriding time-derived progress.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transition_progress: Option<ValueSource>,
    },
    /// `content.splitFlap` reveals or mutates textual content with split-flap glyph cycling.
    #[serde(rename = "content.splitFlap")]
    ContentSplitFlap {
        /// Subject whose content changes.
        subject: TransitionTrackSubject,
        /// Optional per-track scope override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scope: Option<ScopeSpec>,
        /// Bindable flap cycling rate.
        flap_rate: ValueSource,
        /// Bindable number of flap cycles.
        cycles: ValueSource,
        /// Optional named character set, such as `uppercase`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        charset: Option<String>,
        /// Optional cascade delay/amount.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cascade: Option<ValueSource>,
        /// Optional deterministic cascade order.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        order: Option<TransitionCascadeOrder>,
        /// Optional per-track timing override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timing: Option<TransitionTiming>,
        /// Optional normalized 0..1 transition progress source overriding time-derived progress.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transition_progress: Option<ValueSource>,
    },
    /// `content.typewriter` reveals textual content over the transition interval.
    #[serde(rename = "content.typewriter")]
    ContentTypewriter {
        /// Subject whose content changes.
        subject: TransitionTrackSubject,
        /// Optional per-track scope override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scope: Option<ScopeSpec>,
        /// Optional bindable variance in typing reveal rate.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        typing_rate_variance: Option<ValueSource>,
        /// Optional cursor behavior.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cursor: Option<TransitionTextCursor>,
        /// Optional per-track timing override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timing: Option<TransitionTiming>,
        /// Optional normalized 0..1 transition progress source overriding time-derived progress.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transition_progress: Option<ValueSource>,
    },
    /// `style.glistenBand` applies a transient style sweep during a transition interval.
    #[serde(rename = "style.glistenBand")]
    StyleGlistenBand {
        /// Subject whose style is affected.
        subject: TransitionTrackSubject,
        /// Optional per-track scope override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scope: Option<ScopeSpec>,
        /// Bindable band width in cells.
        band_width: ValueSource,
        /// Bindable sweep rate.
        sweep_rate: ValueSource,
        /// Bindable sweep angle in degrees.
        angle_degrees: ValueSource,
        /// Optional leading color source.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        head_color: Option<ValueSource>,
        /// Optional trailing color source.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tail_color: Option<ValueSource>,
        /// Optional per-track timing override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timing: Option<TransitionTiming>,
        /// Optional normalized 0..1 transition progress source overriding time-derived progress.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transition_progress: Option<ValueSource>,
    },
    /// `style.colorFade` interpolates cell colors, distinct from opacity blending.
    #[serde(rename = "style.colorFade")]
    StyleColorFade {
        /// Subject whose style is affected.
        subject: TransitionTrackSubject,
        /// Optional foreground/background/both routing label.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        channel_target: Option<String>,
        /// Starting color source.
        from_color: StyleColorSource,
        /// Ending color source.
        to_color: StyleColorSource,
        /// Optional per-track timing override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timing: Option<TransitionTiming>,
        /// Optional per-track scope override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scope: Option<ScopeSpec>,
        /// Optional normalized 0..1 transition progress source overriding time-derived progress.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transition_progress: Option<ValueSource>,
    },
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_track.rs</FILE> - <DESC>Native v3.1 transition track union</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
