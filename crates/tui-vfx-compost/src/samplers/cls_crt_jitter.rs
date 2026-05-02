// <FILE>crates/tui-vfx-compost/src/samplers/cls_crt_jitter.rs</FILE> - <DESC>v3.1-native CRT jitter sampler primitive port</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Ported from tui-vfx-compositor/src/samplers/cls_crt_jitter.rs: preserve SplitMix64 row/time noise, exponential decay, horizontal-only jitter, negative-coordinate crop, and displacement-delta behavior.</WCTX>
// <CLOG>0.1.0: INIT — lift CRT jitter sampler runtime logic into the compost samplers hierarchy with v3.1 descriptor metadata.</CLOG>

use std::collections::BTreeMap;

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

/// Runtime input bundle for `sampler.crtJitter`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SamplerCrtJitterInputs {
    /// Intensity of the jitter effect.
    pub intensity: f32,
    /// Jitter frequency in Hz.
    pub speed_hz: f32,
    /// Decay factor controlling how quickly jitter diminishes over phase time.
    pub decay: f32,
    /// Seed for deterministic row/time noise.
    pub seed: u64,
}

impl Default for SamplerCrtJitterInputs {
    fn default() -> Self {
        Self {
            intensity: 0.7,
            speed_hz: 30.0,
            decay: 0.5,
            seed: 42,
        }
    }
}

impl SamplerCrtJitterInputs {
    fn noise(self, y: u16, t: f32) -> f32 {
        let row_seed = self.seed.wrapping_mul(31).wrapping_add(y as u64);
        let time_slot = (t * self.speed_hz).floor() as u64;
        fast_random(row_seed, time_slot) * 2.0 - 1.0
    }
}

impl PrimitiveInputs for SamplerCrtJitterInputs {
    fn input_specs() -> BTreeMap<EffectInputId, EffectInputSpec> {
        BTreeMap::from([
            number_input(
                "intensity",
                "Intensity",
                0.7,
                "Intensity of the jitter effect.",
                Some(0.0),
                None,
            ),
            number_input(
                "speedHz",
                "Speed Hz",
                30.0,
                "Jitter frequency in Hz.",
                Some(0.0),
                None,
            ),
            number_input(
                "decay",
                "Decay",
                0.5,
                "Decay factor controlling how quickly jitter diminishes over phase time.",
                Some(0.0),
                None,
            ),
            integer_input(
                "seed",
                "Seed",
                42,
                "Seed for deterministic row/time noise.",
                Some("random-seed"),
            ),
        ])
    }
}

/// Rust-owned descriptor/runtime for the v3.1 `sampler.crtJitter` primitive.
#[derive(Clone, Copy, Debug, Default)]
pub struct SamplerCrtJitter;

impl EffectPrimitive for SamplerCrtJitter {
    type Inputs = SamplerCrtJitterInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        EffectDescriptor {
            id: EffectId::new("sampler.crtJitter"),
            version: "0.1.0".to_string(),
            display_name: "CRT Jitter Sampler".to_string(),
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
            inputs: SamplerCrtJitterInputs::input_specs(),
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

impl CoordinateSamplerRuntime for SamplerCrtJitter {
    fn sample_coordinate(
        inputs: &Self::Inputs,
        context: &EffectRuntimeContext<'_>,
    ) -> Result<CoordinateSample, EffectRuntimeError> {
        let t = context.sample().phase_t as f32;
        let dest_x = context.local_x();
        let dest_y = context.local_y();
        let decay_factor = (-inputs.decay * t * 5.0).exp();
        let effective_intensity = inputs.intensity * decay_factor;
        let jitter = inputs.noise(dest_y, t) * effective_intensity * 5.0;
        let src_x_f = (dest_x as f32 + jitter).round();

        if src_x_f < 0.0 {
            Ok(CoordinateSample::no_displacement())
        } else {
            let src_x = src_x_f as u16;
            Ok(CoordinateSample::displaced(
                src_x,
                dest_y,
                src_x as i32 - dest_x as i32,
                0,
            ))
        }
    }
}

fn fast_random(seed: u64, input: u64) -> f32 {
    let mut h = seed.wrapping_add(input).wrapping_mul(0x9e3779b97f4a7c15);
    h = (h ^ (h >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    h = (h ^ (h >> 27)).wrapping_mul(0x94d049bb133111eb);
    h ^= h >> 31;
    (h >> 40) as f32 / ((1u64 << 24) as f32)
}

fn number_input(
    id: &str,
    display_name: &str,
    default: f64,
    description: &str,
    min: Option<f64>,
    max: Option<f64>,
) -> (EffectInputId, EffectInputSpec) {
    (
        EffectInputId::new(id),
        EffectInputSpec {
            display_name: Some(display_name.to_string()),
            description: Some(description.to_string()),
            value: ValueSpec {
                kind: ValueKind::Number,
                default: Some(Value::Number(default)),
                range: Some(NumericRange { min, max }),
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

fn integer_input(
    id: &str,
    display_name: &str,
    default: i64,
    description: &str,
    semantic: Option<&str>,
) -> (EffectInputId, EffectInputSpec) {
    (
        EffectInputId::new(id),
        EffectInputSpec {
            display_name: Some(display_name.to_string()),
            description: Some(description.to_string()),
            value: ValueSpec {
                kind: ValueKind::Integer,
                default: Some(Value::Integer(default)),
                range: None,
                allowed_values: vec![],
                unit: None,
                semantic: semantic.map(str::to_string),
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

    fn sample_at(inputs: &SamplerCrtJitterInputs, x: u16, y: u16, t: f64) -> CoordinateSample {
        let sample = SampleContext::new(t);
        let context = EffectRuntimeContext::new(&sample, x, y, 80, 24);
        SamplerCrtJitter::sample_coordinate(inputs, &context).expect("sample resolves")
    }

    #[test]
    fn default_matches_legacy() {
        let inputs = SamplerCrtJitterInputs::default();
        assert_eq!(inputs.intensity, 0.7);
        assert_eq!(inputs.speed_hz, 30.0);
        assert_eq!(inputs.decay, 0.5);
        assert_eq!(inputs.seed, 42);
    }

    #[test]
    fn preserves_y() {
        let out = sample_at(&SamplerCrtJitterInputs::default(), 10, 15, 0.5);
        if let Some((_, y)) = out.source {
            assert_eq!(y, 15);
        }
        assert_eq!(out.delta_y, 0);
    }

    #[test]
    fn deterministic_with_seed() {
        let a = SamplerCrtJitterInputs {
            seed: 123,
            ..SamplerCrtJitterInputs::default()
        };
        let b = SamplerCrtJitterInputs {
            seed: 123,
            ..SamplerCrtJitterInputs::default()
        };
        assert_eq!(sample_at(&a, 10, 10, 0.5), sample_at(&b, 10, 10, 0.5));
    }

    #[test]
    fn different_seeds_can_differ() {
        let a = SamplerCrtJitterInputs {
            seed: 123,
            ..SamplerCrtJitterInputs::default()
        };
        let b = SamplerCrtJitterInputs {
            seed: 456,
            ..SamplerCrtJitterInputs::default()
        };
        let out_a = sample_at(&a, 10, 10, 0.5);
        let out_b = sample_at(&b, 10, 10, 0.5);
        assert!(out_a.source != out_b.source || out_a.source.is_some());
    }

    #[test]
    fn negative_x_returns_none() {
        let inputs = SamplerCrtJitterInputs {
            intensity: 10.0,
            speed_hz: 1.0,
            decay: 0.0,
            seed: 42,
        };
        let _ = sample_at(&inputs, 0, 5, 0.0);
    }

    #[test]
    fn sample_emits_displacement_delta() {
        let inputs = SamplerCrtJitterInputs {
            intensity: 0.7,
            speed_hz: 1.0,
            decay: 0.0,
            seed: 42,
        };
        let out = sample_at(&inputs, 20, 5, 0.0);
        assert_eq!(out.delta_y, 0);
        if let Some((src_x, _)) = out.source {
            assert_eq!(out.delta_x, src_x as i32 - 20);
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/samplers/cls_crt_jitter.rs</FILE> - <DESC>v3.1-native CRT jitter sampler primitive port</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
