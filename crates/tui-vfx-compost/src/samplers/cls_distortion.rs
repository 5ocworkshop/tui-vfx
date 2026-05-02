// <FILE>crates/tui-vfx-compost/src/samplers/cls_distortion.rs</FILE> - <DESC>v3.1-native distortion sampler primitive port</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Ported from tui-vfx-compositor/src/samplers/cls_distortion.rs: preserve fixed sine horizontal distortion and displacement-delta behavior as a parameterless v3.1 coordinate sampler.</WCTX>
// <CLOG>0.1.0: INIT — lift generic distortion sampler runtime logic into the compost samplers hierarchy with v3.1 descriptor metadata.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{
    CellAccess, CellChannel, CellWritePolicy, CoordinateSpace, EffectCompletion, EffectDescriptor,
    EffectDomain, EffectId, EffectInputId, EffectInputSpec, EffectLifecycle, RoleSpace,
    RoleWritePolicyKind, ScopeKind, ScopeSupport, WriteSupport,
};

use crate::primitive::{
    CoordinateSample, CoordinateSamplerRuntime, EffectPrimitive, EffectRuntimeContext,
    EffectRuntimeError, NoOutputs, PrimitiveInputs, PrimitiveOutputs,
};

/// Runtime input bundle for `sampler.distortion`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SamplerDistortionInputs;

impl PrimitiveInputs for SamplerDistortionInputs {
    fn input_specs() -> BTreeMap<EffectInputId, EffectInputSpec> {
        BTreeMap::new()
    }
}

/// Rust-owned descriptor/runtime for the v3.1 `sampler.distortion` primitive.
#[derive(Clone, Copy, Debug, Default)]
pub struct SamplerDistortion;

impl EffectPrimitive for SamplerDistortion {
    type Inputs = SamplerDistortionInputs;
    type Outputs = NoOutputs;

    fn descriptor() -> EffectDescriptor {
        EffectDescriptor {
            id: EffectId::new("sampler.distortion"),
            version: "0.1.0".to_string(),
            display_name: "Distortion Sampler".to_string(),
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
            inputs: SamplerDistortionInputs::input_specs(),
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

impl CoordinateSamplerRuntime for SamplerDistortion {
    fn sample_coordinate(
        _inputs: &Self::Inputs,
        context: &EffectRuntimeContext<'_>,
    ) -> Result<CoordinateSample, EffectRuntimeError> {
        let t = context.sample().phase_t as f32;
        let dest_x = context.local_x();
        let dest_y = context.local_y();
        let offset = (t * 10.0 + (dest_y as f32 / 5.0)).sin() * 2.0;
        let src_x_f = dest_x as f32 + offset;

        if src_x_f < 0.0 {
            return Ok(CoordinateSample::no_displacement());
        }

        let src_x = src_x_f.round() as u16;
        let delta_x = src_x as i32 - dest_x as i32;
        Ok(CoordinateSample::displaced(src_x, dest_y, delta_x, 0))
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

    fn sample_at(x: u16, y: u16, t: f64) -> CoordinateSample {
        let sample = SampleContext::new(t);
        let context = EffectRuntimeContext::new(&sample, x, y, 16, 16);
        SamplerDistortion::sample_coordinate(&SamplerDistortionInputs, &context)
            .expect("sample resolves")
    }

    #[test]
    fn row_zero_identity_at_t0() {
        let out = sample_at(5, 0, 0.0);
        assert_eq!(out.source, Some((5, 0)));
        assert_eq!(out.delta_x, 0);
        assert_eq!(out.delta_y, 0);
    }

    #[test]
    fn preserves_y() {
        for y in 0..5 {
            let out = sample_at(5, y, 0.5);
            assert!(out.source.is_some());
            assert_eq!(out.source.unwrap().1, y);
            assert_eq!(out.delta_y, 0);
        }
    }

    #[test]
    fn negative_x_case_does_not_panic() {
        let _ = sample_at(0, 0, std::f64::consts::PI * 0.15);
    }

    #[test]
    fn sample_emits_displacement_delta() {
        let out = sample_at(5, 10, 1.0);
        assert!(
            out.source.is_some(),
            "expected a source coord at this input"
        );
        assert_ne!(out.delta_x, 0, "distortion at y=10, t=1.0 must displace x");
        assert_eq!(out.delta_y, 0, "distortion never displaces y");
        let (source_x, _) = out.source.unwrap();
        assert_eq!(out.delta_x, source_x as i32 - 5);
    }
}

// <FILE>crates/tui-vfx-compost/src/samplers/cls_distortion.rs</FILE> - <DESC>v3.1-native distortion sampler primitive port</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
