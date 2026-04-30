// <FILE>crates/tui-vfx-player-ui/src/cls_player_ui_control.rs</FILE> - <DESC>Descriptor-derived studio control DTO for visual player UI</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Descriptor-driven studio controls: expose descriptor-addressed runtime inputs as well as signal-backed controls.</WCTX>
// <CLOG>0.3.0: MINOR — carry current value, descriptor identity, range, enum, and mutability metadata.
// 0.2.0: MINOR — add target kind, control kind, and runtime input address for descriptor-derived controls.
// 0.1.0: INIT — add generated studio control row model.</CLOG>

/// One generated studio control shown by the player UI.
#[derive(Clone, Debug, PartialEq)]
pub struct PlayerUiControl {
    /// Stable control id shown in scripts and snapshots.
    pub id: String,
    /// Human-readable label.
    pub label: String,
    /// Canonical value kind, such as `number` or `boolean`.
    pub value_kind: String,
    /// Control origin kind, such as effect input or source input.
    pub source_kind: String,
    /// Descriptor id that owns this input.
    pub descriptor_id: String,
    /// Optional recipe-local node id for effect inputs.
    pub node_id: Option<String>,
    /// Optional recipe-local source instance id for source inputs.
    pub source_instance_id: Option<String>,
    /// Descriptor-local input name.
    pub input_name: String,
    /// Recommended UI control kind, such as `slider`, `toggle`, or `colorPicker`.
    pub control_kind: String,
    /// Recipe/default current value serialized for display.
    pub current_value: Option<serde_json::Value>,
    /// Descriptor default value serialized for display.
    pub default_value: Option<serde_json::Value>,
    /// Numeric range when declared by the descriptor.
    pub range: Option<tui_vfx_contract::NumericRange>,
    /// Allowed values when declared by the descriptor.
    pub allowed_values: Vec<String>,
    /// Descriptor runtime mutability label.
    pub runtime_mutability: String,
    /// Whether this input is optional.
    pub optional: bool,
    /// Mutation target kind: signal or runtime input override.
    pub target_kind: String,
    /// Graph signal id mutated by this control.
    pub signal_id: String,
    /// Runtime input override address mutated by this control when not signal-backed.
    pub runtime_input: String,
    /// Effect node/effect context for this control.
    pub source: String,
}

// <FILE>crates/tui-vfx-player-ui/src/cls_player_ui_control.rs</FILE> - <DESC>Descriptor-derived studio control DTO for visual player UI</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
