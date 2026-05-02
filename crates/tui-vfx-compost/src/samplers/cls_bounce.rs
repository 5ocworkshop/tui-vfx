// <FILE>crates/tui-vfx-compost/src/samplers/cls_bounce.rs</FILE> - <DESC>v3.1-native bounce sampler primitive port</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Ported from tui-vfx-compositor/src/samplers/cls_bounce.rs: preserve abs-sine vertical bounce and displacement-delta behavior while mapping amplitude/speed/phaseSpread onto v3.1 primitive inputs.</WCTX>
// <CLOG>0.1.0: INIT — lift bounce sampler runtime logic into the compost samplers hierarchy with v3.1 descriptor metadata.</CLOG>

use std::collections::BTreeMap;
use std::f32::consts::TAU;

use tui_vfx_contract::{
    CellAccess, CellChannel, CellWritePolicy, CoordinateSpace, EffectCompletion, EffectDescriptor,
    EffectDomain, EffectId, EffectInputId, EffectInputSpec, EffectLifecycle, NumericRange,
    RoleSpace, RoleWritePolicyKind, RuntimeMutability, ScopeKind, ScopeSupport, Value, ValueKind,
    ValueSpec, WriteSupport,
};

use crate::primitive::{
    CoordinateSample, CoordinateSamplerRuntime, EffectPrimitive, EffectRuntimeContext,
    EffectRuntimeError, NoOutputs, PrimitiveInputs, PrimitiveOutputs,
};

/// Runtime input bundle for `sampler.bounce`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SamplerBounceInputs {
    /// Bounce height in cells.
    pub amplitude: f32,
    /// Animation speed multiplier.
    pub speed: f32,
    /// Phase offset per column.
    pub phase_spread: f32,
}

impl Default for SamplerBounceInputs {
    fn default() -> Self {
        Self {
            amplitude: 2.0,
            speed: 4.0,
            phase_spread: 0.5,
        }
    }
}

impl PrimitiveInputs for SamplerBounceInputs {
    fn input_specs() -> BTreeMap<EffectInputId, EffectInputSpec> {
        BTreeMap::from([
            number_input("amplitude", "Amplitude", 2.0, "Bounce height in cells."),
            number_input("speed", "Speed", 4.0, "Animation speed multiplier."),
            number_input(
                "phaseSpread",
                "Phase Spread",
                0.5,
                "Phase offset between adjacent columns.",
            ),
        ])
    }
}

/// Rust-owned descriptor/runtime for the v3.1 `sampler.bounce` primitive.
#[derive(Clone, Copy, Debug, Default)]
pub struct SamplerBounce;

impl EffectPrimitive for SamplerBounce {
    type Inputs = SamplerBounceInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        EffectDescriptor {
            id: EffectId::new("sampler.bounce"),
            version: "0.1.0".to_string(),
            display_name: "Bounce Sampler".to_string(),
            category: Some("sampler primitive".to_string()),
            domain: EffectDomain::CoordinateSampler,
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
            inputs: SamplerBounceInputs::input_specs(),
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

impl CoordinateSamplerRuntime for SamplerBounce {
    fn sample_coordinate(
        inputs: &Self::Inputs,
        context: &EffectRuntimeContext<'_>,
    ) -> Result<CoordinateSample, EffectRuntimeError> {
        let t = context.sample().phase_t as f32;
        let dest_x = context.local_x();
        let dest_y = context.local_y();
        let phase = t * inputs.speed * TAU + (dest_x as f32 * inputs.phase_spread);
        let bounce = inputs.amplitude * phase.sin().abs();
        let src_y_f = dest_y as f32 + bounce;

        if src_y_f < 0.0 {
            Ok(CoordinateSample::no_displacement())
        } else {
            let src_y = src_y_f.round() as u16;
            let delta_y = src_y as i32 - dest_y as i32;
            Ok(CoordinateSample::displaced(dest_x, src_y, 0, delta_y))
        }
    }
}

fn number_input(
    id: &str,
    display_name: &str,
    default: f64,
    description: &str,
) -> (EffectInputId, EffectInputSpec) {
    (
        EffectInputId::new(id),
        EffectInputSpec {
            display_name: Some(display_name.to_string()),
            description: Some(description.to_string()),
            value: ValueSpec {
                kind: ValueKind::Number,
                default: Some(Value::Number(default)),
                range: Some(NumericRange {
                    min: Some(0.0),
                    max: None,
                }),
                allowed_values: vec![],
                unit: None,
                semantic: None,
            },
            optional: false,
            bindable: true,
            runtime_mutability: RuntimeMutability::PhaseStart,
        },
    )
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

    fn sample_at(inputs: &SamplerBounceInputs, x: u16, y: u16, t: f64) -> CoordinateSample {
        let sample = SampleContext::new(t);
        let context = EffectRuntimeContext::new(&sample, x, y, 20, 20);
        SamplerBounce::sample_coordinate(inputs, &context).expect("sample resolves")
    }

    #[test]
    fn zero_amplitude_is_identity() {
        let inputs = SamplerBounceInputs {
            amplitude: 0.0,
            speed: 4.0,
            phase_spread: 0.5,
        };
        assert_eq!(sample_at(&inputs, 5, 7, 0.0).source, Some((5, 7)));
        assert_eq!(sample_at(&inputs, 5, 7, 0.5).source, Some((5, 7)));
        assert_eq!(sample_at(&inputs, 5, 7, 1.0).source, Some((5, 7)));
    }

    #[test]
    fn preserves_x_coordinate() {
        let inputs = SamplerBounceInputs::default();
        for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let (x, _) = sample_at(&inputs, 5, 10, t).source.unwrap();
            assert_eq!(x, 5);
        }
    }

    #[test]
    fn y_offset_is_positive() {
        let inputs = SamplerBounceInputs {
            amplitude: 2.0,
            speed: 4.0,
            phase_spread: 0.0,
        };
        for t in [0.0, 0.1, 0.2, 0.3, 0.4, 0.5] {
            let (_, y) = sample_at(&inputs, 0, 10, t).source.unwrap();
            assert!(y >= 10, "Source Y {y} should be >= dest Y 10 at t={t}");
        }
    }

    #[test]
    fn amplitude_affects_max_displacement() {
        let small = SamplerBounceInputs {
            amplitude: 1.0,
            speed: 4.0,
            phase_spread: 0.0,
        };
        let large = SamplerBounceInputs {
            amplitude: 4.0,
            speed: 4.0,
            phase_spread: 0.0,
        };
        let mut max_small = 0u16;
        let mut max_large = 0u16;
        for i in 0..100 {
            let t = i as f64 / 100.0;
            if let Some((_, y)) = sample_at(&small, 0, 10, t).source {
                max_small = max_small.max(y.saturating_sub(10));
            }
            if let Some((_, y)) = sample_at(&large, 0, 10, t).source {
                max_large = max_large.max(y.saturating_sub(10));
            }
        }
        assert!(max_large > max_small);
    }

    #[test]
    fn sample_emits_displacement_delta() {
        let inputs = SamplerBounceInputs {
            amplitude: 2.0,
            speed: 4.0,
            phase_spread: 0.0,
        };
        let out = sample_at(&inputs, 5, 10, 0.0625);
        assert_eq!(out.delta_x, 0);
        assert_eq!(out.delta_y, 2);
    }
}

// <FILE>crates/tui-vfx-compost/src/samplers/cls_bounce.rs</FILE> - <DESC>v3.1-native bounce sampler primitive port</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
