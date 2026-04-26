// <FILE>recyclebin/crates/tui-vfx-compositor/src/overlays/cls_loopback_badge_state.rs</FILE> - <DESC>Per-frame badge activation state for the abandoned hardcoded-cell-painter L3 badge; recycled in favor of recipe-driven badge per Intention 39</DESC>
// <VERS>VERSION: 0.1.0 (recycled)</VERS>
// <WCTX>Loopback Phase L3 first attempt (recycled 2026-04-26). The recipes-side merge already returns `fired_keys`; the wrapper struct turned out to be unnecessary once the badge moved to the recipe path (the host computes "should I render this recipe?" directly from `fired_keys.is_empty()`).</WCTX>
// <CLOG>0.1.0 (recycled): preserved as the artefact surrounding Intention 39's emergence.</CLOG>

//! Per-frame state for the L3 visibility badge overlay.
//!
//! ABANDONED: superseded by recipe-based architecture per Intention 39.

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LoopbackBadgeState {
    pub fired: bool,
    pub force_show: bool,
    pub fired_keys: Vec<String>,
}

impl LoopbackBadgeState {
    pub fn inactive() -> Self {
        Self::default()
    }

    pub fn from_fired_keys(keys: Vec<String>) -> Self {
        Self {
            fired: !keys.is_empty(),
            force_show: false,
            fired_keys: keys,
        }
    }

    pub fn forced() -> Self {
        Self {
            fired: false,
            force_show: true,
            fired_keys: Vec::new(),
        }
    }

    pub fn with_force_show(mut self) -> Self {
        self.force_show = true;
        self
    }

    pub fn is_active(&self) -> bool {
        self.fired || self.force_show
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inactive_default_is_not_active() {
        assert!(!LoopbackBadgeState::inactive().is_active());
    }

    #[test]
    fn from_empty_fired_keys_is_inactive() {
        let state = LoopbackBadgeState::from_fired_keys(Vec::new());
        assert!(!state.fired);
        assert!(!state.is_active());
        assert!(state.fired_keys.is_empty());
    }

    #[test]
    fn from_non_empty_fired_keys_is_active_and_preserves_keys() {
        let state =
            LoopbackBadgeState::from_fired_keys(vec!["a".into(), "b".into()]);
        assert!(state.fired);
        assert!(state.is_active());
        assert_eq!(state.fired_keys, vec!["a", "b"]);
    }

    #[test]
    fn forced_is_active_without_fires() {
        let state = LoopbackBadgeState::forced();
        assert!(!state.fired);
        assert!(state.force_show);
        assert!(state.is_active());
    }

    #[test]
    fn with_force_show_keeps_existing_fired_keys() {
        let state =
            LoopbackBadgeState::from_fired_keys(vec!["x".into()]).with_force_show();
        assert!(state.fired);
        assert!(state.force_show);
        assert_eq!(state.fired_keys, vec!["x"]);
    }
}

// <FILE>recyclebin/crates/tui-vfx-compositor/src/overlays/cls_loopback_badge_state.rs</FILE>
// <VERS>END OF VERSION: 0.1.0 (recycled)</VERS>
