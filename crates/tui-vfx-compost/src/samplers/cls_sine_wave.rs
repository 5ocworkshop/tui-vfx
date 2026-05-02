// <FILE>crates/tui-vfx-compost/src/samplers/cls_sine_wave.rs</FILE> - <DESC>v3.1-native sine-wave sampler primitive port</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Ported from tui-vfx-compositor/src/samplers/cls_sine_wave.rs: preserve bidirectional sine displacement, axis routing, phase offset, and displacement-delta behavior while mapping fields onto v3.1 primitive inputs.</WCTX>
// <CLOG>0.1.0: INIT — lift sine-wave sampler runtime logic into the compost samplers hierarchy with v3.1 descriptor metadata.</CLOG>

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
use crate::samplers::SamplerAxis;

/// Runtime input bundle for `sampler.sineWave`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SamplerSineWaveInputs {
    /// Wave amplitude in cells.
    pub amplitude: f32,
    /// Spatial frequency in waves per cell.
    pub spatial_freq: f32,
    /// Temporal speed multiplier.
    pub speed: f32,
    /// Axis the wave displacement affects.
    pub axis: SamplerAxis,
    /// Phase offset in radians.
    pub phase_offset: f32,
}

impl Default for SamplerSineWaveInputs {
    fn default() -> Self {
        Self {
            amplitude: 2.0,
            spatial_freq: 0.5,
            speed: 10.0,
            axis: SamplerAxis::X,
            phase_offset: 0.0,
        }
    }
}

impl PrimitiveInputs for SamplerSineWaveInputs {
    fn input_specs() -> BTreeMap<EffectInputId, EffectInputSpec> {
        BTreeMap::from([
            number_input("amplitude", "Amplitude", 2.0, "Wave amplitude in cells."),
            number_input(
                "spatialFreq",
                "Spatial Frequency",
                0.5,
                "Spatial frequency in waves per cell.",
            ),
            number_input("speed", "Speed", 10.0, "Temporal animation speed."),
            number_input(
                "phaseOffset",
                "Phase Offset",
                0.0,
                "Phase offset in radians.",
            ),
            (
                EffectInputId::new("axis"),
                EffectInputSpec {
                    display_name: Some("Axis".to_string()),
                    description: Some("Axis the wave displacement affects.".to_string()),
                    value: ValueSpec {
                        kind: ValueKind::Enum,
                        default: Some(Value::Enum(SamplerAxis::X.as_str().to_string())),
                        range: None,
                        allowed_values: SamplerAxis::allowed_values(),
                        unit: None,
                        semantic: Some("axis".to_string()),
                    },
                    optional: false,
                    bindable: true,
                    runtime_mutability: RuntimeMutability::PhaseStart,
                },
            ),
        ])
    }
}

/// Rust-owned descriptor/runtime for the v3.1 `sampler.sineWave` primitive.
#[derive(Clone, Copy, Debug, Default)]
pub struct SamplerSineWave;

impl EffectPrimitive for SamplerSineWave {
    type Inputs = SamplerSineWaveInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        EffectDescriptor {
            id: EffectId::new("sampler.sineWave"),
            version: "0.1.0".to_string(),
            display_name: "Sine Wave Sampler".to_string(),
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
            inputs: SamplerSineWaveInputs::input_specs(),
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

impl CoordinateSamplerRuntime for SamplerSineWave {
    fn sample_coordinate(
        inputs: &Self::Inputs,
        context: &EffectRuntimeContext<'_>,
    ) -> Result<CoordinateSample, EffectRuntimeError> {
        let t = context.sample().phase_t as f32;
        let dest_x = context.local_x();
        let dest_y = context.local_y();
        match inputs.axis {
            SamplerAxis::X => {
                let phase =
                    dest_y as f32 * inputs.spatial_freq + t * inputs.speed + inputs.phase_offset;
                let offset = inputs.amplitude * phase.sin();
                let src_x_f = (dest_x as f32 + offset).round();
                if src_x_f < 0.0 {
                    Ok(CoordinateSample::no_displacement())
                } else {
                    let src_x = src_x_f as u16;
                    let delta_x = src_x as i32 - dest_x as i32;
                    Ok(CoordinateSample::displaced(src_x, dest_y, delta_x, 0))
                }
            }
            SamplerAxis::Y => {
                let phase =
                    dest_x as f32 * inputs.spatial_freq + t * inputs.speed + inputs.phase_offset;
                let offset = inputs.amplitude * phase.sin();
                let src_y_f = (dest_y as f32 + offset).round();
                if src_y_f < 0.0 {
                    Ok(CoordinateSample::no_displacement())
                } else {
                    let src_y = src_y_f as u16;
                    let delta_y = src_y as i32 - dest_y as i32;
                    Ok(CoordinateSample::displaced(dest_x, src_y, 0, delta_y))
                }
            }
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

    fn sample_at(inputs: &SamplerSineWaveInputs, x: u16, y: u16, t: f64) -> CoordinateSample {
        let sample = SampleContext::new(t);
        let context = EffectRuntimeContext::new(&sample, x, y, 20, 20);
        SamplerSineWave::sample_coordinate(inputs, &context).expect("sample resolves")
    }

    #[test]
    fn zero_amplitude_is_identity() {
        let inputs = SamplerSineWaveInputs {
            amplitude: 0.0,
            spatial_freq: 1.0,
            speed: 1.0,
            axis: SamplerAxis::X,
            phase_offset: 0.0,
        };
        assert_eq!(sample_at(&inputs, 5, 7, 0.0).source, Some((5, 7)));
        assert_eq!(sample_at(&inputs, 5, 7, 0.5).source, Some((5, 7)));
        assert_eq!(sample_at(&inputs, 5, 7, 1.0).source, Some((5, 7)));
    }

    #[test]
    fn axis_x_displaces_x_and_preserves_y() {
        let inputs = SamplerSineWaveInputs::default();
        let (_, y) = sample_at(&inputs, 5, 5, 0.0).source.unwrap();
        assert_eq!(y, 5);
    }

    #[test]
    fn axis_y_displaces_y_and_preserves_x() {
        let inputs = SamplerSineWaveInputs {
            axis: SamplerAxis::Y,
            ..SamplerSineWaveInputs::default()
        };
        let (x, _) = sample_at(&inputs, 5, 5, 0.0).source.unwrap();
        assert_eq!(x, 5);
    }

    #[test]
    fn edge_positions_do_not_panic() {
        let inputs = SamplerSineWaveInputs::default();
        let result = sample_at(&inputs, 0, 5, 0.0);
        if let Some((_, y)) = result.source {
            assert_eq!(y, 5);
        }
    }

    #[test]
    fn sample_emits_displacement_delta() {
        let inputs = SamplerSineWaveInputs::default();
        let out = sample_at(&inputs, 10, 5, 0.0);
        assert!(out.source.is_some());
        assert_eq!(out.delta_y, 0);
        let (src_x, _) = out.source.unwrap();
        assert_eq!(out.delta_x, src_x as i32 - 10);
        assert_ne!(out.delta_x, 0);
    }
}

// <FILE>crates/tui-vfx-compost/src/samplers/cls_sine_wave.rs</FILE> - <DESC>v3.1-native sine-wave sampler primitive port</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
