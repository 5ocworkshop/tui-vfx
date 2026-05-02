// <FILE>crates/tui-vfx-compost/src/samplers/cls_shredder.rs</FILE> - <DESC>v3.1-native shredder sampler primitive port</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Ported from tui-vfx-compositor/src/samplers/cls_shredder.rs: preserve stripe-width hardening, alternating strip speeds, accelerated falling gaps, fixed chunk offset mode, and displacement-delta behavior.</WCTX>
// <CLOG>0.1.0: INIT — lift shredder sampler runtime logic into the compost samplers hierarchy with v3.1 descriptor metadata.</CLOG>

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

/// Runtime input bundle for `sampler.shredder`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SamplerShredderInputs {
    /// Width of each vertical strip in cells.
    pub stripe_width: u16,
    /// Speed multiplier for odd-indexed strips.
    pub odd_speed: f32,
    /// Speed multiplier for even-indexed strips.
    pub even_speed: f32,
    /// Optional fixed horizontal row/chunk offset.
    pub fixed_offset: Option<i16>,
}

impl Default for SamplerShredderInputs {
    fn default() -> Self {
        Self::new(2, 3.0, 1.0)
    }
}

impl SamplerShredderInputs {
    /// Create dynamic shredder inputs with legacy stripe-width hardening.
    pub const fn new(stripe_width: u16, odd_speed: f32, even_speed: f32) -> Self {
        Self {
            stripe_width: if stripe_width == 0 { 1 } else { stripe_width },
            odd_speed,
            even_speed,
            fixed_offset: None,
        }
    }

    /// Use fixed horizontal row/chunk offset instead of falling-strip motion.
    pub const fn with_fixed_offset(mut self, offset: i16) -> Self {
        self.fixed_offset = Some(offset);
        self
    }
}

impl PrimitiveInputs for SamplerShredderInputs {
    fn input_specs() -> BTreeMap<EffectInputId, EffectInputSpec> {
        BTreeMap::from([
            integer_input(
                "stripeWidth",
                "Stripe Width",
                Some(2),
                "Width of each vertical strip in cells.",
                Some(1.0),
                None,
                None,
                false,
            ),
            number_input(
                "oddSpeed",
                "Odd Speed",
                3.0,
                "Speed multiplier for odd-indexed strips.",
            ),
            number_input(
                "evenSpeed",
                "Even Speed",
                1.0,
                "Speed multiplier for even-indexed strips.",
            ),
            integer_input(
                "fixedOffset",
                "Fixed Offset",
                None,
                "Optional fixed horizontal row/chunk offset.",
                None,
                None,
                Some("coordinate-offset"),
                true,
            ),
        ])
    }
}

/// Rust-owned descriptor/runtime for the v3.1 `sampler.shredder` primitive.
#[derive(Clone, Copy, Debug, Default)]
pub struct SamplerShredder;

impl EffectPrimitive for SamplerShredder {
    type Inputs = SamplerShredderInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        EffectDescriptor {
            id: EffectId::new("sampler.shredder"),
            version: "0.1.0".to_string(),
            display_name: "Shredder Sampler".to_string(),
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
            inputs: SamplerShredderInputs::input_specs(),
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

impl CoordinateSamplerRuntime for SamplerShredder {
    fn sample_coordinate(
        inputs: &Self::Inputs,
        context: &EffectRuntimeContext<'_>,
    ) -> Result<CoordinateSample, EffectRuntimeError> {
        let dest_x = context.local_x();
        let dest_y = context.local_y();
        let stripe_width = inputs.stripe_width.max(1);

        if let Some(offset) = inputs.fixed_offset {
            if !dest_y.is_multiple_of(2) {
                return Ok(CoordinateSample::passthrough(dest_x, dest_y));
            }
            let strip_idx = dest_x / stripe_width;
            let local_offset = if strip_idx.is_multiple_of(2) {
                offset
            } else {
                -offset
            } as i32;
            let src_x_i = dest_x as i32 - local_offset;
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
        let height = context.height();
        let strip_idx = dest_x / stripe_width;
        let base_speed = if strip_idx.is_multiple_of(2) {
            inputs.even_speed
        } else {
            inputs.odd_speed
        };
        let variation = 1.0 + ((strip_idx as u32 * 17) % 7) as f32 * 0.1;
        let speed = base_speed * variation;
        let t_accel = t.powf(1.2);
        let max_fall = height as f32 * 0.5;
        let fall_offset = speed * t_accel * max_fall;
        let src_y_f = dest_y as f32 - fall_offset;

        if src_y_f < 0.0 {
            Ok(CoordinateSample::no_displacement())
        } else {
            let src_y = src_y_f as u16;
            Ok(CoordinateSample::displaced(
                dest_x,
                src_y,
                0,
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
                range: None,
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
    min: Option<f64>,
    max: Option<f64>,
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
                range: Some(NumericRange { min, max }),
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
        inputs: &SamplerShredderInputs,
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        t: f64,
    ) -> CoordinateSample {
        let sample = SampleContext::new(t);
        let context = EffectRuntimeContext::new(&sample, x, y, w, h);
        SamplerShredder::sample_coordinate(inputs, &context).expect("sample resolves")
    }

    #[test]
    fn zero_time_samples_destination() {
        let inputs = SamplerShredderInputs::default();
        assert_eq!(sample_at(&inputs, 0, 0, 20, 10, 0.0).source, Some((0, 0)));
        assert_eq!(sample_at(&inputs, 5, 5, 20, 10, 0.0).source, Some((5, 5)));
    }

    #[test]
    fn mid_time_creates_strip_offsets() {
        let inputs = SamplerShredderInputs::default();
        let even_result = sample_at(&inputs, 0, 9, 20, 10, 0.5);
        let odd_result = sample_at(&inputs, 2, 9, 20, 10, 0.5);
        assert!(even_result.source.is_some());
        assert!(odd_result.source.is_some());
        let (_, even_src_y) = even_result.source.unwrap();
        let (_, odd_src_y) = odd_result.source.unwrap();
        assert!(odd_src_y <= even_src_y);
    }

    #[test]
    fn late_time_creates_gaps() {
        let inputs = SamplerShredderInputs::default();
        assert_eq!(
            sample_at(&inputs, 0, 0, 20, 10, 1.0),
            CoordinateSample::no_displacement()
        );
    }

    #[test]
    fn stripe_width_is_hardened_to_one() {
        let inputs = SamplerShredderInputs::new(0, 3.0, 1.0);
        assert_eq!(inputs.stripe_width, 1);
        assert!(sample_at(&inputs, 0, 5, 20, 10, 0.3).source.is_some());
    }

    #[test]
    fn stripe_width_affects_strip_assignment() {
        let inputs = SamplerShredderInputs::new(4, 2.0, 1.0);
        let x0 = sample_at(&inputs, 0, 5, 20, 10, 0.3);
        let x3 = sample_at(&inputs, 3, 5, 20, 10, 0.3);
        let x4 = sample_at(&inputs, 4, 5, 20, 10, 0.3);
        let x7 = sample_at(&inputs, 7, 5, 20, 10, 0.3);
        assert_eq!(x0.source.unwrap().1, x3.source.unwrap().1);
        assert_eq!(x4.source.unwrap().1, x7.source.unwrap().1);
    }

    #[test]
    fn fixed_offset_applies_to_even_rows_only() {
        let inputs = SamplerShredderInputs::default().with_fixed_offset(2);
        assert_eq!(
            sample_at(&inputs, 5, 3, 20, 10, 0.5),
            CoordinateSample::passthrough(5, 3)
        );
        assert_eq!(sample_at(&inputs, 5, 4, 20, 10, 0.5).source, Some((3, 4)));
    }

    #[test]
    fn fixed_offset_crops_out_of_bounds() {
        let inputs = SamplerShredderInputs::default().with_fixed_offset(2);
        assert_eq!(
            sample_at(&inputs, 0, 2, 20, 10, 0.5),
            CoordinateSample::no_displacement()
        );
    }

    #[test]
    fn sample_emits_displacement_delta() {
        let inputs = SamplerShredderInputs::default();
        let out = sample_at(&inputs, 2, 9, 20, 10, 0.5);
        assert!(out.source.is_some());
        assert_eq!(out.delta_x, 0);
        let (_, src_y) = out.source.unwrap();
        assert_eq!(out.delta_y, src_y as i32 - 9);
        assert!(out.delta_y < 0);
    }
}

// <FILE>crates/tui-vfx-compost/src/samplers/cls_shredder.rs</FILE> - <DESC>v3.1-native shredder sampler primitive port</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
