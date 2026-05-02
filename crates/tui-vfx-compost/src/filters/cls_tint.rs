// <FILE>crates/tui-vfx-compost/src/filters/cls_tint.rs</FILE> - <DESC>v3.1-native tint filter primitive port</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Ported from tui-vfx-compositor/src/filters/cls_tint.rs: preserve direct strength blending and rounded RGB interpolation while mapping apply_to onto v3.1 channelTarget.</WCTX>
// <CLOG>0.1.0: INIT — lift tint filter runtime logic into the compost filters hierarchy with v3.1 descriptor metadata.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{
    CellAccess, CellChannel, CellWritePolicy, CoordinateSpace, EffectCompletion, EffectDescriptor,
    EffectDomain, EffectId, EffectInputId, EffectInputSpec, EffectLifecycle, NumericRange,
    RoleSpace, RoleWritePolicyKind, RuntimeMutability, ScopeKind, ScopeSupport, Value, ValueKind,
    ValueSpec, WriteSupport,
};
use tui_vfx_types::Color;

use crate::filters::ChannelTarget;
use crate::primitive::{
    CellView, EffectPrimitive, EffectRuntimeContext, EffectRuntimeError, FrameFilterRuntime,
    NoOutputs, PrimitiveInputs, PrimitiveOutputs,
};

/// Runtime input bundle for `filter.tint`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FilterTintInputs {
    /// Tint color blended into targeted channels.
    pub color: Color,
    /// Strength of the tint; 0.0 unchanged, 1.0 full replacement.
    pub strength: f32,
    /// Foreground/background channel target.
    pub channel_target: ChannelTarget,
}

impl Default for FilterTintInputs {
    fn default() -> Self {
        Self {
            color: Color::RED,
            strength: 0.3,
            channel_target: ChannelTarget::Both,
        }
    }
}

impl PrimitiveInputs for FilterTintInputs {
    fn input_specs() -> BTreeMap<EffectInputId, EffectInputSpec> {
        BTreeMap::from([
            (
                EffectInputId::new("color"),
                EffectInputSpec {
                    display_name: Some("Color".to_string()),
                    description: Some("Tint color blended into targeted channels.".to_string()),
                    value: ValueSpec {
                        kind: ValueKind::Color,
                        default: Some(Value::Color(Color::RED)),
                        range: None,
                        allowed_values: vec![],
                        unit: None,
                        semantic: Some("tint-color".to_string()),
                    },
                    optional: false,
                    bindable: true,
                    runtime_mutability: RuntimeMutability::PhaseStart,
                },
            ),
            (
                EffectInputId::new("strength"),
                EffectInputSpec {
                    display_name: Some("Strength".to_string()),
                    description: Some("Blend amount between original and tint color.".to_string()),
                    value: ValueSpec {
                        kind: ValueKind::Number,
                        default: Some(Value::Number(0.3)),
                        range: Some(NumericRange {
                            min: Some(0.0),
                            max: Some(1.0),
                        }),
                        allowed_values: vec![],
                        unit: None,
                        semantic: Some("effect-strength".to_string()),
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
                        "Canonical replacement for legacy apply_to foreground/background routing."
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

/// Rust-owned descriptor/runtime for the v3.1 `filter.tint` primitive.
#[derive(Clone, Copy, Debug, Default)]
pub struct FilterTint;

impl EffectPrimitive for FilterTint {
    type Inputs = FilterTintInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        EffectDescriptor {
            id: EffectId::new("filter.tint"),
            version: "0.1.0".to_string(),
            display_name: "Tint Filter".to_string(),
            category: Some("filter primitive".to_string()),
            domain: EffectDomain::FrameFilter,
            cell_access: CellAccess {
                reads: all_cell_channels(),
                writes: all_cell_channels(),
            },
            scope_support: ScopeSupport {
                kinds: vec![ScopeKind::All],
                coordinate_spaces: vec![CoordinateSpace::DestinationLocal],
                role_spaces: vec![RoleSpace::Destination],
            },
            write_support: WriteSupport {
                cell_policies: vec![CellWritePolicy::WriteCell],
                role_policies: vec![RoleWritePolicyKind::PreserveDestination],
            },
            inputs: FilterTintInputs::input_specs(),
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

impl FrameFilterRuntime for FilterTint {
    fn filter_cell(
        inputs: &Self::Inputs,
        cell: &mut CellView<'_, Self>,
        _context: &EffectRuntimeContext<'_>,
    ) -> Result<(), EffectRuntimeError> {
        if inputs.channel_target.affects_foreground() {
            cell.set_foreground(blend_tint(cell.foreground(), inputs.color, inputs.strength));
        }
        if inputs.channel_target.affects_background() {
            cell.set_background(blend_tint(cell.background(), inputs.color, inputs.strength));
        }
        Ok(())
    }
}

/// Blend `tint` into `base` with legacy rounded RGB interpolation.
pub fn blend_tint(base: Color, tint: Color, strength: f32) -> Color {
    let s = strength.clamp(0.0, 1.0);
    let r = (base.r as f32 * (1.0 - s) + tint.r as f32 * s).round() as u8;
    let g = (base.g as f32 * (1.0 - s) + tint.g as f32 * s).round() as u8;
    let b = (base.b as f32 * (1.0 - s) + tint.b as f32 * s).round() as u8;
    Color::rgb(r, g, b)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SampleContext;
    use tui_vfx_types::{Cell, Modifiers};

    fn apply(inputs: FilterTintInputs, cell: &mut Cell) {
        let sample = SampleContext::default();
        let context = EffectRuntimeContext::new(&sample, 0, 0, 10, 10);
        let mut view = CellView::<FilterTint>::new(cell);
        FilterTint::filter_cell(&inputs, &mut view, &context).expect("filter applies");
    }

    #[test]
    fn default_matches_legacy() {
        let tint = FilterTintInputs::default();
        assert_eq!(tint.color, Color::RED);
        assert_eq!(tint.strength, 0.3);
        assert_eq!(tint.channel_target, ChannelTarget::Both);
    }

    #[test]
    fn applies_to_foreground_only() {
        let mut cell = Cell::styled('x', Color::BLACK, Color::WHITE, Modifiers::NONE);
        apply(
            FilterTintInputs {
                color: Color::RED,
                strength: 1.0,
                channel_target: ChannelTarget::Foreground,
            },
            &mut cell,
        );
        assert_eq!(cell.fg, Color::RED);
        assert_eq!(cell.bg, Color::WHITE);
    }

    #[test]
    fn applies_to_background_only() {
        let mut cell = Cell::styled('x', Color::WHITE, Color::BLACK, Modifiers::NONE);
        apply(
            FilterTintInputs {
                color: Color::GREEN,
                strength: 1.0,
                channel_target: ChannelTarget::Background,
            },
            &mut cell,
        );
        assert_eq!(cell.fg, Color::WHITE);
        assert_eq!(cell.bg, Color::GREEN);
    }

    #[test]
    fn partial_strength_uses_rounded_blend() {
        assert_eq!(
            blend_tint(Color::BLACK, Color::RED, 0.5),
            Color::rgb(128, 0, 0)
        );
    }

    #[test]
    fn both_channels_are_tinted() {
        let mut cell = Cell::styled('x', Color::BLACK, Color::BLACK, Modifiers::NONE);
        apply(
            FilterTintInputs {
                color: Color::RED,
                strength: 0.5,
                channel_target: ChannelTarget::Both,
            },
            &mut cell,
        );
        assert_eq!(cell.fg, Color::rgb(128, 0, 0));
        assert_eq!(cell.bg, Color::rgb(128, 0, 0));
    }
}

// <FILE>crates/tui-vfx-compost/src/filters/cls_tint.rs</FILE> - <DESC>v3.1-native tint filter primitive port</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
