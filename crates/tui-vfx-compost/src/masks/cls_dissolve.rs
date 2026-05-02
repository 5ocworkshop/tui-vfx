// <FILE>crates/tui-vfx-compost/src/masks/cls_dissolve.rs</FILE> - <DESC>v3.1-native dissolve mask primitive port</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Ported from tui-vfx-compositor/src/masks/cls_dissolve.rs: preserve hardened deterministic chunk-noise visibility logic while mapping seed/chunkSize onto v3.1 primitive input specs.</WCTX>
// <CLOG>0.1.0: INIT — lift dissolve mask runtime logic into the compost masks hierarchy with v3.1 descriptor metadata.</CLOG>

use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

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

/// Runtime input bundle for `mask.dissolve`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaskDissolveInputs {
    /// Seed for deterministic randomness.
    pub seed: u64,
    /// Size of dissolve chunks; `1` means individual cells.
    pub chunk_size: u8,
}

impl Default for MaskDissolveInputs {
    fn default() -> Self {
        Self {
            seed: 0,
            chunk_size: 1,
        }
    }
}

impl MaskDissolveInputs {
    /// Create inputs while preserving the legacy minimum chunk-size hardening.
    pub const fn new(seed: u64, chunk_size: u8) -> Self {
        Self {
            seed,
            chunk_size: if chunk_size < 1 { 1 } else { chunk_size },
        }
    }
}

impl PrimitiveInputs for MaskDissolveInputs {
    fn input_specs() -> BTreeMap<EffectInputId, EffectInputSpec> {
        BTreeMap::from([
            (
                EffectInputId::new("seed"),
                EffectInputSpec {
                    display_name: Some("Seed".to_string()),
                    description: Some("Seed for deterministic dissolve noise.".to_string()),
                    value: ValueSpec {
                        kind: ValueKind::Integer,
                        default: Some(Value::Integer(0)),
                        range: Some(NumericRange {
                            min: Some(0.0),
                            max: None,
                        }),
                        allowed_values: vec![],
                        unit: None,
                        semantic: Some("random-seed".to_string()),
                    },
                    optional: false,
                    bindable: true,
                    runtime_mutability: RuntimeMutability::PhaseStart,
                },
            ),
            (
                EffectInputId::new("chunkSize"),
                EffectInputSpec {
                    display_name: Some("Chunk Size".to_string()),
                    description: Some(
                        "Size of grouped cells that dissolve together; one means per-cell noise."
                            .to_string(),
                    ),
                    value: ValueSpec {
                        kind: ValueKind::Integer,
                        default: Some(Value::Integer(1)),
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
            ),
        ])
    }
}

/// Rust-owned descriptor/runtime for the v3.1 `mask.dissolve` primitive.
#[derive(Clone, Copy, Debug, Default)]
pub struct MaskDissolve;

impl EffectPrimitive for MaskDissolve {
    type Inputs = MaskDissolveInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        EffectDescriptor {
            id: EffectId::new("mask.dissolve"),
            version: "0.1.0".to_string(),
            display_name: "Dissolve Mask".to_string(),
            category: Some("mask primitive".to_string()),
            domain: EffectDomain::Mask,
            cell_access: CellAccess {
                reads: all_cell_channels(),
                writes: all_cell_channels(),
            },
            scope_support: ScopeSupport {
                kinds: vec![ScopeKind::All, ScopeKind::Role],
                coordinate_spaces: vec![CoordinateSpace::DestinationLocal],
                role_spaces: vec![RoleSpace::Destination],
            },
            write_support: WriteSupport {
                cell_policies: vec![CellWritePolicy::WriteCell],
                role_policies: vec![RoleWritePolicyKind::PreserveDestination],
            },
            inputs: MaskDissolveInputs::input_specs(),
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

impl MaskRuntime for MaskDissolve {
    fn visibility(
        inputs: &Self::Inputs,
        context: &EffectRuntimeContext<'_>,
    ) -> Result<MaskVisibility, EffectRuntimeError> {
        let t = context.sample().phase_t;
        if t <= 0.0 {
            return Ok(MaskVisibility::HIDDEN);
        }
        if t >= 1.0 {
            return Ok(MaskVisibility::VISIBLE);
        }

        let chunk_size = u16::from(inputs.chunk_size.max(1));
        let chunk_x = context.local_x() / chunk_size;
        let chunk_y = context.local_y() / chunk_size;

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        chunk_x.hash(&mut hasher);
        chunk_y.hash(&mut hasher);
        inputs.seed.hash(&mut hasher);
        let hash = hasher.finish();

        let value = hash as f64 / u64::MAX as f64;
        if value < t {
            Ok(MaskVisibility::VISIBLE)
        } else {
            Ok(MaskVisibility::HIDDEN)
        }
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

    fn context_at(x: u16, y: u16, t: f64) -> EffectRuntimeContext<'static> {
        let sample = Box::leak(Box::new(SampleContext::new(t)));
        EffectRuntimeContext::new(sample, x, y, 10, 10)
    }

    #[test]
    fn progress_zero_is_not_visible() {
        assert_eq!(
            MaskDissolve::visibility(&MaskDissolveInputs::new(42, 1), &context_at(0, 0, 0.0))
                .expect("visibility resolves"),
            MaskVisibility::HIDDEN
        );
    }

    #[test]
    fn progress_one_is_visible() {
        assert_eq!(
            MaskDissolve::visibility(&MaskDissolveInputs::new(42, 1), &context_at(0, 0, 1.0))
                .expect("visibility resolves"),
            MaskVisibility::VISIBLE
        );
    }

    #[test]
    fn same_seed_and_position_are_deterministic() {
        let inputs = MaskDissolveInputs::new(42, 1);
        let first =
            MaskDissolve::visibility(&inputs, &context_at(3, 7, 0.5)).expect("visibility resolves");
        let second =
            MaskDissolve::visibility(&inputs, &context_at(3, 7, 0.5)).expect("visibility resolves");

        assert_eq!(first, second);
    }

    #[test]
    fn chunk_size_groups_cells() {
        let inputs = MaskDissolveInputs::new(42, 2);
        let a =
            MaskDissolve::visibility(&inputs, &context_at(0, 0, 0.5)).expect("visibility resolves");
        let b =
            MaskDissolve::visibility(&inputs, &context_at(1, 1, 0.5)).expect("visibility resolves");

        assert_eq!(a, b);
    }

    #[test]
    fn different_seeds_produce_different_patterns() {
        let mut differ = false;
        for x in 0..5 {
            for y in 0..5 {
                let first = MaskDissolve::visibility(
                    &MaskDissolveInputs::new(1, 1),
                    &context_at(x, y, 0.5),
                )
                .expect("visibility resolves");
                let second = MaskDissolve::visibility(
                    &MaskDissolveInputs::new(2, 1),
                    &context_at(x, y, 0.5),
                )
                .expect("visibility resolves");
                if first != second {
                    differ = true;
                    break;
                }
            }
        }

        assert!(differ, "Different seeds should produce different patterns");
    }
}

// <FILE>crates/tui-vfx-compost/src/masks/cls_dissolve.rs</FILE> - <DESC>v3.1-native dissolve mask primitive port</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
