// <FILE>crates/tui-vfx-compost/src/filters/cls_invert.rs</FILE> - <DESC>v3.1-native invert filter primitive port</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Ported from tui-vfx-compositor/src/filters/cls_invert.rs: preserve foreground/background swap and transparent fallback behavior while mapping apply_to onto v3.1 channelTarget.</WCTX>
// <CLOG>0.1.0: INIT — lift invert filter runtime logic into the compost filters hierarchy with v3.1 descriptor metadata.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{
    CellAccess, CellChannel, CellWritePolicy, CoordinateSpace, EffectCompletion, EffectDescriptor,
    EffectDomain, EffectId, EffectInputId, EffectInputSpec, EffectLifecycle, RoleSpace,
    RoleWritePolicyKind, RuntimeMutability, ScopeKind, ScopeSupport, Value, ValueKind, ValueSpec,
    WriteSupport,
};
use tui_vfx_types::Color;

use crate::filters::ChannelTarget;
use crate::primitive::{
    CellView, EffectPrimitive, EffectRuntimeContext, EffectRuntimeError, FrameFilterRuntime,
    NoOutputs, PrimitiveInputs, PrimitiveOutputs,
};

/// Runtime input bundle for `filter.invert`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FilterInvertInputs {
    /// Foreground/background channel target.
    pub channel_target: ChannelTarget,
}

impl Default for FilterInvertInputs {
    fn default() -> Self {
        Self {
            channel_target: ChannelTarget::Both,
        }
    }
}

impl PrimitiveInputs for FilterInvertInputs {
    fn input_specs() -> BTreeMap<EffectInputId, EffectInputSpec> {
        BTreeMap::from([(
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
        )])
    }
}

/// Rust-owned descriptor/runtime for the v3.1 `filter.invert` primitive.
#[derive(Clone, Copy, Debug, Default)]
pub struct FilterInvert;

impl EffectPrimitive for FilterInvert {
    type Inputs = FilterInvertInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        EffectDescriptor {
            id: EffectId::new("filter.invert"),
            version: "0.1.0".to_string(),
            display_name: "Invert Filter".to_string(),
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
            inputs: FilterInvertInputs::input_specs(),
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

impl FrameFilterRuntime for FilterInvert {
    fn filter_cell(
        inputs: &Self::Inputs,
        cell: &mut CellView<'_, Self>,
        _context: &EffectRuntimeContext<'_>,
    ) -> Result<(), EffectRuntimeError> {
        let old_fg = cell.foreground();
        let old_bg = cell.background();

        match inputs.channel_target {
            ChannelTarget::Foreground => {
                cell.set_foreground(if old_bg == Color::TRANSPARENT {
                    Color::BLACK
                } else {
                    old_bg
                });
            }
            ChannelTarget::Background => {
                cell.set_background(if old_fg == Color::TRANSPARENT {
                    Color::WHITE
                } else {
                    old_fg
                });
            }
            ChannelTarget::Both => {
                cell.set_foreground(if old_bg == Color::TRANSPARENT {
                    Color::BLACK
                } else {
                    old_bg
                });
                cell.set_background(if old_fg == Color::TRANSPARENT {
                    Color::WHITE
                } else {
                    old_fg
                });
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SampleContext;
    use tui_vfx_types::{Cell, Modifiers};

    fn apply(inputs: FilterInvertInputs, cell: &mut Cell) {
        let sample = SampleContext::default();
        let context = EffectRuntimeContext::new(&sample, 0, 0, 10, 10);
        let mut view = CellView::<FilterInvert>::new(cell);
        FilterInvert::filter_cell(&inputs, &mut view, &context).expect("filter applies");
    }

    #[test]
    fn foreground_uses_black_on_transparent_background() {
        let mut cell = Cell::styled(
            'x',
            Color::rgb(1, 2, 3),
            Color::TRANSPARENT,
            Modifiers::NONE,
        );
        apply(
            FilterInvertInputs {
                channel_target: ChannelTarget::Foreground,
            },
            &mut cell,
        );
        assert_eq!(cell.fg, Color::BLACK);
        assert_eq!(cell.bg, Color::TRANSPARENT);
    }

    #[test]
    fn foreground_uses_background_color() {
        let mut cell = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(50, 60, 70),
            Modifiers::NONE,
        );
        apply(
            FilterInvertInputs {
                channel_target: ChannelTarget::Foreground,
            },
            &mut cell,
        );
        assert_eq!(cell.fg, Color::rgb(50, 60, 70));
        assert_eq!(cell.bg, Color::rgb(50, 60, 70));
    }

    #[test]
    fn background_uses_white_on_transparent_foreground() {
        let mut cell = Cell::styled(
            'x',
            Color::TRANSPARENT,
            Color::rgb(1, 2, 3),
            Modifiers::NONE,
        );
        apply(
            FilterInvertInputs {
                channel_target: ChannelTarget::Background,
            },
            &mut cell,
        );
        assert_eq!(cell.fg, Color::TRANSPARENT);
        assert_eq!(cell.bg, Color::WHITE);
    }

    #[test]
    fn both_swaps_colors() {
        let mut cell = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(50, 50, 50),
            Modifiers::NONE,
        );
        apply(FilterInvertInputs::default(), &mut cell);
        assert_eq!(cell.fg, Color::rgb(50, 50, 50));
        assert_eq!(cell.bg, Color::rgb(100, 100, 100));
    }

    #[test]
    fn both_handles_transparent() {
        let mut cell = Cell::styled('x', Color::TRANSPARENT, Color::TRANSPARENT, Modifiers::NONE);
        apply(FilterInvertInputs::default(), &mut cell);
        assert_eq!(cell.fg, Color::BLACK);
        assert_eq!(cell.bg, Color::WHITE);
    }
}

// <FILE>crates/tui-vfx-compost/src/filters/cls_invert.rs</FILE> - <DESC>v3.1-native invert filter primitive port</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
