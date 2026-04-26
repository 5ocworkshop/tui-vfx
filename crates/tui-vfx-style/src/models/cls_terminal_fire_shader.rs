// <FILE>tui-vfx-style/src/models/cls_terminal_fire_shader.rs</FILE> - <DESC>Procedural emissive terminal fire shader</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Document the live downstream consumers — StyleShader path for the color-blend recipes (5 mode debug fixtures) and glyph-rendering path via FireFieldSignal + SamplerRef::TerminalFire (1 glyph fixture, all six pass pipeline-validator).</WCTX>
// <CLOG>0.2.0: extend module docs with the two consumer paths now both shipped: the StyleShader::style_at color path (5 mode recipes) and the ScalarFieldGlyphFilter glyph path via FireFieldSignal + SamplerRef::TerminalFire (1 glyph recipe).</CLOG>

//! Procedural terminal fire shader.
//!
//! Fire is modeled as a thin emissive density field rather than a lit
//! surface: coherent rising turbulence produces temperature, density,
//! smoke, blue-core, and spark fields, then those fields map to emissive
//! terminal colors. Unlike [`super::cls_terminal_water_shader`] which
//! shades a height surface through normals, `TerminalFireShader` is itself
//! the light source — there is no `light_direction` parameter.
//!
//! All scalar/noise math is consumed from `mixed_signals` upstream (per
//! Intention 9). No private helper duplication.
//!
//! # Consumer paths
//!
//! Two shipped paths read this shader's per-cell field; both share one
//! tested math pipeline ([`TerminalFireShader::sample_field_at`]):
//!
//! 1. **Color blend** — implements [`crate::traits::StyleShader`]. Recipes
//!    invoke it via `{ "type": "terminal_fire", ... }` payloads on a
//!    `shader` pipeline step. Reference fixtures:
//!    `recipes/debug_recipes/shaders/primitives/shader_terminal_fire{,_candle,_campfire,_embers,_smoke_plume}_v3.json`.
//! 2. **Glyph render** — wrapped by
//!    [`super::cls_fire_field_signal::FireFieldSignal`] (a
//!    [`mixed_signals::traits::Signal`]) and consumed by
//!    [`tui_vfx_compositor::types::FilterSpec::ScalarFieldGlyph`] via
//!    [`tui_vfx_compositor::types::SamplerRef::TerminalFire`]. Recipes
//!    invoke it via `{ "type": "scalar_field_glyph", "sampler": { "kind":
//!    "terminal_fire", "shader": {...} }, "encoder": {...} }`. Reference
//!    fixture: `shader_terminal_fire_glyph_v3.json`.
//!
//! All six fixtures pass `pipeline-validator` on PROFILE/RENDER/SHADER/OUTPUT.

use crate::models::{ColorConfig, ColorSpace};
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::blend_colors;
use mixed_signals::math::{finite_or, finite_or_clamp, saturate, smoothstep};
use mixed_signals::noise::{fbm3, hash01};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

const MAX_SPARKS: u8 = 32;
const DEFAULT_BASE_WIDTH: f32 = 0.55;
const DEFAULT_MIN_WIDTH: f32 = 0.06;
const DEFAULT_RISE_SPEED: f32 = 2.2;
const DEFAULT_TURBULENCE: f32 = 1.0;
const DEFAULT_INTENSITY: f32 = 1.0;
const DEFAULT_DENSITY: f32 = 1.0;
const DEFAULT_COOLING: f32 = 0.78;
const DEFAULT_FLICKER: f32 = 0.18;
const DEFAULT_BLUE_CORE: f32 = 0.35;
const DEFAULT_WHITE_CORE: f32 = 0.35;
const DEFAULT_SMOKE: f32 = 0.35;
const DEFAULT_ASPECT: f32 = 1.0;
const DEFAULT_SPARK_SEED: u32 = 1;
const DEFAULT_SPARK_COUNT: u8 = 8;
const DEFAULT_SPARK_INTENSITY: f32 = 0.35;
const DEFAULT_SPARK_RISE: f32 = 1.2;
const DEFAULT_SPARK_DRIFT: f32 = 0.25;

/// Which style channels the fire emissive color blends into.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum FireApplyTo {
    Foreground,
    Background,
    #[default]
    Both,
}

/// Flame shape and behavior preset.
///
/// Modes share one field-sampling pipeline; per-mode differences are
/// applied as multiplier tunings (see [`FireMode::tuning`]) so every
/// mode benefits from the same tested math path.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum FireMode {
    /// General flame with wide base, torn top, smoke, and optional sparks.
    #[default]
    Flame,
    /// Narrow flame with stronger blue base and minimal smoke.
    Candle,
    /// Broad, smoky, turbulent flame for campfire/warning surfaces.
    Campfire,
    /// Low flame bed / ember glow. Good for status strips.
    Embers,
    /// Smoke-first plume; useful after flame has died down.
    SmokePlume,
}

#[derive(Debug, Clone, Copy)]
struct FireModeTuning {
    width_mul: f32,
    turbulence_mul: f32,
    smoke_mul: f32,
    blue_mul: f32,
    white_mul: f32,
    spark_mul: f32,
    height_gate: f32,
}

impl FireMode {
    fn tuning(&self) -> FireModeTuning {
        match self {
            FireMode::Flame => FireModeTuning {
                width_mul: 1.0,
                turbulence_mul: 1.0,
                smoke_mul: 1.0,
                blue_mul: 1.0,
                white_mul: 1.0,
                spark_mul: 1.0,
                height_gate: 1.0,
            },
            FireMode::Candle => FireModeTuning {
                width_mul: 0.35,
                turbulence_mul: 0.55,
                smoke_mul: 0.25,
                blue_mul: 1.8,
                white_mul: 1.2,
                spark_mul: 0.0,
                height_gate: 0.88,
            },
            FireMode::Campfire => FireModeTuning {
                width_mul: 1.35,
                turbulence_mul: 1.25,
                smoke_mul: 1.6,
                blue_mul: 0.45,
                white_mul: 0.9,
                spark_mul: 1.8,
                height_gate: 1.0,
            },
            FireMode::Embers => FireModeTuning {
                width_mul: 1.25,
                turbulence_mul: 0.35,
                smoke_mul: 0.45,
                blue_mul: 0.0,
                white_mul: 0.35,
                spark_mul: 0.7,
                height_gate: 0.28,
            },
            FireMode::SmokePlume => FireModeTuning {
                width_mul: 1.0,
                turbulence_mul: 0.8,
                smoke_mul: 2.0,
                blue_mul: 0.0,
                white_mul: 0.0,
                spark_mul: 0.0,
                height_gate: 1.0,
            },
        }
    }
}

/// Author-configurable color palette for fire output.
///
/// Defaults approximate ANSI 256-color flame ramps so authors can override
/// per-theme without rewriting the shader.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct FirePalette {
    pub blue_core: ColorConfig,
    pub white_core: ColorConfig,
    pub yellow: ColorConfig,
    pub orange: ColorConfig,
    pub red: ColorConfig,
    pub smoke: ColorConfig,
}

impl Default for FirePalette {
    fn default() -> Self {
        Self {
            blue_core: ColorConfig::Rgb {
                r: 0,
                g: 215,
                b: 255,
            },
            white_core: ColorConfig::White,
            yellow: ColorConfig::Rgb {
                r: 255,
                g: 215,
                b: 0,
            },
            orange: ColorConfig::Rgb {
                r: 255,
                g: 95,
                b: 0,
            },
            red: ColorConfig::Rgb { r: 175, g: 0, b: 0 },
            smoke: ColorConfig::Rgb {
                r: 88,
                g: 88,
                b: 88,
            },
        }
    }
}

/// Sparse spark/ember settings.
///
/// Sparks are deterministic pseudo-particles derived from `seed`,
/// trajectory index, position, and time. No mutable global state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct FireSparkConfig {
    #[config(default = 1)]
    pub seed: u32,
    /// Number of synthetic spark trajectories. Clamped to `0..=32`.
    #[config(default = 8)]
    pub count: u8,
    /// Spark brightness multiplier.
    #[config(default = 0.35)]
    pub intensity: f32,
    /// Upward spark speed.
    #[config(default = 1.2)]
    pub rise_speed: f32,
    /// Horizontal spark drift.
    #[config(default = 0.25)]
    pub drift: f32,
}

impl Default for FireSparkConfig {
    fn default() -> Self {
        Self {
            seed: DEFAULT_SPARK_SEED,
            count: DEFAULT_SPARK_COUNT,
            intensity: DEFAULT_SPARK_INTENSITY,
            rise_speed: DEFAULT_SPARK_RISE,
            drift: DEFAULT_SPARK_DRIFT,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct FireSparkParams {
    seed: u32,
    count: u8,
    intensity: f32,
    rise_speed: f32,
    drift: f32,
}

impl FireSparkConfig {
    fn sanitized(&self) -> FireSparkParams {
        FireSparkParams {
            seed: self.seed,
            count: self.count.min(MAX_SPARKS),
            intensity: finite_or_clamp(self.intensity, 0.0, 2.0, DEFAULT_SPARK_INTENSITY),
            rise_speed: finite_or_clamp(self.rise_speed, 0.0, 4.0, DEFAULT_SPARK_RISE),
            drift: finite_or_clamp(self.drift, 0.0, 2.0, DEFAULT_SPARK_DRIFT),
        }
    }
}

/// Procedural emissive terminal fire shader.
///
/// See module docs for the field model. All `f32` parameters are
/// sanitized at sample time; out-of-range or non-finite recipe values are
/// replaced with safe defaults rather than producing `NaN` output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct TerminalFireShader {
    #[serde(default)]
    pub mode: FireMode,
    #[serde(default)]
    pub apply_to: FireApplyTo,
    /// Horizontal aspect correction for terminal cells.
    #[config(default = 1.0)]
    pub aspect: f32,
    /// Overall flame width at the base.
    #[config(default = 0.55)]
    pub base_width: f32,
    /// Minimum flame width near the top.
    #[config(default = 0.06)]
    pub min_width: f32,
    /// Wind lean; negative leans left, positive leans right.
    #[config(default = 0.0)]
    pub wind: f32,
    /// Upward noise/animation speed.
    #[config(default = 2.2)]
    pub rise_speed: f32,
    /// Multiplier for domain warp.
    #[config(default = 1.0)]
    pub turbulence: f32,
    /// Global emission/style blend multiplier.
    #[config(default = 1.0)]
    pub intensity: f32,
    /// Global opacity/density multiplier.
    #[config(default = 1.0)]
    pub density: f32,
    /// Cooling with height.
    #[config(default = 0.78)]
    pub cooling: f32,
    /// Amount of coherent flicker added to temperature.
    #[config(default = 0.18)]
    pub flicker_strength: f32,
    /// Blue base/reaction-zone contribution.
    #[config(default = 0.35)]
    pub blue_core_strength: f32,
    /// Hot white core contribution.
    #[config(default = 0.35)]
    pub white_core_strength: f32,
    /// Smoke contribution.
    #[config(default = 0.35)]
    pub smoke_strength: f32,
    #[serde(default)]
    pub sparks: FireSparkConfig,
    #[serde(default)]
    pub palette: FirePalette,
}

impl Default for TerminalFireShader {
    fn default() -> Self {
        Self {
            mode: FireMode::default(),
            apply_to: FireApplyTo::default(),
            aspect: DEFAULT_ASPECT,
            base_width: DEFAULT_BASE_WIDTH,
            min_width: DEFAULT_MIN_WIDTH,
            wind: 0.0,
            rise_speed: DEFAULT_RISE_SPEED,
            turbulence: DEFAULT_TURBULENCE,
            intensity: DEFAULT_INTENSITY,
            density: DEFAULT_DENSITY,
            cooling: DEFAULT_COOLING,
            flicker_strength: DEFAULT_FLICKER,
            blue_core_strength: DEFAULT_BLUE_CORE,
            white_core_strength: DEFAULT_WHITE_CORE,
            smoke_strength: DEFAULT_SMOKE,
            sparks: FireSparkConfig::default(),
            palette: FirePalette::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct FireParams {
    aspect: f32,
    base_width: f32,
    min_width: f32,
    wind: f32,
    rise_speed: f32,
    turbulence: f32,
    intensity: f32,
    density: f32,
    cooling: f32,
    flicker_strength: f32,
    blue_core_strength: f32,
    white_core_strength: f32,
    smoke_strength: f32,
}

impl TerminalFireShader {
    fn params(&self) -> FireParams {
        let base_width = finite_or_clamp(self.base_width, 0.05, 2.0, DEFAULT_BASE_WIDTH);
        let min_width =
            finite_or_clamp(self.min_width, 0.005, 0.5, DEFAULT_MIN_WIDTH).min(base_width);
        FireParams {
            aspect: finite_or_clamp(self.aspect, 0.25, 4.0, DEFAULT_ASPECT),
            base_width,
            min_width,
            wind: finite_or_clamp(self.wind, -1.0, 1.0, 0.0),
            rise_speed: finite_or_clamp(self.rise_speed, 0.0, 12.0, DEFAULT_RISE_SPEED),
            turbulence: finite_or_clamp(self.turbulence, 0.0, 2.0, DEFAULT_TURBULENCE),
            intensity: finite_or_clamp(self.intensity, 0.0, 2.0, DEFAULT_INTENSITY),
            density: finite_or_clamp(self.density, 0.0, 2.0, DEFAULT_DENSITY),
            cooling: finite_or_clamp(self.cooling, 0.0, 2.0, DEFAULT_COOLING),
            flicker_strength: finite_or_clamp(self.flicker_strength, 0.0, 1.0, DEFAULT_FLICKER),
            blue_core_strength: finite_or_clamp(
                self.blue_core_strength,
                0.0,
                1.0,
                DEFAULT_BLUE_CORE,
            ),
            white_core_strength: finite_or_clamp(
                self.white_core_strength,
                0.0,
                1.0,
                DEFAULT_WHITE_CORE,
            ),
            smoke_strength: finite_or_clamp(self.smoke_strength, 0.0, 1.0, DEFAULT_SMOKE),
        }
    }
}

/// Per-cell sample of the fire field.
///
/// Visibility is `pub(crate)` so the sibling
/// [`crate::models::cls_fire_field_signal`] wrapper can route it through
/// the [`mixed_signals::traits::Signal`] surface. Fields stay
/// `pub(crate)` until probe/trace consumers need them publicly.
///
/// Note: unlike [`super::cls_terminal_water_shader::WaterFieldSample`],
/// this struct does NOT cache analytic slopes. Fire's combined
/// pipeline (mask × density × smoothsteps × exp) does not yield free
/// gradients during normal evaluation, so the sibling signal wrapper
/// uses [`mixed_signals::traits::SignalWithSlope`]'s default
/// central-differencing impl (3 evaluations) rather than 1-eval
/// analytic shortcut. If profiling later shows this is the hot path,
/// caching forward differences computed inside `sample_field_at` is
/// the recommended optimization.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct FireSample {
    pub(crate) temperature: f32,
    pub(crate) density: f32,
    pub(crate) smoke: f32,
    pub(crate) blue_core: f32,
    pub(crate) white_core: f32,
    pub(crate) sparks: f32,
    /// Combined emission/intensity in `0.0..=1.0`. This is the value
    /// [`crate::models::cls_fire_field_signal::FireFieldSignal`] emits
    /// as its `Signal` output.
    pub(crate) intensity: f32,
    pub(crate) radius: f32,
    pub(crate) mask: f32,
}

impl TerminalFireShader {
    /// Evaluate the fire field at one cell.
    ///
    /// Coordinate convention: `(x, y)` are raw cell-space (column, row),
    /// `width`/`height` are widget dimensions in cells, `t` is time in
    /// seconds. Internal normalization maps `x` to `[-aspect, +aspect]`
    /// and `y` to `[0.0, 1.0]` with `y=0` at the flame base.
    pub(crate) fn sample_field_at(
        &self,
        x: f32,
        y: f32,
        width: u16,
        height: u16,
        t: f32,
    ) -> FireSample {
        let p = self.params();
        let tuning = self.mode.tuning();
        let seed = self.sparks.seed;

        let (nx, ny) = normalized_fire_coord(x, y, width, height, p.aspect);
        let time = finite_or(t, 0.0) * p.rise_speed.max(0.001);

        let nl = fbm3(
            seed,
            1.5 * nx,
            2.0 * ny - 0.7 * time,
            0.11 * time,
            4,
            0.5,
            2.0,
        );
        let nm = fbm3(
            seed,
            4.0 * nx,
            6.0 * ny - 2.2 * time,
            0.23 * time,
            5,
            0.5,
            2.0,
        );
        let nf = fbm3(
            seed,
            12.0 * nx,
            16.0 * ny - 6.0 * time,
            0.37 * time,
            4,
            0.5,
            2.0,
        );

        let turbulence = (p.turbulence * tuning.turbulence_mul).clamp(0.0, 2.5);
        let xw = nx + turbulence * (0.16 * ny * (1.0 - ny) * nl + 0.05 * ny * nm);
        let yw = (ny + turbulence * (0.05 * (1.0 - ny) * nm + 0.015 * nf)).clamp(0.0, 1.0);

        let base_width = p.base_width * tuning.width_mul;
        let center = p.wind * yw.powf(1.7)
            + 0.08 * yw * (3.1 * yw - 1.7 * time).sin()
            + 0.04 * yw * (8.3 * yw + 2.6 * time).sin()
            + 0.06 * yw * fbm3(seed, 1.2 * yw, -0.35 * time, 0.0, 4, 0.5, 2.0);
        let width_at = (p.min_width
            + base_width * (1.0 - yw).powf(0.85)
            + 0.03 * fbm3(seed, 2.0 * yw, -0.8 * time, 0.0, 4, 0.5, 2.0))
        .max(0.001);
        let radius = ((xw - center).abs() / width_at).max(0.0);

        let height_mask = 1.0 - smoothstep(tuning.height_gate, 1.0, yw);
        let mask = ((-radius.powf(2.0 + 1.8 * yw)).exp()
            * (1.0 - smoothstep(0.78, 1.0, yw))
            * height_mask)
            .clamp(0.0, 1.0);

        let flicker = fbm3(seed, 0.0, -1.8 * time, 0.6 * time, 4, 0.5, 2.0);
        let temperature = saturate(
            1.35 * mask.powf(0.72) + 0.28 * nm + 0.10 * nf - p.cooling * yw
                + p.flicker_strength * flicker,
        );

        let mut density = smoothstep(
            0.08,
            0.42,
            temperature + 0.18 * mask - 0.10 * radius + 0.08 * nm,
        );
        let tongue = smoothstep(0.15, 0.65, mask + 0.35 * nm - 0.50 * yw);
        density = (density * tongue * p.density).clamp(0.0, 1.0);

        let smoke_drift =
            0.35 * fbm3(
                seed,
                2.0 * (nx + 0.15 * (1.2 * time + 3.0 * ny).sin()),
                3.0 * (ny - 0.25 * time),
                0.08 * time,
                4,
                0.5,
                2.0,
            ) * smoothstep(0.65, 1.0, ny);
        let smoke = saturate(
            (smoothstep(0.50, 0.95, yw)
                * (1.0 - smoothstep(0.18, 0.58, temperature))
                * density
                * (0.65 + 0.35 * nl)
                + smoke_drift.max(0.0))
                * p.smoke_strength
                * tuning.smoke_mul,
        );

        let blue_core = ((1.0 - smoothstep(0.02, 0.16, yw))
            * (1.0 - smoothstep(0.0, 0.42, radius))
            * (1.0 - smoothstep(0.35, 0.80, smoke))
            * p.blue_core_strength
            * tuning.blue_mul)
            .clamp(0.0, 1.0);

        let white_core = (density
            * smoothstep(0.72, 0.95, temperature)
            * (1.0 - smoothstep(0.0, 0.32, radius))
            * p.white_core_strength
            * tuning.white_mul)
            .clamp(0.0, 1.0);

        let sparks = self.spark_field(xw, yw, time) * tuning.spark_mul * smoothstep(0.12, 0.95, yw);

        let intensity = saturate(
            (density * (0.20 + 0.80 * temperature.powf(0.65)) + 0.45 * blue_core - 0.25 * smoke
                + 0.35 * white_core
                + 0.8 * sparks)
                * p.intensity,
        );

        FireSample {
            temperature,
            density,
            smoke,
            blue_core,
            white_core,
            sparks,
            intensity,
            radius,
            mask,
        }
    }

    fn sample_field_for_ctx(&self, ctx: &ShaderContext) -> FireSample {
        self.sample_field_at(
            ctx.local_x as f32,
            ctx.local_y as f32,
            ctx.width,
            ctx.height,
            ctx.t as f32,
        )
    }

    fn spark_field(&self, x: f32, y: f32, time: f32) -> f32 {
        let cfg = self.sparks.sanitized();
        if cfg.count == 0 || cfg.intensity <= 0.0 {
            return 0.0;
        }
        let mut sum = 0.0;
        for j in 0..cfg.count {
            let h0 = hash01(cfg.seed ^ (j as u32).wrapping_mul(0x9E37_79B9));
            let h1 = hash01(cfg.seed ^ (j as u32).wrapping_mul(0x85EB_CA6B));
            let h2 = hash01(cfg.seed ^ (j as u32).wrapping_mul(0xC2B2_AE35));
            let phase = h0 * std::f32::consts::TAU;
            let local_t = (time * cfg.rise_speed + h1).fract();
            let sx = (h0 * 2.0 - 1.0) * 0.35
                + cfg.drift * 0.08 * (time * (2.0 + h2 * 3.0) + phase).sin();
            let sy = local_t;
            let dx = x - sx;
            let dy = y - sy;
            let e = (-(dx * dx / 0.0036 + dy * dy / 0.0016)).exp();
            sum += e;
        }
        saturate(sum * cfg.intensity)
    }

    fn color_for_sample(&self, sample: FireSample) -> Color {
        let blue_core: Color = self.palette.blue_core.into();
        let white_core: Color = self.palette.white_core.into();
        let yellow: Color = self.palette.yellow.into();
        let orange: Color = self.palette.orange.into();
        let red: Color = self.palette.red.into();
        let smoke: Color = self.palette.smoke.into();

        if sample.blue_core > 0.25 {
            return blend_colors(
                blue_core,
                white_core,
                (sample.blue_core * sample.intensity).clamp(0.0, 1.0),
                ColorSpace::Rgb,
            );
        }

        if sample.smoke > 0.18 && sample.smoke > sample.density * sample.temperature {
            return smoke;
        }

        if sample.white_core > 0.20 || sample.temperature > 0.82 {
            return blend_colors(
                yellow,
                white_core,
                smoothstep(0.82, 1.0, sample.temperature),
                ColorSpace::Rgb,
            );
        }

        if sample.temperature > 0.58 {
            return blend_colors(
                orange,
                yellow,
                smoothstep(0.58, 0.82, sample.temperature),
                ColorSpace::Rgb,
            );
        }

        if sample.temperature > 0.32 {
            return blend_colors(
                red,
                orange,
                smoothstep(0.32, 0.58, sample.temperature),
                ColorSpace::Rgb,
            );
        }

        if sample.density > 0.05 {
            return red;
        }

        if sample.smoke > 0.10 {
            return smoke;
        }

        Color::TRANSPARENT
    }

    fn apply_fire_color(&self, base: Style, fire: Color, sample: FireSample) -> Style {
        let mut out = base;
        let alpha = (sample.intensity * sample.density.max(0.20)).clamp(0.0, 1.0);
        match self.apply_to {
            FireApplyTo::Foreground => {
                if base.fg != Color::TRANSPARENT {
                    out.fg = blend_colors(base.fg, fire, alpha, ColorSpace::Rgb);
                }
            }
            FireApplyTo::Background => {
                if base.bg != Color::TRANSPARENT {
                    out.bg = blend_colors(base.bg, fire, alpha * 0.75, ColorSpace::Rgb);
                }
            }
            FireApplyTo::Both => {
                if base.fg != Color::TRANSPARENT {
                    out.fg = blend_colors(base.fg, fire, alpha, ColorSpace::Rgb);
                }
                if base.bg != Color::TRANSPARENT {
                    out.bg = blend_colors(base.bg, fire, alpha * 0.65, ColorSpace::Rgb);
                }
            }
        }
        out
    }
}

/// Map raw cell coords + widget dimensions into flame-local coords.
///
/// `nx` is `[-aspect, +aspect]`, `ny` is `[0.0, 1.0]` with `y=0` at the
/// flame base (visually the bottom of the widget).
fn normalized_fire_coord(x: f32, y: f32, width: u16, height: u16, aspect: f32) -> (f32, f32) {
    let w = width.max(1) as f32;
    let h = height.max(1) as f32;
    let nx = if w <= 1.0 {
        0.0
    } else {
        (2.0 * x / (w - 1.0) - 1.0) * aspect
    };
    let ny = if h <= 1.0 { 0.0 } else { 1.0 - y / (h - 1.0) };
    (nx, ny.clamp(0.0, 1.0))
}

impl StyleShader for TerminalFireShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let sample = self.sample_field_for_ctx(ctx);
        if sample.intensity <= 0.001 {
            return base;
        }
        let fire = self.color_for_sample(sample);
        if fire == Color::TRANSPARENT {
            return base;
        }
        self.apply_fire_color(base, fire, sample)
    }

    fn name(&self) -> &'static str {
        "TerminalFire"
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
        let shader = TerminalFireShader::default();
        assert_eq!(shader.mode, FireMode::Flame);
        assert_eq!(shader.apply_to, FireApplyTo::Both);
        assert!(shader.base_width > 0.0);
        assert!(shader.rise_speed > 0.0);
        assert_eq!(shader.sparks.count, DEFAULT_SPARK_COUNT);
    }

    #[test]
    fn default_sample_is_finite_and_bounded() {
        let shader = TerminalFireShader::default();
        let sample = shader.sample_field_at(8.0, 9.0, 32, 12, 0.25);
        assert!(sample.temperature.is_finite());
        assert!(sample.density.is_finite());
        assert!(sample.smoke.is_finite());
        assert!(sample.intensity.is_finite());
        assert!((0.0..=1.0).contains(&sample.temperature));
        assert!((0.0..=1.0).contains(&sample.density));
        assert!((0.0..=1.0).contains(&sample.smoke));
        assert!((0.0..=1.0).contains(&sample.blue_core));
        assert!((0.0..=1.0).contains(&sample.white_core));
        assert!((0.0..=1.0).contains(&sample.intensity));
    }

    #[test]
    fn intensity_varies_by_position() {
        let shader = TerminalFireShader::default();
        let centerline_low = shader.sample_field_at(16.0, 11.0, 32, 12, 0.5).intensity;
        let edge_high = shader.sample_field_at(0.0, 0.0, 32, 12, 0.5).intensity;
        assert!(
            (centerline_low - edge_high).abs() > 0.001,
            "fire field should vary by position; got centerline={centerline_low}, corner={edge_high}"
        );
    }

    #[test]
    fn intensity_varies_by_time() {
        let shader = TerminalFireShader::default();
        let early = shader.sample_field_at(8.0, 8.0, 32, 12, 0.0).intensity;
        let late = shader.sample_field_at(8.0, 8.0, 32, 12, 0.75).intensity;
        assert!(
            (early - late).abs() > 0.001,
            "fire field should animate; got early={early}, late={late}"
        );
    }

    #[test]
    fn smoke_does_not_overwhelm_hot_core() {
        // Sample near the bottom centerline where temperature is highest.
        let shader = TerminalFireShader::default();
        let sample = shader.sample_field_at(16.0, 11.0, 32, 12, 0.2);
        if sample.temperature > 0.58 && sample.density > 0.10 {
            assert!(
                sample.smoke <= sample.density * sample.temperature || sample.blue_core > 0.0,
                "smoke must not win over hot/dense core: smoke={}, d*t={}, blue={}",
                sample.smoke,
                sample.density * sample.temperature,
                sample.blue_core
            );
        }
    }

    #[test]
    fn invalid_params_are_sanitized_without_nan() {
        let shader = TerminalFireShader {
            base_width: f32::NAN,
            min_width: f32::INFINITY,
            wind: f32::NAN,
            rise_speed: -10.0,
            turbulence: f32::INFINITY,
            intensity: f32::NAN,
            density: -1.0,
            cooling: f32::NAN,
            flicker_strength: 99.0,
            blue_core_strength: f32::NAN,
            white_core_strength: f32::INFINITY,
            smoke_strength: f32::NAN,
            ..TerminalFireShader::default()
        };
        let sample = shader.sample_field_at(4.0, 4.0, 32, 12, 0.5);
        assert!(sample.temperature.is_finite());
        assert!(sample.density.is_finite());
        assert!(sample.intensity.is_finite());
        assert!(sample.smoke.is_finite());
    }

    #[test]
    fn style_at_changes_visible_style() {
        let shader = TerminalFireShader::default();
        let ctx = ctx_at(10, 11, 0.4);
        let base = Style {
            fg: Color::WHITE,
            bg: Color::BLACK,
            ..Style::default()
        };
        let styled = shader.style_at(&ctx, base);
        // At the flame base on the centerline, intensity should be non-trivial
        // for the default flame, so style should differ from base.
        assert_ne!(styled, base);
    }

    #[test]
    fn candle_mode_emphasizes_blue_core_at_base() {
        let candle = TerminalFireShader {
            mode: FireMode::Candle,
            blue_core_strength: 0.8,
            ..TerminalFireShader::default()
        };
        let sample = candle.sample_field_at(16.0, 11.0, 32, 12, 0.1);
        let flame = TerminalFireShader::default().sample_field_at(16.0, 11.0, 32, 12, 0.1);
        // Candle's blue_mul is 1.8, default flame's is 1.0 — candle should
        // produce stronger blue at the base for the same blue_core_strength.
        // Use shader with explicit matching strength to compare modes only.
        let flame_strong = TerminalFireShader {
            blue_core_strength: 0.8,
            ..TerminalFireShader::default()
        }
        .sample_field_at(16.0, 11.0, 32, 12, 0.1);
        assert!(
            sample.blue_core >= flame_strong.blue_core,
            "candle blue_core {} should be >= flame blue_core {}",
            sample.blue_core,
            flame_strong.blue_core
        );
        // Spark count is zero in candle tuning.
        assert_eq!(sample.sparks, 0.0);
        let _ = flame; // suppress unused warning if test is removed
    }

    #[test]
    fn smoke_plume_mode_suppresses_blue_and_white_cores() {
        let plume = TerminalFireShader {
            mode: FireMode::SmokePlume,
            blue_core_strength: 1.0,
            white_core_strength: 1.0,
            smoke_strength: 1.0,
            ..TerminalFireShader::default()
        };
        let sample = plume.sample_field_at(16.0, 11.0, 32, 12, 0.5);
        assert_eq!(sample.blue_core, 0.0, "smoke_plume must zero blue core");
        assert_eq!(sample.white_core, 0.0, "smoke_plume must zero white core");
    }

    #[test]
    fn deserializes_from_recipe_shape() {
        let json = serde_json::json!({
            "mode": { "mode": "candle" },
            "apply_to": "both",
            "aspect": 1.0,
            "base_width": 0.18,
            "min_width": 0.035,
            "wind": 0.02,
            "rise_speed": 1.4,
            "turbulence": 0.55,
            "intensity": 0.85,
            "density": 1.0,
            "cooling": 0.78,
            "flicker_strength": 0.18,
            "blue_core_strength": 0.75,
            "white_core_strength": 0.45,
            "smoke_strength": 0.08,
            "sparks": { "seed": 7, "count": 0, "intensity": 0.35, "rise_speed": 1.2, "drift": 0.25 },
            "palette": {
                "blue_core": { "type": "rgb", "r": 0, "g": 215, "b": 255 },
                "white_core": { "type": "white" },
                "yellow": { "type": "rgb", "r": 255, "g": 215, "b": 0 },
                "orange": { "type": "rgb", "r": 255, "g": 95, "b": 0 },
                "red": { "type": "rgb", "r": 175, "g": 0, "b": 0 },
                "smoke": { "type": "rgb", "r": 88, "g": 88, "b": 88 }
            }
        });
        let shader: TerminalFireShader = serde_json::from_value(json).expect("valid candle recipe");
        assert_eq!(shader.mode, FireMode::Candle);
        assert_eq!(shader.sparks.count, 0);
    }

    #[test]
    fn rejects_unknown_fields() {
        let json = serde_json::json!({
            "mode": { "mode": "flame" },
            "not_a_field": true
        });
        let err = serde_json::from_value::<TerminalFireShader>(json).unwrap_err();
        assert!(
            err.to_string().contains("unknown field"),
            "expected unknown-field error, got: {err}"
        );
    }
}

// <FILE>tui-vfx-style/src/models/cls_terminal_fire_shader.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
