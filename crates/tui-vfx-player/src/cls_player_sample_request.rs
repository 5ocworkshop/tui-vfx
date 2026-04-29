// <FILE>crates/tui-vfx-player/src/cls_player_sample_request.rs</FILE> - <DESC>Player sample request DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K0: make phase, time, dimensions, and signals explicit.</WCTX>
// <CLOG>0.1.0: INIT — add sampled frame request contract for RecipePlayer.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{LifecyclePhase, SignalId, Value};

/// Request used to sample a single contract-native recipe frame.
#[derive(Clone, Debug, PartialEq)]
pub struct PlayerSampleRequest {
    /// Lifecycle phase to sample.
    pub phase: LifecyclePhase,
    /// Normalized phase progress in the range expected by K0 adapters.
    pub phase_t: f64,
    /// Optional loop-local normalized progress when a future looping source needs it.
    pub loop_t: Option<f64>,
    /// Optional frame width override.
    pub width: Option<usize>,
    /// Optional frame height override.
    pub height: Option<usize>,
    /// Host signal values available to lifecycle trigger evaluation.
    pub signals: BTreeMap<SignalId, Value>,
}

impl Default for PlayerSampleRequest {
    fn default() -> Self {
        Self {
            phase: LifecyclePhase::Dwell,
            phase_t: 1.0,
            loop_t: None,
            width: None,
            height: None,
            signals: BTreeMap::new(),
        }
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_sample_request.rs</FILE> - <DESC>Player sample request DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
