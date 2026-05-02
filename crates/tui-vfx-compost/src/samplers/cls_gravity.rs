// <FILE>crates/tui-vfx-compost/src/samplers/cls_gravity.rs</FILE> - <DESC>v3.1-native gravity sampler primitive port</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Ported from tui-vfx-compositor/src/samplers/cls_gravity.rs: preserve hardened parabolic acceleration and displacement-delta behavior while mapping acceleration/terminalVelocity/axis onto v3.1 primitive input specs.</WCTX>
// <CLOG>0.1.0: INIT — lift gravity sampler runtime logic into the compost samplers hierarchy with v3.1 descriptor metadata.</CLOG>

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

/// Runtime input bundle for `sampler.gravity`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SamplerGravityInputs {
    /// Acceleration in cells per `t²` unit. Positive moves down/right; negative moves up/left.
    pub acceleration: f32,
    /// Maximum absolute displacement cap in cells.
    pub terminal_velocity: f32,
    /// Axis gravity pulls along.
    pub axis: SamplerAxis,
}

impl Default for SamplerGravityInputs {
    fn default() -> Self {
        Self::new(4.0, 10.0, SamplerAxis::Y)
    }
}

impl SamplerGravityInputs {
    /// Create inputs while preserving the legacy positive terminal-velocity hardening.
    pub fn new(acceleration: f32, terminal_velocity: f32, axis: SamplerAxis) -> Self {
        Self {
            acceleration,
            terminal_velocity: terminal_velocity.abs(),
            axis,
        }
    }
}

impl PrimitiveInputs for SamplerGravityInputs {
    fn input_specs() -> BTreeMap<EffectInputId, EffectInputSpec> {
        BTreeMap::from([
            (
                EffectInputId::new("acceleration"),
                EffectInputSpec {
                    display_name: Some("Acceleration".to_string()),
                    description: Some(
                        "Cells per t² unit; positive moves down/right, negative moves up/left."
                            .to_string(),
                    ),
                    value: ValueSpec {
                        kind: ValueKind::Number,
                        default: Some(Value::Number(4.0)),
                        range: None,
                        allowed_values: vec![],
                        unit: Some("cells/t^2".to_string()),
                        semantic: Some("acceleration".to_string()),
                    },
                    optional: false,
                    bindable: true,
                    runtime_mutability: RuntimeMutability::PhaseStart,
                },
            ),
            (
                EffectInputId::new("terminalVelocity"),
                EffectInputSpec {
                    display_name: Some("Terminal Velocity".to_string()),
                    description: Some("Maximum absolute displacement cap in cells.".to_string()),
                    value: ValueSpec {
                        kind: ValueKind::Number,
                        default: Some(Value::Number(10.0)),
                        range: Some(NumericRange {
                            min: Some(0.0),
                            max: None,
                        }),
                        allowed_values: vec![],
                        unit: Some("cells".to_string()),
                        semantic: Some("displacement-cap".to_string()),
                    },
                    optional: false,
                    bindable: true,
                    runtime_mutability: RuntimeMutability::PhaseStart,
                },
            ),
            (
                EffectInputId::new("axis"),
                EffectInputSpec {
                    display_name: Some("Axis".to_string()),
                    description: Some("Axis along which gravity applies displacement.".to_string()),
                    value: ValueSpec {
                        kind: ValueKind::Enum,
                        default: Some(Value::Enum(SamplerAxis::Y.as_str().to_string())),
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

/// Rust-owned descriptor/runtime for the v3.1 `sampler.gravity` primitive.
#[derive(Clone, Copy, Debug, Default)]
pub struct SamplerGravity;

impl EffectPrimitive for SamplerGravity {
    type Inputs = SamplerGravityInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        EffectDescriptor {
            id: EffectId::new("sampler.gravity"),
            version: "0.1.0".to_string(),
            display_name: "Gravity Sampler".to_string(),
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
            inputs: SamplerGravityInputs::input_specs(),
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

impl CoordinateSamplerRuntime for SamplerGravity {
    fn sample_coordinate(
        inputs: &Self::Inputs,
        context: &EffectRuntimeContext<'_>,
    ) -> Result<CoordinateSample, EffectRuntimeError> {
        let t = context.sample().phase_t as f32;
        let dest_x = context.local_x();
        let dest_y = context.local_y();

        let raw_displacement = 0.5 * inputs.acceleration * t * t;
        let displacement = if inputs.acceleration >= 0.0 {
            raw_displacement.min(inputs.terminal_velocity)
        } else {
            raw_displacement.max(-inputs.terminal_velocity)
        };

        match inputs.axis {
            SamplerAxis::X => {
                let src_x_f = dest_x as f32 + displacement;
                if src_x_f < 0.0 {
                    Ok(CoordinateSample::no_displacement())
                } else {
                    let src_x = src_x_f.round() as u16;
                    let delta_x = src_x as i32 - dest_x as i32;
                    Ok(CoordinateSample::displaced(src_x, dest_y, delta_x, 0))
                }
            }
            SamplerAxis::Y => {
                let src_y_f = dest_y as f32 + displacement;
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

    fn sample_at(inputs: &SamplerGravityInputs, x: u16, y: u16, t: f64) -> CoordinateSample {
        let sample = SampleContext::new(t);
        let context = EffectRuntimeContext::new(&sample, x, y, 20, 20);
        SamplerGravity::sample_coordinate(inputs, &context).expect("sample resolves")
    }

    #[test]
    fn zero_time_samples_destination() {
        let inputs = SamplerGravityInputs::new(10.0, 20.0, SamplerAxis::Y);
        assert_eq!(sample_at(&inputs, 5, 10, 0.0).source, Some((5, 10)));
    }

    #[test]
    fn positive_acceleration_increases_y() {
        let inputs = SamplerGravityInputs::new(8.0, 20.0, SamplerAxis::Y);
        let (_, y0) = sample_at(&inputs, 5, 10, 0.0).source.unwrap();
        let (_, y1) = sample_at(&inputs, 5, 10, 0.5).source.unwrap();
        let (_, y2) = sample_at(&inputs, 5, 10, 1.0).source.unwrap();
        assert!(y1 >= y0, "Should move downward over time");
        assert!(y2 >= y1, "Should accelerate");
    }

    #[test]
    fn negative_acceleration_decreases_y() {
        let inputs = SamplerGravityInputs::new(-8.0, 20.0, SamplerAxis::Y);
        let (_, y0) = sample_at(&inputs, 5, 10, 0.0).source.unwrap();
        let (_, y1) = sample_at(&inputs, 5, 10, 0.5).source.unwrap();
        assert!(y1 <= y0, "Negative accel should move upward");
    }

    #[test]
    fn terminal_velocity_caps_displacement() {
        let inputs = SamplerGravityInputs::new(100.0, 3.0, SamplerAxis::Y);
        let (_, y) = sample_at(&inputs, 5, 10, 10.0).source.unwrap();
        assert!(y <= 10 + 3, "Should be capped at terminal velocity");
    }

    #[test]
    fn x_axis_preserves_y() {
        let inputs = SamplerGravityInputs::new(4.0, 10.0, SamplerAxis::X);
        let (_, y) = sample_at(&inputs, 5, 10, 1.0).source.unwrap();
        assert_eq!(y, 10);
    }

    #[test]
    fn y_axis_preserves_x() {
        let inputs = SamplerGravityInputs::new(4.0, 10.0, SamplerAxis::Y);
        let (x, _) = sample_at(&inputs, 5, 10, 1.0).source.unwrap();
        assert_eq!(x, 5);
    }

    #[test]
    fn returns_none_when_source_negative() {
        let inputs = SamplerGravityInputs::new(-100.0, 50.0, SamplerAxis::Y);
        assert_eq!(sample_at(&inputs, 5, 2, 5.0).source, None);
    }

    #[test]
    fn sample_emits_displacement_delta() {
        let inputs = SamplerGravityInputs::new(8.0, 20.0, SamplerAxis::Y);
        let out = sample_at(&inputs, 5, 10, 1.0);
        assert!(out.source.is_some());
        assert_eq!(out.delta_x, 0);
        assert!(out.delta_y > 0);
        let (_, src_y) = out.source.unwrap();
        assert_eq!(out.delta_y, src_y as i32 - 10);
    }
}

// <FILE>crates/tui-vfx-compost/src/samplers/cls_gravity.rs</FILE> - <DESC>v3.1-native gravity sampler primitive port</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
