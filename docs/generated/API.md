<!-- <FILE>docs/generated/API.md</FILE> - <DESC>Complete TUI-VFX API documentation</DESC> -->
<!-- <VERS>VERSION: 3.0.1</VERS> -->
<!-- <WCTX>Generated API documentation</WCTX> -->
<!-- <CLOG>Auto-generated from code + api_docs.toml</CLOG> -->

# TUI-VFX Complete API Reference

The Pipeline API is the **single unified interface** for applying visual effects to terminal
grids. You provide a source grid, configure a composition, and render a fully composited
frame into a destination grid.

**Key idea:** `CompositionOptions` is the superset entry point (supports every capability,
including shadows). `CompositionSpec` is the serializable, data-driven variant and now
supports shadows and `preserve_unfilled`.


---

## Effect Inventory Summary

| Category | Count (variants) | Primary API |
| --- | --- | --- |
| Masks | 10 (+ `None`) | `tui_vfx_compositor::types::MaskSpec` |
| Filters | 22 (+ `None`) | `tui_vfx_compositor::types::FilterSpec` |
| Samplers | 6 (+ `None`) | `tui_vfx_compositor::types::SamplerSpec` |
| Spatial Shaders | 17 (+ `None`) | `tui_vfx_style::models::SpatialShaderType` |
| Style Effects | 10 (+ `None`) | `tui_vfx_style::models::StyleEffect` |
| Content Transformers | 13 (+ `None`) | `tui_vfx_content::types::ContentEffect` |
| Shadows | 5 styles | `tui_vfx_shadow::ShadowConfig / ShadowSpec` |
| Geometry & Motion | 20+ | `tui_vfx_geometry::types::*` |

---

## Unified Entry Points

### `render_pipeline` (full superset)

```rust
pub fn render_pipeline(
    source: &dyn Grid,
    dest: &mut dyn Grid,
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    options: CompositionOptions<'_>,
    inspector: Option<&mut dyn CompositorInspector>,
)
```

**Use this for everything.** It accepts the full `CompositionOptions` superset, including shadows and `preserve_unfilled`.

### `render_pipeline_with_area`

```rust
pub fn render_pipeline_with_area(
    source: &dyn Grid,
    dest: &mut dyn Grid,
    area: RenderArea,
    options: CompositionOptions<'_>,
    inspector: Option<&mut dyn CompositorInspector>,
)
```

Convenience overload using `RenderArea { width, height, offset_x, offset_y }`.

### `render_pipeline_with_spec` (data-driven)

```rust
pub fn render_pipeline_with_spec(
    source: &dyn Grid,
    dest: &mut dyn Grid,
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    spec: &CompositionSpec,
    inspector: Option<&mut dyn CompositorInspector>,
)
```

Uses `CompositionSpec` (serializable) and `SpatialShaderType` layers, including shadow and `preserve_unfilled` support.

### `render_pipeline_with_spec_area`

```rust
pub fn render_pipeline_with_spec_area(
    source: &dyn Grid,
    dest: &mut dyn Grid,
    area: RenderArea,
    spec: &CompositionSpec,
    inspector: Option<&mut dyn CompositorInspector>,
)
```

---

## Quick Start (Pipeline)

```rust
use tui_vfx::prelude::*;

let options = CompositionOptions::default()
    .with_mask(MaskSpec::Dissolve { seed: 42, chunk_size: 1 })
    .with_filter(FilterSpec::Dim {
        factor: SignalOrFloat::Static(0.5),
        apply_to: ApplyTo::Both,
    });

render_pipeline(&source, &mut dest, width, height, 0, 0, options, None);
```

---

# Part 1: Compositor Pipeline

## CompositionOptions (runtime superset)

```rust
pub struct CompositionOptions<'a> {
    pub sampler_spec: Option<SamplerSpec>,
    pub masks: Cow<'a, [MaskSpec]>,
    pub mask_combine_mode: MaskCombineMode,
    pub filters: Cow<'a, [FilterSpec]>,
    pub shader_layers: SmallVec<[ShaderWithRegion<'a>; 2]>,
    pub shadow: Option<ShadowSpec>,
    pub preserve_unfilled: bool,
    pub t: f64,
    pub loop_t: Option<f64>,
    pub phase: Option<Phase>,
}
```

**Builder methods (ergonomic entry point)**
- `.with_mask(MaskSpec)` / `.with_masks(Cow<[MaskSpec]>)`
- `.with_filter(FilterSpec)` / `.with_filters(Cow<[FilterSpec]>)`
- `.with_mask_combine_mode(MaskCombineMode)`
- `.with_shader_layer(&dyn StyleShader, StyleRegion)`
- `.with_shadow(impl Into<ShadowSpec>)`
- `.with_preserve_unfilled(bool)`

## CompositionSpec (serializable)

```rust
pub struct CompositionSpec {
    pub sampler_spec: Option<SamplerSpec>,
    pub masks: Vec<MaskSpec>,
    pub mask_combine_mode: MaskCombineMode,
    pub filters: Vec<FilterSpec>,
    pub shader_layers: Vec<ShaderLayerSpec>,
    pub shadow: Option<ShadowSpec>,
    pub preserve_unfilled: bool,
    pub t: f64,
    pub loop_t: Option<f64>,
    pub phase: Option<Phase>,
}
```

**Notes:**
- Intended for JSON/TOML-driven configs.
- Uses `ShaderLayerSpec` + `SpatialShaderType` (serializable).
- `shadow` uses the same `ShadowSpec` as runtime `CompositionOptions`.
- `ShadowSpec` is serializable and wraps `ShadowConfig` (style, edges, soft edges, composite mode, grade).
- `preserve_unfilled` defaults to `true` to match runtime behavior.


## Shader Layers

```rust
pub struct ShaderWithRegion<'a> {
    pub shader: &'a dyn StyleShader,
    pub region: StyleRegion,
}
```

```rust
pub struct ShaderLayerSpec {
    pub shader: SpatialShaderType,
    pub region: StyleRegion,
}
```

Use `ShaderWithRegion` for runtime shader instances and `ShaderLayerSpec` for serialized specs.

## Timing: `t`, `loop_t`, `phase`

- `t`: primary animation progress (0.0 → 1.0).
- `loop_t`: optional looped time (0.0 → 1.0 repeating). Used by continuous effects.
- `phase`: optional phase (from `mixed_signals::traits::Phase`) for enter/dwell/exit semantics.

## Render Order

1. Sampler (coordinate distortion)
2. Shadow (if enabled)
3. Element content
4. Masks (applied to element + shadow together)
5. Filters (post-processing)
6. Shaders (per-cell styling)

---

## MaskSpec (10 effects)

Masks control cell visibility based on position and animation progress `t`.

| Variant | Description | Parameters |
| --- | --- | --- |
| `Blinds` | Venetian blinds effect | `orientation`, `count` |
| `Cellular` | Cellular/organic pattern reveal | `pattern`, `seed`, `cell_count` |
| `Checkers` | Checkerboard pattern reveal | `cell_size` |
| `Diamond` | Diamond-shaped expand from center | `soft_edge` |
| `Dissolve` | Random pixel dissolve effect | `seed`, `chunk_size` |
| `Iris` | Iris/spotlight reveal from center | `shape`, `soft_edge` |
| `NoiseDither` | Dithered noise pattern reveal | `seed`, `matrix` |
| `None` | No mask applied — content is fully visible | - |
| `PathReveal` | Path-based reveal (spiral, radial sweep) | `path`, `soft_edge` |
| `Radial` | Radial reveal expanding from configurable origin | `origin`, `soft_edge` |
| `Wipe` | Linear wipe reveal/hide from one edge to another | `soft_edge` |

### Wipe semantics

`Wipe` supports **reveal** or **hide** direction:
- `reveal`: content appears traveling in this direction
- `hide`: content disappears traveling in this direction
- `direction`: legacy alias for `reveal`

Exactly one of `reveal`, `hide`, or `direction` should be set.


### MaskCombineMode

How to combine multiple masks

| Mode | Behavior |
| --- | --- |
| `All` | Visible only if ALL masks pass (AND) |
| `Any` | Visible if ANY mask passes (OR) |
| `Blend { ratio }` | Smooth blend between masks (ratio 0.0 → 1.0) |

### WipeDirection

Direction for wipe transitions (16 variants)

Cardinal:
`LeftToRight`, `RightToLeft`, `TopToBottom`, `BottomToTop`

Diagonal:
`TopLeftToBottomRight`, `TopRightToBottomLeft`, `BottomLeftToTopRight`, `BottomRightToTopLeft`

Aliases:
`FromLeft`, `FromRight`, `FromTop`, `FromBottom`

Center-out (curtains opening):
`HorizontalCenterOut`, `VerticalCenterOut`

Edges-in (curtains closing):
`HorizontalEdgesIn`, `VerticalEdgesIn`

### Orientation

Orientation for effects

`Horizontal`, `Vertical`

### IrisShape

Shape for iris mask

`Circle`, `Diamond`, `Box`

### DitherMatrix

Dither matrix for noise dither mask

`Bayer4`, `Bayer8`

### RadialOrigin

Origin point for radial effects

`Center`, `TopLeft`, `TopRight`, `BottomLeft`, `BottomRight`, `Custom { x, y }`

### CellularPattern

Pattern for cellular mask

`Voronoi`, `Hexagonal`, `Organic`

---

## FilterSpec (22 effects)

Filters modify cell colors/styles after rendering (applied in order).

| Variant | Description | Parameters |
| --- | --- | --- |
| `BracketEmphasis` | Brackets that appear around content based on progress | `left`, `right`, `progress` |
| `BrailleDust` | Stochastic braille dust for frosted glass texture | `density`, `hz`, `seed` |
| `ColorBridgedShade` | Color-bridged shade for smooth opacity rendering | `opacity`, `fg_color`, `bg_color` |
| `Crt` | CRT monitor post-processing effect | `scanline_strength`, `glow` |
| `Dim` | Dim/darken the output | `factor`, `apply_to` |
| `DotIndicator` | Simple dot/bullet indicator adjacent to content | `indicator_char`, `position`, `progress` |
| `GlistenSweep` | Diagonal glisten sweep effect | `boost`, `band_width`, `speed`, `progress`, `powerline_mode`, `boost_separator_bg` |
| `Greyscale` | Greyscale/desaturate filter using BT.601 luminance | `strength`, `apply_to` |
| `HoverBar` | Progress-driven partial bar indicator for hover states | `base_eighths`, `max_eighths`, `position`, `progress` |
| `InterlaceCurtain` | Scanline/interlace effect for backdrop dimming | `density`, `dim_factor`, `scroll_speed` |
| `Invert` | Invert colors | `apply_to` |
| `KittScanner` | Horizontal ping-pong scanner effect (KITT/Larson) | `boost`, `band_width`, `bps`, `progress`, `apply_to`, `powerline_mode`, `boost_separator_bg` |
| `MotionBlur` | Motion blur trail effect with directional dimming | `trail_length`, `opacity_decay`, `direction` |
| `None` | No filter effect | - |
| `PatternFill` | Pattern fill effect for background textures | `pattern`, `color`, `only_empty` |
| `PillButton` | Pill-shaped button with gradient edges | `edge_width`, `glisten`, `progress` |
| `RigidShake` | Rigid body shake filter with damped oscillation | `shake_period`, `num_shakes`, `pause_duration`, `max_eighths` |
| `ShadeScanner` | Ping-pong scanner that dims text with shade overlay | `shade_color`, `bps`, `progress` |
| `SubCellShake` | Sub-cell shake using partial vertical blocks | `amplitude`, `frequency`, `seed`, `edge_only` |
| `SubPixelBar` | Sub-pixel progress bar with 8x resolution | `progress`, `direction` |
| `Tint` | Apply a color tint | `color`, `strength`, `apply_to` |
| `UnderlineWipe` | Horizontal underline that wipes in based on progress | `direction`, `progress`, `gradient`, `glisten` |
| `Vignette` | Vignette darkening around edges | `strength`, `radius` |

### RigidShake

**RigidShake** requires **margin cells** around the widget area. Apply to an area that includes the margins so the partial-block extensions can render.

### Signal-driven parameters (`SignalOrFloat`)

Many filter, sampler, and content parameters accept `SignalOrFloat` from `mixed_signals`,
allowing either static values or values driven by external signals at runtime.
Use `SignalOrFloat::Static(...)` for fixed values, or the signal form for dynamic control
based on your `SignalContext`.


### SubCellShake

**SubCellShake** (filter) uses partial blocks to simulate physical vibration; **SubCellShake** (shader) is a color oscillation effect.

### ApplyTo

Which color channels to apply effect to

`Foreground`, `Background`, `Both`

### PatternType

Pattern for pattern fill filter

`Single { char }`, `Checkerboard { char_a, char_b }`, `HorizontalLines { line_char, spacing }`, `VerticalLines { line_char, spacing }`

### BraillePatternType

Pattern for braille dust filter

`SingleDot`, `OneToTwoDots`, `OneToThreeDots`, `OneToFourDots`

### MotionBlurDirection

Direction for motion blur

`Left`, `Right`, `Up`, `Down`

### SubPixelBarDirection

Direction for sub-pixel progress bar

`Horizontal`, `Vertical`

---

## SamplerSpec (6 effects)

Samplers transform coordinate space before cell lookup.

| Variant | Description | Parameters |
| --- | --- | --- |
| `Crt` | CRT monitor effect with scanlines and curvature | `scanline_strength`, `jitter`, `curvature` |
| `CrtJitter` | CRT crash/jitter effect with decay | `intensity`, `speed_hz`, `decay_ms` |
| `FaultLine` | Fault line displacement effect | `seed`, `intensity`, `split_bias` |
| `None` | No coordinate transformation | - |
| `Ripple` | Circular ripple distortion from center point | `amplitude`, `wavelength`, `speed`, `center` |
| `Shredder` | Paper shredder effect with alternating strips | `stripe_width`, `odd_speed`, `even_speed` |
| `SineWave` | Sinusoidal wave distortion | `axis`, `amplitude`, `frequency`, `speed`, `phase` |

### Axis

Axis for wave effects

`X`, `Y`

### RippleCenter

Center point for ripple effect

`Center`, `Point { x, y }`

---

# Part 2: Shadows

Shadows come from the `tui-vfx-shadow` crate and can be rendered **directly**
or **integrated** into the compositor via `ShadowSpec`.

Shadow compositing is controlled by `composite_mode`: the default `GlyphOverlay` replaces
destination content with shadow glyphs, while `GradeUnderlying` preserves destination glyphs
and applies color grading (desaturate, dim, tint) scaled by shadow coverage.
Use `.with_dramatic_grade()` for a visible preset with stronger background than foreground grading.


## ShadowConfig

```rust
pub struct ShadowConfig {
    pub style: ShadowStyle,
    pub offset_x: i8,
    pub offset_y: i8,
    pub color: Color,
    pub surface_color: Option<Color>,
    pub edges: ShadowEdges,
    pub soft_edges: bool,
    pub composite_mode: ShadowCompositeMode,
    pub grade: Option<ShadowGradeConfig>,
}
```

## ShadowStyle
- `Braille` — Braille patterns for dithered/density-based shadows
- `Gradient` — Multi-layer gradient shadow with decreasing intensity
- `HalfBlock` — Half-block characters for soft sub-cell shadows
- `MediumShade` — Medium-shade character cells for textured full-cell shadows
- `Solid` — Solid color cells (space with background color)

## ShadowEdges (bitflags)
`RIGHT`, `BOTTOM`, `LEFT`, `TOP`, plus convenience `BOTTOM_RIGHT`, `TOP_LEFT`, `ALL`.

**Rule:** edges only render when the offset direction matches (e.g., `RIGHT` requires `offset_x > 0`).

## ShadowCompositeMode

- `GlyphOverlay` (default) — shadow glyphs replace destination content
- `GradeUnderlying` — destination glyphs preserved; color grading applied

## ShadowGradeConfig

Controls dim, desaturate, and tint strengths for grade-underlying mode. Use `ShadowGradeConfig::dramatic()` for the recommended visible preset.

## Compositor integration

```rust
// Standard glyph-overlay shadow (default)
let options = CompositionOptions::default()
    .with_shadow(ShadowSpec::new(my_shadow_config));

// Dramatic grade-underlying shadow
let dramatic_config = ShadowConfig::new(Color::BLACK.with_alpha(180))
    .with_offset(2, 1)
    .with_dramatic_grade();
let options = CompositionOptions::default()
    .with_shadow(ShadowSpec::new(dramatic_config));
```

For full guidance and examples, see `docs/HOWTO_SHADOWS.md`.

---

# Part 3: Style System

## StyleRegion

Apply styles to targeted regions:

`All`, `TextOnly`, `BorderOnly`, `BackgroundOnly`, `Rows(Vec<u16>)`, `RowRange { start, end }`, `Cell { x, y }`, `Cells(Vec<CellCoord>)`, `Column(u16)`, `Columns(Vec<u16>)`, `ColumnRange { start, end }`, `Modulo { axis, modulus, remainder }`

`ModuloAxis`: `Horizontal`, `Vertical`

## Spatial Shaders (`SpatialShaderType`)

These are serializable shader variants for use in `CompositionSpec` and `ShaderLayerSpec`.

| Shader | Parameters |
| --- | --- |
| `AmbientOcclusion` | `intensity`, `radius`, `edges`, `falloff` |
| `BarberPole` | `speed`, `stripe_width`, `gap_width` |
| `Bevel` | `light_direction`, `highlight_intensity`, `shadow_intensity`, `edge_width` |
| `BorderSweep` | `speed`, `length` |
| `ChromaticEdge` | `intensity`, `edge_width`, `horizontal` |
| `FocusedRowGradient` | `selected_row_ratio`, `falloff_distance`, `apply_to` |
| `GlistenBand` | `speed`, `band_width`, `direction`, `angle_deg` |
| `GlitchLines` | `intensity`, `max_lines`, `speed` |
| `Glow` | `radius`, `intensity`, `falloff`, `pulse_speed` |
| `Highlighter` | - |
| `LinearGradient` | `angle_deg` |
| `NeonFlicker` | `stability`, `segment`, `dim_amount` |
| `PulseWave` | `frequency`, `speed`, `direction`, `wavelength` |
| `Radar` | `speed`, `tail_length` |
| `Reflect` | `speed` |
| `RevealWipe` | `direction` |
| `StochasticSparkle` | `sparkle_density`, `brightness_boost`, `speed`, `apply_to` |
| `SubCellShake` | `amplitude`, `frequency`, `axis`, `chromatic` |

### Shader-specific supporting enums

- `GlistenDirection`: `Forward`, `Reverse`, `PingPong`
- `GlistenApplyTo`: `Foreground`, `Background`, `Both`
- `WaveDirection`: `Horizontal`, `Vertical`, `Radial`, `Diagonal`
- `ApplyToColor`: `Foreground`, `Background`, `Both`
- `RevealDirection`: `LeftToRight`, `RightToLeft`, `TopToBottom`, `BottomToTop`
- `NoiseType`: `Uniform`, `Gaussian`
- `SegmentMode`: `Cell`, `Row`, `Column`
- `AOEdges`: `BottomRight`, `TopLeft`, `All`, `Inner`
- `FalloffType`: `Linear`, `Quadratic`, `Exponential`
- `LightDirection`: `TopLeft`, `TopRight`, `BottomLeft`, `BottomRight`, `Top`, `Bottom`, `Left`, `Right`
- `ShakeAxis`: `Horizontal`, `Vertical`, `Both`
- `SparkleTarget`: `Foreground`, `Background`, `Both`

### Runtime-only shaders (trait objects)

If you implement a custom `StyleShader` at runtime, it can be used with
`CompositionOptions::with_shader_layer(&shader, ...)` but will not be serializable
via `SpatialShaderType` unless you add a new variant.


---

## StyleEffect (temporal effects)

```rust
pub enum StyleEffect {
    ColorFade { color_space: Rgb },
    ColorShift { hue_shift: 0deg, saturation_shift: 0, lightness_shift: 0 },
    FadeIn { apply_to: Both, ease: Type(Linear) },
    FadeOut { apply_to: Both, ease: Type(Linear) },
    Glitch { seed: 42, intensity: 0.3 },
    ItalicWindow { start: 0, end: 1 },
    NeonFlicker { stability: 0.8 },
    Pulse { frequency: 1 },
    Rainbow { speed: 1 },
    RigidShakeStyle { shake_period: 0.29s, num_shakes: 4, pause_duration: 0.52s },
    Spatial { angle_deg: 0 },
}
```

### StyleConfig

### StyleLayer

### StyleTransition

### FadeSpec / FadeChain

### Color configuration

---

# Part 4: Content Transformers

Content transformers modify text strings based on animation progress.

```rust
pub enum ContentEffect {
    Dissolve { replacement: Space, pattern: Random, direction: LeftToRight, seed: 0 },
    GlitchShift { shift_amount: 4, glitch_start: Static(0.0), glitch_end: Static(0.0), seed: 42 },
    Marquee { speed: Static(0.0), width: 20 },
    Mirror { axis: Horizontal },
    Morph { source: , progression: Linear, direction: LeftToRight, seed: 0 },
    Numeric { format: {} },
    Odometer,
    Redact { symbol: █ },
    Scramble { resolve_pace: Static(0.0), charset: Alphanumeric, seed: 42 },
    ScrambleGlitchShift { resolve_pace: Static(0.0), charset: Alphanumeric, scramble_seed: 42, shift_amount: 4, glitch_start: Static(0.0), glitch_end: Static(0.0) },
    SlideShift { start_col: 0, end_col: 0, start_row: 0, shift_col: 0, shift_width: 1, row_shift: 0, line_mode: Block, flow_mode: StayShifted },
    SplitFlap { speed: Static(0.0), cascade: Static(0.0) },
    Typewriter { speed_variance: Static(0.0) },
    WrapIndicator { prefix: » , suffix:  « },
}
```

### Supporting types

**ScrambleCharset:** `Alphanumeric`, `Binary`, `Matrix`, `Katakana`

**MirrorAxis:** `Horizontal`, `Vertical`

**SlideShiftLineMode:** `Block`, `FirstLineOnly`

**SlideShiftFlowMode:** `StayShifted`, `FlowBack`

**DissolveReplacement:** `Space`, `Dot`, `Custom(char)`

**DissolveDirection:** `LeftToRight`, `RightToLeft`, `CenterIn`, `CenterOut`

**DissolvePattern:** `Sequential`, `Random`, `EdgeIn`, `EdgeOut`, `Clustered { cluster_size }`, `ByWord`, `ByLine`

**MorphDirection:** `LeftToRight`, `RightToLeft`, `Simultaneous`

---

# Part 5: Geometry & Motion

These types drive transitions, motion paths, easing, and layout resolution.

## TransitionSpec

These types drive transitions, motion paths, easing, and layout resolution.

## MotionSpec

## SlideDirection

`Default`, `FromTop`, `FromBottom`, `FromLeft`, `FromRight`, `FromTopLeft`, `FromTopRight`, `FromBottomLeft`, `FromBottomRight`

## PathType (motion paths)

`Linear`, `Arc { bulge }`, `Bezier { control_x, control_y }`, `Spring { stiffness, damping }`, `Bounce { bounces, decay }`, `Squash`, `Hover`, `Rectilinear { x_first }`, `Spiral { rotations }`, `Step { steps }`, `Projectile { arc_height, gravity }`, `Friction { drag }`, `Orbit { revolutions, direction }`, `Pendulum { amplitude, oscillations, damping }`

## EasingCurve

## EasingType (selection)

`Linear`, `EaseInQuad`, `EaseOutQuad`, `EaseInOutQuad`, `EaseInCubic`, `EaseOutCubic`, `EaseInOutCubic`, `EaseInQuart`, `EaseOutQuart`, `EaseInOutQuart`, `EaseInQuint`, `EaseOutQuint`, `EaseInOutQuint`, `EaseInSine`, `EaseOutSine`, `EaseInOutSine`, `EaseInExpo`, `EaseOutExpo`, `EaseInOutExpo`, `EaseInCirc`, `EaseOutCirc`, `EaseInOutCirc`, `EaseInBack`, `EaseOutBack`, `EaseInOutBack`, `EaseInElastic`, `EaseOutElastic`, `EaseInOutElastic`, `EaseInBounce`, `EaseOutBounce`, `EaseInOutBounce`

## SnappingStrategy

`Floor`, `Round`, `Stochastic { seed }`

## Placement and layout types

## 

`Center`, `TopLeft`, `TopCenter`, `TopRight`, `MiddleLeft`, `MiddleCenter`, `MiddleRight`, `BottomLeft`, `BottomCenter`, `BottomRight`

## 

`None`, `Uniform { intensity: f32, frequency: f32 }`, `Horizontal { intensity: f32, frequency: f32 }`, `Vertical { intensity: f32, frequency: f32 }`, `Decay { intensity: f32, decay: f32, frequency: f32 }`

---

# Part 6: Prelude & Imports

Recommended for most users:

```rust
use tui_vfx::prelude::*;
```

The prelude includes:

```rust
// Types
pub use tui_vfx_types::{Anchor, BoundaryMode, Cell, Color, Grid, GridExt, Modifiers,
    OwnedGrid, Point, Rect, Size, Style};


// Core schema
pub use tui_vfx_core::{ConfigSchema, FieldMeta, Range, ScalarValue, SchemaField, SchemaNode};


// Geometry
pub use tui_vfx_geometry::easing::EasingType;


// Compositor pipeline
pub use tui_vfx_compositor::pipeline::{
    CompositionOptions, CompositionSpec, ShaderLayerSpec, ShaderWithRegion, ShadowSpec,
    render_pipeline, render_pipeline_with_spec, render_pipeline_with_spec_area,
};


// Compositor types
pub use tui_vfx_compositor::types::{
    ApplyTo, Axis, DitherMatrix, FilterSpec, IrisShape, MaskCombineMode, MaskSpec, Orientation,
    RippleCenter, SamplerSpec, WipeDirection,
};


// Style
pub use tui_vfx_style::models::{
    BlendMode, ColorConfig, ColorRamp, ColorSpace, FadeDirection, FadeEffect, FadeSpec,
    Gradient, StyleConfig, StyleEffect, StyleLayer, StyleTransition,
};


// Content
pub use tui_vfx_content::prelude::*;


// Shadows
pub use tui_vfx_shadow::{ShadowCompositeMode, ShadowConfig, ShadowEdges, ShadowGradeConfig, ShadowStyle, render_shadow, render_shadow_simple};

```

---

<!-- <FILE>docs/generated/API.md</FILE> - <DESC>Complete TUI-VFX API documentation</DESC> -->
<!-- <VERS>END OF VERSION: 3.0.1</VERS> -->
