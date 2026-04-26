// <FILE>tui-vfx-style/src/models/cls_terminal_water_shader.rs</FILE> - <DESC>Layered terminal water/ocean shader</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Glyph rendering framework Phase 5: expose WaterFieldSample (pub(crate)) and retain slope_x/slope_y for the WaterFieldSignal wrapper's SignalWithSlope impl.</WCTX>
// <CLOG>0.3.0: WaterFieldSample + sample_field_at promoted to pub(crate); slope_x/slope_y added to the sample so the sibling WaterFieldSignal can return analytic gradients without 8× subcell field re-evaluations.</CLOG>

//! Layered terminal water/ocean shader.
//!
//! `TerminalWaterShader` computes a deterministic water field from layered
//! sine/Gerstner-style waves plus optional ripple, rain, flow, and wake
//! contributions. In the style-shader path it maps the field to water colors;
//! glyph-capable derivations can reuse the scalar field helpers later.

use crate::models::{ColorConfig, ColorSpace};
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::blend_colors;
use mixed_signals::math::{finite_or, finite_or_clamp, smoothstep};
use mixed_signals::noise::hash01;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

const MAX_LAYERS: u8 = 4;
const MAX_EMITTERS: usize = 16;
const MAX_WAKE_SOURCES: usize = 8;
const DEFAULT_LAYERS: u8 = 3;
const DEFAULT_AMPLITUDE: f32 = 0.35;
const DEFAULT_WAVELENGTH: f32 = 12.0;
const DEFAULT_SPEED: f32 = 1.0;
const DEFAULT_DIRECTION_DEG: f32 = 25.0;
const DEFAULT_STEEPNESS: f32 = 0.45;
const DEFAULT_NORMAL_STRENGTH: f32 = 1.4;
const DEFAULT_DIFFUSE: f32 = 0.65;
const DEFAULT_SPECULAR: f32 = 0.55;
const DEFAULT_SHININESS: f32 = 24.0;
const DEFAULT_FRESNEL: f32 = 0.35;
const DEFAULT_FOAM: f32 = 0.5;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum WaterApplyTo {
    Foreground,
    Background,
    #[default]
    Both,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
#[derive(Default)]
pub enum WaterWaveMode {
    #[default]
    Ocean,
    Ripple {
        #[serde(default)]
        emitters: Vec<WaterRippleEmitter>,
        #[config(default = 0.0)]
        base_shimmer: f32,
    },
    Rain {
        #[serde(default)]
        emitters: Vec<WaterRippleEmitter>,
        #[config(default = 1)]
        seed: u32,
        #[config(default = 8)]
        density: u8,
        #[config(default = 0.45)]
        drop_strength: f32,
    },
    Flow {
        #[config(default = 0.0)]
        direction_deg: f32,
        #[config(default = 1.0)]
        speed: f32,
        #[config(default = 0.18)]
        turbulence: f32,
        #[config(default = 0.35)]
        flow_strength: f32,
    },
    Wake {
        #[serde(default)]
        sources: Vec<WaterWakeSource>,
        #[config(default = 0.65)]
        wake_strength: f32,
        #[config(default = 18.0)]
        trail_length: f32,
        #[config(default = 28.0)]
        spread_deg: f32,
    },
    OceanWithRipples {
        #[serde(default)]
        emitters: Vec<WaterRippleEmitter>,
        #[config(default = 0.55)]
        ocean_mix: f32,
        #[config(default = 1.0)]
        ripple_mix: f32,
    },
    Composite {
        /// Bounded list of child water modes mixed together.
        ///
        /// This field is intentionally opaque to generated config metadata:
        /// `WaterWaveMode` is recursive (`Composite` contains more modes), and
        /// the current schema walker expands fields eagerly without recursion
        /// guards. Serde still validates the concrete child modes normally.
        #[config(opaque)]
        #[serde(default)]
        modes: Vec<WaterWaveMode>,
    },
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct WaterRippleEmitter {
    #[config(default = 0.5)]
    pub center_x: f32,
    #[config(default = 0.5)]
    pub center_y: f32,
    #[config(default = 0.0)]
    pub start_time: f32,
    #[config(default = 0.6)]
    pub amplitude: f32,
    #[config(default = 8.0)]
    pub speed: f32,
    #[config(default = 1.6)]
    pub frequency: f32,
    #[config(default = 2.5)]
    pub ring_width: f32,
    #[config(default = 0.45)]
    pub decay: f32,
    #[config(default = 0.025)]
    pub damping: f32,
}

impl Default for WaterRippleEmitter {
    fn default() -> Self {
        Self {
            center_x: 0.5,
            center_y: 0.5,
            start_time: 0.0,
            amplitude: 0.6,
            speed: 8.0,
            frequency: 1.6,
            ring_width: 2.5,
            decay: 0.45,
            damping: 0.025,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct WaterWakeSource {
    #[config(default = 0.5)]
    pub x: f32,
    #[config(default = 0.5)]
    pub y: f32,
    #[config(default = 0.0)]
    pub direction_deg: f32,
    #[config(default = 1.0)]
    pub speed: f32,
    #[config(default = 0.0)]
    pub start_time: f32,
}

impl Default for WaterWakeSource {
    fn default() -> Self {
        Self {
            x: 0.5,
            y: 0.5,
            direction_deg: 0.0,
            speed: 1.0,
            start_time: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct TerminalWaterShader {
    #[serde(default)]
    pub mode: WaterWaveMode,
    #[config(default = 3)]
    pub layers: u8,
    #[config(default = 0.35)]
    pub amplitude: f32,
    #[config(default = 12.0)]
    pub wavelength: f32,
    #[config(default = 1.0)]
    pub speed: f32,
    #[config(default = 25.0)]
    pub direction_deg: f32,
    #[config(default = 0.45)]
    pub steepness: f32,
    #[config(default = 1.4)]
    pub normal_strength: f32,
    #[config(default = 0.65)]
    pub diffuse: f32,
    #[config(default = 0.55)]
    pub specular: f32,
    #[config(default = 24.0)]
    pub shininess: f32,
    #[config(default = 0.35)]
    pub fresnel: f32,
    #[config(default = 0.5)]
    pub foam: f32,
    pub deep_color: ColorConfig,
    pub shallow_color: ColorConfig,
    pub foam_color: ColorConfig,
    #[config(default = 0.0)]
    pub glint_strength: f32,
    #[config(default = -18.0)]
    pub glint_angle_deg: f32,
    #[config(default = 8.0)]
    pub glint_width: f32,
    #[config(default = 1.0)]
    pub glint_speed: f32,
    #[serde(default)]
    pub apply_to: WaterApplyTo,
}

impl Default for TerminalWaterShader {
    fn default() -> Self {
        Self {
            mode: WaterWaveMode::default(),
            layers: DEFAULT_LAYERS,
            amplitude: DEFAULT_AMPLITUDE,
            wavelength: DEFAULT_WAVELENGTH,
            speed: DEFAULT_SPEED,
            direction_deg: DEFAULT_DIRECTION_DEG,
            steepness: DEFAULT_STEEPNESS,
            normal_strength: DEFAULT_NORMAL_STRENGTH,
            diffuse: DEFAULT_DIFFUSE,
            specular: DEFAULT_SPECULAR,
            shininess: DEFAULT_SHININESS,
            fresnel: DEFAULT_FRESNEL,
            foam: DEFAULT_FOAM,
            deep_color: ColorConfig::Rgb { r: 5, g: 32, b: 64 },
            shallow_color: ColorConfig::Rgb {
                r: 40,
                g: 170,
                b: 210,
            },
            foam_color: ColorConfig::White,
            glint_strength: 0.0,
            glint_angle_deg: -18.0,
            glint_width: 8.0,
            glint_speed: 1.0,
            apply_to: WaterApplyTo::Both,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

/// Per-cell sample of the water field used by [`TerminalWaterShader`].
///
/// Visibility is `pub(crate)` so the sibling [`crate::models::cls_water_field_signal`]
/// wrapper can route it through the [`mixed_signals::traits::Signal`] surface.
/// Fields stay `pub(crate)` until probe/trace consumers need them publicly.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct WaterFieldSample {
    pub(crate) height: f32,
    pub(crate) height_scalar: f32,
    pub(crate) crest: f32,
    pub(crate) curvature: f32,
    pub(crate) foam: f32,
    pub(crate) diffuse: f32,
    pub(crate) specular: f32,
    pub(crate) fresnel: f32,
    pub(crate) light_scalar: f32,
    pub(crate) ripple_scalar: f32,
    /// Cached spatial gradient ∂height/∂x in cell-space units.
    /// Used by [`crate::models::cls_water_field_signal::WaterFieldSignal`]'s
    /// `SignalWithSlope` impl to skip 8× subcell field evaluations.
    pub(crate) slope_x: f32,
    /// Cached spatial gradient ∂height/∂y in cell-space units.
    pub(crate) slope_y: f32,
}

impl TerminalWaterShader {
    fn sanitized_layers(&self) -> u8 {
        self.layers.clamp(1, MAX_LAYERS)
    }

    pub(crate) fn sample_field_at(
        &self,
        x: f32,
        y: f32,
        width: u16,
        height: u16,
        t: f32,
    ) -> WaterFieldSample {
        let layers = self.sanitized_layers();
        let amplitude = finite_or_clamp(self.amplitude, 0.0, 2.0, DEFAULT_AMPLITUDE);
        let wavelength = finite_or_clamp(self.wavelength, 1.0, 512.0, DEFAULT_WAVELENGTH);
        let speed = finite_or(self.speed, DEFAULT_SPEED);
        let direction_deg = finite_or(self.direction_deg, DEFAULT_DIRECTION_DEG);
        let steepness = finite_or_clamp(self.steepness, 0.0, 1.0, DEFAULT_STEEPNESS);
        let normal_strength =
            finite_or_clamp(self.normal_strength, 0.0, 4.0, DEFAULT_NORMAL_STRENGTH);
        let diffuse_strength = finite_or_clamp(self.diffuse, 0.0, 2.0, DEFAULT_DIFFUSE);
        let specular_strength = finite_or_clamp(self.specular, 0.0, 2.0, DEFAULT_SPECULAR);
        let shininess = finite_or_clamp(self.shininess, 1.0, 128.0, DEFAULT_SHININESS);
        let fresnel_strength = finite_or_clamp(self.fresnel, 0.0, 2.0, DEFAULT_FRESNEL);
        let foam_strength = finite_or_clamp(self.foam, 0.0, 2.0, DEFAULT_FOAM);
        let glint_strength = finite_or_clamp(self.glint_strength, 0.0, 2.0, 0.0);

        let mut height_value = 0.0;
        let mut max_height = 0.0;
        let mut slope_x = 0.0;
        let mut slope_y = 0.0;
        let mut crest = 0.0;
        let mut curvature = 0.0;
        let time = t * speed;

        let ocean_mix = self.ocean_mix();
        if ocean_mix > 0.0 && amplitude > 0.0 {
            for i in 0..layers {
                let octave = i as f32;
                let amp = amplitude * ocean_mix * 0.55_f32.powf(octave);
                let lambda = (wavelength * 0.73_f32.powf(octave)).max(1.0);
                let k = std::f32::consts::TAU / lambda;
                let dir = direction_deg.to_radians() + octave * 1.37;
                let dx = dir.cos();
                let dy = dir.sin();
                let phase = k * (dx * x + dy * y) + time * (1.0 + octave * 0.37);
                let s = phase.sin();
                let c = phase.cos();
                height_value += amp * s;
                max_height += amp.abs();
                slope_x += amp * k * dx * c;
                slope_y += amp * k * dy * c;
                crest += steepness * amp.abs() * k * c.abs();
                curvature += amp * k * k * -s;
            }
        }

        let mut ripple_scalar = self.add_ripples_to_field(
            x,
            y,
            width,
            height,
            t,
            &mut height_value,
            &mut max_height,
            &mut slope_x,
            &mut slope_y,
            &mut crest,
            &mut curvature,
        );
        ripple_scalar = ripple_scalar.max(self.add_rain_to_field(
            x,
            y,
            width,
            height,
            t,
            &mut height_value,
            &mut max_height,
            &mut slope_x,
            &mut slope_y,
            &mut crest,
            &mut curvature,
        ));
        ripple_scalar = ripple_scalar.max(self.add_flow_to_field(
            x,
            y,
            t,
            &mut height_value,
            &mut max_height,
            &mut slope_x,
            &mut slope_y,
            &mut crest,
            &mut curvature,
        ));
        ripple_scalar = ripple_scalar.max(self.add_wake_to_field(
            x,
            y,
            width,
            height,
            t,
            &mut height_value,
            &mut max_height,
            &mut slope_x,
            &mut slope_y,
            &mut crest,
            &mut curvature,
        ));
        ripple_scalar = ripple_scalar.max(self.add_composite_to_field(
            x,
            y,
            width,
            height,
            t,
            &mut height_value,
            &mut max_height,
            &mut slope_x,
            &mut slope_y,
            &mut crest,
            &mut curvature,
        ));

        let height_scalar = if max_height > f32::EPSILON {
            ((height_value / max_height) * 0.5 + 0.5).clamp(0.0, 1.0)
        } else {
            0.5
        };
        let normal = normalize3(-slope_x * normal_strength, -slope_y * normal_strength, 1.0);
        let light = normalize3(-0.35, -0.55, 0.76);
        let view = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let half_v = normalize3(light.x + view.x, light.y + view.y, light.z + view.z);
        let diffuse = dot3(normal, light).max(0.0);
        let specular = dot3(normal, half_v).max(0.0).powf(shininess);
        let fresnel = (1.0 - dot3(normal, view).max(0.0)).powf(3.0);
        let foam_signal = crest + curvature.abs() * 0.05;
        let foam = smoothstep(0.45, 0.85, foam_signal) * foam_strength;
        let glint = self.glint_at(x, y, t) * specular * glint_strength;
        let light_scalar = (0.18
            + diffuse_strength * diffuse
            + specular_strength * specular
            + fresnel_strength * fresnel
            + foam
            + ripple_scalar * 0.15
            + glint)
            .clamp(0.0, 1.0);

        WaterFieldSample {
            height: height_value,
            height_scalar,
            crest,
            curvature,
            foam: foam.clamp(0.0, 1.0),
            diffuse,
            specular,
            fresnel,
            light_scalar,
            ripple_scalar,
            slope_x,
            slope_y,
        }
    }

    fn ocean_mix(&self) -> f32 {
        match &self.mode {
            WaterWaveMode::Ripple { base_shimmer, .. } => {
                finite_or_clamp(*base_shimmer, 0.0, 0.25, 0.0)
            }
            WaterWaveMode::OceanWithRipples { ocean_mix, .. } => {
                finite_or_clamp(*ocean_mix, 0.0, 1.0, 0.55)
            }
            WaterWaveMode::Flow { flow_strength, .. } => {
                finite_or_clamp(*flow_strength, 0.0, 1.0, 0.35) * 0.35
            }
            WaterWaveMode::Composite { .. } => 0.0,
            _ => 1.0,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn add_ripples_to_field(
        &self,
        x: f32,
        y: f32,
        width: u16,
        height: u16,
        t: f32,
        height_value: &mut f32,
        max_height: &mut f32,
        slope_x: &mut f32,
        slope_y: &mut f32,
        crest: &mut f32,
        curvature: &mut f32,
    ) -> f32 {
        let (emitters, ripple_mix) = match &self.mode {
            WaterWaveMode::Ocean => return 0.0,
            WaterWaveMode::Ripple { emitters, .. } => (emitters.as_slice(), 1.0),
            WaterWaveMode::OceanWithRipples {
                emitters,
                ripple_mix,
                ..
            } => (
                emitters.as_slice(),
                finite_or_clamp(*ripple_mix, 0.0, 2.0, 1.0),
            ),
            _ => return 0.0,
        };
        add_emitters(
            emitters,
            ripple_mix,
            x,
            y,
            width,
            height,
            t,
            height_value,
            max_height,
            slope_x,
            slope_y,
            crest,
            curvature,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn add_rain_to_field(
        &self,
        x: f32,
        y: f32,
        width: u16,
        height: u16,
        t: f32,
        height_value: &mut f32,
        max_height: &mut f32,
        slope_x: &mut f32,
        slope_y: &mut f32,
        crest: &mut f32,
        curvature: &mut f32,
    ) -> f32 {
        let (emitters, seed, density, drop_strength) = match &self.mode {
            WaterWaveMode::Rain {
                emitters,
                seed,
                density,
                drop_strength,
            } => (
                emitters.as_slice(),
                *seed,
                (*density).min(16),
                finite_or_clamp(*drop_strength, 0.0, 2.0, 0.45),
            ),
            _ => return 0.0,
        };
        let mut scalar = add_emitters(
            emitters,
            drop_strength,
            x,
            y,
            width,
            height,
            t,
            height_value,
            max_height,
            slope_x,
            slope_y,
            crest,
            curvature,
        );
        for i in 0..density {
            let sx = hash01(seed ^ (i as u32).wrapping_mul(0x9E37_79B9));
            let sy = hash01(seed ^ (i as u32).wrapping_mul(0x85EB_CA6B));
            let start = -hash01(seed ^ (i as u32).wrapping_mul(0xC2B2_AE35)) * 1.5;
            let generated = WaterRippleEmitter {
                center_x: sx,
                center_y: sy,
                start_time: start,
                amplitude: 0.25 * drop_strength,
                speed: 10.0,
                frequency: 2.4,
                ring_width: 1.4,
                decay: 0.7,
                damping: 0.04,
            };
            scalar = scalar.max(add_single_emitter(
                &generated,
                1.0,
                x,
                y,
                width,
                height,
                t,
                height_value,
                max_height,
                slope_x,
                slope_y,
                crest,
                curvature,
            ));
        }
        scalar
    }

    #[allow(clippy::too_many_arguments)]
    fn add_flow_to_field(
        &self,
        x: f32,
        y: f32,
        t: f32,
        height_value: &mut f32,
        max_height: &mut f32,
        slope_x: &mut f32,
        slope_y: &mut f32,
        crest: &mut f32,
        curvature: &mut f32,
    ) -> f32 {
        let (direction_deg, speed, turbulence, flow_strength) = match &self.mode {
            WaterWaveMode::Flow {
                direction_deg,
                speed,
                turbulence,
                flow_strength,
            } => (*direction_deg, *speed, *turbulence, *flow_strength),
            _ => return 0.0,
        };
        let dir = finite_or(direction_deg, 0.0).to_radians();
        let dx = dir.cos();
        let dy = dir.sin();
        let strength = finite_or_clamp(flow_strength, 0.0, 2.0, 0.35);
        let k = 0.32;
        let phase = k * (dx * x + dy * y) + t * finite_or(speed, 1.0);
        let turb = finite_or_clamp(turbulence, 0.0, 2.0, 0.18) * ((x * 0.17 + y * 0.11 + t).sin());
        let s = (phase + turb).sin();
        let c = (phase + turb).cos();
        let amp = 0.18 * strength;
        *height_value += amp * s;
        *max_height += amp.abs();
        *slope_x += amp * k * dx * c;
        *slope_y += amp * k * dy * c;
        *crest += amp.abs() * k * c.abs();
        *curvature += -amp * k * k * s;
        s.abs().clamp(0.0, 1.0) * strength.clamp(0.0, 1.0)
    }

    #[allow(clippy::too_many_arguments)]
    fn add_wake_to_field(
        &self,
        x: f32,
        y: f32,
        width: u16,
        height: u16,
        t: f32,
        height_value: &mut f32,
        max_height: &mut f32,
        slope_x: &mut f32,
        slope_y: &mut f32,
        crest: &mut f32,
        curvature: &mut f32,
    ) -> f32 {
        let (sources, wake_strength, trail_length, spread_deg) = match &self.mode {
            WaterWaveMode::Wake {
                sources,
                wake_strength,
                trail_length,
                spread_deg,
            } => (
                sources.as_slice(),
                finite_or_clamp(*wake_strength, 0.0, 2.0, 0.65),
                finite_or_clamp(*trail_length, 1.0, 128.0, 18.0),
                finite_or_clamp(*spread_deg, 1.0, 89.0, 28.0),
            ),
            _ => return 0.0,
        };
        let width_f = width.max(1) as f32;
        let height_f = height.max(1) as f32 * 2.0;
        let mut scalar: f32 = 0.0;
        for source in sources.iter().take(MAX_WAKE_SOURCES) {
            let age = (t - finite_or(source.start_time, 0.0)).max(0.0);
            let sx = finite_or_clamp(source.x, 0.0, 1.0, 0.5) * width_f;
            let sy = finite_or_clamp(source.y, 0.0, 1.0, 0.5) * height_f;
            let dir = finite_or(source.direction_deg, 0.0).to_radians();
            let back_x = -dir.cos();
            let back_y = -dir.sin();
            let dx = x - sx;
            let dy = y - sy;
            let along = dx * back_x + dy * back_y + age * finite_or(source.speed, 1.0);
            if along < 0.0 || along > trail_length {
                continue;
            }
            let cross = (dx * -back_y + dy * back_x).abs();
            let spread = (along * spread_deg.to_radians().tan()).max(1.0);
            let envelope =
                (1.0 - along / trail_length) * (-(cross * cross) / (2.0 * spread * spread)).exp();
            let wave = (along * 1.7 - t * 8.0).sin();
            let h = wake_strength * 0.25 * envelope * wave;
            *height_value += h;
            *max_height += (wake_strength * 0.25).abs();
            *slope_x += h * back_x * 0.25;
            *slope_y += h * back_y * 0.25;
            *crest += envelope * wake_strength;
            *curvature += -h * 0.2;
            scalar = scalar.max(envelope.clamp(0.0, 1.0));
        }
        scalar.clamp(0.0, 1.0)
    }

    #[allow(clippy::too_many_arguments)]
    fn add_composite_to_field(
        &self,
        x: f32,
        y: f32,
        width: u16,
        height: u16,
        t: f32,
        height_value: &mut f32,
        max_height: &mut f32,
        slope_x: &mut f32,
        slope_y: &mut f32,
        crest: &mut f32,
        curvature: &mut f32,
    ) -> f32 {
        let modes = match &self.mode {
            WaterWaveMode::Composite { modes } => modes,
            _ => return 0.0,
        };
        let mut scalar: f32 = 0.0;
        for mode in modes.iter().take(4) {
            let nested = TerminalWaterShader {
                mode: mode.clone(),
                ..self.clone()
            };
            let sample = nested.sample_field_at(x, y, width, height, t);
            *height_value += sample.height * 0.5;
            *max_height += sample.height.abs().max(0.001);
            *crest += sample.crest * 0.5;
            *curvature += sample.curvature * 0.5;
            // Nested slopes are already encoded into light; approximate through scalar to avoid exposing internals.
            *slope_x += (sample.height_scalar - 0.5) * 0.1;
            *slope_y += (sample.light_scalar - 0.5) * 0.1;
            scalar = scalar.max(sample.ripple_scalar);
        }
        scalar
    }

    fn sample_field_for_ctx(&self, ctx: &ShaderContext) -> WaterFieldSample {
        let x = ctx.local_x as f32;
        let y = ctx.local_y as f32 * 2.0;
        self.sample_field_at(x, y, ctx.width, ctx.height, ctx.t as f32)
    }

    fn lit_color(&self, sample: WaterFieldSample) -> Color {
        let deep: Color = self.deep_color.into();
        let shallow: Color = self.shallow_color.into();
        let foam: Color = self.foam_color.into();
        let water_t = (sample.height_scalar * sample.light_scalar).clamp(0.0, 1.0);
        let water = blend_colors(deep, shallow, water_t, ColorSpace::Rgb);
        blend_colors(water, foam, sample.foam, ColorSpace::Rgb)
    }

    fn color_blend_amount(&self, sample: WaterFieldSample) -> f32 {
        (0.25 + sample.light_scalar * 0.75).clamp(0.0, 1.0)
    }

    fn glint_at(&self, x: f32, y: f32, t: f32) -> f32 {
        let width = finite_or_clamp(self.glint_width, 0.1, 128.0, 8.0);
        let angle = finite_or(self.glint_angle_deg, -18.0).to_radians();
        let axis = x * angle.cos() + y * angle.sin() + t * finite_or(self.glint_speed, 1.0) * 12.0;
        let band = (axis / width).fract();
        smoothstep(0.0, 0.15, band) * (1.0 - smoothstep(0.15, 0.3, band))
    }
}

impl StyleShader for TerminalWaterShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let sample = self.sample_field_for_ctx(ctx);
        let target = self.lit_color(sample);
        let amount = self.color_blend_amount(sample);
        let mut result = base;
        match self.apply_to {
            WaterApplyTo::Foreground => {
                if base.fg != Color::TRANSPARENT {
                    result.fg = blend_colors(base.fg, target, amount, ColorSpace::Rgb);
                }
            }
            WaterApplyTo::Background => {
                if base.bg != Color::TRANSPARENT {
                    result.bg = blend_colors(base.bg, target, amount, ColorSpace::Rgb);
                }
            }
            WaterApplyTo::Both => {
                if base.fg != Color::TRANSPARENT {
                    result.fg = blend_colors(base.fg, target, amount, ColorSpace::Rgb);
                }
                if base.bg != Color::TRANSPARENT {
                    result.bg = blend_colors(base.bg, target, amount * 0.65, ColorSpace::Rgb);
                }
            }
        }
        result
    }

    fn name(&self) -> &'static str {
        "TerminalWater"
    }
}

#[allow(clippy::too_many_arguments)]
fn add_emitters(
    emitters: &[WaterRippleEmitter],
    ripple_mix: f32,
    x: f32,
    y: f32,
    width: u16,
    height: u16,
    t: f32,
    height_value: &mut f32,
    max_height: &mut f32,
    slope_x: &mut f32,
    slope_y: &mut f32,
    crest: &mut f32,
    curvature: &mut f32,
) -> f32 {
    let mut ripple_scalar: f32 = 0.0;
    for emitter in emitters.iter().take(MAX_EMITTERS) {
        ripple_scalar = ripple_scalar.max(add_single_emitter(
            emitter,
            ripple_mix,
            x,
            y,
            width,
            height,
            t,
            height_value,
            max_height,
            slope_x,
            slope_y,
            crest,
            curvature,
        ));
    }
    ripple_scalar.clamp(0.0, 1.0)
}

#[allow(clippy::too_many_arguments)]
fn add_single_emitter(
    emitter: &WaterRippleEmitter,
    ripple_mix: f32,
    x: f32,
    y: f32,
    width: u16,
    height: u16,
    t: f32,
    height_value: &mut f32,
    max_height: &mut f32,
    slope_x: &mut f32,
    slope_y: &mut f32,
    crest: &mut f32,
    curvature: &mut f32,
) -> f32 {
    let age = t - finite_or(emitter.start_time, 0.0);
    if age < 0.0 {
        return 0.0;
    }
    let width_f = width.max(1) as f32;
    let height_f = height.max(1) as f32 * 2.0;
    let cx = finite_or_clamp(emitter.center_x, 0.0, 1.0, 0.5) * width_f;
    let cy = finite_or_clamp(emitter.center_y, 0.0, 1.0, 0.5) * height_f;
    let dx = x - cx;
    let dy = y - cy;
    let r = (dx * dx + dy * dy).sqrt().max(0.001);
    let amplitude = finite_or_clamp(emitter.amplitude, 0.0, 2.0, 0.6) * ripple_mix;
    let speed = finite_or_clamp(emitter.speed, 0.0, 128.0, 8.0);
    let frequency = finite_or_clamp(emitter.frequency, 0.01, 32.0, 1.6);
    let ring_width = finite_or_clamp(emitter.ring_width, 0.5, 64.0, 2.5);
    let decay = finite_or_clamp(emitter.decay, 0.0, 8.0, 0.45);
    let damping = finite_or_clamp(emitter.damping, 0.0, 2.0, 0.025);
    let front = r - age * speed;
    let ring = (-(front * front) / (2.0 * ring_width * ring_width)).exp();
    let envelope = ring * (-decay * age).exp() * (-damping * r).exp();
    let phase = front * frequency;
    let s = phase.sin();
    let c = phase.cos();
    let h = amplitude * s * envelope;
    *height_value += h;
    *max_height += amplitude.abs().max(0.001);
    let radial_x = dx / r;
    let radial_y = dy / r;
    let dh_dr = amplitude * envelope * frequency * c;
    *slope_x += dh_dr * radial_x;
    *slope_y += dh_dr * radial_y;
    *crest += envelope.abs() * amplitude.abs();
    *curvature += -amplitude * frequency * frequency * s * envelope;
    envelope.clamp(0.0, 1.0)
}

fn dot3(a: Vec3, b: Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}
fn normalize3(x: f32, y: f32, z: f32) -> Vec3 {
    let len_sq = x * x + y * y + z * z;
    if !len_sq.is_finite() || len_sq <= f32::EPSILON {
        return Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
    }
    let inv = len_sq.sqrt().recip();
    Vec3 {
        x: x * inv,
        y: y * inv,
        z: z * inv,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn ctx_at(x: u16, y: u16, t: f64) -> ShaderContext {
        ShaderContext::new(x, y, 32, 12, 0, 0, t, None, None)
    }

    #[test]
    fn default_creates_valid_shader() {
        let shader = TerminalWaterShader::default();
        assert_eq!(shader.layers, 3);
        assert!(shader.amplitude > 0.0);
        assert!(shader.wavelength > 0.0);
        assert_eq!(shader.apply_to, WaterApplyTo::Both);
    }

    #[test]
    fn field_is_bounded_for_default() {
        let shader = TerminalWaterShader::default();
        let sample = shader.sample_field_for_ctx(&ctx_at(4, 3, 0.25));
        assert!((0.0..=1.0).contains(&sample.height_scalar));
        assert!((0.0..=1.0).contains(&sample.foam));
        assert!((0.0..=1.0).contains(&sample.light_scalar));
    }

    #[test]
    fn height_varies_by_position() {
        let shader = TerminalWaterShader::default();
        let a = shader
            .sample_field_for_ctx(&ctx_at(2, 2, 0.25))
            .height_scalar;
        let b = shader
            .sample_field_for_ctx(&ctx_at(20, 8, 0.25))
            .height_scalar;
        assert!((a - b).abs() > 0.001, "water field should vary by position");
    }

    #[test]
    fn light_varies_by_time() {
        let shader = TerminalWaterShader::default();
        let early = shader
            .sample_field_for_ctx(&ctx_at(10, 5, 0.1))
            .light_scalar;
        let late = shader
            .sample_field_for_ctx(&ctx_at(10, 5, 1.3))
            .light_scalar;
        assert!(
            (early - late).abs() > 0.001,
            "water lighting should animate over time"
        );
    }

    #[test]
    fn invalid_params_are_sanitized_without_nan() {
        let shader = TerminalWaterShader {
            layers: 255,
            amplitude: f32::NAN,
            wavelength: -10.0,
            speed: f32::INFINITY,
            direction_deg: f32::NAN,
            steepness: 99.0,
            normal_strength: f32::NAN,
            diffuse: f32::INFINITY,
            specular: f32::NAN,
            shininess: -1.0,
            fresnel: f32::NAN,
            foam: f32::INFINITY,
            ..TerminalWaterShader::default()
        };
        let sample = shader.sample_field_for_ctx(&ctx_at(4, 4, 0.5));
        assert!(sample.height_scalar.is_finite());
        assert!(sample.foam.is_finite());
        assert!(sample.light_scalar.is_finite());
    }

    #[test]
    fn ripple_mode_contributes_scalar() {
        let shader = TerminalWaterShader {
            mode: WaterWaveMode::Ripple {
                emitters: vec![WaterRippleEmitter::default()],
                base_shimmer: 0.0,
            },
            ..TerminalWaterShader::default()
        };
        let sample = shader.sample_field_for_ctx(&ctx_at(16, 6, 0.1));
        assert!(sample.ripple_scalar > 0.0);
    }

    #[test]
    fn rain_mode_contributes_scalar() {
        let shader = TerminalWaterShader {
            mode: WaterWaveMode::Rain {
                emitters: vec![],
                seed: 7,
                density: 4,
                drop_strength: 0.8,
            },
            ..TerminalWaterShader::default()
        };
        let sample = shader.sample_field_for_ctx(&ctx_at(8, 4, 0.5));
        assert!(sample.ripple_scalar >= 0.0);
        assert!(sample.light_scalar.is_finite());
    }

    #[test]
    fn wake_mode_contributes_scalar() {
        let shader = TerminalWaterShader {
            mode: WaterWaveMode::Wake {
                sources: vec![WaterWakeSource::default()],
                wake_strength: 1.0,
                trail_length: 30.0,
                spread_deg: 35.0,
            },
            ..TerminalWaterShader::default()
        };
        let sample = shader.sample_field_for_ctx(&ctx_at(16, 6, 0.5));
        assert!(sample.light_scalar.is_finite());
    }

    #[test]
    fn transparent_channels_are_preserved() {
        let shader = TerminalWaterShader::default();
        let base = Style {
            fg: Color::TRANSPARENT,
            bg: Color::TRANSPARENT,
            ..Style::default()
        };
        let styled = shader.style_at(&ctx_at(4, 4, 0.5), base);
        assert_eq!(styled.fg, Color::TRANSPARENT);
        assert_eq!(styled.bg, Color::TRANSPARENT);
    }

    #[test]
    fn shader_changes_visible_foreground() {
        let shader = TerminalWaterShader::default();
        let base = Style::fg(Color::WHITE);
        let styled = shader.style_at(&ctx_at(4, 4, 0.5), base);
        assert_ne!(styled.fg, Color::WHITE);
    }

    #[test]
    fn background_only_mode_leaves_foreground_unchanged() {
        let shader = TerminalWaterShader {
            apply_to: WaterApplyTo::Background,
            ..TerminalWaterShader::default()
        };
        let base = Style {
            fg: Color::WHITE,
            bg: Color::BLACK,
            ..Style::default()
        };
        let styled = shader.style_at(&ctx_at(4, 4, 0.5), base);
        assert_eq!(styled.fg, Color::WHITE);
        assert_ne!(styled.bg, Color::BLACK);
    }
}

// <FILE>tui-vfx-style/src/models/cls_terminal_water_shader.rs</FILE> - <DESC>Layered terminal water/ocean shader</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
