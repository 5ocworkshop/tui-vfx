<!-- <FILE>docs/API_HAND.md</FILE> - <DESC>Hand-maintained TUI-VFX API documentation</DESC> -->
<!-- <VERS>VERSION: 2.10.0</VERS> -->
<!-- <WCTX>Fix shader speed documentation after speed-truncation bugfix</WCTX> -->
<!-- <CLOG>Add timing contract section; clarify speed is caller-controlled via loop_t, not shader-internal</CLOG> -->

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
| Filters | 14 (+ `None`) | `tui_vfx_compositor::types::FilterSpec` |
| Samplers | 6 (+ `None`) | `tui_vfx_compositor::types::SamplerSpec` |
| Spatial Shaders | 18 | `tui_vfx_style::models::SpatialShaderType` |
| Style Effects | 11 | `tui_vfx_style::models::StyleEffect` |
| Content Transformers | 13 | `tui_vfx_content::types::ContentEffect` |
| Shadows | 4 styles | `tui_vfx_shadow::ShadowConfig` / `ShadowSpec` |
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

**Use this for everything.** It accepts the full `CompositionOptions` superset,
including shadows and `preserve_unfilled`.

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

Uses `CompositionSpec` (serializable) and `SpatialShaderType` layers, including shadow
and `preserve_unfilled` support.

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

**Builder methods (ergonomic entry point):**
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
- `ShadowSpec` is serializable and wraps `ShadowConfig` (style, edges, soft edges).
- `preserve_unfilled` defaults to `true` to match runtime behavior.

## Shader Layers

```rust
pub struct ShaderWithRegion<'a> {
    pub shader: &'a dyn StyleShader,
    pub region: StyleRegion,
}

pub struct ShaderLayerSpec {
    pub shader: SpatialShaderType,
    pub region: StyleRegion,
}
```

Use `ShaderWithRegion` for runtime shader instances and `ShaderLayerSpec` for serialized specs.

## Timing: `t`, `loop_t`, `phase`

- `t`: primary animation progress (0.0 → 1.0). Drives one-shot effects (masks, fades).
- `loop_t`: optional looped time (0.0 → 1.0 repeating). **Required for continuous shader effects.** The compositor clamps `shader_t` to `[0, 1]` before passing it to shaders.
- `phase`: optional phase (from `mixed_signals::traits::Phase`) for enter/dwell/exit semantics.

### Shader timing contract (important)

Spatial shaders (GlistenBand, BorderSweep, Radar, Reflect, Orbit, etc.) treat `t` as a
**pure position parameter**: `t=0.0` = effect at start position, `t=1.0` = effect at end
position. Shaders do **not** scale `t` internally — the `speed` field on shader structs is
retained for serialization compatibility but is **not used during rendering** (as of v0.2.2).

**The caller controls sweep rate by driving `loop_t`:**

```rust
// Example: glisten that completes one sweep every 3 seconds
let speed = 0.3; // from theme config
let loop_t = (elapsed_seconds as f64 * speed as f64).fract();
let options = CompositionOptions {
    t: phase_progress,
    loop_t: Some(loop_t),
    ..Default::default()
}.with_shader_layer(&shader, StyleRegion::All);
```

If you omit `loop_t`, the compositor falls back to `t` (phase progress), which means
the shader sweeps once over the phase duration — appropriate for enter/exit animations
but not for continuous dwell effects.

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
| `None` | Fully visible | - |
| `Wipe` | Linear edge reveal/hide | `reveal`, `hide`, `direction`, `soft_edge` |
| `Dissolve` | Random pixel reveal | `seed`, `chunk_size` |
| `Checkers` | Checkerboard pattern | `cell_size` |
| `Blinds` | Venetian blinds | `orientation`, `count` |
| `Iris` | Spotlight/iris reveal | `shape`, `soft_edge` |
| `Diamond` | Diamond expand | `soft_edge` |
| `NoiseDither` | Dithered noise | `seed`, `matrix` |
| `PathReveal` | Spiral / radial sweep | `path`, `soft_edge` |
| `Radial` | Radial expansion | `origin`, `soft_edge` |
| `Cellular` | Organic cells | `pattern`, `seed`, `cell_count` |

### Wipe semantics

`Wipe` supports **reveal** or **hide** direction:
- `reveal`: content appears traveling in this direction
- `hide`: content disappears traveling in this direction
- `direction`: legacy alias for `reveal`

Exactly one of `reveal`, `hide`, or `direction` should be set.

### WipeDirection (16 variants)

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

### Other mask enums

- `Orientation`: `Horizontal`, `Vertical`
- `IrisShape`: `Circle`, `Diamond`, `Box`
- `DitherMatrix`: `Bayer4`, `Bayer8`
- `RadialOrigin`: `Center`, `TopLeft`, `TopRight`, `BottomLeft`, `BottomRight`, `Custom { x, y }`
- `RevealPathType`:
  - `Spiral { rotations, direction }`
  - `Radial { start_angle, direction }`
- `SpiralDirection`: `Clockwise`, `CounterClockwise`
- `CellularPattern`: `Voronoi`, `Hexagonal`, `Organic`

### MaskCombineMode

| Mode | Behavior |
| --- | --- |
| `All` | Visible only if ALL masks pass (AND) |
| `Any` | Visible if ANY mask passes (OR) |
| `Blend { ratio }` | Smooth blend between masks (ratio 0.0 → 1.0) |

---

## FilterSpec (14 effects)

Filters modify cell colors/styles after rendering (applied in order).

| Variant | Description | Parameters |
| --- | --- | --- |
| `None` | No effect | - |
| `Dim` | Darken output | `factor: SignalOrFloat`, `apply_to: ApplyTo` |
| `Invert` | Invert colors | `apply_to` |
| `Tint` | Color overlay | `color: ColorConfig`, `strength: SignalOrFloat`, `apply_to` |
| `Vignette` | Edge darkening | `strength: SignalOrFloat`, `radius: SignalOrFloat` |
| `Crt` | CRT scanlines/glow | `scanline_strength: SignalOrFloat`, `glow: SignalOrFloat` |
| `PatternFill` | Background texture | `pattern: PatternType`, `color: Option<ColorConfig>`, `only_empty: bool` |
| `Greyscale` | BT.601 desaturation | `strength: SignalOrFloat`, `apply_to` |
| `BrailleDust` | Animated braille dust | `density`, `hz`, `seed`, `pattern`, `color` |
| `InterlaceCurtain` | Scanline dimming | `density`, `dim_factor`, `scroll_speed` |
| `MotionBlur` | Directional trail | `trail_length`, `opacity_decay`, `direction` |
| `ColorBridgedShade` | Shade chars (░▒▓█) | `opacity`, `fg_color`, `bg_color` |
| `SubPixelBar` | 8x progress bar | `progress`, `direction`, `filled_color`, `unfilled_color`, `animated` |
| `SubCellShake` | Partial-block vibration | `amplitude`, `frequency`, `seed`, `edge_only`, `filled_color`, `bg_color` |
| `RigidShake` | Damped rigid shake | `shake_period`, `num_shakes`, `pause_duration`, `max_eighths`, `base_eighths`, `damping`, `element_color`, `bg_color`, `inner_width`, `margin_width` |

### ApplyTo
`Foreground`, `Background`, `Both` (default). Aliases: `fg`, `bg`.

### PatternType
- `Single { char }`
- `Checkerboard { char_a, char_b }`
- `HorizontalLines { line_char, spacing }`
- `VerticalLines { line_char, spacing }`

### BraillePatternType
`SingleDot`, `OneToTwoDots`, `OneToThreeDots`, `OneToFourDots`

### MotionBlurDirection
`Left`, `Right`, `Up`, `Down`

### SubPixelBarDirection
`Horizontal` (▏▎▍▌▋▊▉█), `Vertical` (▁▂▃▄▅▆▇█)

### Notes
- **RigidShake** requires **margin cells** around the widget area. Apply to an area that
  includes the margins so the partial-block extensions can render.
- **SubCellShake** (filter) uses partial blocks to simulate physical vibration; **SubCellShake**
  (shader) is a color oscillation effect.

### Signal-driven parameters (`SignalOrFloat`)

Many filter, sampler, and content parameters accept `SignalOrFloat` from `mixed_signals`,
allowing either static values or values driven by external signals at runtime.
Use `SignalOrFloat::Static(...)` for fixed values, or the signal form for dynamic control
based on your `SignalContext`.

---

## SamplerSpec (6 effects)

Samplers transform coordinate space before cell lookup.

| Variant | Description | Parameters |
| --- | --- | --- |
| `None` | No transform | - |
| `SineWave` | Sinusoidal wave | `axis`, `amplitude`, `frequency`, `speed`, `phase` |
| `Ripple` | Circular ripple | `amplitude`, `wavelength`, `speed`, `center` |
| `Shredder` | Paper shredder | `stripe_width`, `odd_speed`, `even_speed` |
| `FaultLine` | Displacement fault | `seed`, `intensity`, `split_bias` |
| `Crt` | CRT distortion | `scanline_strength`, `jitter`, `curvature` |
| `CrtJitter` | CRT crash/jitter | `intensity`, `speed_hz`, `decay_ms` |

### Axis
`X`, `Y`

### RippleCenter
`Center`, `Point { x, y }`

---

# Part 2: Shadows

Shadows come from the `tui-vfx-shadow` crate and can be rendered **directly**
or **integrated** into the compositor via `ShadowSpec`.

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
}
```

## ShadowStyle
- `HalfBlock` (default) — best quality sub-cell shadows
- `Braille { density }` — 2x4 subpixel grid, font-dependent
- `Solid` — maximum compatibility
- `Gradient { layers }` — multi-layer soft shadow

## ShadowEdges (bitflags)
`RIGHT`, `BOTTOM`, `LEFT`, `TOP`, plus convenience `BOTTOM_RIGHT`, `TOP_LEFT`, `ALL`.

**Rule:** edges only render when the offset direction matches (e.g., `RIGHT` requires `offset_x > 0`).

## Compositor integration

```rust
let options = CompositionOptions::default()
    .with_shadow(ShadowSpec::new(my_shadow_config));
```

For full guidance and examples, see `docs/HOWTO_SHADOWS.md`.

---

# Part 3: Style System

## StyleRegion

Apply styles to targeted regions:

`All`, `TextOnly`, `BorderOnly`, `BackgroundOnly`, `Rows(Vec<u16>)`, `RowRange { start, end }`,
`Cell { x, y }`, `Cells(Vec<CellCoord>)`, `Column(u16)`, `Columns(Vec<u16>)`,
`ColumnRange { start, end }`, `Modulo { axis, modulus, remainder }`

`ModuloAxis`: `Horizontal`, `Vertical`

## Spatial Shaders (`SpatialShaderType`)

These are serializable shader variants for use in `CompositionSpec` and `ShaderLayerSpec`.

| Shader | Parameters |
| --- | --- |
| `LinearGradient` | `gradient: Gradient`, `angle_deg` |
| `BarberPole` | `speed`, `stripe_width`, `gap_width`, `color` |
| `Radar` | `speed`, `tail_length`, `color` |
| `BorderSweep` | `speed`, `length`, `color` |
| `Highlighter` | `color` |
| `Reflect` | `speed`, `color` |
| `ChromaticEdge` | `intensity`, `edge_width`, `horizontal` |
| `GlistenBand` | `speed`, `band_width`, `angle_deg`, `head`, `tail`, `direction`, `repeat_count`, `apply_to`, `blend_strength` |
| `GlitchLines` | `seed`, `intensity`, `max_lines`, `speed`, `flash_chance`, `pulse_color`, `pulse_speed`, `italic_on_flash`, `flash_hold`, `noise_type` |
| `NeonFlicker` | `stability`, `seed`, `segment`, `dim_amount`, `speed`, `flash_chance`, `decay_rate`, `noise_type` |
| `PulseWave` | `frequency`, `speed`, `color`, `direction`, `wavelength` |
| `FocusedRowGradient` | `selected_row`, `selected_row_ratio`, `falloff_distance`, `bright_color`, `dim_color`, `apply_to` |
| `RevealWipe` | `direction` |
| `StochasticSparkle` | `sparkle_density`, `brightness_boost`, `speed`, `seed`, `apply_to`, `noise_type` |
| `AmbientOcclusion` | `intensity`, `radius`, `edges`, `falloff`, `shadow_color` |
| `Bevel` | `light_direction`, `highlight_intensity`, `shadow_intensity`, `edge_width` |
| `Glow` | `color`, `radius`, `falloff`, `intensity`, `pulse_speed` |
| `SubCellShake` | `amplitude`, `frequency`, `axis`, `chromatic`, `seed`, `edge_only`, `edge_width` |

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
    FadeIn { apply_to: FadeApplyTo, ease: EasingCurve },
    FadeOut { apply_to: FadeApplyTo, ease: EasingCurve },
    Pulse { frequency: f32, color: Color },
    Rainbow { speed: f32 },
    Glitch { seed: u64, intensity: f32, italic_start: Option<f32>, italic_end: Option<f32> },
    NeonFlicker { stability: f32 },
    Spatial { shader: SpatialShaderType },
    ItalicWindow { start: f32, end: f32 },
    ColorShift { hue_shift: f32, saturation_shift: f32, lightness_shift: f32 },
    ColorFade { target: Color, color_space: ColorSpace },
    RigidShakeStyle { shake_period: f32, num_shakes: u8, pause_duration: f32 },
}
```

### StyleConfig

```rust
pub struct StyleConfig {
    pub fg: Option<ColorConfig>,
    pub bg: Option<ColorConfig>,
    pub add_modifier: Vec<ModifierConfig>,
    pub sub_modifier: Vec<ModifierConfig>,
}
```

`ModifierConfig`: `Bold`, `Dim`, `Italic`, `Underlined`, `SlowBlink`, `RapidBlink`,
`Reversed`, `Hidden`, `CrossedOut`.

### StyleLayer

```rust
pub struct StyleLayer {
    pub region: StyleRegion,
    pub enter_effect: Option<StyleEffect>,
    pub enter_region: Option<StyleRegion>,
    pub dwell_effect: Option<StyleEffect>,
    pub dwell_region: Option<StyleRegion>,
    pub exit_effect: Option<StyleEffect>,
    pub exit_region: Option<StyleRegion>,
}
```

### StyleTransition

```rust
pub struct StyleTransition {
    pub start: Style,
    pub end: Style,
    pub ease: EasingCurve,
    pub color_space: ColorSpace,
}
```

### FadeSpec / FadeChain

```rust
pub struct FadeSpec {
    pub from: FadeTarget,
    pub to: FadeTarget,
    pub apply_to: FadeApplyTo,
    pub ease: EasingCurve,
    pub space: ColorSpace,
    pub envelope: Option<FadeEnvelope>,
}
```

`FadeTarget`: `Black`, `White`, `Transparent`, `Base`, `Color { color: ColorConfig }`

`FadeApplyTo`: `Foreground`, `Background`, `Both`

`FadeEnvelope`: `{ attack, release }` for attack/hold/release shaping.

`FadeChain` combines segments: `FadeSegment { fade: FadeSpec, weight }`.

### Color configuration

```rust
pub enum ColorConfig {
    Reset, Black, Red, Green, Yellow, Blue, Magenta, Cyan, Gray, LightGray, DarkGray,
    LightRed, LightGreen, LightYellow, LightBlue, LightMagenta, LightCyan, White,
    Rgb { r: u8, g: u8, b: u8 },
    Indexed { value: u8 },
}
```

```rust
pub enum ColorSpace { Rgb, Hsl }
```

```rust
pub enum BlendMode { Normal, Additive, Multiply, Screen, Overlay, Mix }
```

```rust
pub struct ColorRamp { pub stops: Vec<ColorStop>, pub space: ColorSpace }
pub struct ColorStop { pub position: f32, pub color: ColorConfig }
```

```rust
pub struct Gradient { pub stops: Vec<(f32, Color)>, pub space: ColorSpace }
```

---

# Part 4: Content Transformers

Content transformers modify text strings based on animation progress.

```rust
pub enum ContentEffect {
    Typewriter { speed_variance: SignalOrFloat, cursor: Option<TypewriterCursor> },
    Scramble { resolve_pace: SignalOrFloat, charset: ScrambleCharset, seed: u64 },
    GlitchShift { shift_amount: u8, glitch_start: SignalOrFloat, glitch_end: SignalOrFloat, seed: u64 },
    ScrambleGlitchShift { resolve_pace: SignalOrFloat, charset: ScrambleCharset, scramble_seed: u64,
        shift_amount: u8, glitch_start: SignalOrFloat, glitch_end: SignalOrFloat },
    SplitFlap { speed: SignalOrFloat, cascade: SignalOrFloat },
    Odometer,
    Redact { symbol: char },
    Numeric { format: String },
    Marquee { speed: SignalOrFloat, width: u16 },
    SlideShift { start_col: i16, end_col: i16, start_row: i16, shift_col: i16, shift_width: u16,
        row_shift: i16, line_mode: SlideShiftLineMode, flow_mode: SlideShiftFlowMode },
    Mirror { axis: MirrorAxis },
    Dissolve { replacement: DissolveReplacement, pattern: DissolvePattern, direction: DissolveDirection, seed: u64 },
    Morph { source: String, progression: MorphProgression, direction: MorphDirection, seed: u64 },
}
```

### Supporting types

**TypewriterCursor (struct):**
`character`, `blink_interval`, `show_while_typing`, `show_after_complete`

**ScrambleCharset:** `Alphanumeric`, `Binary`, `Matrix`, `Katakana`

**MirrorAxis:** `Horizontal`, `Vertical`

**SlideShiftLineMode:** `Block`, `FirstLineOnly`

**SlideShiftFlowMode:** `StayShifted`, `FlowBack`

**DissolveReplacement:** `Space`, `Dot`, `Custom(char)`

**DissolveDirection:** `LeftToRight`, `RightToLeft`, `CenterIn`, `CenterOut`

**DissolvePattern:**
`Sequential`, `Random`, `EdgeIn`, `EdgeOut`, `Clustered { cluster_size }`, `ByWord`, `ByLine`

**MorphDirection:** `LeftToRight`, `RightToLeft`, `Simultaneous`

**MorphProgression:**
`Linear`, `Scatter`, `Wave`, `Density`, `Binary`, `Braille`, `DensityReveal`, `DensityConceal`,
`BrailleReveal`, `BrailleRevealDown`, `BrailleWaveUp`, `BrailleWaveDown`, `BrailleRandomUp`,
`BrailleRandomDown`, `BrailleByWordUp`, `BrailleByWordDown`, `BrailleByLineUp`, `BrailleByLineDown`,
`BrailleHalfCellWipe`, `BrailleHalfCellWipeByWord`

---

# Part 5: Geometry & Motion

These types drive transitions, motion paths, easing, and layout resolution.

## TransitionSpec

```rust
pub struct TransitionSpec {
    pub duration_ms: u64,
    pub ease: EasingCurve,
    pub path: PathType,
    pub snap: SnappingStrategy,
    pub quantize_steps: Option<u32>,
}
```

## MotionSpec

```rust
pub struct MotionSpec {
    pub duration_ms: u64,
    pub ease: EasingCurve,
    pub path: PathType,
    pub snap: SnappingStrategy,
    pub from: Option<PlacementSpec>,
    pub via: Option<PlacementSpec>,
    pub to: Option<PlacementSpec>,
}
```

### SlideDirection

`Default`, `FromTop`, `FromBottom`, `FromLeft`, `FromRight`,
`FromTopLeft`, `FromTopRight`, `FromBottomLeft`, `FromBottomRight`

### PathType (motion paths)

- `Linear`
- `Arc { bulge }`
- `Bezier { control_x, control_y }` (quadratic spatial curve)
- `Spring { stiffness, damping }`
- `Bounce { bounces, decay }`
- `Squash`
- `Hover`
- `Rectilinear { x_first }`
- `Spiral { rotations }`
- `Step { steps }`
- `Projectile { arc_height, gravity }`
- `Friction { drag }`
- `Orbit { revolutions, direction }`
- `Pendulum { amplitude, oscillations, damping }`

### EasingCurve

```rust
pub enum EasingCurve {
    Type(EasingType),
    Bezier { x1: f32, y1: f32, x2: f32, y2: f32 },
}
```

### EasingType (selection)

`Linear`, `EaseInQuad`, `EaseOutQuad`, `EaseInOutQuad`, `EaseInCubic`, `EaseOutCubic`,
`EaseInOutCubic`, `EaseInQuart`, `EaseOutQuart`, `EaseInOutQuart`, `EaseInQuint`,
`EaseOutQuint`, `EaseInOutQuint`, `EaseInSine`, `EaseOutSine`, `EaseInOutSine`,
`EaseInExpo`, `EaseOutExpo`, `EaseInOutExpo`, `EaseInCirc`, `EaseOutCirc`, `EaseInOutCirc`,
`EaseInBack`, `EaseOutBack`, `EaseInOutBack`, `EaseInElastic`, `EaseOutElastic`,
`EaseInOutElastic`, `EaseInBounce`, `EaseOutBounce`, `EaseInOutBounce`

### SnappingStrategy

`Floor`, `Round`, `Stochastic { seed }`

### Placement and layout types

```rust
pub enum PlacementSpec {
    Absolute(Position),
    FramePermille { x_permille: u16, y_permille: u16 },
    Anchor { anchor: Anchor },
    Offscreen { direction: SlideDirection, margin_cells: u16 },
}
```

```rust
pub enum PositionSpec {
    Absolute(Position),
    FramePermille { x_permille: u16, y_permille: u16 },
}
```

```rust
pub enum AnchorSpec {
    Simple(Anchor),
    WithOffset {
        position: Anchor,
        offset_horizontal_percent: f32,
        offset_vertical_percent: f32,
        offset_horizontal_cells: i16,
        offset_vertical_cells: i16,
        offset_horizontal_pixels: i32,
        offset_vertical_pixels: i32,
    },
}
```

```rust
pub enum Origin {
    Center, TopLeft, TopCenter, TopRight, MiddleLeft, MiddleCenter,
    MiddleRight, BottomLeft, BottomCenter, BottomRight,
}
```

```rust
pub enum RectScaleSpec {
    RectScale { origin: Origin, min_width: u16, min_height: u16 },
    RectScaleSpring { origin: Origin, min_width: u16, min_height: u16, stiffness: f32, damping: f32 },
    Squash {
        origin: Origin, min_width: u16, min_height: u16,
        width_start: f32, width_end: f32, height_start: f32, height_end: f32,
    },
}
```

```rust
pub enum Shake {
    None,
    Uniform { intensity: f32, frequency: f32 },
    Horizontal { intensity: f32, frequency: f32 },
    Vertical { intensity: f32, frequency: f32 },
    Decay { intensity: f32, decay: f32, frequency: f32 },
}
```

```rust
pub struct Timeline { pub start_ms: u64, pub duration_ms: u64 }
pub struct Keyframe { pub time: f32, pub value: f32 }
pub struct KeyframeTimeline { pub keyframes: Vec<Keyframe>, pub easing: EasingType }
```

```rust
pub enum TimeWarpCurve {
    Linear { start: f32, end: f32 },
    Speed { multiplier: f32 },
    Sigmoid { steepness: f32 },
}
```

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
pub use tui_vfx_shadow::{ShadowConfig, ShadowEdges, ShadowStyle, render_shadow, render_shadow_simple};
```

---

<!-- <FILE>docs/API_HAND.md</FILE> - <DESC>Hand-maintained TUI-VFX API documentation</DESC> -->
<!-- <VERS>END OF VERSION: 2.10.0</VERS> -->
