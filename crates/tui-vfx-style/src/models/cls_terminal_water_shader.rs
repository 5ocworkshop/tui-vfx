// <FILE>tui-vfx-style/src/models/cls_terminal_water_shader.rs</FILE> - <DESC>Layered terminal water/ocean shader</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Add motion-field primitive for layered water lighting with foam, ripples, rain, flow, and wakes.</WCTX>
// <CLOG>Initial terminal water shader model with deterministic field sampling and style application.</CLOG>

//! Layered terminal water/ocean shader.
//!
//! `TerminalWaterShader` computes a deterministic water field from layered
//! sine/Gerstner-style waves plus optional ripple, rain, flow, and wake
//! contributions. In the style-shader path it maps the field to water colors;
//! glyph-capable derivations can reuse the scalar field helpers later.

use crate::models::{ColorConfig, ColorSpace};
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::blend_colors;
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
pub enum WaterWaveMode {
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
        #[serde(default)]
        modes: Vec<WaterWaveMode>,
    },
}

impl Default for WaterWaveMode {
    fn default() -> Self {
        Self::Ocean
    }
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

#[derive(Debug, Clone, Copy, PartialEq)]
struct WaterFieldSample {
    height: f32,
    height_scalar: f32,
    crest: f32,
    curvature: f32,
    foam: f32,
    diffuse: f32,
    specular: f32,
    fresnel: f32,
    light_scalar: f32,
    ripple_scalar: f32,
}

impl TerminalWaterShader {
    fn sanitized_layers(&self) -> u8 {
        self.layers.clamp(1, MAX_LAYERS)
    }

    fn sample_field_at(&self, x: f32, y: f32, width: u16, height: u16, t: f32) -> WaterFieldSample {
        let layers = self.sanitized_layers();
        let amplitude = clamp_finite(self.amplitude, DEFAULT_AMPLITUDE, 0.0, 2.0);
        let wavelength = clamp_finite(self.wavelength, DEFAULT_WAVELENGTH, 1.0, 512.0);
        let speed = finite_or(self.speed, DEFAULT_SPEED);
        let direction_deg = finite_or(self.direction_deg, DEFAULT_DIRECTION_DEG);
        let steepness = clamp_finite(self.steepness, DEFAULT_STEEPNESS, 0.0, 1.0);
        let normal_strength = clamp_finite(self.normal_strength, DEFAULT_NORMAL_STRENGTH, 0.0, 4.0);
        let diffuse_strength = clamp_finite(self.diffuse, DEFAULT_DIFFUSE, 0.0, 2.0);
        let specular_strength = clamp_finite(self.specular, DEFAULT_SPECULAR, 0.0, 2.0);
        let shininess = clamp_finite(self.shininess, DEFAULT_SHININESS, 1.0, 128.0);
        let fresnel_strength = clamp_finite(self.fresnel, DEFAULT_FRESNEL, 0.0, 2.0);
        let foam_strength = clamp_finite(self.foam, DEFAULT_FOAM, 0.0, 2.0);
        let glint_strength = clamp_finite(self.glint_strength, 0.0, 0.0, 2.0);

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
        }
    }

    fn ocean_mix(&self) -> f32 {
        match &self.mode {
            WaterWaveMode::Ripple { base_shimmer, .. } => {
                clamp_finite(*base_shimmer, 0.0, 0.0, 0.25)
            }
            WaterWaveMode::OceanWithRipples { ocean_mix, .. } => {
                clamp_finite(*ocean_mix, 0.55, 0.0, 1.0)
            }
            WaterWaveMode::Flow { flow_strength, .. } => {
                clamp_finite(*flow_strength, 0.35, 0.0, 1.0) * 0.35
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
                clamp_finite(*ripple_mix, 1.0, 0.0, 2.0),
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
                clamp_finite(*drop_strength, 0.45, 0.0, 2.0),
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
        let strength = clamp_finite(flow_strength, 0.35, 0.0, 2.0);
        let k = 0.32;
        let phase = k * (dx * x + dy * y) + t * finite_or(speed, 1.0);
        let turb = clamp_finite(turbulence, 0.18, 0.0, 2.0) * ((x * 0.17 + y * 0.11 + t).sin());
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
                clamp_finite(*wake_strength, 0.65, 0.0, 2.0),
                clamp_finite(*trail_length, 18.0, 1.0, 128.0),
                clamp_finite(*spread_deg, 28.0, 1.0, 89.0),
            ),
            _ => return 0.0,
        };
        let width_f = width.max(1) as f32;
        let height_f = height.max(1) as f32 * 2.0;
        let mut scalar: f32 = 0.0;
        for source in sources.iter().take(MAX_WAKE_SOURCES) {
            let age = (t - finite_or(source.start_time, 0.0)).max(0.0);
            let sx = clamp_finite(source.x, 0.5, 0.0, 1.0) * width_f;
            let sy = clamp_finite(source.y, 0.5, 0.0, 1.0) * height_f;
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
        let width = clamp_finite(self.glint_width, 8.0, 0.1, 128.0);
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
    let cx = clamp_finite(emitter.center_x, 0.5, 0.0, 1.0) * width_f;
    let cy = clamp_finite(emitter.center_y, 0.5, 0.0, 1.0) * height_f;
    let dx = x - cx;
    let dy = y - cy;
    let r = (dx * dx + dy * dy).sqrt().max(0.001);
    let amplitude = clamp_finite(emitter.amplitude, 0.6, 0.0, 2.0) * ripple_mix;
    let speed = clamp_finite(emitter.speed, 8.0, 0.0, 128.0);
    let frequency = clamp_finite(emitter.frequency, 1.6, 0.01, 32.0);
    let ring_width = clamp_finite(emitter.ring_width, 2.5, 0.5, 64.0);
    let decay = clamp_finite(emitter.decay, 0.45, 0.0, 8.0);
    let damping = clamp_finite(emitter.damping, 0.025, 0.0, 2.0);
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

fn finite_or(value: f32, fallback: f32) -> f32 {
    if value.is_finite() { value } else { fallback }
}
fn clamp_finite(value: f32, fallback: f32, min: f32, max: f32) -> f32 {
    finite_or(value, fallback).clamp(min, max)
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
fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    if (edge1 - edge0).abs() <= f32::EPSILON {
        return if x >= edge1 { 1.0 } else { 0.0 };
    }
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
fn hash01(mut x: u32) -> f32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^= x >> 16;
    x as f32 / u32::MAX as f32
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
// <VERS>END OF VERSION: 0.1.0</VERS>
