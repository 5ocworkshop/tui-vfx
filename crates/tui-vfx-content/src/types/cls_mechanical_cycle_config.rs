// <FILE>tui-vfx-content/src/types/cls_mechanical_cycle_config.rs</FILE> - <DESC>Top-level mechanical cycle config aggregating source, route, cascade, and settle</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 of mechanical circular content cycles plan: shared schema-bearing config consumed by Odometer and SplitFlap to describe ordered/circular face traversal.</WCTX>
// <CLOG>0.1.0: introduce MechanicalCycleConfig with default-Pair source so existing recipes stay byte-identical when the field is absent.</CLOG>

use serde::{Deserialize, Serialize};

use super::cls_mechanical_cycle_cascade::MechanicalCascadePolicy;
use super::cls_mechanical_cycle_cascade::MechanicalSettleConfig;
use super::cls_mechanical_cycle_route::MechanicalRouteConfig;
use super::cls_mechanical_cycle_source::MechanicalContentSource;

/// Shared mechanical cycle vocabulary that lets odometer drums, flap stacks,
/// and slot reels traverse the same kind of ordered face supply.
///
/// `Default` produces the explicit `Pair` source with `Forward`/`Simultaneous`
/// scheduling and no settle. Existing Odometer and SplitFlap recipes that omit
/// the `mechanical` field render byte-identically to this default.
///
/// # JSON
///
/// ```json
/// {
///   "source": { "type": "preset", "preset": "decimal_digits" },
///   "route":  { "direction": "numeric_delta" },
///   "cascade": { "type": "numeric_carry", "stagger_fraction": 0.35 },
///   "settle":  { "type": "spring", "overshoot": 0.12, "settle_fraction": 0.18 }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct MechanicalCycleConfig {
    /// What faces exist between source and target.
    #[serde(default)]
    pub source: MechanicalContentSource,
    /// How to choose a route through the cycle.
    #[serde(default)]
    pub route: MechanicalRouteConfig,
    /// How tiles are scheduled relative to each other.
    #[serde(default)]
    pub cascade: MechanicalCascadePolicy,
    /// Optional settle behavior applied at the end of the cycle.
    #[serde(default)]
    pub settle: MechanicalSettleConfig,
}

impl MechanicalCycleConfig {
    /// True when this config is identical to [`MechanicalCycleConfig::default`].
    ///
    /// Used as a `skip_serializing_if` predicate so explicit-Pair configs and
    /// absent fields produce the same JSON in serialized recipes.
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::super::cls_mechanical_cycle_cascade::{
        MechanicalCascadePolicy, MechanicalSettleConfig, UnchangedCellPolicy,
    };
    use super::super::cls_mechanical_cycle_route::CycleDirectionPolicy;
    use super::super::cls_mechanical_cycle_source::{CycleWrapMode, MechanicalContentSource};
    use super::*;

    #[test]
    fn default_is_pair_simultaneous_no_settle() {
        let cfg = MechanicalCycleConfig::default();
        assert_eq!(cfg.source, MechanicalContentSource::Pair);
        assert_eq!(cfg.route.direction, CycleDirectionPolicy::Forward);
        assert_eq!(cfg.cascade, MechanicalCascadePolicy::Simultaneous);
        assert_eq!(cfg.settle, MechanicalSettleConfig::None);
        assert!(cfg.is_default());
    }

    #[test]
    fn default_serde_roundtrip() {
        let cfg = MechanicalCycleConfig::default();
        let json = serde_json::to_string(&cfg).unwrap();
        let back: MechanicalCycleConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(back, cfg);
        assert!(back.is_default());
    }

    #[test]
    fn empty_object_parses_to_default() {
        let cfg: MechanicalCycleConfig = serde_json::from_str("{}").unwrap();
        assert_eq!(cfg, MechanicalCycleConfig::default());
    }

    #[test]
    fn full_decimal_recipe_parses() {
        let json = r#"{
            "source": { "type": "preset", "preset": "decimal_digits" },
            "route":  { "direction": "numeric_delta", "tie_breaker": "forward" },
            "cascade": { "type": "numeric_carry", "stagger_fraction": 0.35, "unchanged": "hold" },
            "settle":  { "type": "spring", "overshoot": 0.12, "settle_fraction": 0.18 }
        }"#;
        let cfg: MechanicalCycleConfig = serde_json::from_str(json).unwrap();
        assert!(matches!(
            cfg.cascade,
            MechanicalCascadePolicy::NumericCarry {
                unchanged: UnchangedCellPolicy::Hold,
                ..
            }
        ));
        assert_eq!(cfg.route.direction, CycleDirectionPolicy::NumericDelta);
    }

    #[test]
    fn ordered_circular_three_face_recipe_parses() {
        let json = r#"{
            "source": {
                "type": "ordered",
                "wrap": "circular",
                "faces": ["A", "B", "C"]
            }
        }"#;
        let cfg: MechanicalCycleConfig = serde_json::from_str(json).unwrap();
        assert!(!cfg.is_default(), "ordered source must not match default");
        match &cfg.source {
            MechanicalContentSource::Ordered { faces, wrap } => {
                assert_eq!(faces.len(), 3);
                assert_eq!(*wrap, CycleWrapMode::Circular);
            }
            other => panic!("unexpected source: {other:?}"),
        }
    }

    #[test]
    fn unknown_field_rejected() {
        let parsed: Result<MechanicalCycleConfig, _> =
            serde_json::from_str(r#"{"source":{"type":"pair"},"flair":"none"}"#);
        assert!(parsed.is_err());
    }
}

// <FILE>tui-vfx-content/src/types/cls_mechanical_cycle_config.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
