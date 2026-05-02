// <FILE>crates/tui-vfx-compost/src/masks/cls_checkers.rs</FILE> - <DESC>v3.1-native checkers mask primitive port</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Ported from tui-vfx-compositor/src/masks/cls_checkers.rs: preserve checkerboard parity reveal thresholds while mapping cell_size onto v3.1 cellSize.</WCTX>
// <CLOG>0.1.0: INIT — lift checkers mask runtime logic into the compost masks hierarchy with v3.1 descriptor metadata.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{
    CellAccess, CellChannel, CellWritePolicy, CoordinateSpace, EffectCompletion, EffectDescriptor,
    EffectDomain, EffectId, EffectInputId, EffectInputSpec, EffectLifecycle, NumericRange,
    RoleSpace, RoleWritePolicyKind, RuntimeMutability, ScopeKind, ScopeSupport, Value, ValueKind,
    ValueSpec, WriteSupport,
};

use crate::primitive::{
    EffectPrimitive, EffectRuntimeContext, EffectRuntimeError, MaskRuntime, MaskVisibility,
    NoOutputs, PrimitiveInputs, PrimitiveOutputs,
};

/// Runtime input bundle for `mask.checkers`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaskCheckersInputs {
    /// Size of each checker cell in terminal cells.
    pub cell_size: u16,
}

impl Default for MaskCheckersInputs {
    fn default() -> Self {
        Self::new(2)
    }
}

impl MaskCheckersInputs {
    /// Create inputs while preserving the legacy minimum cell-size hardening.
    pub const fn new(cell_size: u16) -> Self {
        Self {
            cell_size: if cell_size < 1 { 1 } else { cell_size },
        }
    }
}

impl PrimitiveInputs for MaskCheckersInputs {
    fn input_specs() -> BTreeMap<EffectInputId, EffectInputSpec> {
        BTreeMap::from([(
            EffectInputId::new("cellSize"),
            EffectInputSpec {
                display_name: Some("Cell Size".to_string()),
                description: Some("Size of each checker block in terminal cells.".to_string()),
                value: ValueSpec {
                    kind: ValueKind::Integer,
                    default: Some(Value::Integer(2)),
                    range: Some(NumericRange {
                        min: Some(1.0),
                        max: None,
                    }),
                    allowed_values: vec![],
                    unit: Some("cells".to_string()),
                    semantic: Some("cell-cluster-size".to_string()),
                },
                optional: false,
                bindable: true,
                runtime_mutability: RuntimeMutability::PhaseStart,
            },
        )])
    }
}

/// Rust-owned descriptor/runtime for the v3.1 `mask.checkers` primitive.
#[derive(Clone, Copy, Debug, Default)]
pub struct MaskCheckers;

impl EffectPrimitive for MaskCheckers {
    type Inputs = MaskCheckersInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        EffectDescriptor {
            id: EffectId::new("mask.checkers"),
            version: "0.1.0".to_string(),
            display_name: "Checkers Mask".to_string(),
            category: Some("mask primitive".to_string()),
            domain: EffectDomain::Mask,
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
            inputs: MaskCheckersInputs::input_specs(),
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

impl MaskRuntime for MaskCheckers {
    fn visibility(
        inputs: &Self::Inputs,
        context: &EffectRuntimeContext<'_>,
    ) -> Result<MaskVisibility, EffectRuntimeError> {
        let cell_size = inputs.cell_size.max(1);
        let block_x = context.local_x() / cell_size;
        let block_y = context.local_y() / cell_size;
        let is_even = (block_x + block_y).is_multiple_of(2);
        let visible = if is_even {
            context.sample().phase_t > 0.25
        } else {
            context.sample().phase_t > 0.75
        };
        Ok(if visible {
            MaskVisibility::VISIBLE
        } else {
            MaskVisibility::HIDDEN
        })
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

    fn visible_at(inputs: &MaskCheckersInputs, x: u16, y: u16, t: f64) -> bool {
        let sample = SampleContext::new(t);
        let context = EffectRuntimeContext::new(&sample, x, y, 10, 10);
        MaskCheckers::visibility(inputs, &context).expect("visibility resolves")
            == MaskVisibility::VISIBLE
    }

    #[test]
    fn alternating_pattern_matches_legacy_thresholds() {
        let inputs = MaskCheckersInputs::default();
        assert!(!visible_at(&inputs, 0, 0, 0.2));
        assert!(visible_at(&inputs, 0, 0, 0.3));
        assert!(!visible_at(&inputs, 2, 0, 0.7));
        assert!(visible_at(&inputs, 2, 0, 0.8));
        assert!(!visible_at(&inputs, 0, 2, 0.7));
        assert!(visible_at(&inputs, 2, 2, 0.3));
    }

    #[test]
    fn custom_cell_size_groups_blocks() {
        let inputs = MaskCheckersInputs::new(4);
        assert!(visible_at(&inputs, 0, 0, 0.3));
        assert!(!visible_at(&inputs, 4, 0, 0.7));
        assert!(visible_at(&inputs, 4, 0, 0.8));
    }

    #[test]
    fn zero_cell_size_is_hardened_to_one() {
        assert_eq!(MaskCheckersInputs::new(0).cell_size, 1);
    }
}

// <FILE>crates/tui-vfx-compost/src/masks/cls_checkers.rs</FILE> - <DESC>v3.1-native checkers mask primitive port</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
