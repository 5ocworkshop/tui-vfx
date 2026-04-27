# Terminal Water / Ocean Shader Implementation Plan

Date: 2026-04-26
Repository: `/usr/projects/tui-vfx`
Related recipe repository: `/usr/projects/tui-vfx-recipes`
Status: implementation-ready planning artifact; no source files changed by this planning pass.
Audience: junior Rust developer implementing under existing `tui-vfx` conventions.

## 0. One-paragraph goal

Add a new additive spatial shader named `terminal_water` that renders both terminal ocean/water fields and still-pool pebble ripples. The shader computes layered sine/Gerstner-style ocean waves, optional radial damped ripple emitters, analytic slopes/normals, diffuse/specular/Fresnel lighting, and foam from crest steepness/curvature. The style-shader implementation must work inside the current `StyleShader` API, which appears to return only `Style`, so style-shader output is **color/light styling only** unless a glyph/cell mutation path is intentionally used elsewhere. The math must be structured so a glyph-capable primitive/filter can reuse the same `light_scalar_at` field and derive all 256 Unicode braille glyphs (`U+2800..U+28FF`) by either density sorting or true 8-dot subcell sampling.

## 1. Non-goals and constraints

### Non-goals / defer-unless-supported

Do **not** implement these in the style-shader-only path unless the codebase already exposes a clear glyph/cell mutation hook or runtime binding path:

- glyph replacement inside `TerminalWaterShader`;
- FFT/spectral ocean simulation;
- persistent simulation state;
- none for water behaviors: ripple, rain, flow, glint, and wake/trail should all be represented in the plan; implementation sequencing is a delivery choice, not a scope exclusion;
- external displacement/sampler integration unless existing pipeline seams make it cheap;
- caustics;
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

- `crates/tui-vfx-style/src/traits/tr_style_shader.rs`
- `crates/tui-vfx-style/src/traits/mod.rs`

The inspected `StyleShader` path uses:

```rust
fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style
```

That means a spatial shader can change foreground/background/modifiers, but likely cannot replace the cell glyph. Therefore:

- Style-shader path: `TerminalWaterShader` computes water lighting and applies color/style.
- Glyph-capable path: a filter/content primitive reuses the same field math for 256-braille output.

Do not expose a `glyph_ramp` field in the style-shader schema if it cannot affect output.

## 2. Initial repo orientation commands

Run these first from `/usr/projects/tui-vfx`:

```bash
cd /usr/projects/tui-vfx
ofpf-orientation --root /usr/projects/tui-vfx
ofpf-hotspots --root /usr/projects/tui-vfx
```

Then inspect these exact files:

```bash
sed -n '1,260p' crates/tui-vfx-style/src/models/cls_pulse_wave_shader.rs
sed -n '1,220p' crates/tui-vfx-style/src/models/cls_radial_spiral_shader.rs
sed -n '1,760p' crates/tui-vfx-style/src/models/cls_spatial_shader_type.rs
sed -n '1,220p' crates/tui-vfx-style/src/models/mod.rs
sed -n '1,240p' crates/tui-vfx-style/src/models/v3/enum_vfx_motion_field_behavior.rs
sed -n '1,220p' crates/tui-vfx-style/src/models/v3/cls_vfx_motion_field_shader.rs
sed -n '1,520p' crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs
sed -n '380,470p' xtask/src/docs/effect_metadata.rs
```

Useful grep:

```bash
rg -n "PulseWave|RadialSpiral|MotionField|SpatialShaderType|key_parameters|terse_description" crates/tui-vfx-style xtask/src/docs
```

## 3. Existing patterns to follow

### `PulseWaveShader`

Reference: `crates/tui-vfx-style/src/models/cls_pulse_wave_shader.rs`

Patterns to copy:

- derives: `Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema`;
- `#[serde(deny_unknown_fields)]`;
- defaults through `impl Default` and `#[config(default = ...)]`;
- private math helpers;
- `StyleShader for ...` implementation preserving transparent channels;
- unit tests in same file.

### `RadialSpiralShader`

Reference: `crates/tui-vfx-style/src/models/cls_radial_spiral_shader.rs`

Patterns to copy:

- procedural field helper method;
- bounded field value test;
- `Style::fg(Color::WHITE)` style-change test;
- use of `blend_colors` and `ColorSpace::Rgb`.

### `SpatialShaderType`

Reference: `crates/tui-vfx-style/src/models/cls_spatial_shader_type.rs`

Integration points:

- import new shader type;
- add enum variant;
- dispatch in `impl StyleShader for SpatialShaderType`;
- include in `name()`;
- include in `terse_description()`;
- include in `key_parameters()`;
- include in documentation category text near the top of the file;
- ensure V3 family conversion sees it through the motion-field conversion helpers.

### V3 motion-field family

References:

- `crates/tui-vfx-style/src/models/v3/enum_vfx_motion_field_behavior.rs`
- `crates/tui-vfx-style/src/models/v3/cls_vfx_motion_field_shader.rs`
- `crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs`

`terminal_water` belongs in `VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::MotionField(_))` because it is a dynamic spatial field. The lighting/foam are output encodings of that field, not a separate material-light family.


## 3.5 Planned water behaviors: ocean, ripple, rain, flow, glint, and wake

The plan should support all high-value water behaviors as one coherent primitive family rather than separate unrelated shader variants. Use one shader with a mode enum plus shared lighting controls so ocean, ripple, rain, flow, and wake/trail can share normals, lighting, color, foam/ring highlight, docs, and braille derivations. Reflection glint should be a shared lighting term, not a separate wave mode. Wind/weather derivation is explicitly a future thought after water works: water parameters naturally map to wind direction/speed/gusts, but do not expand the already-large water implementation around weather yet.

### Proposed mode enum

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum WaterWaveMode {
    /// Layered sine/Gerstner-style traveling ocean field.
    Ocean,
    /// Still pool with authored or runtime-bound radial ripple emitters.
    Ripple {
        #[serde(default)]
        emitters: Vec<WaterRippleEmitter>,
        /// Subtle residual shimmer so a pool is not perfectly dead after ripples fade.
        #[config(default = 0.0)]
        base_shimmer: f32,
    },
    /// Rain-like water surface: many small staggered drop ripples.
    Rain {
        #[serde(default)]
        emitters: Vec<WaterRippleEmitter>,
        /// Deterministic seed for generated drops when recipes do not enumerate all emitters.
        #[config(default = 1)]
        seed: u32,
        /// Number of generated drops to synthesize in the hot path cap. Clamp 0..=16.
        #[config(default = 8)]
        density: u8,
        /// Drop amplitude multiplier. Clamp 0..=2.
        #[config(default = 0.45)]
        drop_strength: f32,
    },
    /// Low-amplitude directional current/flow field for calm ambient motion.
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
    /// Moving source wake/trail, intended for cursor/path-bound visual trails.
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
    /// Layer ocean waves with ripple emitters.
    OceanWithRipples {
        #[serde(default)]
        emitters: Vec<WaterRippleEmitter>,
        /// Mix for the ocean base. Clamp 0..=1.
        #[config(default = 0.55)]
        ocean_mix: f32,
        /// Mix for ripple contribution. Clamp 0..=2.
        #[config(default = 1.0)]
        ripple_mix: f32,
    },
    /// Compose several water behaviors without requiring another enum variant for every combination.
    Composite {
        #[serde(default)]
        modes: Vec<WaterWaveMode>,
    },
}
```

### Ripple emitter struct

Use authored emitters and support runtime-populated emitters where existing cursor/path infrastructure can provide them. Binding fields can be wired through existing shader runtime params when available.

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct WaterRippleEmitter {
    /// Center x in normalized widget coordinates, 0.0 left to 1.0 right.
    #[config(default = 0.5)]
    pub center_x: f32,
    /// Center y in normalized widget coordinates, 0.0 top to 1.0 bottom.
    #[config(default = 0.5)]
    pub center_y: f32,
    /// Start time in shader seconds. Values <= current time are active.
    #[config(default = 0.0)]
    pub start_time: f32,
    /// Ripple amplitude. Clamp 0.0..=2.0.
    #[config(default = 0.6)]
    pub amplitude: f32,
    /// Ring travel speed in cells/second-ish shader units. Clamp >= 0.0.
    #[config(default = 8.0)]
    pub speed: f32,
    /// Oscillation frequency around the expanding ring.
    #[config(default = 1.6)]
    pub frequency: f32,
    /// Width of the visible ring envelope in cells. Clamp >= 0.5.
    #[config(default = 2.5)]
    pub ring_width: f32,
    /// Temporal decay as the ripple ages. Clamp >= 0.0.
    #[config(default = 0.45)]
    pub decay: f32,
    /// Spatial damping as distance increases. Clamp >= 0.0.
    #[config(default = 0.025)]
    pub damping: f32,
}
```

### Wake source struct

Use wake sources for cursor/path trails. Existing runtime path/cursor infrastructure can populate these directly or through runtime-bound fields.

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct WaterWakeSource {
    /// Current source x in normalized widget coordinates.
    #[config(default = 0.5)]
    pub x: f32,
    /// Current source y in normalized widget coordinates.
    #[config(default = 0.5)]
    pub y: f32,
    /// Previous/source-tail x in normalized widget coordinates.
    #[config(default = 0.45)]
    pub prev_x: f32,
    /// Previous/source-tail y in normalized widget coordinates.
    #[config(default = 0.5)]
    pub prev_y: f32,
    /// Optional runtime binding key for x.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_binding: Option<String>,
    /// Optional runtime binding key for y.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y_binding: Option<String>,
    /// Wake amplitude. Clamp 0..=2.
    #[config(default = 0.55)]
    pub amplitude: f32,
    /// Wake oscillation frequency along the trail.
    #[config(default = 1.4)]
    pub frequency: f32,
    /// Wake decay perpendicular to the trail.
    #[config(default = 0.18)]
    pub lateral_decay: f32,
}
```

Default `TerminalWaterShader` can remain `WaterWaveMode::Ocean` to preserve the original ocean intent, but docs/debug recipes should include ripple, rain, flow, glint, and wake examples. Ripple and wake should be presented prominently because they map directly to UI interactions.

## 4. File-by-file implementation checklist

### 4.1 Add new shader file

Add:

```text
crates/tui-vfx-style/src/models/cls_terminal_water_shader.rs
```

Start with the repository metadata header/footer style used by nearby files:

```rust
// <FILE>tui-vfx-style/src/models/cls_terminal_water_shader.rs</FILE> - <DESC>Layered terminal water/ocean shader</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Add motion-field primitive for layered water lighting with foam.</WCTX>
// <CLOG>0.1.0: implement color/light style-shader path with reusable scalar field math for braille derivations.</CLOG>

//! Layered terminal water/ocean shader.
//!
//! `TerminalWaterShader` computes a deterministic water field from layered
//! sine/Gerstner-style waves, then derives analytic slopes, normals, lighting,
//! and foam. The style-shader path applies the result as foreground/background color because
//! the current `StyleShader` API returns only `Style`.
//!
//! Future glyph-capable derivations should reuse the scalar field helpers here
//! to produce 256-braille output from the same light field rather than running
//! a divergent simulation.
```

### 4.2 Register module and exports

Edit `crates/tui-vfx-style/src/models/mod.rs`.

Add near other shader modules:

```rust
pub mod cls_terminal_water_shader;
```

Add public exports near `PulseWaveShader` / `RadialSpiralShader` exports:

```rust
pub use cls_terminal_water_shader::{TerminalWaterShader, WaterApplyTo};
```

If a glyph derivation is implemented in this same module, export those types only once they are operational.

### 4.3 Add `SpatialShaderType` variant

Edit `crates/tui-vfx-style/src/models/cls_spatial_shader_type.rs`.

Update the import list from `crate::models::{ ... }` to include:

```rust
cls_terminal_water_shader::TerminalWaterShader,
```

Add enum variant after `RadialSpiral` or near `PulseWave`:

```rust
/// Layered terminal ocean/water field with normals, light, Fresnel, and foam.
TerminalWater(TerminalWaterShader),
```

Because the enum has `#[serde(tag = "type", rename_all = "snake_case")]`, this serializes as:

```json
{ "type": "terminal_water", ... }
```

Update all matches:

```rust
SpatialShaderType::TerminalWater(s) => s.style_at(ctx, base),
```

```rust
SpatialShaderType::TerminalWater(_) => "TerminalWater",
```

```rust
SpatialShaderType::TerminalWater(_) => {
    "Layered terminal ocean field with normals, lighting, Fresnel, and foam"
}
```

`key_parameters()` arm:

```rust
SpatialShaderType::TerminalWater(s) => vec![
    ("layers", format!("{}", s.layers)),
    ("amplitude", format!("{:.2}", s.amplitude)),
    ("wavelength", format!("{:.1} cells", s.wavelength)),
    ("speed", format!("{:.2}", s.speed)),
    ("direction_deg", format!("{:.1}deg", s.direction_deg)),
    ("steepness", format!("{:.2}", s.steepness)),
    ("normal_strength", format!("{:.2}", s.normal_strength)),
    ("diffuse", format!("{:.2}", s.diffuse)),
    ("specular", format!("{:.2}", s.specular)),
    ("fresnel", format!("{:.2}", s.fresnel)),
    ("foam", format!("{:.2}", s.foam)),
    ("glint_strength", format!("{:.2}", s.glint_strength)),
    ("glint_angle_deg", format!("{:.1}deg", s.glint_angle_deg)),
    ("glint_width", format!("{:.1} cells", s.glint_width)),
    ("apply_to", format!("{:?}", s.apply_to)),
],
```

Update top-level rustdoc category list to mention `TerminalWater` under animated/procedural effects and in the primitive migration note near `PulseWave`, `Radar`, `Orbit`, `RadialSpiral`.

### 4.4 Add V3 behavior variant

Edit `crates/tui-vfx-style/src/models/v3/enum_vfx_motion_field_behavior.rs`.

Import the supporting apply enum if needed:

```rust
use crate::models::{ColorConfig, WaterApplyTo, WaterRippleEmitter, WaterWaveMode};
```

Add variant after `RadialSpiral`:

```rust
/// Layered terminal ocean/ripple water field with normals, light, Fresnel, and foam.
TerminalWater {
    /// Wave generation mode: ocean, ripple, or ocean with ripples.
    #[serde(default)]
    mode: WaterWaveMode,
    /// Number of wave layers/octaves for ocean modes. Runtime clamp: 1..=4.
    #[config(default = 3)]
    layers: u8,
    /// Base wave amplitude.
    #[config(default = 0.35)]
    amplitude: f32,
    /// Base wavelength in cells.
    #[config(default = 12.0)]
    wavelength: f32,
    /// Animation speed multiplier.
    #[config(default = 1.0)]
    speed: f32,
    /// Primary wave direction in degrees.
    #[config(default = 25.0)]
    direction_deg: f32,
    /// Gerstner-style crest steepness.
    #[config(default = 0.45)]
    steepness: f32,
    /// Slope-to-normal lighting strength.
    #[config(default = 1.4)]
    normal_strength: f32,
    /// Diffuse light strength.
    #[config(default = 0.65)]
    diffuse: f32,
    /// Specular light strength.
    #[config(default = 0.55)]
    specular: f32,
    /// Specular exponent.
    #[config(default = 24.0)]
    shininess: f32,
    /// Fresnel rim/reflection strength.
    #[config(default = 0.35)]
    fresnel: f32,
    /// Foam/whitecap strength.
    #[config(default = 0.5)]
    foam: f32,
    /// Deep-water color.
    deep_color: ColorConfig,
    /// Lit/shallow-water color.
    shallow_color: ColorConfig,
    /// Foam/specular crest color.
    foam_color: ColorConfig,
    /// Water-reflection glint strength; glisten-band-like control modulated by specular normals.
    #[config(default = 0.0)]
    glint_strength: f32,
    /// Direction of the glint/glisten streak in degrees.
    #[config(default = -18.0)]
    glint_angle_deg: f32,
    /// Width of the glint/glisten band in cells.
    #[config(default = 8.0)]
    glint_width: f32,
    /// Speed of the moving glint/glisten band.
    #[config(default = 1.0)]
    glint_speed: f32,
    /// Style channels affected by the color/light output.
    #[serde(default)]
    apply_to: WaterApplyTo,
},
```

### 4.5 Convert legacy ↔ V3

Edit `crates/tui-vfx-style/src/models/v3/cls_vfx_motion_field_shader.rs`.

Add imports:

```rust
TerminalWaterShader, WaterApplyTo, WaterRippleEmitter, WaterWaveMode,
```

Extend `from_legacy_spatial_shader`:

```rust
SpatialShaderType::TerminalWater(shader) => Some(Self::from(shader)),
```

Add conversion:

```rust
impl From<&TerminalWaterShader> for VfxMotionFieldShader {
    fn from(shader: &TerminalWaterShader) -> Self {
        Self {
            behavior: VfxMotionFieldBehavior::TerminalWater {
                mode: shader.mode.clone(),
                layers: shader.layers,
                amplitude: shader.amplitude,
                wavelength: shader.wavelength,
                speed: shader.speed,
                direction_deg: shader.direction_deg,
                steepness: shader.steepness,
                normal_strength: shader.normal_strength,
                diffuse: shader.diffuse,
                specular: shader.specular,
                shininess: shader.shininess,
                fresnel: shader.fresnel,
                foam: shader.foam,
                deep_color: shader.deep_color.clone(),
                shallow_color: shader.shallow_color.clone(),
                foam_color: shader.foam_color.clone(),
                glint_strength: shader.glint_strength,
                glint_angle_deg: shader.glint_angle_deg,
                glint_width: shader.glint_width,
                glint_speed: shader.glint_speed,
                apply_to: shader.apply_to,
            },
        }
    }
}
```

### 4.6 Lower V3 back to executable shader

Edit `crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs`.

Add `TerminalWaterShader` to imports.

In `impl From<&VfxMotionFieldShader> for SpatialShaderType`, add match arm:

```rust
VfxMotionFieldBehavior::TerminalWater {
    mode,
    layers,
    amplitude,
    wavelength,
    speed,
    direction_deg,
    steepness,
    normal_strength,
    diffuse,
    specular,
    shininess,
    fresnel,
    foam,
    deep_color,
    shallow_color,
    foam_color,
    glint_strength,
    glint_angle_deg,
    glint_width,
    glint_speed,
    apply_to,
} => SpatialShaderType::TerminalWater(TerminalWaterShader {
    mode: mode.clone(),
    layers: *layers,
    amplitude: *amplitude,
    wavelength: *wavelength,
    speed: *speed,
    direction_deg: *direction_deg,
    steepness: *steepness,
    normal_strength: *normal_strength,
    diffuse: *diffuse,
    specular: *specular,
    shininess: *shininess,
    fresnel: *fresnel,
    foam: *foam,
    deep_color: deep_color.clone(),
    shallow_color: shallow_color.clone(),
    foam_color: foam_color.clone(),
    glint_strength: *glint_strength,
    glint_angle_deg: *glint_angle_deg,
    glint_width: *glint_width,
    glint_speed: *glint_speed,
    apply_to: *apply_to,
}),
```

## 5. Concrete shader implementation skeleton

Use this as a starting point for `cls_terminal_water_shader.rs`. Adjust import paths if the compiler indicates local naming differences.

```rust
use crate::models::{ColorConfig, ColorSpace};
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::blend_colors;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

const MAX_LAYERS: u8 = 4;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
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
    OceanWithRipples {
        #[serde(default)]
        emitters: Vec<WaterRippleEmitter>,
        #[config(default = 0.55)]
        ocean_mix: f32,
        #[config(default = 1.0)]
        ripple_mix: f32,
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
            shallow_color: ColorConfig::Rgb { r: 40, g: 170, b: 210 },
            foam_color: ColorConfig::White,
            glint_strength: 0.0,
            glint_angle_deg: -18.0,
            glint_width: 8.0,
            glint_speed: 1.0,
            apply_to: WaterApplyTo::Both,
        }
    }
}
```

If `ColorConfig::Rgb` is not the exact variant shape, inspect `cls_color_config.rs` and adjust.

## 5.9 Cross-plan helper note for terminal fire

A sibling plan now exists for [`terminal_fire`](completed/tui-vfx-terminal-fire-shader-plan.md). If water and fire are implemented in the same development window, extract only the small helper functions that both plans actually need:

- `utils/fnc_scalar_math.rs`: `saturate`, `smoothstep`, `lerp_f32`, `safe_div`, and finite-value guards.
- `utils/fnc_field_coords.rs`: normalized field coordinate conversion from `ShaderContext`, exposing both top-down and bottom-up `y` conventions.
- `utils/fnc_procedural_noise.rs`: deterministic `hash01`, value noise, and capped `fbm2`/`fbm3` for water shimmer/flow and fire turbulence/smoke.
- `utils/fnc_glyph_ramp.rs`: ramp indexing and Unicode braille bit mapping, but only when a glyph-capable primitive/filter exists.

Do not create a broad “natural phenomena” abstraction. Water remains a lit height surface; fire remains an emissive density field. Share scalar/noise/glyph utilities, not domain models.

## 6. Scalar field helper design

Make the core math reusable and testable. Keep helpers private for the style-shader path unless a glyph/filter implementation needs public access. If a glyph primitive lives in another crate, consider `pub(crate)` or a small public `TerminalWaterSample` only after API review.

Suggested structs:

```rust
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
```

### Sanitization helpers

```rust
fn finite_or(value: f32, fallback: f32) -> f32 {
    if value.is_finite() { value } else { fallback }
}

fn clamp_finite(value: f32, fallback: f32, min: f32, max: f32) -> f32 {
    finite_or(value, fallback).clamp(min, max)
}
```

### Small math helpers

```rust
fn dot3(a: Vec3, b: Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

fn normalize3(x: f32, y: f32, z: f32) -> Vec3 {
    let len_sq = x * x + y * y + z * z;
    if !len_sq.is_finite() || len_sq <= f32::EPSILON {
        return Vec3 { x: 0.0, y: 0.0, z: 1.0 };
    }
    let inv = len_sq.sqrt().recip();
    Vec3 { x: x * inv, y: y * inv, z: z * inv }
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    if (edge1 - edge0).abs() <= f32::EPSILON {
        return if x >= edge1 { 1.0 } else { 0.0 };
    }
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
```

### Field sampling

Implement field sampling so braille subcell code can call the same conceptual function with offset coordinates. For ripple emitters, prefer passing `width` and `height` into the sampling helper so normalized emitter centers convert to actual widget-local cell coordinates. If using the skeleton below, adjust `sample_field_at` to accept `width`/`height`; do not hardcode dimensions.

```rust
impl TerminalWaterShader {
    fn sanitized_layers(&self) -> u8 {
        self.layers.clamp(1, MAX_LAYERS)
    }

    fn sample_field_at(
        &self,
        x: f32,
        y: f32,
        width: u16,
        height: u16,
        t: f32,
    ) -> WaterFieldSample {
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

        let mut height_value = 0.0;
        let mut max_height = 0.0;
        let mut slope_x = 0.0;
        let mut slope_y = 0.0;
        let mut crest = 0.0;
        let mut curvature = 0.0;
        let time = t * speed;

        let ocean_mix = match &self.mode {
            WaterWaveMode::Ripple { base_shimmer, .. } => {
                clamp_finite(*base_shimmer, 0.0, 0.0, 0.25)
            }
            WaterWaveMode::OceanWithRipples { ocean_mix, .. } => {
                clamp_finite(*ocean_mix, 0.55, 0.0, 1.0)
            }
            WaterWaveMode::Ocean => 1.0,
        };

        // Ocean base. In pure ripple mode this usually contributes zero, except for an
        // optional tiny `base_shimmer` so a still pool can retain subtle light motion.
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

        let ripple_scalar = self.add_ripples_to_field(
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

        let height_scalar = if max_height > f32::EPSILON {
            ((height_value / max_height) * 0.5 + 0.5).clamp(0.0, 1.0)
        } else {
            0.5
        };

        let normal = normalize3(-slope_x * normal_strength, -slope_y * normal_strength, 1.0);
        let light = normalize3(-0.35, -0.55, 0.76);
        let view = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
        let half_v = normalize3(light.x + view.x, light.y + view.y, light.z + view.z);

        let diffuse = dot3(normal, light).max(0.0);
        let specular = dot3(normal, half_v).max(0.0).powf(shininess);
        let fresnel = (1.0 - dot3(normal, view).max(0.0)).powf(3.0);
        let foam_signal = crest + curvature.abs() * 0.05;
        let foam = smoothstep(0.45, 0.85, foam_signal) * foam_strength;

        let light_scalar = (
            0.18
                + diffuse_strength * diffuse
                + specular_strength * specular
                + fresnel_strength * fresnel
                + foam
                + ripple_scalar * 0.15
        )
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
            WaterWaveMode::OceanWithRipples { emitters, ripple_mix, .. } => {
                (emitters.as_slice(), clamp_finite(*ripple_mix, 1.0, 0.0, 2.0))
            }
        };

        let width_f = width.max(1) as f32;
        // Match the caller's y aspect correction: y coordinates are scaled by 2.0.
        let height_f = height.max(1) as f32 * 2.0;
        let mut ripple_scalar: f32 = 0.0;

        for emitter in emitters.iter().take(4) {
            let age = t - finite_or(emitter.start_time, 0.0);
            if age < 0.0 {
                continue;
            }

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
            ripple_scalar = ripple_scalar.max(envelope.clamp(0.0, 1.0));

            let radial_x = dx / r;
            let radial_y = dy / r;
            let dh_dr = amplitude * envelope * frequency * c;
            *slope_x += dh_dr * radial_x;
            *slope_y += dh_dr * radial_y;
            *crest += envelope.abs() * amplitude.abs();
            *curvature += -amplitude * frequency * frequency * s * envelope;
        }

        ripple_scalar.clamp(0.0, 1.0)
    }

    fn sample_field_for_ctx(&self, ctx: &ShaderContext) -> WaterFieldSample {
        // Terminal cells are roughly twice as tall as wide in many fonts; scaling y
        // makes the wave field read less vertically stretched.
        let x = ctx.local_x as f32;
        let y = ctx.local_y as f32 * 2.0;
        self.sample_field_at(x, y, ctx.width, ctx.height, ctx.t as f32)
    }
}

```

## 7. Style application implementation

Follow existing transparent-channel behavior from `PulseWaveShader` and `RadialSpiralShader`.

```rust
impl TerminalWaterShader {
    fn lit_color(&self, sample: WaterFieldSample) -> Color {
        let deep: Color = self.deep_color.into();
        let shallow: Color = self.shallow_color.into();
        let foam: Color = self.foam_color.into();

        let water_t = (sample.height_scalar * sample.light_scalar).clamp(0.0, 1.0);
        let water = blend_colors(deep, shallow, water_t, ColorSpace::Rgb);
        blend_colors(water, foam, sample.foam, ColorSpace::Rgb)
    }

    fn color_blend_amount(&self, sample: WaterFieldSample) -> f32 {
        // Keep some base glyph/style identity while still making the water visible.
        (0.25 + sample.light_scalar * 0.75).clamp(0.0, 1.0)
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
                    // Background changes should be calmer than foreground changes.
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
```

If `Style` does not expose `fg`/`bg` directly in this crate version, mirror the access pattern from `PulseWaveShader` exactly.

## 8. Unit tests for new shader file

Add tests at the bottom of `cls_terminal_water_shader.rs`.

Start with these exact tests and adjust only for compile errors caused by local API details:

```rust
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
        let a = shader.sample_field_for_ctx(&ctx_at(2, 2, 0.25)).height_scalar;
        let b = shader.sample_field_for_ctx(&ctx_at(20, 8, 0.25)).height_scalar;
        assert!((a - b).abs() > 0.001, "water field should vary by position");
    }

    #[test]
    fn light_varies_by_time() {
        let shader = TerminalWaterShader::default();
        let early = shader.sample_field_for_ctx(&ctx_at(10, 5, 0.1)).light_scalar;
        let late = shader.sample_field_for_ctx(&ctx_at(10, 5, 1.3)).light_scalar;
        assert!((early - late).abs() > 0.001, "water lighting should animate over time");
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
```

If `Style::default()` or struct update syntax differs, inspect `crates/tui-vfx-types/src/style.rs` and nearby tests.

## 9. Spatial enum tests

Look for existing tests under:

```text
crates/tui-vfx-style/tests/models/
crates/tui-vfx-style/src/models/v3/test_*.rs
```

Add a focused test file if no suitable one exists:

```text
crates/tui-vfx-style/tests/models/test_cls_terminal_water_shader.rs
```

Test serde round-trip:

```rust
use tui_vfx_style::models::{SpatialShaderType, TerminalWaterShader};

#[test]
fn terminal_water_deserializes_from_snake_case_type() {
    let json = serde_json::json!({
        "type": "terminal_water",
        "layers": 3,
        "amplitude": 0.35,
        "wavelength": 12.0,
        "speed": 0.8,
        "direction_deg": 20.0,
        "steepness": 0.45,
        "normal_strength": 1.3,
        "diffuse": 0.7,
        "specular": 0.5,
        "shininess": 28.0,
        "fresnel": 0.35,
        "foam": 0.55,
        "deep_color": { "type": "rgb", "r": 5, "g": 32, "b": 64 },
        "shallow_color": { "type": "rgb", "r": 40, "g": 170, "b": 210 },
        "foam_color": { "type": "white" },
        "apply_to": "both"
    });

    let shader: SpatialShaderType = serde_json::from_value(json).expect("valid terminal_water");
    assert!(matches!(shader, SpatialShaderType::TerminalWater(_)));
    assert_eq!(shader.name(), "TerminalWater");
    assert_eq!(shader.v3_family_label(), "motion_field");
}

#[test]
fn terminal_water_key_parameters_include_lighting_controls() {
    let shader = SpatialShaderType::TerminalWater(TerminalWaterShader::default());
    let keys: Vec<_> = shader.key_parameters().into_iter().map(|(k, _)| k).collect();
    for expected in ["layers", "amplitude", "wavelength", "diffuse", "specular", "fresnel", "foam"] {
        assert!(keys.contains(&expected), "missing key parameter {expected}");
    }
}
```

If integration tests cannot import crate internals as expected, place tests inside the relevant module test files instead.

## 10. V3 tests

Update `crates/tui-vfx-style/src/models/v3/test_vfx_motion_field_shader.rs` or add a new test in that module.

Suggested tests:

```rust
#[test]
fn terminal_water_legacy_maps_to_motion_field_family() {
    let legacy = SpatialShaderType::TerminalWater(TerminalWaterShader::default());
    let family = legacy.v3_spatial_shader_family();
    match family {
        VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::MotionField(shader)) => {
            assert!(matches!(shader.behavior, VfxMotionFieldBehavior::TerminalWater { .. }));
        }
        other => panic!("expected motion-field primitive, got {other:?}"),
    }
}

#[test]
fn terminal_water_v3_lowers_to_legacy_shader() {
    let family = VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::MotionField(
        VfxMotionFieldShader {
            behavior: VfxMotionFieldBehavior::TerminalWater {
                layers: 3,
                amplitude: 0.35,
                wavelength: 12.0,
                speed: 0.8,
                direction_deg: 20.0,
                steepness: 0.45,
                normal_strength: 1.3,
                diffuse: 0.7,
                specular: 0.5,
                shininess: 28.0,
                fresnel: 0.35,
                foam: 0.55,
                deep_color: ColorConfig::Rgb { r: 5, g: 32, b: 64 },
                shallow_color: ColorConfig::Rgb { r: 40, g: 170, b: 210 },
                foam_color: ColorConfig::White,
                apply_to: WaterApplyTo::Both,
            },
        },
    ));

    let lowered = try_lower_v3_spatial_shader_family(&family).expect("lowering succeeds");
    assert!(matches!(lowered, SpatialShaderType::TerminalWater(_)));
}
```

Adjust imports based on surrounding test style.

## 11. Docs and schema updates

### Update docs metadata extraction

Edit `xtask/src/docs/effect_metadata.rs`.

Add import near other shader structs:

```rust
cls_terminal_water_shader::TerminalWaterShader,
```

Add to `variants: Vec<SpatialShaderType>` after `RadialSpiral` or near `PulseWave`:

```rust
SpatialShaderType::TerminalWater(TerminalWaterShader::default()),
```

### Regenerate docs

Run:

```bash
cargo xtask docs generate
```

Then verify:

```bash
python3 - <<'PY'
import json
p='docs/generated/effect_schemas.json'
d=json.load(open(p))
assert 'terminal_water' in d['categories']['shaders']['variants']
print(json.dumps(d['categories']['shaders']['variants']['terminal_water'], indent=2)[:2000])
PY
```

Also inspect generated capabilities for the new shader:

```bash
rg -n "terminal_water|TerminalWater|terminal ocean|water" docs/generated docs CAPABILITIES.md README.md
```

Only manually update human docs if generated docs do not already cover author-facing information. Good candidates:

- `docs/CAPABILITIES_REFERENCE.md`
- `docs/API_HAND.md`
- `docs/RECIPE_AUTHORING_WORKFLOW.md` only if documenting the primitive-first fixture or glyph-deferred caveat.

## 12. Recipe updates in `/usr/projects/tui-vfx-recipes`

Do after the `tui-vfx` crate compiles and schema is regenerated.

### 12.1 Primitive-first debug recipes

Add two primitive fixtures: one ripple-first fixture and one ocean fixture. The ripple fixture should be the primary authoring example because it maps directly to UI moments like click impacts, focus pings, and alert pulses.

#### Ripple fixture

Add:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/shaders/primitives/shader_terminal_water_ripple_v3.json
```

Payload core:

```json
{
  "type": "terminal_water",
  "mode": {
    "mode": "ripple",
    "emitters": [
      {
        "center_x": 0.5,
        "center_y": 0.5,
        "start_time": 0.0,
        "amplitude": 0.75,
        "speed": 8.0,
        "frequency": 1.8,
        "ring_width": 2.2,
        "decay": 0.35,
        "damping": 0.02
      }
    ],
    "base_shimmer": 0.02
  },
  "layers": 1,
  "amplitude": 0.0,
  "wavelength": 12.0,
  "speed": 0.8,
  "direction_deg": 20.0,
  "steepness": 0.2,
  "normal_strength": 1.6,
  "diffuse": 0.45,
  "specular": 0.9,
  "shininess": 36.0,
  "fresnel": 0.45,
  "foam": 0.0,
  "deep_color": { "type": "rgb", "r": 4, "g": 20, "b": 42 },
  "shallow_color": { "type": "rgb", "r": 50, "g": 170, "b": 210 },
  "foam_color": { "type": "white" },
  "apply_to": "both"
}
```

#### Ocean fixture

### 12.1 Primitive-first debug recipe

Add:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/shaders/primitives/shader_terminal_water_v3.json
```

Use this full fixture as a starting point:

```json
{
  "schema_version": 3,
  "id": "debug.shader.primitive.terminal_water.v3",
  "title": "Debug: Shader Primitive - Terminal Water",
  "description": "Primitive-first fixture for terminal_water: layered waves, normals, diffuse/specular/Fresnel light, and foam encoded as color/light styling.",
  "version": "3.0.0",
  "last_updated": "2026-04-26",
  "metadata": {
    "aesthetic_tags": ["water", "ocean", "shader", "motion_field", "procedural_field"],
    "mood": "immersive",
    "related_themes": ["theme-neutral"],
    "use_cases": ["debug_preview", "primitive_reference"],
    "maturity_era": "experimental",
    "authoring_notes": "Color/style shader path. 256-braille subcell glyph derivation should use a glyph-capable filter/content primitive that can reuse the same light scalar.",
    "last_reviewed": "2026-04-26"
  },
  "config": {
    "message": "TERMINAL OCEAN FIELD  ~ ~ ~  LIGHT FROM WAVES",
    "layout": { "width": 64, "height": 10, "anchor": "center" },
    "lifecycle": { "auto_dismiss_ms": 12000 },
    "border": { "type": "rounded", "trim": "none" },
    "pipeline": {
      "step": {
        "kind": "parallel",
        "children": [
          {
            "kind": "style_effect",
            "scope": { "kind": "content", "value": "text" },
            "payload": {
              "base_style": {
                "foreground": { "type": "rgb", "r": 120, "g": 220, "b": 255 },
                "background": { "type": "rgb", "r": 2, "g": 12, "b": 30 }
              }
            }
          },
          {
            "kind": "shader",
            "scope": { "kind": "content", "value": "text" },
            "payload": {
              "type": "terminal_water",
              "layers": 3,
              "amplitude": 0.35,
              "wavelength": 12.0,
              "speed": 0.8,
              "direction_deg": 20.0,
              "steepness": 0.45,
              "normal_strength": 1.3,
              "diffuse": 0.7,
              "specular": 0.5,
              "shininess": 28.0,
              "fresnel": 0.35,
              "foam": 0.55,
              "deep_color": { "type": "rgb", "r": 5, "g": 32, "b": 64 },
              "shallow_color": { "type": "rgb", "r": 40, "g": 170, "b": 210 },
              "foam_color": { "type": "white" },
              "apply_to": "both"
            }
          }
        ]
      }
    }
  }
}
```

### 12.2 Optional complex recipe

Add after primitive recipe passes QC:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/complex/complex_terminal_ocean_panel.json
```

Composition idea:

- content base style: deep navy background, pale cyan foreground;
- content shader: `terminal_water`;
- border shader: existing `edge_sheen` or `glisten_band`;
- optional mask/sampler: existing `radial` or `ripple` to verify composition.

Keep this optional so the first implementation does not get blocked by recipe aesthetics.

## 13. 256-braille follow-up design

> **Status: Implemented (2026-04).** The design captured here landed via the
> shared glyph-rendering framework — see
> [`completed/tui-vfx-glyph-rendering-framework-plan.md`](completed/tui-vfx-glyph-rendering-framework-plan.md)
> for the framework, plus the working debug recipe at
> `recipes/debug_recipes/shaders/primitives/shader_terminal_water_glyph_v3.json`
> in `tui-vfx-recipes`. The encoder vocabulary lives in
> `tui-vfx-types/src/glyph/cls_glyph_encoder.rs` (variants
> `BrailleSubcell`, `BrailleEighths`, `BlockHorizontal`, `BlockVertical`,
> `Ramp`); the unifying filter lives in
> `tui-vfx-compositor/src/filters/cls_scalar_field_glyph_filter.rs`; the
> water sampler that drives it lives in
> `tui-vfx-style/src/models/cls_water_field_signal.rs`; the recipe-layer
> wire-up lives in `cls_filter_spec.rs`'s `FilterSpec::ScalarFieldGlyph`
> variant. The original §13 design content below is preserved as historical
> context for *why* the framework took the shape it did.

This section is for the glyph-capable primitive/filter. Do not implement inside the style-shader path unless a glyph mutation API already exists.

### 13.1 Why 256 braille, not a short ramp

A 10-character ramp loses local wave geometry. Unicode braille gives 256 possible bitmasks in one terminal cell, so a crest can occupy only the upper-left dots, foam can sparkle sparsely, and specular streaks can be subcell-thin.

### 13.2 Dot layout and bit mapping

Braille visual layout:

```text
1 4
2 5
3 6
7 8
```

Unicode bit mapping:

```text
bit 0 = dot 1
bit 1 = dot 2
bit 2 = dot 3
bit 3 = dot 4
bit 4 = dot 5
bit 5 = dot 6
bit 6 = dot 7
bit 7 = dot 8
```

Function:

```rust
fn braille_from_samples(samples: [f32; 8], threshold: f32) -> char {
    let mut bits = 0u8;
    for (i, value) in samples.iter().enumerate() {
        if *value >= threshold {
            bits |= 1 << i;
        }
    }
    char::from_u32(0x2800 + bits as u32).unwrap_or('⠀')
}
```

### 13.3 Subcell sampling offsets

For a cell at integer `(x, y)`, sample these local offsets. Remember terminal cells are taller; preserve the same y-aspect correction used by the style-shader path.

```rust
const BRAILLE_DOT_OFFSETS: [(f32, f32); 8] = [
    (0.25, 0.125), // dot 1
    (0.25, 0.375), // dot 2
    (0.25, 0.625), // dot 3
    (0.75, 0.125), // dot 4
    (0.75, 0.375), // dot 5
    (0.75, 0.625), // dot 6
    (0.25, 0.875), // dot 7
    (0.75, 0.875), // dot 8
];
```

Pseudo-code for glyph filter:

```rust
let mut samples = [0.0; 8];
for (i, (ox, oy)) in BRAILLE_DOT_OFFSETS.iter().copied().enumerate() {
    // Match the style-shader coordinate convention. If it uses y * 2.0, apply that here too.
    let sx = cell_x as f32 + ox;
    let sy = (cell_y as f32 + oy) * 2.0;
    samples[i] = shader.sample_field_at(sx, sy, t).light_scalar;
}
let glyph = braille_from_samples(samples, threshold);
```

### 13.4 Density mode using all 256 chars

If subcell sampling is too expensive for a first glyph-capable filter, use all 256 chars sorted by density.

```rust
fn braille_popcount(ch: char) -> Option<u32> {
    let code = ch as u32;
    if !(0x2800..=0x28ff).contains(&code) {
        return None;
    }
    Some((code - 0x2800).count_ones())
}

fn build_density_ramp() -> Vec<char> {
    let mut chars: Vec<char> = (0x2800..=0x28ff)
        .filter_map(char::from_u32)
        .collect();
    chars.sort_by_key(|ch| {
        let bits = (*ch as u32 - 0x2800) as u8;
        // Primary: density. Secondary: raw bit pattern for deterministic order.
        (bits.count_ones(), bits)
    });
    chars
}
```

Then:

```rust
let idx = (light_scalar.clamp(0.0, 1.0) * 255.0).round() as usize;
let glyph = density_ramp[idx];
```

Subcell mode is visually better; density mode is cheaper.

### 13.5 Future braille tests

When implemented:

```rust
#[test]
fn braille_blank_and_full_are_correct() {
    assert_eq!(braille_from_samples([0.0; 8], 0.5), '⠀');
    assert_eq!(braille_from_samples([1.0; 8], 0.5), '⣿');
}

#[test]
fn individual_dots_map_to_unicode_bits() {
    for i in 0..8 {
        let mut samples = [0.0; 8];
        samples[i] = 1.0;
        let ch = braille_from_samples(samples, 0.5);
        assert_eq!(ch as u32, 0x2800 + (1u32 << i));
    }
}

#[test]
fn density_ramp_uses_all_256_braille_chars() {
    let ramp = build_density_ramp();
    assert_eq!(ramp.len(), 256);
    assert_eq!(ramp.first().copied(), Some('⠀'));
    assert_eq!(ramp.last().copied(), Some('⣿'));
    for pair in ramp.windows(2) {
        let a = braille_popcount(pair[0]).unwrap();
        let b = braille_popcount(pair[1]).unwrap();
        assert!(a <= b);
    }
}
```

## 13.6 Additional water behavior math notes

### Rain

Rain does not need fundamentally new math. Treat it as multiple small ripple emitters with staggered `start_time`, lower amplitude, faster decay, and smaller `ring_width`. If recipes enumerate emitters, use them directly. If `density` is used, synthesize up to 16 deterministic drops from `seed` with a tiny integer hash; do not allocate in the hot path.

### Flow/current

Flow is low-amplitude directional movement. It can reuse the ocean loop with `layers = 1..2`, smaller amplitude, and a directional phase drift. Implementation can either map `Flow` to ocean parameters internally or add a cheap `flow_scalar = sin(k * dot(dir, p) + t * speed)` term that contributes to height/slope.

### Reflection glint / sun streak

Glint should be a lighting term, not a wave mode. Reuse the existing `glisten_band` mental model — angle, width/band width, speed, strength/intensity — but make it materially water-specific by multiplying the moving band by the water specular/normal response. In other words, `glisten_band` is a generic overlay sweep; `water glint` is reflected light on a computed water surface.

Recommended implementation shape:

```rust
let glint_strength = clamp_finite(self.glint_strength, 0.0, 0.0, 2.0);
let glint_width = clamp_finite(self.glint_width, 8.0, 0.5, 128.0);
let glint_speed = finite_or(self.glint_speed, 1.0);
let angle = finite_or(self.glint_angle_deg, -18.0).to_radians();
let axis = x * angle.cos() + y * angle.sin();
let moving = axis - t * glint_speed * 8.0;
let band = (-(moving * moving) / (2.0 * glint_width * glint_width)).exp();
let glint = band * specular * glint_strength;
light_scalar = (light_scalar + glint).clamp(0.0, 1.0);
```

When wiring docs, describe these controls using the same vocabulary authors already know from `GlistenBandShader`: angle, band width, speed, and intensity/strength. Do not copy the entire `GlistenBandShader` implementation; reuse the concept and naming, then keep the water glint tied to water normals/specular.

### Wake/trail

Wake uses existing cursor/path trail infrastructure conceptually: a source has current and previous positions. Convert normalized source coordinates to local cell coordinates, derive a direction vector from previous to current, and create a V/trailing disturbance behind the source. A pragmatic formula:

```rust
let along = dot(cell - source, -direction);
let side = abs(cross(cell - source, direction));
let trail = smoothstep(trail_length, 0.0, along) * exp(-side * lateral_decay);
let phase = along * frequency - t * speed;
let wake_height = amplitude * sin(phase) * trail;
```

Only contribute when `along >= 0.0` so the wake appears behind the moving source. Runtime bindings (`x_binding`, `y_binding`) should resolve through existing shader runtime params if available, mirroring existing binding patterns such as focus/cursor shaders.

## 14. Validation rules

Implement sanitization in helper methods, not custom deserialization, unless the project already has validation hooks.

| Field | Runtime behavior |
| --- | --- |
| `mode` | default to `Ocean`; cap authored ripple emitters to first 4 in hot path |
| `layers` | clamp to `1..=4` |
| `amplitude` | finite fallback to default; clamp `0.0..=2.0` |
| `wavelength` | finite fallback to default; clamp/min `>= 1.0` |
| `speed` | finite fallback to default; allow negative only if reverse animation is desired; otherwise clamp `0.0..=8.0` |
| `direction_deg` | finite fallback to default; no clamp required |
| `steepness` | clamp `0.0..=1.0` |
| `normal_strength` | clamp `0.0..=4.0` |
| `diffuse` | clamp `0.0..=2.0` |
| `specular` | clamp `0.0..=2.0` |
| `shininess` | clamp `1.0..=128.0` |
| `fresnel` | clamp `0.0..=2.0` |
| `foam` | clamp `0.0..=2.0`, final foam scalar clamp `0.0..=1.0` |
| `glint_strength` | clamp `0.0..=2.0` |
| `glint_angle_deg` | finite fallback `-18.0` |
| `glint_width` | clamp `0.5..=128.0` |
| `glint_speed` | finite fallback `1.0` |
| `WaterRippleEmitter.center_x/y` | clamp normalized coordinates to `0.0..=1.0` |
| `WaterRippleEmitter.start_time` | finite fallback `0.0`; inactive while `ctx.t < start_time` |
| `WaterRippleEmitter.speed` | clamp `0.0..=128.0` |
| `WaterRippleEmitter.frequency` | clamp `0.01..=32.0` |
| `WaterRippleEmitter.ring_width` | clamp `0.5..=64.0` |
| `WaterRippleEmitter.decay/damping` | clamp `>= 0.0` with sane upper bounds |

Recommended Style-shader path: allow negative `speed` for reverse wave travel if tests still pass; document it as animation direction reversal. If this feels too broad, clamp to `0.0..=8.0` and add a test.

## 15. Performance requirements

The shader runs per cell per frame, so keep it cheap.

Required:

- no allocations in `style_at`;
- at most 4 wave layers;
- no neighbor sampling in the style-shader hot path;
- no random number generation;
- no heap-built ramps;
- no new dependencies;
- no `serde_json` work in hot path.

Acceptable per cell:

- up to 4 `sin`/`cos` pairs;
- several dot products and `powf` calls for specular/Fresnel.

If performance is concerning after implementation, reduce default `layers` to `2` or reduce specular math, but keep schema capable of `4`.

Optional benchmark:

- compare render cost to `RadialSpiralShader` and `PulseWaveShader` on a representative grid.

## 16. Verification commands

From `/usr/projects/tui-vfx`:

```bash
cargo fmt --all
cargo test -p tui-vfx-style terminal_water
cargo test -p tui-vfx-style vfx_motion_field
cargo test -p tui-vfx-style try_lower_v3_spatial_shader_family
cargo test -p tui-vfx-style
cargo xtask docs generate
cargo test -p tui-vfx
```

If `cargo test -p tui-vfx` is too broad or slow, at minimum run the crates touched by this patch:

```bash
cargo test -p tui-vfx-style
cargo test -p tui-vfx-core
cargo test -p tui-vfx
```

From `/usr/projects/tui-vfx-recipes`, after recipe fixture updates:

```bash
cargo test
cargo run -q --bin pipeline-validator -- --debug-recipes-qc
```

If the binary name differs, inspect:

```bash
cargo run --bin
rg -n "debug-recipes-qc|pipeline-validator" .
```

## 17. Manual visual QA

After recipes are in place, run the project’s existing preview/debug recipe workflow. Start from `docs/RECIPE_AUTHORING_WORKFLOW.md`, which says primitive-first debug recipes should be validated before complex showcases.

Manual visual expectations:

- water field moves continuously over time;
- color changes are visible but not flashing aggressively;
- foam appears near crests/high curvature, not uniformly everywhere;
- specular/Fresnel creates occasional bright glints;
- background-only and foreground-only modes behave as named;
- terminal resize should not panic or distort catastrophically.

## 18. Common implementation pitfalls

1. **Adding glyph fields too early**
   - If `StyleShader` cannot mutate glyphs, a `glyph_ramp` field would be misleading schema. Do not add it to the style-shader-only schema.

2. **Forgetting generated docs**
   - `ConfigSchema` generates schemas, but `effect_metadata.rs` must include a representative variant for docs/capability metadata.

3. **Missing one `SpatialShaderType` match arm**
   - The compiler will catch non-exhaustive matches in some places, but helpers like docs lists can be missed. Search `SpatialShaderType::RadialSpiral` and mirror nearby additions.

4. **NaN propagation**
   - Any NaN in params can poison colors. Always sanitize before math.

5. **Transparent channel mutation**
   - Existing shaders preserve transparent fg/bg. Preserve that behavior.

6. **Overly bright background**
   - Background should usually blend less strongly than foreground to avoid solid flashing rectangles. Use a multiplier like `0.65` for bg in `Both` mode.

7. **V3 family mismatch**
   - Put this in `motion_field`, not `material_light`; it is a dynamic field first.

## 19. Suggested commit structure

Use small, reviewable commits if committing manually:

1. `Add terminal water shader primitive math`
   - new shader file, tests, module export.
2. `Register terminal water in spatial shader catalog`
   - `SpatialShaderType` integration and enum tests.
3. `Thread terminal water through V3 motion-field lowering`
   - V3 behavior/conversion/lowering tests.
4. `Document terminal water shader schema and recipes`
   - docs metadata, generated docs, debug recipes.

Follow Lore commit protocol in repository AGENTS if asked to commit.

## 20. Acceptance criteria

Implementation is complete when all of these are true:

- `serde_json` can deserialize `{ "type": "terminal_water", ... }` into `SpatialShaderType::TerminalWater`.
- `TerminalWaterShader::style_at` changes visible fg/bg according to `apply_to` and preserves transparent channels.
- Field samples are finite and bounded for normal and invalid parameter inputs.
- Field/light values vary by position and time.
- `SpatialShaderType::name`, `terse_description`, and `key_parameters` include `TerminalWater`.
- `v3_spatial_shader_family()` classifies the shader as primitive `motion_field`.
- V3 `TerminalWater` lowers back to executable `SpatialShaderType::TerminalWater`.
- `docs/generated/effect_schemas.json` includes `terminal_water` under shader variants, including `mode`, ripple emitters, rain controls, flow controls, wake sources, and glint controls.
- Primitive debug recipe validates in `/usr/projects/tui-vfx-recipes`.
- Targeted tests pass.
- Existing shaders and recipes are unchanged.

## 20.5 Future weather/wind relationship

The planned weather feature can reuse this water work later, but do not expand the current water implementation around wind until the water behaviors are working and tested. Ocean/ripple/flow math gives a natural bridge to weather:

```rust
pub struct WaterWind {
    pub direction_deg: f32,
    pub speed: f32,
    pub gust_strength: f32,
    pub turbulence: f32,
}
```

Future mapping:

```rust
let wind_speed_norm = wind.speed.clamp(0.0, 1.0);
wavelength = lerp(18.0, 6.0, wind_speed_norm);
amplitude = lerp(0.08, 0.55, wind_speed_norm);
steepness = lerp(0.10, 0.75, wind_speed_norm);
speed = lerp(0.30, 2.20, wind_speed_norm);
direction_deg = wind.direction_deg;
```

Weather uses this to express:

- calm pond / low wind;
- windy lake / directional chop;
- storm rain surface / many ripples;
- gust bands / temporary glint and amplitude boosts;
- wind-driven current/flow.

Keep this section as future design context only. The current plan should expose enough water controls that a weather layer can derive presets later without needing to redesign the shader.

## 21. Extension roadmap

### 21.1 Glyph-capable 256-braille water

> **Status: Implemented (2026-04).** Landed as part of the shared
> glyph-rendering framework rather than as a water-specific primitive.
> See `completed/tui-vfx-glyph-rendering-framework-plan.md` and the debug recipe
> `shader_terminal_water_glyph_v3.json` in `tui-vfx-recipes`. The
> framework's `GlyphEncoder::BrailleSubcell` is the high-quality mode the
> §21.1 sketch called out; the framework also provides `BrailleEighths`
> (eighths dot count, equivalent to the legacy `BrailleDensity` sketch)
> and the block-bar / ramp variants for consumers that don't want full
> subcell sampling. The original §21.1 sketch is preserved below for
> archaeology.

Add a filter or content primitive that can mutate cell glyphs. It should reuse the same water field math and provide:

```rust
pub enum WaterGlyphMode {
    Preserve,
    BrailleDensity,
    BrailleSubcell,
}
```

`BrailleSubcell` should be the high-quality mode.

### 21.2 Runtime ripple inputs

Add optional runtime inputs where not already covered by path/cursor infrastructure:

- cursor position;
- click/impact emitters;
- ripple age/amplitude/radius.

These should perturb `height` or `light_scalar`, not replace the wave model.

### 21.3 Spectral ocean

After the core water behaviors are stable:

- more layers or directional spectra;
- wind direction/speed presets;
- choppiness controls;
- approximate caustic bands.

Keep spectral mode opt-in because of performance.

## 22. Suggested implementation order for a junior developer

1. Create `cls_terminal_water_shader.rs` from the skeleton in Sections 5-8, including the mode enum and structs for ripple/rain/flow/wake with behavior math allowed to land in staged commits if needed.
2. Run `cargo test -p tui-vfx-style terminal_water` and fix compile errors.
3. Register exports in `models/mod.rs`.
4. Add `SpatialShaderType::TerminalWater` and all match arms from Section 4.3.
5. Add/adjust spatial enum tests from Section 9.
6. Add V3 behavior/conversions/lowering from Sections 4.4-4.6.
7. Add V3 tests from Section 10.
8. Update `xtask/src/docs/effect_metadata.rs` and run docs generation.
9. Add primitive debug recipe in `tui-vfx-recipes`.
10. Run all verification commands in Section 16.
11. If any broad test fails due to unrelated existing failures, capture exact output and still ensure targeted tests pass.

