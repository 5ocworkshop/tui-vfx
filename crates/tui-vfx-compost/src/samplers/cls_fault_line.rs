// <FILE>crates/tui-vfx-compost/src/samplers/cls_fault_line.rs</FILE> - <DESC>v3.1-native fault-line sampler primitive port</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Ported from tui-vfx-compositor/src/samplers/cls_fault_line.rs: preserve deterministic split, split-bias clamp, fixed lower-half offset, right-edge crop, and displacement-delta behavior.</WCTX>
// <CLOG>0.1.0: INIT — lift fault-line sampler runtime logic into the compost samplers hierarchy with v3.1 descriptor metadata.</CLOG>

use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

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

/// Runtime input bundle for `sampler.faultLine`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SamplerFaultLineInputs {
    /// Seed for deterministic split position variation.
    pub seed: u64,
    /// Displacement intensity multiplier.
    pub intensity: f32,
    /// Bias for split position (-1.0 to 1.0).
    pub split_bias: f32,
    /// Optional fixed lower-half horizontal offset.
    pub fixed_offset: Option<i16>,
}

impl Default for SamplerFaultLineInputs {
    fn default() -> Self {
        Self::new(42, 1.0, 0.0)
    }
}

impl SamplerFaultLineInputs {
    /// Create dynamic fault-line inputs with legacy split-bias hardening.
    pub fn new(seed: u64, intensity: f32, split_bias: f32) -> Self {
        Self {
            seed,
            intensity,
            split_bias: split_bias.clamp(-1.0, 1.0),
            fixed_offset: None,
        }
    }

    /// Use fixed lower-half horizontal offset instead of dynamic split motion.
    pub const fn with_fixed_offset(mut self, offset: i16) -> Self {
        self.fixed_offset = Some(offset);
        self
    }

    fn split_y(&self, height: u16) -> u16 {
        if height < 3 {
            return height / 2;
        }
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.seed.hash(&mut hasher);
        let hash = hasher.finish();
        let base_split = (hash % height as u64) as f32;
        let biased = base_split + (self.split_bias * height as f32 * 0.3);
        biased.clamp(1.0, (height - 1) as f32) as u16
    }
}

impl PrimitiveInputs for SamplerFaultLineInputs {
    fn input_specs() -> BTreeMap<EffectInputId, EffectInputSpec> {
        BTreeMap::from([
            integer_input(
                "seed",
                "Seed",
                Some(42),
                "Seed for deterministic split position variation.",
                Some("random-seed"),
                false,
            ),
            number_input(
                "intensity",
                "Intensity",
                1.0,
                "Displacement intensity multiplier.",
                Some(0.0),
                None,
            ),
            number_input(
                "splitBias",
                "Split Bias",
                0.0,
                "Bias for split position (-1.0 to 1.0).",
                Some(-1.0),
                Some(1.0),
            ),
            integer_input(
                "fixedOffset",
                "Fixed Offset",
                None,
                "Optional fixed lower-half horizontal offset.",
                Some("coordinate-offset"),
                true,
            ),
        ])
    }
}

/// Rust-owned descriptor/runtime for the v3.1 `sampler.faultLine` primitive.
#[derive(Clone, Copy, Debug, Default)]
pub struct SamplerFaultLine;

impl EffectPrimitive for SamplerFaultLine {
    type Inputs = SamplerFaultLineInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        EffectDescriptor {
            id: EffectId::new("sampler.faultLine"),
            version: "0.1.0".to_string(),
            display_name: "Fault Line Sampler".to_string(),
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
            inputs: SamplerFaultLineInputs::input_specs(),
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

impl CoordinateSamplerRuntime for SamplerFaultLine {
    fn sample_coordinate(
        inputs: &Self::Inputs,
        context: &EffectRuntimeContext<'_>,
    ) -> Result<CoordinateSample, EffectRuntimeError> {
        let dest_x = context.local_x();
        let dest_y = context.local_y();
        let height = context.height();

        if let Some(offset) = inputs.fixed_offset {
            if dest_y < height / 2 {
                return Ok(CoordinateSample::passthrough(dest_x, dest_y));
            }
            let src_x_i = dest_x as i32 - offset as i32;
            if src_x_i < 0 || src_x_i >= context.width() as i32 {
                return Ok(CoordinateSample::no_displacement());
            }
            let src_x = src_x_i as u16;
            return Ok(CoordinateSample::displaced(
                src_x,
                dest_y,
                src_x as i32 - dest_x as i32,
                0,
            ));
        }

        let t = context.sample().phase_t as f32;
        let split_y = inputs.split_y(height);
        let offset = ((1.0 - t) * 20.0 * inputs.intensity).round() as i32;
        let src_x_i = if dest_y < split_y {
            dest_x as i32 - offset
        } else {
            dest_x as i32 + offset
        };

        if src_x_i < 0 || src_x_i >= context.width() as i32 {
            Ok(CoordinateSample::no_displacement())
        } else {
            let src_x = src_x_i as u16;
            Ok(CoordinateSample::displaced(
                src_x,
                dest_y,
                src_x as i32 - dest_x as i32,
                0,
            ))
        }
    }
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
    default: Option<i64>,
    description: &str,
    semantic: Option<&str>,
    optional: bool,
) -> (EffectInputId, EffectInputSpec) {
    (
        EffectInputId::new(id),
        EffectInputSpec {
            display_name: Some(display_name.to_string()),
            description: Some(description.to_string()),
            value: ValueSpec {
                kind: ValueKind::Integer,
                default: default.map(Value::Integer),
                range: None,
                allowed_values: vec![],
                unit: None,
                semantic: semantic.map(str::to_string),
            },
            optional,
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

    fn sample_at(
        inputs: &SamplerFaultLineInputs,
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        t: f64,
    ) -> CoordinateSample {
        let sample = SampleContext::new(t);
        let context = EffectRuntimeContext::new(&sample, x, y, w, h);
        SamplerFaultLine::sample_coordinate(inputs, &context).expect("sample resolves")
    }

    #[test]
    fn small_height_does_not_panic() {
        let inputs = SamplerFaultLineInputs::new(1, 1.0, 0.0);
        let _ = sample_at(&inputs, 5, 0, 10, 1, 0.5);
        let _ = sample_at(&inputs, 5, 0, 10, 2, 0.5);
    }

    #[test]
    fn identity_at_t1() {
        let inputs = SamplerFaultLineInputs::new(1, 1.0, 0.0);
        assert_eq!(sample_at(&inputs, 5, 0, 10, 10, 1.0).source, Some((5, 0)));
    }

    #[test]
    fn displacement_at_t0() {
        let inputs = SamplerFaultLineInputs::new(1, 1.0, 0.0);
        let result = sample_at(&inputs, 50, 0, 100, 10, 0.0);
        assert!(result.source.is_some());
        assert_ne!(result.source.unwrap().0, 50);
    }

    #[test]
    fn negative_x_returns_none() {
        let inputs = SamplerFaultLineInputs::new(1, 1.0, 0.0);
        assert_eq!(sample_at(&inputs, 5, 0, 100, 10, 0.0).source, None);
    }

    #[test]
    fn fixed_offset_crops_right_edge() {
        let inputs = SamplerFaultLineInputs::new(1, 1.0, 0.0).with_fixed_offset(-4);
        assert_eq!(sample_at(&inputs, 9, 9, 10, 10, 0.5).source, None);
    }

    #[test]
    fn sample_emits_displacement_delta() {
        let inputs = SamplerFaultLineInputs::new(1, 1.0, 0.0);
        let out = sample_at(&inputs, 50, 0, 100, 10, 0.5);
        assert!(out.source.is_some());
        assert_eq!(out.delta_y, 0);
        let (src_x, _) = out.source.unwrap();
        assert_eq!(out.delta_x, src_x as i32 - 50);
        assert_ne!(out.delta_x, 0);
    }
}

// <FILE>crates/tui-vfx-compost/src/samplers/cls_fault_line.rs</FILE> - <DESC>v3.1-native fault-line sampler primitive port</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
