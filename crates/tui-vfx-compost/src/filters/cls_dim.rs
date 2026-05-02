// <FILE>crates/tui-vfx-compost/src/filters/cls_dim.rs</FILE> - <DESC>Rust-owned v3.1 filter.dim primitive declaration and runtime</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 first port: derive filter.dim truth from v3.1 contract descriptor shape and legacy dim semantics while avoiding legacy imports and generated descriptor artifacts.</WCTX>
// <CLOG>0.1.0: INIT — add filter.dim descriptor and FrameFilterRuntime using shared channel-target and dim-color helpers.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{
    CellAccess, CellChannel, CellWritePolicy, CoordinateSpace, EffectCompletion, EffectDescriptor,
    EffectDomain, EffectId, EffectInputId, EffectInputSpec, EffectLifecycle, NumericRange,
    RoleSpace, RoleWritePolicyKind, RuntimeMutability, ScopeKind, ScopeSupport, Value, ValueKind,
    ValueSpec, WriteSupport,
};

use crate::filters::{ChannelTarget, dim_color};
use crate::primitive::{
    CellView, EffectPrimitive, EffectRuntimeContext, EffectRuntimeError, FrameFilterRuntime,
    NoOutputs, PrimitiveInputs, PrimitiveOutputs,
};

/// Runtime input bundle for `filter.dim`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FilterDimInputs {
    /// Dimming amount: `0.0` leaves colors unchanged, `1.0` turns targeted RGB channels black.
    pub factor: f32,
    /// Foreground/background channel target.
    pub channel_target: ChannelTarget,
}

impl Default for FilterDimInputs {
    fn default() -> Self {
        Self {
            factor: 0.3,
            channel_target: ChannelTarget::Both,
        }
    }
}

impl PrimitiveInputs for FilterDimInputs {
    fn input_specs() -> BTreeMap<EffectInputId, EffectInputSpec> {
        BTreeMap::from([
            (
                EffectInputId::new("factor"),
                EffectInputSpec {
                    display_name: Some("Factor".to_string()),
                    description: None,
                    value: ValueSpec {
                        kind: ValueKind::Number,
                        default: Some(Value::Number(0.3)),
                        range: Some(NumericRange {
                            min: Some(0.0),
                            max: Some(1.0),
                        }),
                        allowed_values: vec![],
                        unit: None,
                        semantic: Some("brightness-scale".to_string()),
                    },
                    optional: false,
                    bindable: true,
                    runtime_mutability: RuntimeMutability::PhaseStart,
                },
            ),
            (
                EffectInputId::new("channelTarget"),
                EffectInputSpec {
                    display_name: Some("Channel Target".to_string()),
                    description: Some(
                        "Canonical replacement for old foreground/background channel-scoped filter payloads."
                            .to_string(),
                    ),
                    value: ValueSpec {
                        kind: ValueKind::Enum,
                        default: Some(Value::Enum(ChannelTarget::Both.as_str().to_string())),
                        range: None,
                        allowed_values: ChannelTarget::allowed_values(),
                        unit: None,
                        semantic: Some("channel-target".to_string()),
                    },
                    optional: false,
                    bindable: true,
                    runtime_mutability: RuntimeMutability::PhaseStart,
                },
            ),
        ])
    }
}

/// Rust-owned descriptor/runtime for the v3.1 `filter.dim` primitive.
#[derive(Clone, Copy, Debug, Default)]
pub struct FilterDim;

impl EffectPrimitive for FilterDim {
    type Inputs = FilterDimInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        EffectDescriptor {
            id: EffectId::new("filter.dim"),
            version: "0.1.0".to_string(),
            display_name: "Dim Filter".to_string(),
            category: Some("debug primitive".to_string()),
            domain: EffectDomain::FrameFilter,
            cell_access: CellAccess {
                reads: all_cell_channels(),
                writes: all_cell_channels(),
            },
            scope_support: ScopeSupport {
                kinds: vec![ScopeKind::All],
                coordinate_spaces: vec![
                    CoordinateSpace::DestinationLocal,
                    CoordinateSpace::SampledSource,
                ],
                role_spaces: vec![RoleSpace::SampledSource, RoleSpace::Destination],
            },
            write_support: WriteSupport {
                cell_policies: vec![
                    CellWritePolicy::WriteCell,
                    CellWritePolicy::SkipTransparentEmpty,
                ],
                role_policies: vec![
                    RoleWritePolicyKind::PreserveDestination,
                    RoleWritePolicyKind::CopySampledSource,
                ],
            },
            inputs: FilterDimInputs::input_specs(),
            outputs: NoOutputs::output_specs(),
            lifecycle: EffectLifecycle {
                completion: EffectCompletion::Instant,
                resettable: true,
                seekable: true,
                deterministic_with_seed: true,
            },
        }
    }
}

impl FrameFilterRuntime for FilterDim {
    fn filter_cell(
        inputs: &Self::Inputs,
        cell: &mut CellView<'_, Self>,
        _context: &EffectRuntimeContext<'_>,
    ) -> Result<(), EffectRuntimeError> {
        let factor = inputs.factor.clamp(0.0, 1.0);
        if inputs.channel_target.affects_foreground() {
            cell.set_foreground(dim_color(cell.foreground(), factor));
        }
        if inputs.channel_target.affects_background() {
            cell.set_background(dim_color(cell.background(), factor));
        }
        Ok(())
    }
}

fn all_cell_channels() -> Vec<CellChannel> {
    vec![
        CellChannel::Glyph,
        CellChannel::Foreground,
        CellChannel::Background,
        CellChannel::Modifiers,
        CellChannel::ModifierAlpha,
        CellChannel::Role,
    ]
}

// <FILE>crates/tui-vfx-compost/src/filters/cls_dim.rs</FILE> - <DESC>Rust-owned v3.1 filter.dim primitive declaration and runtime</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
