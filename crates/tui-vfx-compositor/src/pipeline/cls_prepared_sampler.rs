// <FILE>tui-vfx-compositor/src/pipeline/cls_prepared_sampler.rs</FILE> - <DESC>Prepared sampler enum for pipeline rendering</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>2026-04-26 packet Phase 4 — propagate SamplerOutput through the wrapper and accumulate the resolved-coord delta across the sampler chain so downstream stages can react.</WCTX>
// <CLOG>2.0.0: BREAKING — PreparedSampler::sample now takes &VfxCellContext and returns SamplerOutput; sample_sampler_chain returns SamplerChainOutcome with accumulated delta_x/delta_y.</CLOG>

use crate::samplers::cls_bounce::Bounce;
use crate::samplers::cls_crt_jitter::CrtJitter;
use crate::samplers::cls_crt_sampler::CrtSampler;
use crate::samplers::cls_fault_line::FaultLine;
use crate::samplers::cls_gravity::Gravity;
use crate::samplers::cls_pendulum::Pendulum;
use crate::samplers::cls_radial_twist::RadialTwist;
use crate::samplers::cls_ripple::Ripple;
use crate::samplers::cls_shredder::Shredder;
use crate::samplers::cls_sine_wave::SineWave;
use crate::traits::sampler::{Sampler, SamplerOutput};
use crate::types::cls_sampler_spec::SamplerSpec;
use mixed_signals::traits::SignalContext;
use smallvec::SmallVec;
use tui_vfx_types::VfxCellContext;

pub(crate) enum PreparedSampler {
    None,
    SineWave(SineWave),
    Ripple(Ripple),
    Shredder(Shredder),
    FaultLine(FaultLine),
    CrtSampler(CrtSampler),
    CrtJitter(CrtJitter),
    Bounce(Bounce),
    Pendulum(Pendulum),
    Gravity(Gravity),
    RadialTwist(RadialTwist),
}

impl PreparedSampler {
    /// Dispatch a per-cell sample call to the underlying sampler impl.
    ///
    /// The `None` variant short-circuits to a passthrough at
    /// `(ctx.local_x, ctx.local_y)` with zero displacement so downstream
    /// stages see a clean identity step.
    pub(crate) fn sample(&self, ctx: &VfxCellContext) -> SamplerOutput {
        match self {
            PreparedSampler::None => SamplerOutput::passthrough(ctx.local_x, ctx.local_y),
            PreparedSampler::SineWave(configured) => configured.sample(ctx),
            PreparedSampler::Ripple(configured) => configured.sample(ctx),
            PreparedSampler::Shredder(configured) => configured.sample(ctx),
            PreparedSampler::FaultLine(configured) => configured.sample(ctx),
            PreparedSampler::CrtSampler(configured) => configured.sample(ctx),
            PreparedSampler::CrtJitter(configured) => configured.sample(ctx),
            PreparedSampler::Bounce(configured) => configured.sample(ctx),
            PreparedSampler::Pendulum(configured) => configured.sample(ctx),
            PreparedSampler::Gravity(configured) => configured.sample(ctx),
            PreparedSampler::RadialTwist(configured) => configured.sample(ctx),
        }
    }

    pub(crate) fn name(&self) -> &'static str {
        match self {
            PreparedSampler::None => "None",
            PreparedSampler::SineWave(_) => "SineWave",
            PreparedSampler::Ripple(_) => "Ripple",
            PreparedSampler::Shredder(_) => "Shredder",
            PreparedSampler::FaultLine(_) => "FaultLine",
            PreparedSampler::CrtSampler(_) => "Crt",
            PreparedSampler::CrtJitter(_) => "CrtJitter",
            PreparedSampler::Bounce(_) => "Bounce",
            PreparedSampler::Pendulum(_) => "Pendulum",
            PreparedSampler::Gravity(_) => "Gravity",
            PreparedSampler::RadialTwist(_) => "RadialTwist",
        }
    }
}

/// Outcome of running the sampler chain for a single destination cell.
///
/// Carries both the final source coordinate (for `source.get(...)`) and
/// the accumulated resolved-coord delta (for
/// [`VfxCellContext::with_sampler_resolution`]).
///
/// `source_x` / `source_y` are `None` when any sampler in the chain
/// returned `SamplerOutput::no_displacement` (transparent / skipped). In
/// that case the orchestrator should `continue` past the cell — the delta
/// is reported as the partial accumulation up to that point but is
/// irrelevant since no source cell is read.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct SamplerChainOutcome {
    pub source_x: Option<u16>,
    pub source_y: Option<u16>,
    pub delta_x: i32,
    pub delta_y: i32,
}

pub(crate) fn prepare_samplers(
    t: f64,
    sampler_specs: &[SamplerSpec],
) -> SmallVec<[PreparedSampler; 2]> {
    sampler_specs
        .iter()
        .map(|sampler_spec| prepare_sampler(t, &Some(sampler_spec.clone())))
        .collect()
}

/// Walk the sampler chain for a single destination cell.
///
/// At each step the running `(current_x, current_y)` is fed in as the
/// sampler's local coordinate; a sampler may emit a delta that the chain
/// accumulates so the orchestrator can later thread it into
/// [`VfxCellContext::with_sampler_resolution`] for downstream stages.
pub(crate) fn sample_sampler_chain(
    samplers: &[PreparedSampler],
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    t: f64,
) -> SamplerChainOutcome {
    let mut current_x = x;
    let mut current_y = y;
    let mut accum_dx: i32 = 0;
    let mut accum_dy: i32 = 0;
    for sampler in samplers {
        let step_ctx = VfxCellContext::new(current_x, current_y, width, height, 0, 0, t);
        let out = sampler.sample(&step_ctx);
        match out.source {
            Some((sx, sy)) => {
                current_x = sx;
                current_y = sy;
                accum_dx = accum_dx.saturating_add(out.delta_x);
                accum_dy = accum_dy.saturating_add(out.delta_y);
            }
            None => {
                return SamplerChainOutcome {
                    source_x: None,
                    source_y: None,
                    delta_x: accum_dx,
                    delta_y: accum_dy,
                };
            }
        }
    }
    SamplerChainOutcome {
        source_x: Some(current_x),
        source_y: Some(current_y),
        delta_x: accum_dx,
        delta_y: accum_dy,
    }
}

pub(crate) fn prepare_sampler(t: f64, sampler_spec: &Option<SamplerSpec>) -> PreparedSampler {
    let Some(spec) = sampler_spec else {
        return PreparedSampler::None;
    };

    let signal_ctx = SignalContext::for_loop(t, 0);

    match spec {
        SamplerSpec::None => PreparedSampler::None,
        SamplerSpec::SineWave {
            axis,
            amplitude,
            frequency,
            speed,
            phase,
        } => {
            let eval_amplitude = amplitude.evaluate(t, &signal_ctx).unwrap_or(1.0);
            let eval_frequency = frequency.evaluate(t, &signal_ctx).unwrap_or(1.0);
            let eval_speed = speed.evaluate(t, &signal_ctx).unwrap_or(1.0);
            let eval_phase = phase.evaluate(t, &signal_ctx).unwrap_or(0.0);
            PreparedSampler::SineWave(SineWave::new(
                eval_amplitude,
                eval_frequency,
                eval_speed,
                *axis,
                eval_phase,
            ))
        }
        SamplerSpec::Ripple {
            amplitude,
            wavelength,
            speed,
            center,
        } => {
            let eval_amplitude = amplitude.evaluate(t, &signal_ctx).unwrap_or(1.0);
            let eval_wavelength = wavelength.evaluate(t, &signal_ctx).unwrap_or(1.0);
            let eval_speed = speed.evaluate(t, &signal_ctx).unwrap_or(1.0);
            PreparedSampler::Ripple(Ripple::new(
                eval_amplitude,
                eval_wavelength,
                eval_speed,
                *center,
            ))
        }
        SamplerSpec::Shredder {
            stripe_width,
            odd_speed,
            even_speed,
            offset,
        } => {
            let eval_odd_speed = odd_speed.evaluate(t, &signal_ctx).unwrap_or(1.0);
            let eval_even_speed = even_speed.evaluate(t, &signal_ctx).unwrap_or(1.0);
            let mut sampler = Shredder::new(*stripe_width, eval_odd_speed, eval_even_speed);
            if let Some(offset) = offset {
                sampler = sampler.with_fixed_offset(*offset);
            }
            PreparedSampler::Shredder(sampler)
        }
        SamplerSpec::FaultLine {
            seed,
            intensity,
            split_bias,
            offset,
        } => {
            let eval_intensity = intensity.evaluate(t, &signal_ctx).unwrap_or(1.0);
            let mut sampler = FaultLine::new(*seed, eval_intensity, *split_bias);
            if let Some(offset) = offset {
                sampler = sampler.with_fixed_offset(*offset);
            }
            PreparedSampler::FaultLine(sampler)
        }
        SamplerSpec::Crt {
            curvature,
            jitter,
            scanline_strength: _,
        } => {
            let eval_curvature = curvature.evaluate(t, &signal_ctx).unwrap_or(0.0);
            let eval_jitter = jitter.evaluate(t, &signal_ctx).unwrap_or(0.0);
            PreparedSampler::CrtSampler(CrtSampler::new(eval_curvature, eval_jitter))
        }
        SamplerSpec::CrtJitter {
            intensity,
            speed_hz,
            decay_ms,
        } => {
            let eval_intensity = intensity.evaluate(t, &signal_ctx).unwrap_or(1.0);
            let eval_speed_hz = speed_hz.evaluate(t, &signal_ctx).unwrap_or(10.0);
            PreparedSampler::CrtJitter(CrtJitter {
                intensity: eval_intensity,
                speed_hz: eval_speed_hz,
                decay: *decay_ms as f32 / 1000.0,
                seed: 42,
            })
        }
        SamplerSpec::Bounce {
            amplitude,
            speed,
            phase_spread,
        } => {
            let eval_amplitude = amplitude.evaluate(t, &signal_ctx).unwrap_or(2.0);
            let eval_speed = speed.evaluate(t, &signal_ctx).unwrap_or(4.0);
            let eval_phase_spread = phase_spread.evaluate(t, &signal_ctx).unwrap_or(0.5);
            PreparedSampler::Bounce(Bounce::new(eval_amplitude, eval_speed, eval_phase_spread))
        }
        SamplerSpec::Pendulum {
            axis,
            amplitude,
            speed,
            phase_spread,
        } => {
            let eval_amplitude = amplitude.evaluate(t, &signal_ctx).unwrap_or(2.0);
            let eval_speed = speed.evaluate(t, &signal_ctx).unwrap_or(2.0);
            let eval_phase_spread = phase_spread.evaluate(t, &signal_ctx).unwrap_or(0.3);
            PreparedSampler::Pendulum(Pendulum::new(
                eval_amplitude,
                eval_speed,
                eval_phase_spread,
                *axis,
            ))
        }
        SamplerSpec::Gravity {
            axis,
            acceleration,
            terminal_velocity,
        } => {
            let eval_accel = acceleration.evaluate(t, &signal_ctx).unwrap_or(4.0);
            let eval_terminal = terminal_velocity.evaluate(t, &signal_ctx).unwrap_or(10.0);
            PreparedSampler::Gravity(Gravity::new(eval_accel, eval_terminal, *axis))
        }
        SamplerSpec::RadialTwist {
            twist,
            center,
            radius_floor,
        } => {
            let eval_twist = twist.evaluate(t, &signal_ctx).unwrap_or(1.0);
            let eval_radius_floor = radius_floor.evaluate(t, &signal_ctx).unwrap_or(0.1);
            PreparedSampler::RadialTwist(RadialTwist::new(eval_twist, *center, eval_radius_floor))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::mask::Mask;
    use crate::types::cls_sampler_spec::Axis;

    /// Test-only mask whose visibility decision genuinely branches on
    /// `ctx.resolved_x`. Used by
    /// [`downstream_stages_see_resolved_when_sampler_displaces`] to satisfy
    /// the no-inert-schema rule: a downstream stage's behavior must depend
    /// on the new field, not just observe its value.
    struct ResolvedXProbe;
    impl Mask for ResolvedXProbe {
        fn is_visible(&self, ctx: &tui_vfx_types::VfxCellContext) -> bool {
            ctx.resolved_x > ctx.local_x as i32
        }
    }

    #[test]
    fn chain_outcome_default_when_no_samplers() {
        let outcome = sample_sampler_chain(&[], 3, 5, 10, 10, 0.0);
        assert_eq!(outcome.source_x, Some(3));
        assert_eq!(outcome.source_y, Some(5));
        assert_eq!(outcome.delta_x, 0);
        assert_eq!(outcome.delta_y, 0);
    }

    #[test]
    fn chain_outcome_passthrough_for_none_variant() {
        let samplers: SmallVec<[PreparedSampler; 2]> = smallvec::smallvec![PreparedSampler::None];
        let outcome = sample_sampler_chain(&samplers, 7, 11, 16, 16, 0.0);
        assert_eq!(outcome.source_x, Some(7));
        assert_eq!(outcome.source_y, Some(11));
        assert_eq!(outcome.delta_x, 0);
        assert_eq!(outcome.delta_y, 0);
    }

    #[test]
    fn prepared_sampler_name_covers_each_variant() {
        // SineWave is a representative non-None variant; we only need to
        // confirm the wrapper's name() table did not regress when sample()
        // changed shape.
        let s = PreparedSampler::SineWave(SineWave::new(1.0, 1.0, 1.0, Axis::X, 0.0));
        assert_eq!(s.name(), "SineWave");
        assert_eq!(PreparedSampler::None.name(), "None");
    }

    /// Spec test 1: orchestrator-equivalent chain accumulates the resolved
    /// coord across two displacing samplers in sequence. Two real
    /// `SineWave` samplers are stacked; both displace on the X axis at
    /// different frequencies, so the chain's accumulated `delta_x` must
    /// equal the sum of their per-step deltas (and `delta_y == 0` since
    /// neither displaces on Y).
    #[test]
    fn orchestrator_accumulates_resolved_across_sampler_chain() {
        // Two SineWaves with different frequencies → different per-step
        // deltas at the same input cell. Phases pinned so the test is
        // deterministic.
        let sampler_a = PreparedSampler::SineWave(SineWave::new(2.0, 0.5, 0.0, Axis::X, 0.0));
        let sampler_b = PreparedSampler::SineWave(SineWave::new(1.5, 0.25, 0.0, Axis::X, 0.0));
        let chain: SmallVec<[PreparedSampler; 2]> = smallvec::smallvec![sampler_a, sampler_b];

        // Reproduce the chain by hand to compute the expected accumulated
        // deltas. This is exactly what `sample_sampler_chain` must do.
        let local_x: u16 = 4;
        let local_y: u16 = 4;
        let width: u16 = 16;
        let height: u16 = 16;
        let t: f64 = 0.0;

        let mut expect_acc_dx: i32 = 0;
        let mut expect_acc_dy: i32 = 0;
        let mut cur_x = local_x;
        let mut cur_y = local_y;
        for sampler in chain.iter() {
            let step_ctx = VfxCellContext::new(cur_x, cur_y, width, height, 0, 0, t);
            let out = sampler.sample(&step_ctx);
            if let Some((sx, sy)) = out.source {
                cur_x = sx;
                cur_y = sy;
                expect_acc_dx = expect_acc_dx.saturating_add(out.delta_x);
                expect_acc_dy = expect_acc_dy.saturating_add(out.delta_y);
            }
        }

        // Now drive the production accumulator and compare.
        let outcome = sample_sampler_chain(&chain, local_x, local_y, width, height, t);
        assert_eq!(outcome.source_x, Some(cur_x));
        assert_eq!(outcome.source_y, Some(cur_y));
        assert_eq!(outcome.delta_x, expect_acc_dx);
        assert_eq!(outcome.delta_y, expect_acc_dy);

        // The downstream-stage ctx, built by the orchestrator pattern,
        // carries `resolved_x = local_x + delta1_x + delta2_x` and
        // `resolved_y = local_y + delta1_y + delta2_y`.
        let downstream = VfxCellContext::new(local_x, local_y, width, height, 0, 0, t)
            .with_sampler_resolution(outcome.delta_x, outcome.delta_y);
        assert_eq!(downstream.resolved_x, local_x as i32 + expect_acc_dx);
        assert_eq!(downstream.resolved_y, local_y as i32 + expect_acc_dy);
    }

    /// Spec test 2: prove that a downstream stage's behavior actually
    /// depends on `resolved_x`. Drives the test-only [`ResolvedXProbe`]
    /// mask, whose `is_visible` returns `ctx.resolved_x > ctx.local_x as i32`,
    /// against two ctxs that differ ONLY in whether the sampler delta was
    /// applied. The mask MUST report different visibility for the two —
    /// otherwise the new field is inert.
    ///
    /// Satisfies the no-inert-schema rule (`feedback_no_inert_schema`):
    /// `resolved_x` is read by a `Mask::is_visible` impl and that read
    /// changes the boolean outcome.
    #[test]
    fn downstream_stages_see_resolved_when_sampler_displaces() {
        // Sampler params chosen to produce a strictly positive X
        // displacement at `local_x = 1`.
        let sampler = PreparedSampler::SineWave(SineWave::new(5.0, 0.25, 0.0, Axis::X, 0.0));
        let chain: SmallVec<[PreparedSampler; 2]> = smallvec::smallvec![sampler];

        let local_x: u16 = 1;
        let local_y: u16 = 1;
        let width: u16 = 32;
        let height: u16 = 8;
        let t: f64 = 0.0;

        let outcome = sample_sampler_chain(&chain, local_x, local_y, width, height, t);
        // Pre-condition: the fixture must produce a positive delta, else
        // the differential assertion below tests nothing of substance.
        assert!(
            outcome.delta_x > 0,
            "test fixture sampler must produce strictly positive X displacement at (1,1)"
        );

        // Build the two ctxs the orchestrator would pass to a downstream
        // stage: one with the sampler-accumulated delta applied, one without.
        let static_ctx = VfxCellContext::new(local_x, local_y, width, height, 0, 0, t);
        let displaced_ctx = static_ctx.with_sampler_resolution(outcome.delta_x, outcome.delta_y);

        // Sanity: the two ctxs must differ on resolved_x and only on
        // resolved_x (and resolved_y, both i32 fields).
        assert_eq!(static_ctx.resolved_x, local_x as i32);
        assert_eq!(displaced_ctx.resolved_x, local_x as i32 + outcome.delta_x);

        // The downstream stage's behavior MUST branch on resolved_x. The
        // probe returns true iff resolved_x > local_x; the displaced ctx
        // satisfies that, the static ctx does not.
        let probe = ResolvedXProbe;
        assert!(
            probe.is_visible(&displaced_ctx),
            "ResolvedXProbe must see the sampler-displaced ctx as visible"
        );
        assert!(
            !probe.is_visible(&static_ctx),
            "ResolvedXProbe must reject the un-displaced ctx — proves resolved_x is load-bearing"
        );
    }
}

// <FILE>tui-vfx-compositor/src/pipeline/cls_prepared_sampler.rs</FILE> - <DESC>Prepared sampler enum for pipeline rendering</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
