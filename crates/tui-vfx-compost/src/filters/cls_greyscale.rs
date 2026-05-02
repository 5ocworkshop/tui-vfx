// <FILE>crates/tui-vfx-compost/src/filters/cls_greyscale.rs</FILE> - <DESC>v3.1-native greyscale filter primitive port</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Ported from tui-vfx-compositor/src/filters/cls_greyscale.rs: preserve BT.601 luminance and strength blending behavior while mapping apply_to onto v3.1 channelTarget.</WCTX>
// <CLOG>0.1.0: INIT — lift greyscale filter runtime logic into the compost filters hierarchy with v3.1 descriptor metadata.</CLOG>

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

/// Runtime input bundle for `filter.greyscale`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FilterGreyscaleInputs {
    /// Strength of the greyscale effect; 0.0 unchanged, 1.0 full greyscale.
    pub strength: f32,
    /// Foreground/background channel target.
    pub channel_target: ChannelTarget,
}

impl Default for FilterGreyscaleInputs {
    fn default() -> Self {
        Self {
            strength: 1.0,
            channel_target: ChannelTarget::Both,
        }
    }
}

impl PrimitiveInputs for FilterGreyscaleInputs {
    fn input_specs() -> BTreeMap<EffectInputId, EffectInputSpec> {
        BTreeMap::from([
            (
                EffectInputId::new("strength"),
                EffectInputSpec {
                    display_name: Some("Strength".to_string()),
                    description: Some(
                        "Blend amount between original color and BT.601 greyscale.".to_string(),
                    ),
                    value: ValueSpec {
                        kind: ValueKind::Number,
                        default: Some(Value::Number(1.0)),
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

/// Rust-owned descriptor/runtime for the v3.1 `filter.greyscale` primitive.
#[derive(Clone, Copy, Debug, Default)]
pub struct FilterGreyscale;

impl EffectPrimitive for FilterGreyscale {
    type Inputs = FilterGreyscaleInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        EffectDescriptor {
            id: EffectId::new("filter.greyscale"),
            version: "0.1.0".to_string(),
            display_name: "Greyscale Filter".to_string(),
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
            inputs: FilterGreyscaleInputs::input_specs(),
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

impl FrameFilterRuntime for FilterGreyscale {
    fn filter_cell(
        inputs: &Self::Inputs,
        cell: &mut CellView<'_, Self>,
        _context: &EffectRuntimeContext<'_>,
    ) -> Result<(), EffectRuntimeError> {
        if inputs.channel_target.affects_foreground() {
            cell.set_foreground(apply_greyscale(cell.foreground(), inputs.strength));
        }
        if inputs.channel_target.affects_background() {
            cell.set_background(apply_greyscale(cell.background(), inputs.strength));
        }
        Ok(())
    }
}

/// Convert RGB to greyscale using BT.601 luminance coefficients.
pub fn greyscale_luminance(r: u8, g: u8, b: u8) -> u8 {
    (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32).round() as u8
}

fn apply_greyscale(color: Color, strength: f32) -> Color {
    let grey = greyscale_luminance(color.r, color.g, color.b);
    let s = strength.clamp(0.0, 1.0);
    let r = (color.r as f32 * (1.0 - s) + grey as f32 * s).round() as u8;
    let g = (color.g as f32 * (1.0 - s) + grey as f32 * s).round() as u8;
    let b = (color.b as f32 * (1.0 - s) + grey as f32 * s).round() as u8;
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

    fn apply(inputs: FilterGreyscaleInputs, cell: &mut Cell) {
        let sample = SampleContext::default();
        let context = EffectRuntimeContext::new(&sample, 0, 0, 10, 10);
        let mut view = CellView::<FilterGreyscale>::new(cell);
        FilterGreyscale::filter_cell(&inputs, &mut view, &context).expect("filter applies");
    }

    #[test]
    fn bt601_luminance_matches_legacy_primary_colors() {
        assert_eq!(greyscale_luminance(255, 0, 0), 76);
        assert_eq!(greyscale_luminance(0, 255, 0), 150);
        assert_eq!(greyscale_luminance(0, 0, 255), 29);
        assert!(greyscale_luminance(255, 255, 255) >= 254);
        assert_eq!(greyscale_luminance(0, 0, 0), 0);
    }

    #[test]
    fn full_strength_desaturates_foreground() {
        let mut cell = Cell::styled(
            'x',
            Color::rgb(255, 0, 0),
            Color::rgb(255, 255, 255),
            Modifiers::NONE,
        );
        apply(FilterGreyscaleInputs::default(), &mut cell);
        assert_eq!(cell.fg, Color::rgb(76, 76, 76));
    }

    #[test]
    fn zero_strength_leaves_color_unchanged() {
        let mut cell = Cell::styled('x', Color::rgb(255, 0, 0), Color::BLACK, Modifiers::NONE);
        apply(
            FilterGreyscaleInputs {
                strength: 0.0,
                channel_target: ChannelTarget::Foreground,
            },
            &mut cell,
        );
        assert_eq!(cell.fg, Color::rgb(255, 0, 0));
    }

    #[test]
    fn half_strength_blends_with_grey() {
        let mut cell = Cell::styled('x', Color::rgb(255, 0, 0), Color::BLACK, Modifiers::NONE);
        apply(
            FilterGreyscaleInputs {
                strength: 0.5,
                channel_target: ChannelTarget::Foreground,
            },
            &mut cell,
        );
        assert!((164..=166).contains(&cell.fg.r));
        assert!((37..=39).contains(&cell.fg.g));
    }

    #[test]
    fn channel_target_limits_application() {
        let mut cell = Cell::styled(
            'x',
            Color::rgb(255, 0, 0),
            Color::rgb(0, 255, 0),
            Modifiers::NONE,
        );
        apply(
            FilterGreyscaleInputs {
                strength: 1.0,
                channel_target: ChannelTarget::Foreground,
            },
            &mut cell,
        );
        assert_eq!(cell.fg.r, cell.fg.g);
        assert_eq!(cell.fg.g, cell.fg.b);
        assert_eq!(cell.bg, Color::rgb(0, 255, 0));
    }
}

// <FILE>crates/tui-vfx-compost/src/filters/cls_greyscale.rs</FILE> - <DESC>v3.1-native greyscale filter primitive port</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
