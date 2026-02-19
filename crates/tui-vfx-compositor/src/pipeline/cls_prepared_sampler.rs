// <FILE>tui-vfx-compositor/src/pipeline/cls_prepared_sampler.rs</FILE> - <DESC>Prepared sampler enum for pipeline rendering</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Add pendulum sampler for swaying animations</WCTX>
// <CLOG>Add Pendulum variant to PreparedSampler</CLOG>

use crate::samplers::cls_bounce::Bounce;
use crate::samplers::cls_crt_jitter::CrtJitter;
use crate::samplers::cls_crt_sampler::CrtSampler;
use crate::samplers::cls_fault_line::FaultLine;
use crate::samplers::cls_pendulum::Pendulum;
use crate::samplers::cls_ripple::Ripple;
use crate::samplers::cls_shredder::Shredder;
use crate::samplers::cls_sine_wave::SineWave;
use crate::traits::sampler::Sampler;
use crate::types::cls_sampler_spec::SamplerSpec;
use mixed_signals::traits::SignalContext;

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
        let sampled = match self {
            PreparedSampler::None => Some((x, y)),
            PreparedSampler::SineWave(configured) => configured.sample(x, y, width, height, t),
            PreparedSampler::Ripple(configured) => configured.sample(x, y, width, height, t),
            PreparedSampler::Shredder(configured) => configured.sample(x, y, width, height, t),
            PreparedSampler::FaultLine(configured) => configured.sample(x, y, width, height, t),
            PreparedSampler::CrtSampler(configured) => configured.sample(x, y, width, height, t),
            PreparedSampler::CrtJitter(configured) => configured.sample(x, y, width, height, t),
            PreparedSampler::Bounce(configured) => configured.sample(x, y, width, height, t),
            PreparedSampler::Pendulum(configured) => configured.sample(x, y, width, height, t),
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
        }
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
    }
}

// <FILE>tui-vfx-compositor/src/pipeline/cls_prepared_sampler.rs</FILE> - <DESC>Prepared sampler enum for pipeline rendering</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
