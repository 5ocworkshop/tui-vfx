// <FILE>tui-vfx-content/src/types/cls_mechanical_cycle_route.rs</FILE> - <DESC>Public schema for choosing a route through a mechanical face cycle (direction, tie-breaker, missing-face policy)</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 of mechanical circular content cycles plan: route selection vocabulary that controls how the cycle is traversed between source and target.</WCTX>
// <CLOG>0.1.0: introduce MechanicalRouteConfig, CycleDirectionPolicy, CycleTieBreaker, CycleMissingFacePolicy.</CLOG>

use serde::{Deserialize, Serialize};

/// How a route is selected through a [`MechanicalContentSource`].
///
/// Route selection is independent of window motion: `direction: Reverse`
/// changes which face comes next in the cycle, but it does not flip the
/// existing [`OdometerDirection`] visual roll. Authors who want decrement to
/// visibly roll downward set both `OdometerDirection::Down` and
/// `CycleDirectionPolicy::Reverse` explicitly.
///
/// [`MechanicalContentSource`]: super::cls_mechanical_cycle_source::MechanicalContentSource
/// [`OdometerDirection`]: super::cls_content_effect::OdometerDirection
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(deny_unknown_fields)]
pub struct MechanicalRouteConfig {
    /// Direction the route walks the cycle.
    #[serde(default)]
    pub direction: CycleDirectionPolicy,
    /// Resolves ties when `direction = Shortest` and forward and reverse
    /// distances are equal.
    #[serde(default)]
    pub tie_breaker: CycleTieBreaker,
    /// Number of full additional wraps to insert before the final face.
    /// `0` is the default; slot reels typically use `2`+ to feel like a
    /// physical wheel spinning past several values before settling.
    #[serde(default)]
    pub extra_rotations: u16,
    /// What to do when source or target face is not present in the cycle.
    #[serde(default)]
    pub missing_face: CycleMissingFacePolicy,
}

/// Direction the route walks through an ordered cycle.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum CycleDirectionPolicy {
    /// Walk forward through the cycle indices.
    #[default]
    Forward,
    /// Walk in reverse through the cycle indices.
    Reverse,
    /// Pick whichever traversal has fewer intermediate faces; ties resolved
    /// by [`CycleTieBreaker`].
    Shortest,
    /// Choose forward or reverse from numeric increment/decrement context.
    /// Requires faces to be exactly the decimal digits `0`..=`9`.
    NumericDelta,
    /// Reserved for future override sources. Recipes currently must not set
    /// `Authored`; the validator rejects it.
    Authored,
}

/// Tie-breaker for `CycleDirectionPolicy::Shortest`.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum CycleTieBreaker {
    /// Pick the forward route when distances are equal.
    #[default]
    Forward,
    /// Pick the reverse route when distances are equal.
    Reverse,
}

/// What to do when a route endpoint isn't present in the cycle.
///
/// `Error` is the strict default; recipes that ship today should opt in to
/// fallbacks deliberately, with awareness that fallbacks change the visible
/// behavior.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum CycleMissingFacePolicy {
    /// Reject the recipe at validation time.
    #[default]
    Error,
    /// Fall back to a direct `[from, to]` Pair route for this tile.
    PairFallback,
    /// Insert the missing face at the end of the cycle and continue.
    InsertAtEnd,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_default_is_forward_strict() {
        let cfg = MechanicalRouteConfig::default();
        assert_eq!(cfg.direction, CycleDirectionPolicy::Forward);
        assert_eq!(cfg.tie_breaker, CycleTieBreaker::Forward);
        assert_eq!(cfg.extra_rotations, 0);
        assert_eq!(cfg.missing_face, CycleMissingFacePolicy::Error);
    }

    #[test]
    fn empty_object_parses_to_default() {
        let cfg: MechanicalRouteConfig = serde_json::from_str("{}").unwrap();
        assert_eq!(cfg, MechanicalRouteConfig::default());
    }

    #[test]
    fn direction_reverse_parses() {
        let cfg: MechanicalRouteConfig =
            serde_json::from_str(r#"{"direction":"reverse"}"#).unwrap();
        assert_eq!(cfg.direction, CycleDirectionPolicy::Reverse);
    }

    #[test]
    fn extra_rotations_parses() {
        let cfg: MechanicalRouteConfig = serde_json::from_str(r#"{"extra_rotations":3}"#).unwrap();
        assert_eq!(cfg.extra_rotations, 3);
    }

    #[test]
    fn missing_face_pair_fallback_parses() {
        let cfg: MechanicalRouteConfig =
            serde_json::from_str(r#"{"missing_face":"pair_fallback"}"#).unwrap();
        assert_eq!(cfg.missing_face, CycleMissingFacePolicy::PairFallback);
    }

    #[test]
    fn unknown_field_rejected() {
        let parsed: Result<MechanicalRouteConfig, _> =
            serde_json::from_str(r#"{"direction":"forward","spin":"yes"}"#);
        assert!(parsed.is_err());
    }
}

// <FILE>tui-vfx-content/src/types/cls_mechanical_cycle_route.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
