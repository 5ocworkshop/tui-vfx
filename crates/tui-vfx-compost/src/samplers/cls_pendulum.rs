// <FILE>crates/tui-vfx-compost/src/samplers/cls_pendulum.rs</FILE> - <DESC>v3.1-native pendulum sampler primitive port</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Ported from tui-vfx-compositor/src/samplers/cls_pendulum.rs: preserve bidirectional sine displacement and per-axis delta behavior while mapping amplitude/speed/phaseSpread/axis onto v3.1 primitive inputs.</WCTX>
// <CLOG>0.1.0: INIT — lift pendulum sampler runtime logic into the compost samplers hierarchy with v3.1 descriptor metadata.</CLOG>

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
use crate::samplers::SamplerAxis;

/// Runtime input bundle for `sampler.pendulum`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SamplerPendulumInputs {
    /// Swing amplitude in cells.
    pub amplitude: f32,
    /// Animation speed multiplier.
    pub speed: f32,
    /// Phase offset per position.
    pub phase_spread: f32,
    /// Axis the pendulum swings along.
    pub axis: SamplerAxis,
}

impl Default for SamplerPendulumInputs {
    fn default() -> Self {
        Self {
            amplitude: 2.0,
            speed: 2.0,
            phase_spread: 0.3,
            axis: SamplerAxis::X,
        }
    }
}

impl PrimitiveInputs for SamplerPendulumInputs {
    fn input_specs() -> BTreeMap<EffectInputId, EffectInputSpec> {
        BTreeMap::from([
            number_input("amplitude", "Amplitude", 2.0, "Swing amplitude in cells."),
            number_input("speed", "Speed", 2.0, "Animation speed multiplier."),
            number_input(
                "phaseSpread",
                "Phase Spread",
                0.3,
                "Phase offset between adjacent positions.",
            ),
            (
                EffectInputId::new("axis"),
                EffectInputSpec {
                    display_name: Some("Axis".to_string()),
                    description: Some("Axis along which the pendulum swings.".to_string()),
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

/// Rust-owned descriptor/runtime for the v3.1 `sampler.pendulum` primitive.
#[derive(Clone, Copy, Debug, Default)]
pub struct SamplerPendulum;

impl EffectPrimitive for SamplerPendulum {
    type Inputs = SamplerPendulumInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        EffectDescriptor {
            id: EffectId::new("sampler.pendulum"),
            version: "0.1.0".to_string(),
            display_name: "Pendulum Sampler".to_string(),
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
            inputs: SamplerPendulumInputs::input_specs(),
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

impl CoordinateSamplerRuntime for SamplerPendulum {
    fn sample_coordinate(
        inputs: &Self::Inputs,
        context: &EffectRuntimeContext<'_>,
    ) -> Result<CoordinateSample, EffectRuntimeError> {
        let t = context.sample().phase_t as f32;
        let dest_x = context.local_x();
        let dest_y = context.local_y();

        match inputs.axis {
            SamplerAxis::X => {
                let phase = t * inputs.speed * TAU + (dest_y as f32 * inputs.phase_spread);
                let offset = inputs.amplitude * phase.sin();
                let src_x_f = dest_x as f32 + offset;
                if src_x_f < 0.0 {
                    Ok(CoordinateSample::no_displacement())
                } else {
                    let src_x = src_x_f.round() as u16;
                    let delta_x = src_x as i32 - dest_x as i32;
                    Ok(CoordinateSample::displaced(src_x, dest_y, delta_x, 0))
                }
            }
            SamplerAxis::Y => {
                let phase = t * inputs.speed * TAU + (dest_x as f32 * inputs.phase_spread);
                let offset = inputs.amplitude * phase.sin();
                let src_y_f = dest_y as f32 + offset;
                if src_y_f < 0.0 {
                    Ok(CoordinateSample::no_displacement())
                } else {
                    let src_y = src_y_f.round() as u16;
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

    fn sample_at(inputs: &SamplerPendulumInputs, x: u16, y: u16, t: f64) -> CoordinateSample {
        let sample = SampleContext::new(t);
        let context = EffectRuntimeContext::new(&sample, x, y, 20, 20);
        SamplerPendulum::sample_coordinate(inputs, &context).expect("sample resolves")
    }

    #[test]
    fn zero_amplitude_is_identity() {
        let inputs = SamplerPendulumInputs {
            amplitude: 0.0,
            speed: 2.0,
            phase_spread: 0.3,
            axis: SamplerAxis::X,
        };
        assert_eq!(sample_at(&inputs, 5, 7, 0.0).source, Some((5, 7)));
        assert_eq!(sample_at(&inputs, 5, 7, 0.5).source, Some((5, 7)));
        assert_eq!(sample_at(&inputs, 5, 7, 1.0).source, Some((5, 7)));
    }

    #[test]
    fn axis_x_preserves_y() {
        let inputs = SamplerPendulumInputs::default();
        for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let (_, y) = sample_at(&inputs, 5, 10, t).source.unwrap();
            assert_eq!(y, 10);
        }
    }

    #[test]
    fn axis_y_preserves_x() {
        let inputs = SamplerPendulumInputs {
            axis: SamplerAxis::Y,
            ..SamplerPendulumInputs::default()
        };
        for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let (x, _) = sample_at(&inputs, 5, 10, t).source.unwrap();
            assert_eq!(x, 5);
        }
    }

    #[test]
    fn swings_bidirectionally() {
        let inputs = SamplerPendulumInputs {
            amplitude: 2.0,
            speed: 1.0,
            phase_spread: 0.0,
            axis: SamplerAxis::X,
        };
        let (x_center, _) = sample_at(&inputs, 10, 5, 0.0).source.unwrap();
        let (x_right, _) = sample_at(&inputs, 10, 5, 0.25).source.unwrap();
        let (x_left, _) = sample_at(&inputs, 10, 5, 0.75).source.unwrap();
        assert!(x_right >= x_center);
        assert!(x_left <= x_center);
    }

    #[test]
    fn sample_emits_displacement_delta() {
        let inputs = SamplerPendulumInputs {
            amplitude: 2.0,
            speed: 1.0,
            phase_spread: 0.0,
            axis: SamplerAxis::X,
        };
        let out = sample_at(&inputs, 10, 5, 0.25);
        assert_eq!(out.delta_y, 0);
        assert_ne!(out.delta_x, 0);
        let (src_x, _) = out.source.unwrap();
        assert_eq!(out.delta_x, src_x as i32 - 10);
    }
}

// <FILE>crates/tui-vfx-compost/src/samplers/cls_pendulum.rs</FILE> - <DESC>v3.1-native pendulum sampler primitive port</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
