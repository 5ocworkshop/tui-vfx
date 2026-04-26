// <FILE>crates/tui-vfx-debug/src/inspection/cls_role_map_source.rs</FILE> - <DESC>RoleMapSource — discriminant for where a RoleMap came from</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Pipeline observability Unit A — discriminant on RoleMapMaterialized so a consumer can distinguish geometric inference from explicit producer-tagged roles vs externally-injected roles. The focused_row_btop bug stayed hidden for 30+ minutes because the trace and production used different role-map sources without saying so.</WCTX>
// <CLOG>0.1.0: initial enum with Inferred / ExplicitFromProducer / Injected variants. Producer name carried as String so widget identity is visible in the event without a side lookup.</CLOG>

//! Discriminant for where a `RoleMap` came from.
//!
//! Attached to `crate::inspection::TraceEvent::RoleMapMaterialized`.
//! The discriminant is what makes the focused_row_btop bug class
//! detectable without source archaeology: when production calls
//! `apply_composition_with_roles` (semantic-buffer roles) and a
//! diagnostic preview calls `apply_composition` (geometric inference),
//! the two paths emit `RoleMapMaterialized` events with different
//! `RoleMapSource` discriminants. A consumer subscribing to both can
//! see the divergence in one query.
//!
//! Variants are forward-compatible — adding a new producer or a new
//! injection mode is additive.

use serde::{Deserialize, Serialize};

/// Where a `RoleMap` came from at the moment it was materialized.
///
/// Lands on `crate::inspection::TraceEvent::RoleMapMaterialized` so
/// downstream consumers can distinguish geometric inference from
/// explicit producer-tagged roles, and identify which producer when
/// applicable.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoleMapSource {
    /// Roles inferred geometrically (e.g. by `infer_source_roles`)
    /// rather than supplied by an upstream producer.
    Inferred,
    /// Roles supplied by an upstream producer (e.g. a gt-design widget
    /// that built a `SemanticBuffer`).
    ///
    /// `producer` is the widget identity string (e.g. `"ContentShell::card"`)
    /// — preserved on the event so a consumer does not need a side
    /// lookup to know which widget tagged the cells.
    ExplicitFromProducer {
        /// Widget or producer identity that materialized the role map.
        producer: String,
    },
    /// Roles injected externally (test harness, CLI override, replay
    /// from a tape).
    Injected,
}

#[cfg(test)]
mod tests {
    use super::RoleMapSource;

    #[test]
    fn inferred_round_trips() {
        let s = RoleMapSource::Inferred;
        let json = serde_json::to_string(&s).expect("serialize");
        let back: RoleMapSource = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(s, back);
    }

    #[test]
    fn explicit_from_producer_round_trips() {
        let s = RoleMapSource::ExplicitFromProducer {
            producer: "ContentShell::card".to_string(),
        };
        let json = serde_json::to_string(&s).expect("serialize");
        let back: RoleMapSource = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(s, back);
    }

    #[test]
    fn injected_round_trips() {
        let s = RoleMapSource::Injected;
        let json = serde_json::to_string(&s).expect("serialize");
        let back: RoleMapSource = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(s, back);
    }
}

// <FILE>crates/tui-vfx-debug/src/inspection/cls_role_map_source.rs</FILE> - <DESC>RoleMapSource</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
