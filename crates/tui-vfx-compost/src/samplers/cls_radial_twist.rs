// <FILE>crates/tui-vfx-compost/src/samplers/cls_radial_twist.rs</FILE> - <DESC>v3.1-native radial-twist sampler primitive port</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Ported from tui-vfx-compositor/src/samplers/cls_radial_twist.rs and mixed-signals radial_twist_warp: preserve center-weighted twist, radius-floor hardening, bounds cropping, and displacement-delta behavior.</WCTX>
// <CLOG>0.1.0: INIT — lift radial-twist sampler runtime logic into the compost samplers hierarchy with v3.1 descriptor metadata.</CLOG>

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
use crate::samplers::RippleCenter;

/// Runtime input bundle for `sampler.radialTwist`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SamplerRadialTwistInputs {
    /// Twist strength in radians over a full phase.
    pub twist: f32,
    /// Center point for the twist field.
    pub center: RippleCenter,
    /// Minimum normalized radius used to keep center rotation finite.
    pub radius_floor: f32,
}

impl Default for SamplerRadialTwistInputs {
    fn default() -> Self {
        Self::new(1.0, RippleCenter::Center, 0.1)
    }
}

impl SamplerRadialTwistInputs {
    /// Create radial twist inputs with legacy radius-floor hardening.
    pub fn new(twist: f32, center: RippleCenter, radius_floor: f32) -> Self {
        Self {
            twist,
            center,
            radius_floor: finite_or(radius_floor, 0.1).abs().max(0.0001),
        }
    }
}

impl PrimitiveInputs for SamplerRadialTwistInputs {
    fn input_specs() -> BTreeMap<EffectInputId, EffectInputSpec> {
        BTreeMap::from([
            number_input(
                "twist",
                "Twist",
                1.0,
                "Twist strength in radians over a full phase.",
                None,
            ),
            (
                EffectInputId::new("center"),
                EffectInputSpec {
                    display_name: Some("Center".to_string()),
                    description: Some(
                        "Twist origin, either { kind: center } or { kind: cell, x, y }."
                            .to_string(),
                    ),
                    value: ValueSpec {
                        kind: ValueKind::Structured,
                        default: Some(Value::Structured(
                            RippleCenter::Center.as_structured_value(),
                        )),
                        range: None,
                        allowed_values: vec![],
                        unit: None,
                        semantic: Some("ripple-center".to_string()),
                    },
                    optional: false,
                    bindable: true,
                    runtime_mutability: RuntimeMutability::PhaseStart,
                },
            ),
            number_input(
                "radiusFloor",
                "Radius Floor",
                0.1,
                "Minimum normalized radius used to keep center rotation finite.",
                Some(0.0001),
            ),
        ])
    }
}

/// Rust-owned descriptor/runtime for the v3.1 `sampler.radialTwist` primitive.
#[derive(Clone, Copy, Debug, Default)]
pub struct SamplerRadialTwist;

impl EffectPrimitive for SamplerRadialTwist {
    type Inputs = SamplerRadialTwistInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        EffectDescriptor {
            id: EffectId::new("sampler.radialTwist"),
            version: "0.1.0".to_string(),
            display_name: "Radial Twist Sampler".to_string(),
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
            inputs: SamplerRadialTwistInputs::input_specs(),
            outputs: NoOutputs::output_specs(),
            lifecycle: EffectLifecycle {
                completion: EffectCompletion::Instant,
                resettable: true,
                seekable: true,
                deterministic_with_seed: false,
            },
        }
    }
}

impl CoordinateSamplerRuntime for SamplerRadialTwist {
    fn sample_coordinate(
        inputs: &Self::Inputs,
        context: &EffectRuntimeContext<'_>,
    ) -> Result<CoordinateSample, EffectRuntimeError> {
        let dest_x = context.local_x();
        let dest_y = context.local_y();
        let width = context.width();
        let height = context.height();
        let t = context.sample().phase_t;

        if width == 0 || height == 0 {
            return Ok(CoordinateSample::no_displacement());
        }

        let (center_x, center_y) = inputs.center.coordinate(width, height);
        let scale = width.max(height).max(1) as f32 / 2.0;
        let norm_x = (dest_x as f32 - center_x) / scale;
        let norm_y = (dest_y as f32 - center_y) / scale;
        let twist = inputs.twist * t as f32;
        let (warped_x, warped_y) =
            radial_twist_warp(norm_x, norm_y, 0.0, 0.0, twist, inputs.radius_floor);
        let src_x_f = center_x + warped_x * scale;
        let src_y_f = center_y + warped_y * scale;

        if src_x_f < 0.0 || src_y_f < 0.0 || src_x_f >= width as f32 || src_y_f >= height as f32 {
            Ok(CoordinateSample::no_displacement())
        } else {
            let src_x = src_x_f.round() as u16;
            let src_y = src_y_f.round() as u16;
            Ok(CoordinateSample::displaced(
                src_x,
                src_y,
                src_x as i32 - dest_x as i32,
                src_y as i32 - dest_y as i32,
            ))
        }
    }
}

fn radial_twist_warp(
    x: f32,
    y: f32,
    center_x: f32,
    center_y: f32,
    twist: f32,
    radius_floor: f32,
) -> (f32, f32) {
    let x = finite_or(x, 0.0);
    let y = finite_or(y, 0.0);
    let center_x = finite_or(center_x, 0.0);
    let center_y = finite_or(center_y, 0.0);
    let twist = finite_or(twist, 0.0);
    let radius_floor = finite_or(radius_floor, 0.1).abs().max(0.0001);

    let dx = x - center_x;
    let dy = y - center_y;
    let radius = (dx * dx + dy * dy).sqrt().max(radius_floor);
    let angle = twist / radius;
    let sin = angle.sin();
    let cos = angle.cos();

    (
        center_x + dx * cos - dy * sin,
        center_y + dx * sin + dy * cos,
    )
}

fn finite_or(value: f32, fallback: f32) -> f32 {
    if value.is_finite() { value } else { fallback }
}

fn number_input(
    id: &str,
    display_name: &str,
    default: f64,
    description: &str,
    min: Option<f64>,
) -> (EffectInputId, EffectInputSpec) {
    (
        EffectInputId::new(id),
        EffectInputSpec {
            display_name: Some(display_name.to_string()),
            description: Some(description.to_string()),
            value: ValueSpec {
                kind: ValueKind::Number,
                default: Some(Value::Number(default)),
                range: Some(NumericRange { min, max: None }),
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

    fn sample_at(inputs: &SamplerRadialTwistInputs, x: u16, y: u16, t: f64) -> CoordinateSample {
        let sample = SampleContext::new(t);
        let context = EffectRuntimeContext::new(&sample, x, y, 20, 10);
        SamplerRadialTwist::sample_coordinate(inputs, &context).expect("sample resolves")
    }

    #[test]
    fn zero_twist_is_identity() {
        let inputs = SamplerRadialTwistInputs::new(0.0, RippleCenter::Center, 0.1);
        assert_eq!(sample_at(&inputs, 3, 4, 1.0).source, Some((3, 4)));
    }

    #[test]
    fn twist_remaps_off_center_cells() {
        let inputs = SamplerRadialTwistInputs::default();
        let result = sample_at(&inputs, 15, 5, 1.0);
        assert!(matches!(result.source, Some((_, y)) if y != 5));
    }

    #[test]
    fn center_cell_remains_finite() {
        let inputs = SamplerRadialTwistInputs::new(8.0, RippleCenter::Center, 0.1);
        assert_eq!(sample_at(&inputs, 10, 5, 1.0).source, Some((10, 5)));
    }

    #[test]
    fn point_center_remains_finite() {
        let inputs = SamplerRadialTwistInputs::new(8.0, RippleCenter::Cell { x: 3, y: 4 }, 0.1);
        assert_eq!(sample_at(&inputs, 3, 4, 1.0).source, Some((3, 4)));
    }

    #[test]
    fn radius_floor_is_hardened() {
        let inputs = SamplerRadialTwistInputs::new(1.0, RippleCenter::Center, 0.0);
        assert_eq!(inputs.radius_floor, 0.0001);
    }

    #[test]
    fn sample_emits_displacement_delta() {
        let out = sample_at(&SamplerRadialTwistInputs::default(), 15, 5, 1.0);
        let (src_x, src_y) = out.source.expect("twist samples source");
        assert_eq!(out.delta_x, src_x as i32 - 15);
        assert_eq!(out.delta_y, src_y as i32 - 5);
        assert!(out.delta_x != 0 || out.delta_y != 0);
    }
}

// <FILE>crates/tui-vfx-compost/src/samplers/cls_radial_twist.rs</FILE> - <DESC>v3.1-native radial-twist sampler primitive port</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
