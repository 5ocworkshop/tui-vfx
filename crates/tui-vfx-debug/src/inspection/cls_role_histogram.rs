// <FILE>crates/tui-vfx-debug/src/inspection/cls_role_histogram.rs</FILE> - <DESC>RoleHistogram — per-RoleTag cell counts attached to scope-evaluation events</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Pipeline observability Unit A — payload on ScopeEvaluated, RoleMapMaterialized, and PipelineSkipReason::ScopeMatchedZeroCells. Makes role-mismatch bugs (e.g. focused_row_btop) visible in one event by exposing the per-role cell counts the scope predicate saw.</WCTX>
// <CLOG>0.1.0: initial struct with five field counts (background, text, border, indicator, highlight) matching the RoleTag variants the scope predicate distinguishes today.</CLOG>

//! Per-`RoleTag` cell counts for scope-evaluation events.
//!
//! When a stage's scope predicate runs, it visits every cell in the
//! stage area and tallies how many cells carry each role. The histogram
//! is attached to `ScopeEvaluated`, `RoleMapMaterialized`, and the
//! `PipelineSkipReason::ScopeMatchedZeroCells` skip reason so a consumer
//! can read "predicate matched zero text cells, but the area had 320
//! background cells" without joining across events.
//!
//! # Why a fixed struct, not a `HashMap<RoleTag, u32>`?
//!
//! The set of roles is closed-vocabulary; a fixed struct round-trips
//! through serde with stable field names, costs no allocation, and
//! lets `--format sqlite` query individual role counts directly. New
//! roles are added by editing this struct (a deliberate friction point:
//! adding a role is an audit-worthy change).
//!
//! Every field defaults to zero via `#[serde(default)]` so a future
//! taxonomy revision can add a role and older tapes still deserialize.

use serde::{Deserialize, Serialize};

/// Per-`RoleTag` cell counts for one scope evaluation.
///
/// All fields default to zero on deserialization (forward-compat with
/// future role additions). The sum across fields equals the number of
/// cells the scope predicate visited.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RoleHistogram {
    /// Cells tagged `RoleTag::Background`.
    #[serde(default)]
    pub background: u32,
    /// Cells tagged `RoleTag::Text`.
    #[serde(default)]
    pub text: u32,
    /// Cells tagged `RoleTag::Border`.
    #[serde(default)]
    pub border: u32,
    /// Cells tagged `RoleTag::Indicator`.
    #[serde(default)]
    pub indicator: u32,
    /// Cells tagged `RoleTag::Highlight`.
    #[serde(default)]
    pub highlight: u32,
}

impl RoleHistogram {
    /// The empty histogram — every role count is zero.
    pub const EMPTY: RoleHistogram = RoleHistogram {
        background: 0,
        text: 0,
        border: 0,
        indicator: 0,
        highlight: 0,
    };

    /// Sum of all role counts. Equals the total number of cells the
    /// scope predicate visited.
    pub const fn total(self) -> u32 {
        self.background + self.text + self.border + self.indicator + self.highlight
    }
}

#[cfg(test)]
mod tests {
    use super::RoleHistogram;

    #[test]
    fn empty_is_zero() {
        let h = RoleHistogram::EMPTY;
        assert_eq!(h.total(), 0);
    }

    #[test]
    fn total_sums_all_fields() {
        let h = RoleHistogram {
            background: 320,
            text: 0,
            border: 16,
            indicator: 4,
            highlight: 1,
        };
        assert_eq!(h.total(), 320 + 16 + 4 + 1);
    }

    #[test]
    fn round_trips_through_json() {
        let h = RoleHistogram {
            background: 320,
            text: 5,
            border: 16,
            indicator: 0,
            highlight: 1,
        };
        let json = serde_json::to_string(&h).expect("serialize");
        let back: RoleHistogram = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(h, back);
    }

    #[test]
    fn missing_field_defaults_to_zero() {
        let json = r#"{"background": 320}"#;
        let h: RoleHistogram = serde_json::from_str(json).expect("deserialize");
        assert_eq!(h.background, 320);
        assert_eq!(h.text, 0);
        assert_eq!(h.border, 0);
        assert_eq!(h.indicator, 0);
        assert_eq!(h.highlight, 0);
    }
}

// <FILE>crates/tui-vfx-debug/src/inspection/cls_role_histogram.rs</FILE> - <DESC>RoleHistogram</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
