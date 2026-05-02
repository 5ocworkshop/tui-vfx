// <FILE>crates/tui-vfx-compost/src/samplers/cls_ripple.rs</FILE> - <DESC>v3.1-native ripple sampler primitive port</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Ported from tui-vfx-compositor/src/samplers/cls_ripple.rs: preserve center selection, radial sine displacement, negative-coordinate cropping, rounded source coordinates, and displacement-delta behavior.</WCTX>
// <CLOG>0.1.0: INIT — lift ripple sampler runtime logic into the compost samplers hierarchy with v3.1 descriptor metadata.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{
    CellAccess, CellChannel, CellWritePolicy, CoordinateSpace, EffectCompletion, EffectDescriptor,
    EffectDomain, EffectId, EffectInputId, EffectInputSpec, EffectLifecycle, NumericRange,
    RoleSpace, RoleWritePolicyKind, RuntimeMutability, ScopeKind, ScopeSupport, StructuredValue,
    Value, ValueKind, ValueSpec, WriteSupport,
};

use crate::primitive::{
    CoordinateSample, CoordinateSamplerRuntime, EffectPrimitive, EffectRuntimeContext,
    EffectRuntimeError, NoOutputs, PrimitiveInputs, PrimitiveOutputs,
};

/// Center point for `sampler.ripple` radial displacement.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RippleCenter {
    /// Ripple from the center of the destination region.
    #[default]
    Center,
    /// Ripple from a specific destination-local cell coordinate.
    Cell { x: u16, y: u16 },
}

impl RippleCenter {
    pub(crate) fn as_structured_value(self) -> StructuredValue {
        match self {
            Self::Center => StructuredValue::Object(BTreeMap::from([(
                "kind".to_string(),
                StructuredValue::String("center".to_string()),
            )])),
            Self::Cell { x, y } => StructuredValue::Object(BTreeMap::from([
                (
                    "kind".to_string(),
                    StructuredValue::String("cell".to_string()),
                ),
                ("x".to_string(), StructuredValue::Number(f64::from(x))),
                ("y".to_string(), StructuredValue::Number(f64::from(y))),
            ])),
        }
    }

    pub(crate) fn coordinate(self, width: u16, height: u16) -> (f32, f32) {
        match self {
            Self::Center => (width as f32 / 2.0, height as f32 / 2.0),
            Self::Cell { x, y } => (x as f32, y as f32),
        }
    }
}

/// Runtime input bundle for `sampler.ripple`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SamplerRippleInputs {
    /// Wave amplitude in cells.
    pub amplitude: f32,
    /// Distance between ripple peaks in cells.
    pub wavelength: f32,
    /// Speed of ripple propagation.
    pub speed: f32,
    /// Center point of the ripple.
    pub center: RippleCenter,
}

impl Default for SamplerRippleInputs {
    fn default() -> Self {
        Self {
            amplitude: 1.5,
            wavelength: 4.0,
            speed: 2.0,
            center: RippleCenter::Center,
        }
    }
}

impl PrimitiveInputs for SamplerRippleInputs {
    fn input_specs() -> BTreeMap<EffectInputId, EffectInputSpec> {
        BTreeMap::from([
            number_input("amplitude", "Amplitude", 1.5, "Wave amplitude in cells."),
            number_input(
                "wavelength",
                "Wavelength",
                4.0,
                "Distance between ripple peaks in cells.",
            ),
            number_input("speed", "Speed", 2.0, "Speed of ripple propagation."),
            (
                EffectInputId::new("center"),
                EffectInputSpec {
                    display_name: Some("Center".to_string()),
                    description: Some(
                        "Ripple origin, either { kind: center } or { kind: cell, x, y }."
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
        ])
    }
}

/// Rust-owned descriptor/runtime for the v3.1 `sampler.ripple` primitive.
#[derive(Clone, Copy, Debug, Default)]
pub struct SamplerRipple;

impl EffectPrimitive for SamplerRipple {
    type Inputs = SamplerRippleInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        EffectDescriptor {
            id: EffectId::new("sampler.ripple"),
            version: "0.1.0".to_string(),
            display_name: "Ripple Sampler".to_string(),
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
            inputs: SamplerRippleInputs::input_specs(),
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

impl CoordinateSamplerRuntime for SamplerRipple {
    fn sample_coordinate(
        inputs: &Self::Inputs,
        context: &EffectRuntimeContext<'_>,
    ) -> Result<CoordinateSample, EffectRuntimeError> {
        let dest_x = context.local_x();
        let dest_y = context.local_y();
        let t = context.sample().phase_t as f32;
        let (center_x, center_y) = inputs.center.coordinate(context.width(), context.height());

        let dx = dest_x as f32 - center_x;
        let dy = dest_y as f32 - center_y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < 0.001 {
            return Ok(CoordinateSample::passthrough(dest_x, dest_y));
        }

        let phase = dist / inputs.wavelength - t * inputs.speed;
        let displacement = inputs.amplitude * phase.sin();
        let nx = dx / dist;
        let ny = dy / dist;
        let src_x_f = dest_x as f32 + nx * displacement;
        let src_y_f = dest_y as f32 + ny * displacement;

        if src_x_f < 0.0 || src_y_f < 0.0 {
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

    fn sample_at(inputs: &SamplerRippleInputs, x: u16, y: u16, t: f64) -> CoordinateSample {
        let sample = SampleContext::new(t);
        let context = EffectRuntimeContext::new(&sample, x, y, 20, 20);
        SamplerRipple::sample_coordinate(inputs, &context).expect("sample resolves")
    }

    #[test]
    fn default_matches_legacy_constructor() {
        let inputs = SamplerRippleInputs::default();
        assert_eq!(inputs.amplitude, 1.5);
        assert_eq!(inputs.wavelength, 4.0);
        assert_eq!(inputs.speed, 2.0);
        assert_eq!(inputs.center, RippleCenter::Center);
    }

    #[test]
    fn center_cell_is_passthrough() {
        let out = sample_at(&SamplerRippleInputs::default(), 10, 10, 0.0);
        assert_eq!(out, CoordinateSample::passthrough(10, 10));
    }

    #[test]
    fn off_center_cell_displaces_radially() {
        let out = sample_at(&SamplerRippleInputs::default(), 15, 10, 0.0);
        assert!(out.source.is_some());
        assert_eq!(out.delta_y, 0);
        let (src_x, src_y) = out.source.unwrap();
        assert_eq!(src_y, 10);
        assert_eq!(out.delta_x, src_x as i32 - 15);
        assert_ne!(out.delta_x, 0);
    }

    #[test]
    fn point_center_overrides_region_center() {
        let inputs = SamplerRippleInputs {
            center: RippleCenter::Cell { x: 3, y: 4 },
            ..SamplerRippleInputs::default()
        };
        let out = sample_at(&inputs, 3, 4, 0.5);
        assert_eq!(out, CoordinateSample::passthrough(3, 4));
    }

    #[test]
    fn negative_source_coordinate_returns_none() {
        let inputs = SamplerRippleInputs {
            amplitude: 10.0,
            wavelength: 1.0,
            speed: 0.0,
            center: RippleCenter::Cell { x: 10, y: 0 },
        };
        let out = sample_at(&inputs, 1, 0, 0.0);
        assert_eq!(out, CoordinateSample::no_displacement());
    }

    #[test]
    fn sample_emits_displacement_delta() {
        let out = sample_at(&SamplerRippleInputs::default(), 15, 10, 0.0);
        let (src_x, src_y) = out.source.expect("ripple samples source");
        assert_eq!(out.delta_x, src_x as i32 - 15);
        assert_eq!(out.delta_y, src_y as i32 - 10);
    }
}

// <FILE>crates/tui-vfx-compost/src/samplers/cls_ripple.rs</FILE> - <DESC>v3.1-native ripple sampler primitive port</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
