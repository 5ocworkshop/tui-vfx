// <FILE>tui-vfx-compositor/src/pipeline/cls_prepared_sampler.rs</FILE> - <DESC>Prepared sampler enum for pipeline rendering</DESC>
// <VERS>VERSION: 1.5.0</VERS>
// <WCTX>Slice 6.6 §F.3 — fix pre-existing sampler dispatcher drift blocking build</WCTX>
// <CLOG>1.5.0: PreparedSampler::sample and sample_sampler_chain now build VfxCellContext per call and delegate via the Sampler trait; fixes build break from incomplete F.4 prep work.</CLOG>

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
use crate::traits::sampler::Sampler;
use crate::types::cls_sampler_spec::SamplerSpec;
use tui_vfx_types::VfxCellContext;
use mixed_signals::traits::SignalContext;
use smallvec::SmallVec;

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
    pub(crate) fn sample(
        &self,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        t: f64,
    ) -> (Option<u16>, Option<u16>) {
        let ctx = VfxCellContext::new(x, y, width, height, 0, 0, t);
        let sampled = match self {
            PreparedSampler::None => Some((x, y)),
            PreparedSampler::SineWave(configured) => configured.sample(&ctx),
            PreparedSampler::Ripple(configured) => configured.sample(&ctx),
            PreparedSampler::Shredder(configured) => configured.sample(&ctx),
            PreparedSampler::FaultLine(configured) => configured.sample(&ctx),
            PreparedSampler::CrtSampler(configured) => configured.sample(&ctx),
            PreparedSampler::CrtJitter(configured) => configured.sample(&ctx),
            PreparedSampler::Bounce(configured) => configured.sample(&ctx),
            PreparedSampler::Pendulum(configured) => configured.sample(&ctx),
            PreparedSampler::Gravity(configured) => configured.sample(&ctx),
            PreparedSampler::RadialTwist(configured) => configured.sample(&ctx),
        };
        match sampled {
            Some((sx, sy)) => (Some(sx), Some(sy)),
            None => (None, None),
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

pub(crate) fn prepare_samplers(
    t: f64,
    sampler_specs: &[SamplerSpec],
) -> SmallVec<[PreparedSampler; 2]> {
    sampler_specs
        .iter()
        .map(|sampler_spec| prepare_sampler(t, &Some(sampler_spec.clone())))
        .collect()
}

pub(crate) fn sample_sampler_chain(
    samplers: &[PreparedSampler],
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    t: f64,
) -> (Option<u16>, Option<u16>) {
    let mut current_x = x;
    let mut current_y = y;
    for sampler in samplers {
        match sampler.sample(current_x, current_y, width, height, t) {
            (Some(next_x), Some(next_y)) => {
                current_x = next_x;
                current_y = next_y;
            }
            _ => return (None, None),
        }
    }
    (Some(current_x), Some(current_y))
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
        } => {
            let eval_odd_speed = odd_speed.evaluate(t, &signal_ctx).unwrap_or(1.0);
            let eval_even_speed = even_speed.evaluate(t, &signal_ctx).unwrap_or(1.0);
            PreparedSampler::Shredder(Shredder::new(
                *stripe_width,
                eval_odd_speed,
                eval_even_speed,
            ))
        }
        SamplerSpec::FaultLine {
            seed,
            intensity,
            split_bias,
        } => {
            let eval_intensity = intensity.evaluate(t, &signal_ctx).unwrap_or(1.0);
            PreparedSampler::FaultLine(FaultLine::new(*seed, eval_intensity, *split_bias))
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

// <FILE>tui-vfx-compositor/src/pipeline/cls_prepared_sampler.rs</FILE> - <DESC>Prepared sampler enum for pipeline rendering</DESC>
// <VERS>END OF VERSION: 1.5.0</VERS>
