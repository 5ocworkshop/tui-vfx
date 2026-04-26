<!-- <FILE>docs/design/tui-vfx-terminal-fire-shader-plan.md</FILE> - <DESC>Implementation plan for a terminal fire/flame shader primitive.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Add a discoverable implementation plan for procedural terminal fire next to the terminal water shader plan.</WCTX> -->
<!-- <CLOG>0.1.0: initial fire shader plan with procedural density/temperature fields, smoke, blue core, sparks, stateful upgrade path, and shared helper recommendations with water.</CLOG> -->

# Terminal Fire / Flame Shader Implementation Plan

Date: 2026-04-26
Repository: `/usr/projects/tui-vfx`
Related recipe repository: `/usr/projects/tui-vfx-recipes`
Status: implementation-ready planning artifact; no source files changed by this planning pass.
Audience: junior Rust developer implementing under existing `tui-vfx` conventions.
Related plan: [`tui-vfx-terminal-water-shader-plan.md`](tui-vfx-terminal-water-shader-plan.md)

## 0. One-paragraph goal

Add a new additive spatial shader named `terminal_fire` that renders fire as a thin volumetric/emissive terminal field. Unlike `terminal_water`, which shades a height surface through normals, `terminal_fire` synthesizes coherent fields for temperature/brightness `T(x,y,t)`, density/opacity `D(x,y,t)`, smoke/soot `S(x,y,t)`, optional blue reaction-zone core `B(x,y,t)`, and sparse sparks/embers `E(x,y,t)`. The style-shader path should apply emissive color/light styling without assuming glyph mutation. A later glyph-capable path can reuse the same field math to map brightness/density to ASCII/block/braille ramps.

## 1. Mental model

Water:

```text
surface height -> slope/normal -> diffuse/specular/Fresnel lighting -> style/glyph
```

Fire:

```text
fuel/base + rising turbulence -> temperature/density/smoke -> emissive color + opacity -> style/glyph
```

The most important implementation difference is that fire has no light direction in the water sense. Fire is itself the light source. Avoid adding diffuse/specular controls to `terminal_fire`; use temperature, density, smoke, blue core, white core, glow, flicker, sparks, and palette controls instead.

## 2. Non-goals and constraints

### Non-goals / defer-unless-supported

Do **not** implement these in the first style-shader pass unless the codebase already exposes a clear cell/glyph mutation or stateful simulation hook:

- full CFD / pressure projection / stable-fluids simulation;
- persistent per-cell state grids in `StyleShader::style_at`;
- real blackbody color physics;
- particle systems that require global mutable shader state;
- glyph replacement inside `TerminalFireShader` if the current style path still returns only `Style`;
- new dependencies.

### Contract constraints

Follow repository AGENTS guidance:

- Additive changes only; existing public contracts and recipe behavior must not change.
- No new dependencies without explicit approval.
- Keep files cohesive; do not split only to satisfy line-count aesthetics.
- Add meaningful tests before or alongside implementation.
- Update docs/schema/tooling outputs after adding public shader surface.

### Current API constraint

Read before implementing:

```bash
sed -n '1,220p' crates/tui-vfx-style/src/traits/tr_style_shader.rs
sed -n '1,220p' crates/tui-vfx-style/src/traits/cls_shader_context.rs
```

If the active API remains:

```rust
fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style
```

then the shader can change foreground/background/modifiers, but cannot replace the cell glyph. Therefore:

- style-shader path: `TerminalFireShader` computes fire fields and applies emissive color/style;
- glyph-capable path: a filter/content primitive reuses the same fire sample math for ramped glyph output.

Do not expose a `glyph_ramp` field in the style-shader schema if it cannot affect output. If docs mention glyph ramps, mark them as derivation guidance for a glyph-capable primitive.

## 3. Initial repo orientation commands

Run these first from `/usr/projects/tui-vfx`:

```bash
cd /usr/projects/tui-vfx
ofpf-orientation --root /usr/projects/tui-vfx
ofpf-hotspots --root /usr/projects/tui-vfx
```

Then inspect these exact files:

```bash
sed -n '1,260p' crates/tui-vfx-style/src/models/cls_pulse_wave_shader.rs
sed -n '1,260p' crates/tui-vfx-style/src/models/cls_radial_spiral_shader.rs
sed -n '1,260p' crates/tui-vfx-style/src/models/cls_neon_flicker_shader.rs
sed -n '1,260p' crates/tui-vfx-style/src/models/cls_glow_shader.rs
sed -n '1,760p' crates/tui-vfx-style/src/models/cls_spatial_shader_type.rs
sed -n '1,220p' crates/tui-vfx-style/src/models/mod.rs
sed -n '1,260p' crates/tui-vfx-style/src/models/v3/enum_vfx_motion_field_behavior.rs
sed -n '1,260p' crates/tui-vfx-style/src/models/v3/enum_vfx_stochastic_texture_behavior.rs
sed -n '1,260p' crates/tui-vfx-style/src/models/v3/cls_vfx_motion_field_shader.rs
sed -n '1,560p' crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs
sed -n '380,470p' xtask/src/docs/effect_metadata.rs
```

Useful grep:

```bash
rg -n "PulseWave|RadialSpiral|NeonFlicker|Glow|Stochastic|MotionField|SpatialShaderType|key_parameters|terse_description" crates/tui-vfx-style xtask/src/docs
```

## 4. Existing patterns to follow

### `PulseWaveShader` and `RadialSpiralShader`

Reference files:

- `crates/tui-vfx-style/src/models/cls_pulse_wave_shader.rs`
- `crates/tui-vfx-style/src/models/cls_radial_spiral_shader.rs`

Copy these patterns:

- derives: `Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema`;
- `#[serde(deny_unknown_fields)]`;
- defaults through `impl Default` plus `#[config(default = ...)]` where appropriate;
- private math helpers that are deterministic and easy to unit test;
- `StyleShader for ...` implementation preserving transparent channels;
- unit tests in the same file for bounded field values and visible style changes.

### `NeonFlickerShader`

Reference: `crates/tui-vfx-style/src/models/cls_neon_flicker_shader.rs`

Use it for deterministic seed/flicker conventions, but do **not** copy its high-frequency random flicker as the primary fire detail. Fire should use coherent rising noise, not frame-to-frame static.

### `GlowShader`

Reference: `crates/tui-vfx-style/src/models/cls_glow_shader.rs`

Use it for emissive blending intuition and transparent-channel handling. Fire may internally compute `I` (emission/brightness) and blend a palette color into fg/bg by `I * intensity`.

### V3 family placement

`terminal_fire` is a dynamic procedural field. Prefer the V3 motion-field family unless the implementation team decides to create a new volumetric/emissive family later.

Recommended classification:

```rust
VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::MotionField(_))
```

Rationale: fire is an animated spatial field, and the current V3 family set already has motion-field/stochastic/material-light buckets. It is not a static material light like bevel/glow, and it is more structured than stochastic sparkle. If the V3 taxonomy grows a `volumetric_field` or `emissive_field` family later, `terminal_fire` can migrate through the V3 compatibility layer.

## 5. Likely files/modules to touch

### Style crate

Add:

```text
crates/tui-vfx-style/src/models/cls_terminal_fire_shader.rs
```

Modify:

```text
crates/tui-vfx-style/src/models/mod.rs
crates/tui-vfx-style/src/models/cls_spatial_shader_type.rs
crates/tui-vfx-style/src/models/v3/enum_vfx_motion_field_behavior.rs
crates/tui-vfx-style/src/models/v3/cls_vfx_motion_field_shader.rs
crates/tui-vfx-style/src/models/v3/fnc_lower_legacy_spatial_shader.rs
crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs
crates/tui-vfx-style/src/models/v3/test_vfx_motion_field_shader.rs
crates/tui-vfx-style/src/models/v3/test_try_lower_v3_spatial_shader_family.rs
```

Consider adding shared helper modules if implementing fire and water together (see Section 13):

```text
crates/tui-vfx-style/src/utils/fnc_scalar_math.rs
crates/tui-vfx-style/src/utils/fnc_procedural_noise.rs
crates/tui-vfx-style/src/utils/fnc_glyph_ramp.rs        # only if glyph-capable path lands
```

### Tests

Prefer same-file unit tests first. Add integration-style tests only if the project already has a matching pattern:

```text
crates/tui-vfx-style/tests/models/test_cls_terminal_fire_shader.rs
```

### Docs/tooling

Modify:

```text
xtask/src/docs/effect_metadata.rs
docs/design/tui-vfx-v3-schema-draft.json
docs/design/tui-vfx-v3-schema-overview.md
docs/design/tui-vfx-v3-recipe-vocabulary.md
docs/design/tui-vfx-v3-capability-catalog.md
docs/INDEX.md
docs/design/tui-vfx-v3-INDEX.md
```

Regenerate generated docs if that is the established workflow:

```bash
cargo run -p xtask -- docs
```

If the exact command differs, inspect `xtask` and existing docs generation scripts before inventing a new command.

### Recipe repository

Add debug recipes under the existing shader primitive area:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/shaders/primitives/shader_terminal_fire_v3.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/shaders/primitives/shader_terminal_fire_candle_v3.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/shaders/primitives/shader_terminal_fire_smoke_sparks_v3.json
```

Validate with the sibling repo’s established validator/preview commands.

## 6. Proposed Rust types

### Main shader struct

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct TerminalFireShader {
    /// Flame shape and behavior preset.
    #[serde(default)]
    pub mode: FireMode,

    /// Which style channels receive the emissive fire color.
    #[serde(default)]
    pub apply_to: FireApplyTo,

    /// Horizontal aspect correction for terminal cells.
    #[config(default = 1.0)]
    pub aspect: f32,

    /// Overall flame width at the base. Clamp 0.05..=2.0.
    #[config(default = 0.55)]
    pub base_width: f32,

    /// Minimum flame width near the top. Clamp 0.005..=0.5.
    #[config(default = 0.06)]
    pub min_width: f32,

    /// Wind lean; negative leans left, positive leans right. Clamp -1..=1.
    #[config(default = 0.0)]
    pub wind: f32,

    /// Upward noise/animation speed. Clamp 0..=12.
    #[config(default = 2.2)]
    pub rise_speed: f32,

    /// Multiplier for domain warp. Clamp 0..=2.
    #[config(default = 1.0)]
    pub turbulence: f32,

    /// Global emission/style blend multiplier. Clamp 0..=2.
    #[config(default = 1.0)]
    pub intensity: f32,

    /// Global opacity/density multiplier. Clamp 0..=2.
    #[config(default = 1.0)]
    pub density: f32,

    /// Cooling with height. Clamp 0..=2.
    #[config(default = 0.78)]
    pub cooling: f32,

    /// Amount of coherent flicker added to temperature. Clamp 0..=1.
    #[config(default = 0.18)]
    pub flicker_strength: f32,

    /// Blue base/reaction-zone contribution. Clamp 0..=1.
    #[config(default = 0.35)]
    pub blue_core_strength: f32,

    /// Hot white core contribution. Clamp 0..=1.
    #[config(default = 0.35)]
    pub white_core_strength: f32,

    /// Smoke contribution. Clamp 0..=1.
    #[config(default = 0.35)]
    pub smoke_strength: f32,

    /// Sparse spark/ember settings.
    #[serde(default)]
    pub sparks: FireSparkConfig,

    /// Author colors for terminal themes that do not want the built-in palette.
    #[serde(default)]
    pub palette: FirePalette,
}
```

### Mode enum

Use modes as presets plus limited behavior differences, not wholly separate shader implementations.

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum FireMode {
    /// General flame with wide base, torn top, smoke, and optional sparks.
    Flame,
    /// Narrow flame with stronger blue base and less smoke.
    Candle,
    /// Broad, smoky, turbulent flame for campfire / warning surfaces.
    Campfire,
    /// Low flame bed / ember glow. Good for status strips.
    Embers,
    /// Smoke-first plume; useful after flame has died down.
    SmokePlume,
}

impl Default for FireMode {
    fn default() -> Self { Self::Flame }
}
```

### Apply-to enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(rename_all = "snake_case")]
pub enum FireApplyTo {
    Foreground,
    Background,
    Both,
}

impl Default for FireApplyTo {
    fn default() -> Self { Self::Both }
}
```

### Palette struct

Keep palette controls small and compatible with existing `ColorConfig` conventions.

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct FirePalette {
    #[serde(default = "default_blue_core")]
    pub blue_core: ColorConfig,
    #[serde(default = "default_white_core")]
    pub white_core: ColorConfig,
    #[serde(default = "default_yellow")]
    pub yellow: ColorConfig,
    #[serde(default = "default_orange")]
    pub orange: ColorConfig,
    #[serde(default = "default_red")]
    pub red: ColorConfig,
    #[serde(default = "default_smoke")]
    pub smoke: ColorConfig,
}
```

Default colors can use RGB values approximating the ANSI palette named in the prompt:

```rust
blue_core:   rgb(0, 215, 255)   // ANSI-ish 45/51
white_core:  rgb(255, 255, 255)
yellow:      rgb(255, 215, 0)   // 220/226 feel
orange:      rgb(255, 95, 0)    // 202/208 feel
red:         rgb(175, 0, 0)     // 124/160 feel
smoke:       rgb(88, 88, 88)    // 238/240 feel
```

If `ColorConfig` supports indexed colors directly, prefer indices only in recipe examples; keep Rust defaults RGB for broad compatibility.

### Spark config

Do not create mutable global particles in the style shader. Use deterministic pseudo-particles derived from seed, index, position, and time.

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct FireSparkConfig {
    /// Deterministic seed.
    #[config(default = 1)]
    pub seed: u32,
    /// Number of synthetic spark trajectories. Clamp 0..=32.
    #[config(default = 8)]
    pub count: u8,
    /// Spark brightness multiplier. Clamp 0..=2.
    #[config(default = 0.35)]
    pub intensity: f32,
    /// Upward spark speed. Clamp 0..=4.
    #[config(default = 1.2)]
    pub rise_speed: f32,
    /// Horizontal spark drift. Clamp 0..=2.
    #[config(default = 0.25)]
    pub drift: f32,
}
```

### Internal sample struct

Keep this private at first. Make it `pub(crate)` only if tests or glyph/filter code need it.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct FireSample {
    temperature: f32,
    density: f32,
    smoke: f32,
    blue_core: f32,
    white_core: f32,
    sparks: f32,
    intensity: f32,
    radius: f32,
    mask: f32,
}
```

## 7. Validation and sanitization rules

Sanitize in helper methods, not custom deserialization, unless this project already has validation hooks.

Recommended clamps:

```text
aspect:             0.25..=4.0
base_width:         0.05..=2.0
min_width:          0.005..=0.5 and <= base_width
wind:               -1.0..=1.0
rise_speed:         0.0..=12.0
turbulence:         0.0..=2.0
intensity:          0.0..=2.0
density:            0.0..=2.0
cooling:            0.0..=2.0
flicker_strength:   0.0..=1.0
blue_core_strength: 0.0..=1.0
white_core_strength:0.0..=1.0
smoke_strength:     0.0..=1.0
spark count:        0..=32
```

Always guard division by widget width/height. For `W <= 1` or `H <= 1`, use stable defaults (`x=0`, `y=0` or `y=0.5`) and return finite values.

## 8. Coordinate system

Map a terminal cell `(i, j)` into flame-local coordinates:

```rust
fn normalized_fire_coord(ctx: &ShaderContext, aspect: f32) -> (f32, f32) {
    let width = ctx.area.width.max(1) as f32;
    let height = ctx.area.height.max(1) as f32;
    let col = ctx.local_x as f32;
    let row = ctx.local_y as f32;

    let x = if width <= 1.0 {
        0.0
    } else {
        (2.0 * col / (width - 1.0) - 1.0) * aspect
    };

    // y=0 at fuel/base, y=1 at top.
    let y = if height <= 1.0 {
        0.0
    } else {
        1.0 - row / (height - 1.0)
    };

    (x, y.clamp(0.0, 1.0))
}
```

This differs from many screen coordinates because bottom is the flame base. Tests should cover bottom, middle, top, and single-row/single-column areas.

## 9. Fire field math

Implement field sampling in a single function:

```rust
impl TerminalFireShader {
    fn sample_field_at(&self, x: f32, y: f32, time: f32) -> FireSample {
        // sanitize controls, compute coherent noise, domain warp, mask,
        // temperature, density, smoke, blue core, sparks, and intensity.
    }
}
```

### 9.1 Noise foundation

Use coherent deterministic noise, not random per-frame noise. If no coherent noise helper exists, add a small dependency-free value noise helper in `utils/fnc_procedural_noise.rs` or keep a private helper until both fire and water need it.

Minimum helper API:

```rust
pub fn hash3(seed: u32, x: i32, y: i32, z: i32) -> f32;
pub fn value_noise3(seed: u32, x: f32, y: f32, z: f32) -> f32; // -1..=1
pub fn fbm3(seed: u32, x: f32, y: f32, z: f32, octaves: u8, gain: f32, lacunarity: f32) -> f32;
```

Good defaults:

```text
octaves:   4..=6
gain:      0.5
lacunarity:2.0
```

Fire samples should move upward through noise by subtracting time from the vertical coordinate:

```rust
let nl = fbm3(seed, 1.5 * x, 2.0 * y - 0.7 * time, 0.11 * time, 4, 0.5, 2.0);
let nm = fbm3(seed, 4.0 * x, 6.0 * y - 2.2 * time, 0.23 * time, 5, 0.5, 2.0);
let nf = fbm3(seed, 12.0 * x, 16.0 * y - 6.0 * time, 0.37 * time, 4, 0.5, 2.0);
```

### 9.2 Domain warp

Do not paste noise only onto brightness. Warp coordinates first:

```rust
let turbulence = self.turbulence.clamp(0.0, 2.0);
let xw = x + turbulence * (0.16 * y * (1.0 - y) * nl + 0.05 * y * nm);
let yw = (y + turbulence * (0.05 * (1.0 - y) * nm + 0.015 * nf)).clamp(0.0, 1.0);
```

This is what makes tongues roll and tear. Keep warp finite and clamped.

### 9.3 Macro flame shape

Centerline:

```rust
let center = wind * yw.powf(1.7)
    + 0.08 * yw * (3.1 * yw - 1.7 * time).sin()
    + 0.04 * yw * (8.3 * yw + 2.6 * time).sin()
    + 0.06 * yw * fbm3(seed, 1.2 * yw, -0.35 * time, 0.0, 4, 0.5, 2.0);
```

Width:

```rust
let width = (min_width
    + base_width * (1.0 - yw).powf(0.85)
    + 0.03 * fbm3(seed, 2.0 * yw, -0.8 * time, 0.0, 4, 0.5, 2.0))
    .max(0.001);
```

Normalized horizontal distance:

```rust
let r = ((xw - center).abs() / width).max(0.0);
```

Soft flame mask:

```rust
let mask = (-r.powf(2.0 + 1.8 * yw)).exp()
    * (1.0 - smoothstep(0.78, 1.0, yw));
```

### 9.4 Temperature

```rust
let flicker = fbm3(seed, 0.0, -1.8 * time, 0.6 * time, 4, 0.5, 2.0);
let t = saturate(
    1.35 * mask.powf(0.72)
        + 0.28 * nm
        + 0.10 * nf
        - cooling * yw
        + flicker_strength * flicker,
);
```

Mode adjustments:

```rust
match self.mode {
    FireMode::Candle => { /* narrower width, stronger blue, lower smoke */ }
    FireMode::Campfire => { /* wider base, stronger turbulence/smoke/sparks */ }
    FireMode::Embers => { /* low y emphasis, low height, warmer red/orange */ }
    FireMode::SmokePlume => { /* low T, higher S, minimal blue/white */ }
    FireMode::Flame => { /* defaults */ }
}
```

Prefer implementing mode as sanitized multiplier methods instead of branching throughout the whole shader:

```rust
struct FireModeTuning {
    width_mul: f32,
    turbulence_mul: f32,
    smoke_mul: f32,
    blue_mul: f32,
    spark_mul: f32,
    height_cutoff: f32,
}
```

### 9.5 Density / opacity

```rust
let mut d = smoothstep(0.08, 0.42, t + 0.18 * mask - 0.10 * r + 0.08 * nm);
let tongue = smoothstep(0.15, 0.65, mask + 0.35 * nm - 0.50 * yw);
d = (d * tongue * self.density.clamp(0.0, 2.0)).clamp(0.0, 1.0);
```

Density determines how strongly the fire color should override/blend with the existing style.

### 9.6 Smoke / soot

Smoke should appear near the upper flame and trailing edges, not in the hottest core:

```rust
let mut s = smoothstep(0.50, 0.95, yw)
    * (1.0 - smoothstep(0.18, 0.58, t))
    * d
    * (0.65 + 0.35 * nl);

let xs = x + 0.15 * (1.2 * time + 3.0 * y).sin();
let ys = y - 0.25 * time;
let smoke_drift = 0.35 * fbm3(seed, 2.0 * xs, 3.0 * ys, 0.08 * time, 4, 0.5, 2.0)
    * smoothstep(0.65, 1.0, y);
s = saturate((s + smoke_drift.max(0.0)) * self.smoke_strength.clamp(0.0, 1.0));
```

Smoke should only win color selection when:

```rust
s > 0.18 && s > d * t
```

Otherwise smoke will cover the hot core.

### 9.7 Blue reaction-zone core

```rust
let b = (1.0 - smoothstep(0.02, 0.16, yw))
    * (1.0 - smoothstep(0.0, 0.42, r))
    * (1.0 - smoothstep(0.35, 0.80, s))
    * self.blue_core_strength.clamp(0.0, 1.0);
```

Use this heavily for `Candle`, moderately for `Flame`, lightly for `Campfire`, and usually not at all for `Embers` or `SmokePlume`.

### 9.8 Brightness / emission

```rust
let white_core = d
    * smoothstep(0.72, 0.95, t)
    * (1.0 - smoothstep(0.0, 0.32, r))
    * self.white_core_strength.clamp(0.0, 1.0);

let mut i = d * (0.20 + 0.80 * t.powf(0.65))
    + 0.45 * b
    - 0.25 * s
    + 0.35 * white_core;

i = saturate(i * self.intensity.clamp(0.0, 2.0));
```

### 9.9 Sparks / embers

Use deterministic sparse trajectories. Do not allocate per cell in the hot path.

```rust
fn spark_field(&self, x: f32, y: f32, time: f32) -> f32 {
    let cfg = self.sparks.sanitized();
    let mut sum = 0.0;

    for j in 0..cfg.count.min(32) {
        let h0 = hash01(cfg.seed ^ (j as u32).wrapping_mul(0x9E37_79B9));
        let h1 = hash01(cfg.seed ^ (j as u32).wrapping_mul(0x85EB_CA6B));
        let h2 = hash01(cfg.seed ^ (j as u32).wrapping_mul(0xC2B2_AE35));
        let phase = h0 * 6.2831855;
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
```

Only let sparks show in/near the flame column and above the base:

```rust
let sparks = self.spark_field(xw, yw, time) * smoothstep(0.12, 0.95, yw);
i = saturate(i + 0.8 * sparks);
```

## 10. Color mapping

Do not map brightness alone to color. Choose color from semantic fields:

```rust
fn sample_color(&self, sample: FireSample) -> Color {
    if sample.blue_core > 0.25 {
        return blend_colors(blue_core, white_core, sample.blue_core * sample.intensity, ColorSpace::Rgb);
    }

    if sample.smoke > 0.18 && sample.smoke > sample.density * sample.temperature {
        return smoke;
    }

    if sample.white_core > 0.20 || sample.temperature > 0.82 {
        return blend_colors(yellow, white_core, smoothstep(0.82, 1.0, sample.temperature), ColorSpace::Rgb);
    }

    if sample.temperature > 0.58 {
        return blend_colors(orange, yellow, smoothstep(0.58, 0.82, sample.temperature), ColorSpace::Rgb);
    }

    if sample.temperature > 0.32 {
        return blend_colors(red, orange, smoothstep(0.32, 0.58, sample.temperature), ColorSpace::Rgb);
    }

    if sample.density > 0.05 {
        return red;
    }

    if sample.smoke > 0.10 {
        return smoke;
    }

    Color::TRANSPARENT
}
```

If `ColorRamp` is already a good fit, consider a future `FirePalette::Ramp(ColorRamp)` alternative. For first implementation, explicit colors are easier to test and explain.

## 11. Style application

Implement `StyleShader` like existing shaders:

```rust
impl StyleShader for TerminalFireShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let (x, y) = normalized_fire_coord(ctx, self.aspect.clamp(0.25, 4.0));
        let sample = self.sample_field_at(x, y, ctx.time as f32);

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
```

Blend rules:

```rust
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
```

For empty cells with transparent fg/bg, existing shaders may not visibly render. Do not invent a new visibility rule without inspecting current expectations. If recipe preview surfaces use non-transparent base styles for procedural backgrounds, document that in recipes.

## 12. Glyph ramp derivation guidance

Style shaders cannot replace glyphs if they only return `Style`, but the plan should prepare a glyph-capable primitive/filter.

Suggested ramps:

```text
subtle:       "   .`',:^;~+=x*#MW&8%B@$"
heavy blocks: "  ░▒▓█"
fire ascii:   "  .,'^:;!~+=x*#%@"
braille:      U+2800..U+28FF by 8-dot subcell sampling
```

For a glyph-capable fire primitive, compute a high-quality braille glyph by sampling the same `FireSample::intensity` at eight subcell positions. Dot mapping:

```rust
const BRAILLE_DOTS: [(u8, f32, f32); 8] = [
    (0x01, 0.25, 0.125), // dot 1
    (0x02, 0.25, 0.375), // dot 2
    (0x04, 0.25, 0.625), // dot 3
    (0x40, 0.25, 0.875), // dot 7
    (0x08, 0.75, 0.125), // dot 4
    (0x10, 0.75, 0.375), // dot 5
    (0x20, 0.75, 0.625), // dot 6
    (0x80, 0.75, 0.875), // dot 8
];
```

Threshold each subcell with a small blue-noise/hash offset so the edge tears naturally:

```rust
let mut bits = 0u8;
for (bit, ox, oy) in BRAILLE_DOTS {
    let sx = cell_x + (ox - 0.5) / width;
    let sy = cell_y + (oy - 0.5) / height;
    let i = shader.sample_field_at(sx, sy, time).intensity;
    let jitter = 0.04 * (hash01(seed ^ bit as u32) - 0.5);
    if i > 0.35 + jitter {
        bits |= bit;
    }
}
let glyph = char::from_u32(0x2800 + bits as u32).unwrap_or(' ');
```

For fire, braille density is useful for sparks/smoke tongues; for large flames, ASCII/block ramps may read better. Expose glyph mode only in a glyph-capable path, not in the style-only shader.

## 13. Shared helpers to abstract across water and fire

Looking across this plan and `tui-vfx-terminal-water-shader-plan.md`, several helpers are worth sharing **if both shaders are implemented close together**. Do not over-abstract before the second user exists; if implementing fire first, private helpers are acceptable. If implementing water and fire together, extract these small dependency-free helpers.

### 13.1 Scalar math helpers

Suggested file:

```text
crates/tui-vfx-style/src/utils/fnc_scalar_math.rs
```

Recommended API:

```rust
#[inline]
pub fn saturate(x: f32) -> f32 { x.clamp(0.0, 1.0) }

#[inline]
pub fn finite_or(x: f32, fallback: f32) -> f32 {
    if x.is_finite() { x } else { fallback }
}

#[inline]
pub fn safe_div(num: f32, den: f32, fallback: f32) -> f32 {
    if den.abs() > 1.0e-6 { num / den } else { fallback }
}

#[inline]
pub fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

#[inline]
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    if (edge1 - edge0).abs() <= f32::EPSILON {
        return if x >= edge1 { 1.0 } else { 0.0 };
    }
    let u = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    u * u * (3.0 - 2.0 * u)
}
```

Shared by:

- water: foam thresholds, Fresnel/glint bands, ripple envelopes, parameter sanitization;
- fire: masks, density, smoke gating, blue/white core thresholds, parameter sanitization.

### 13.2 Normalized field coordinates

Suggested file:

```text
crates/tui-vfx-style/src/utils/fnc_field_coords.rs
```

API:

```rust
pub struct NormalizedFieldCoord {
    pub x: f32,
    pub y_top_down: f32,
    pub y_bottom_up: f32,
    pub width: f32,
    pub height: f32,
}

pub fn normalized_field_coord(ctx: &ShaderContext, aspect: f32) -> NormalizedFieldCoord;
```

Shared by:

- water: usually top-down or centered coordinates for waves/ripples;
- fire: bottom-up `y` for base-to-top flames.

Keep this helper tiny. Do not make it depend on either shader type.

### 13.3 Coherent procedural noise

Suggested file:

```text
crates/tui-vfx-style/src/utils/fnc_procedural_noise.rs
```

API:

```rust
pub fn hash01(seed: u32) -> f32;
pub fn hash2(seed: u32, x: i32, y: i32) -> f32;
pub fn hash3(seed: u32, x: i32, y: i32, z: i32) -> f32;
pub fn value_noise2(seed: u32, x: f32, y: f32) -> f32;
pub fn value_noise3(seed: u32, x: f32, y: f32, z: f32) -> f32;
pub fn fbm2(seed: u32, x: f32, y: f32, octaves: u8, gain: f32, lacunarity: f32) -> f32;
pub fn fbm3(seed: u32, x: f32, y: f32, z: f32, octaves: u8, gain: f32, lacunarity: f32) -> f32;
```

Shared by:

- water: flow turbulence, rain emitter jitter, residual shimmer, spectral-ish later extension;
- fire: rising turbulence, domain warp, smoke drift, flicker, deterministic sparks.

Performance rule: no heap allocation; octave count capped to 6 in hot paths.

### 13.4 Glyph ramp helpers

Suggested file only when a glyph-capable primitive lands:

```text
crates/tui-vfx-style/src/utils/fnc_glyph_ramp.rs
```

API:

```rust
pub fn ramp_index(value: f32, len: usize) -> usize;
pub fn glyph_from_ramp(value: f32, ramp: &str) -> char;
pub fn braille_from_bits(bits: u8) -> char;
pub const BRAILLE_DOTS: [(u8, f32, f32); 8];
```

Shared by:

- water: 256-braille subcell water surface derivation;
- fire: 256-braille density/emission derivation;
- future smoke/weather: rain bands, fog, snow, dust.

### 13.5 Palette/style helpers

Existing helpers already cover much of this:

- `crate::utils::blend_colors`
- `crate::utils::{blend_style_to_color, blend_style_to_color_in_space}`
- `ColorConfig`, `ColorSpace`, `ColorRamp`, `SignalColor`

Do **not** create a generic `NaturalPhenomenaPalette` abstraction. Water and fire color decisions are materially different:

- water blends deep/shallow/foam/specular;
- fire branches on blue core, smoke, temperature, white core.

The only shared piece should be a small transparent-channel-safe style blend helper if current utilities do not already satisfy the need.

## 14. V3 schema/lowering plan

### 14.1 Legacy `SpatialShaderType`

Add a variant similar to:

```rust
#[serde(rename = "terminal_fire")]
TerminalFire(TerminalFireShader),
```

Then update all match arms in `cls_spatial_shader_type.rs`:

- `StyleShader::style_at` dispatch;
- `name()`;
- `terse_description()`;
- `key_parameters()`;
- legacy-to-V3 family conversion if present;
- try-from-V3 payload normalization if present.

Expected metadata:

```text
name: TerminalFire
terse_description: Emissive procedural flame/smoke field with coherent rising turbulence, blue core, and sparks.
key_parameters: mode, base_width, wind, rise_speed, turbulence, intensity, density, smoke_strength, blue_core_strength, sparks.count
```

### 14.2 V3 motion-field behavior

Add a behavior in `enum_vfx_motion_field_behavior.rs`:

```rust
#[serde(rename = "terminal_fire")]
TerminalFire(TerminalFireShader),
```

If the V3 type stores behavior and common fields separately, follow nearby pattern exactly. The goal is for V3 recipes to express:

```json
{
  "family": "primitive",
  "primitive": "motion_field",
  "behavior": {
    "type": "terminal_fire",
    "mode": { "mode": "flame" },
    "base_width": 0.55,
    "wind": 0.1,
    "turbulence": 1.0,
    "intensity": 1.0
  }
}
```

Adjust exact shape to existing schema conventions.

### 14.3 Lowering

Add both directions where the current code supports them:

- legacy `SpatialShaderType::TerminalFire` -> V3 motion-field primitive;
- V3 motion-field `TerminalFire` -> legacy executable `SpatialShaderType::TerminalFire`.

Tests must prove round-trip compatibility for default and non-default fields.

## 15. Recipe examples

### 15.1 General flame fixture

Path:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/shaders/primitives/shader_terminal_fire_v3.json
```

Example payload shape; adapt to current V3 recipe syntax:

```json
{
  "id": "debug.shader.primitive.terminal_fire.v3",
  "schema_version": "3.0",
  "name": "Terminal fire primitive",
  "description": "Primitive-first fixture for terminal_fire: coherent rising turbulence, emissive temperature/density, smoke, blue reaction-zone core, and sparks.",
  "metadata": {
    "aesthetic_tags": ["fire", "flame", "shader", "motion_field", "procedural_field", "emissive"],
    "debug": true
  },
  "effects": [
    {
      "id": "terminal_fire_flame",
      "kind": "shader",
      "phase": "dwell",
      "shader": {
        "type": "terminal_fire",
        "mode": { "mode": "flame" },
        "apply_to": "both",
        "base_width": 0.58,
        "min_width": 0.06,
        "wind": 0.08,
        "rise_speed": 2.2,
        "turbulence": 1.0,
        "intensity": 1.0,
        "density": 1.0,
        "cooling": 0.78,
        "blue_core_strength": 0.35,
        "white_core_strength": 0.35,
        "smoke_strength": 0.35,
        "sparks": { "seed": 7, "count": 8, "intensity": 0.35, "rise_speed": 1.2, "drift": 0.25 }
      }
    }
  ]
}
```

### 15.2 Candle fixture

Use a narrow base, stronger blue core, fewer sparks, and low smoke:

```json
{
  "type": "terminal_fire",
  "mode": { "mode": "candle" },
  "base_width": 0.18,
  "min_width": 0.035,
  "wind": 0.02,
  "rise_speed": 1.4,
  "turbulence": 0.55,
  "intensity": 0.85,
  "smoke_strength": 0.08,
  "blue_core_strength": 0.75,
  "white_core_strength": 0.45,
  "sparks": { "count": 0 }
}
```

### 15.3 Smoke and sparks fixture

Use this to validate upper smoke gating and deterministic sparks:

```json
{
  "type": "terminal_fire",
  "mode": { "mode": "campfire" },
  "base_width": 0.78,
  "wind": -0.18,
  "rise_speed": 2.8,
  "turbulence": 1.35,
  "intensity": 1.1,
  "density": 1.1,
  "smoke_strength": 0.75,
  "sparks": { "seed": 19, "count": 16, "intensity": 0.65, "rise_speed": 1.8, "drift": 0.5 }
}
```

## 16. Tests

### 16.1 Same-file unit tests

Add tests at the bottom of `cls_terminal_fire_shader.rs`.

Minimum tests:

```rust
#[test]
fn default_sample_is_finite_and_bounded() {
    let shader = TerminalFireShader::default();
    let sample = shader.sample_field_at(0.0, 0.3, 0.0);
    assert!(sample.temperature.is_finite());
    assert!((0.0..=1.0).contains(&sample.temperature));
    assert!((0.0..=1.0).contains(&sample.density));
    assert!((0.0..=1.0).contains(&sample.smoke));
    assert!((0.0..=1.0).contains(&sample.intensity));
}

#[test]
fn flame_varies_by_position() {
    let shader = TerminalFireShader::default();
    let a = shader.sample_field_at(0.0, 0.2, 0.5).intensity;
    let b = shader.sample_field_at(0.7, 0.8, 0.5).intensity;
    assert!((a - b).abs() > 0.001);
}

#[test]
fn flame_varies_by_time() {
    let shader = TerminalFireShader::default();
    let early = shader.sample_field_at(0.1, 0.35, 0.0).intensity;
    let late = shader.sample_field_at(0.1, 0.35, 0.75).intensity;
    assert!((early - late).abs() > 0.001);
}

#[test]
fn smoke_does_not_win_over_hot_core() {
    let shader = TerminalFireShader::default();
    let sample = shader.sample_field_at(0.0, 0.08, 0.2);
    if sample.temperature > 0.58 && sample.density > 0.1 {
        assert!(sample.smoke <= sample.density * sample.temperature || sample.blue_core > 0.0);
    }
}

#[test]
fn style_at_changes_visible_style() {
    let shader = TerminalFireShader::default();
    let ctx = make_ctx_at(10, 16, 20, 20, 0.4);
    let base = Style::default().fg(Color::WHITE).bg(Color::BLACK);
    let styled = shader.style_at(&ctx, base);
    assert_ne!(styled, base);
}
```

Adjust `make_ctx_at` to the repository’s test support helpers.

### 16.2 Serialization tests

```rust
#[test]
fn terminal_fire_deserializes_from_snake_case_type() {
    let shader: SpatialShaderType = serde_json::from_value(json!({
        "type": "terminal_fire",
        "mode": { "mode": "flame" },
        "base_width": 0.55,
        "wind": 0.1,
        "sparks": { "count": 4 }
    })).expect("valid terminal_fire");

    assert_eq!(shader.name(), "TerminalFire");
}

#[test]
fn terminal_fire_rejects_unknown_fields() {
    let err = serde_json::from_value::<TerminalFireShader>(json!({
        "not_a_field": true
    })).unwrap_err();
    assert!(err.to_string().contains("unknown field"));
}
```

### 16.3 V3 tests

Add tests matching current V3 shape:

- legacy `TerminalFire` classifies as primitive motion field;
- V3 motion-field `terminal_fire` lowers to `SpatialShaderType::TerminalFire`;
- non-default `wind`, `turbulence`, `smoke_strength`, and `sparks.count` survive lowering;
- schema generation includes `terminal_fire` and nested `sparks` fields.

### 16.4 Helper tests if helpers are extracted

For `fnc_scalar_math.rs`:

- `smoothstep(0, 1, -1) == 0`;
- `smoothstep(0, 1, 2) == 1`;
- degenerate edges do not divide by zero;
- `finite_or(f32::NAN, 0.5) == 0.5`.

For `fnc_procedural_noise.rs`:

- deterministic for same seed/coords;
- different seeds differ;
- outputs are finite and bounded `[-1, 1]`;
- `fbm3` handles octave `0` by returning `0` or documented fallback.

## 17. Performance constraints

`style_at` runs per cell per frame. Keep it lean.

Rules:

- no heap allocation in `style_at`;
- cap noise octaves at 6;
- cap spark count at 32 and default to 8;
- no string parsing in hot path;
- sanitize once through cheap locals, not repeated expensive conversions;
- avoid `powf` explosion by using only a few per sample; if profiling shows cost, replace stable powers with multiplies or approximations;
- keep generated smoke/spark loops deterministic and bounded;
- preserve transparent colors without extra blending work when possible.

Expected cost for default fire:

```text
3 fbm samples for NL/NM/NF at 4-5 octaves
2-3 additional fbm samples for center/flicker/smoke drift
0-8 deterministic sparks by default
several smoothstep/blend operations
```

If preview frame time regresses, first reduce default sparks and octaves before changing visual math.

## 18. Documentation updates

Update or regenerate:

- `docs/generated/effect_schemas.json` if generated schema is committed;
- `docs/generated/*` API inventories if present;
- `docs/design/tui-vfx-v3-schema-overview.md` with `terminal_fire` shape;
- `docs/design/tui-vfx-v3-recipe-vocabulary.md` with authoring vocabulary: flame, candle, campfire, embers, smoke plume, blue core, sparks;
- `docs/design/tui-vfx-v3-capability-catalog.md` with capability listing;
- `CAPABILITIES.md` if it tracks primitive shader inventory;
- Rustdoc for `TerminalFireShader`, `FireMode`, `FireSparkConfig`, and `FirePalette`.

Rustdoc should include the mental model:

```rust
/// Procedural terminal fire shader.
///
/// Fire is modeled as a thin emissive density field rather than a lit surface:
/// coherent rising turbulence produces temperature, density, smoke, blue-core,
/// and spark fields, then those fields map to emissive terminal colors.
```

## 19. Debug recipe placement and validation

Add recipes under:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/shaders/primitives/
```

Recommended fixtures:

1. `shader_terminal_fire_v3.json` — default flame.
2. `shader_terminal_fire_candle_v3.json` — narrow blue-core candle.
3. `shader_terminal_fire_smoke_sparks_v3.json` — smoky campfire/sparks stress test.

Validation commands depend on the sibling repo. Start by inspecting:

```bash
cd /usr/projects/tui-vfx-recipes
rg -n "debug_recipes|pipeline-validator|validate|preview" README.md docs scripts crates Cargo.toml
```

Then run the existing validator command for these fixtures. Do not invent a new validation tool.

## 20. Compatibility and migration notes

- This is additive: no existing recipes should change.
- Use snake_case schema names: `terminal_fire`, `base_width`, `min_width`, `rise_speed`, `blue_core_strength`, `white_core_strength`, `smoke_strength`.
- Keep `#[serde(deny_unknown_fields)]` to catch recipe typos.
- Prefer RGB default colors in Rust for terminals without indexed-color semantics; recipes may mention ANSI 256-color approximations in docs.
- If V3 taxonomy later gains `volumetric_field` or `emissive_field`, keep `terminal_fire` accepted through the old motion-field path for compatibility.
- If glyph output lands later, add it as a separate primitive/filter or explicit glyph-capable mode without changing style-only behavior.

## 21. Stateful fluid-ish upgrade path

Do not put stateful simulation into the first `StyleShader` pass. Capture it as a future primitive if/when the runtime supports persistent per-effect grids.

Future state:

```rust
struct FireGridState {
    temperature: Vec<f32>,
    density: Vec<f32>,
    fuel: Vec<f32>,
    smoke: Vec<f32>,
    velocity: Vec<[f32; 2]>,
    width: u16,
    height: u16,
}
```

Future update loop:

1. add source fuel/heat at base or authored emitter shapes;
2. add buoyancy force upward from temperature/density;
3. advect temperature, fuel, density, and smoke through velocity;
4. diffuse/decay scalars;
5. add vorticity/curl detail;
6. optionally project velocity if a stable-fluids helper exists;
7. render `T/D/S` through the same palette/glyph mapping as stateless fire.

Practical constraints for terminal runtime:

```text
grid:              terminal W × H, or half-res
dt:                1/30 to 1/60
temperature decay: 0.6–1.8 per second
density decay:     0.4–1.2 per second
buoyancy beta:     1.0–5.0
vorticity epsilon: 0.05–0.4
diffusion:         tiny; terminal resolution already blurs
```

This upgrade enables persistent plumes, interaction with cursor/mouse disturbances, and fire that continues evolving after emitters move.

## 22. Implementation phases

### Phase 1 — Grounding and helper decision

1. Run the orientation commands in Section 3.
2. Inspect existing shader files and V3 lowering files.
3. Decide whether water implementation is happening in the same branch.
4. If yes, add shared scalar/noise helpers first with tests.
5. If no, keep fire helpers private in `cls_terminal_fire_shader.rs` and leave extraction for the second shader.

### Phase 2 — Core fire shader

1. Create `cls_terminal_fire_shader.rs`.
2. Add `TerminalFireShader`, `FireMode`, `FireApplyTo`, `FirePalette`, `FireSparkConfig`, and private `FireSample`.
3. Implement defaults and sanitization.
4. Implement `normalized_fire_coord`, coherent noise/fbm, domain warp, mask, temperature, density, smoke, blue core, white core, sparks, intensity.
5. Implement color selection and `StyleShader::style_at`.
6. Add same-file unit tests.

### Phase 3 — Spatial shader registration

1. Export new types in `models/mod.rs`.
2. Add `SpatialShaderType::TerminalFire`.
3. Update dispatch/name/description/key-parameters.
4. Add serialization and metadata tests.

### Phase 4 — V3 integration

1. Add V3 motion-field behavior for terminal fire.
2. Update legacy-to-V3 and V3-to-legacy lowering.
3. Add V3 tests for classification and lowering.
4. Update schema draft/overview if hand-maintained.

### Phase 5 — Docs/tooling/recipes

1. Update `xtask/src/docs/effect_metadata.rs`.
2. Regenerate generated docs/schemas using the established command.
3. Add debug recipes in `/usr/projects/tui-vfx-recipes`.
4. Validate the recipes.
5. Update docs indexes if new docs are added.

### Phase 6 — Verification and review

Run targeted tests first, then broader checks:

```bash
cargo test -p tui-vfx-style terminal_fire
cargo test -p tui-vfx-style vfx_motion_field
cargo test -p tui-vfx-style try_lower_v3_spatial_shader_family
cargo test -p tui-vfx-style spatial_shader_type
cargo test -p xtask docs
```

If the workspace has known unrelated failures, capture exact output and verify all `terminal_fire` targeted tests pass.

## 23. Risks and mitigations

1. **Static/noisy fire**
   - Risk: using random frame noise makes TV static, not flame.
   - Mitigation: coherent upward fbm and domain warp are mandatory.

2. **Fire appears flat**
   - Risk: mapping only `I` to color loses blue core/smoke/hot core structure.
   - Mitigation: branch color from `B`, `S`, `T`, `D`, and white core.

3. **Smoke covers the flame**
   - Risk: upper smoke overlays hot core.
   - Mitigation: smoke wins only when `S > D * T`.

4. **Too expensive per cell**
   - Risk: many noise octaves and spark loops hurt frame rate.
   - Mitigation: cap octaves/sparks, no allocation, test preview performance.

5. **Wrong V3 family**
   - Risk: fire is not purely material-light or stochastic.
   - Mitigation: use motion-field now; document possible future `emissive_field` migration.

6. **Glyph fields exposed before glyph mutation exists**
   - Risk: schema lies to authors.
   - Mitigation: keep glyph ramps documented as derivation only unless implemented in a glyph-capable primitive.

7. **Over-abstraction**
   - Risk: generic natural-effects framework slows delivery.
   - Mitigation: extract only scalar/noise/glyph helpers that are demonstrably shared with water.

## 24. Suggested commit structure

Use small, reviewable commits if committing manually:

1. `Add shared procedural field helpers`
   - only if implementing water/fire together; scalar math/noise tests.
2. `Add terminal fire shader primitive math`
   - new shader file and same-file tests.
3. `Register terminal fire in spatial shader catalog`
   - `SpatialShaderType` integration and tests.
4. `Thread terminal fire through V3 motion-field lowering`
   - V3 behavior/conversion/lowering tests.
5. `Document terminal fire schema and recipes`
   - docs metadata, generated docs, debug recipes.

Follow Lore commit protocol in repository AGENTS if asked to commit.

## 25. Acceptance criteria

Implementation is complete when all of these are true:

- `serde_json` can deserialize `{ "type": "terminal_fire", ... }` into `SpatialShaderType::TerminalFire`.
- `TerminalFireShader::style_at` changes visible fg/bg according to `apply_to` and preserves transparent channels.
- Field samples are finite and bounded for normal and invalid parameter inputs.
- Field values vary by position and time.
- Fire uses coherent rising turbulence/domain warp, not frame-static noise.
- Blue core appears near the base/center when enabled.
- Smoke appears near upper/cooler regions and does not cover the hot core.
- Sparks are deterministic from seed and capped for performance.
- `SpatialShaderType::name`, `terse_description`, and `key_parameters` include `TerminalFire`.
- V3 classifies/lowers `terminal_fire` through the selected primitive family.
- Generated schema/docs include `terminal_fire`, `mode`, palette fields, smoke/blue/white core controls, and spark controls.
- Primitive debug recipes validate in `/usr/projects/tui-vfx-recipes`.
- Targeted tests pass.
- Existing shaders and recipes are unchanged.

## 26. Junior developer quick-start checklist

1. Read this plan and the water plan’s implementation sections for style-shader integration patterns.
2. Run OFPF orientation/hotspot commands.
3. Inspect `PulseWaveShader`, `RadialSpiralShader`, `NeonFlickerShader`, `GlowShader`, and `SpatialShaderType`.
4. Decide shared helper extraction only after confirming both water and fire are in scope for the branch.
5. Implement `TerminalFireShader::default()` and structs/enums exactly first.
6. Implement scalar helpers and a deterministic `fbm3`.
7. Implement `sample_field_at` and get finite/bounded tests passing.
8. Implement `color_for_sample` and `style_at`.
9. Register in `models/mod.rs` and `SpatialShaderType`.
10. Add V3 behavior/lowering and tests.
11. Update docs/schema metadata.
12. Add debug recipes.
13. Run targeted tests and recipe validation.
14. Only then consider optional glyph-capable braille rendering or stateful simulation.

<!-- <FILE>docs/design/tui-vfx-terminal-fire-shader-plan.md</FILE> - <DESC>Implementation plan for a terminal fire/flame shader primitive.</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
