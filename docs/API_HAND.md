<!-- <FILE>docs/API_HAND.md</FILE> - <DESC>Hand-maintained TUI-VFX API documentation</DESC> -->
<!-- <VERS>VERSION: 2.18.0</VERS> -->
<!-- <WCTX>Audit recommendations 2.1 + 2.2 — extend the LinearGradient row of the spatial-shader fields table with the new `apply_to` and `intensity` fields and the projection-based angle semantics. Note the relationship to the `gradient_overlay` authoring sugar.</WCTX> -->
<!-- <CLOG>2.18.0: MINOR — LinearGradient row updated to list apply_to (LinearGradientApplyTo) and intensity (f32, 0–1, default 1.0). Note added that the gradient_overlay authoring sugar canonicalises to this shape and that both fields survive the canonicalisation. Note added that angle_deg is now a true projection axis at any angle.
2.17.0: MINOR — WipeDirection section grew from 16 to 24 variants (corner-out and corner-in Euclidean quadrant arcs added) with a note distinguishing the corner-arc wavefront from the Manhattan-diagonal sweep. Documented `corner_down_*` / `corner_up_*` author-friendly serde aliases. Noted the shared canonical home in tui-vfx-geometry.
2.16.0: MINOR — Document the tui_vfx_content::cursor module (Cursor, CursorBlink, GrowIn, Wake, CursorState, CursorPaintOps, and the fnc_advance_cursor / fnc_render_cursor / fnc_cursor_grow_in_glyph helpers)</CLOG>

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

### `run_probe` (structured observability)

```rust
pub fn run_probe(
    scene: &ProbeSceneSpec,
    request: &ProbeRequest,
) -> Result<ProbeReport, ProbeError>
```

Engine-owned single-frame observability entry point from `tui-vfx-probe`.

Phase-1 surface:
- direct-engine input via `ProbeSceneSpec`
- selectors: `all`, `non-empty`, `modified`
- JSON-friendly output via `ProbeReport`
- compositor-stage `last_touch` attribution and richer trace events (sampler source coords, mask visibility, shader/filter before/after snapshots, including full foreground and background color data)

### `pipeline-probe` (CLI)

```bash
pipeline-probe \
  --input probe-scene.json \
  --format json|ndjson \
  --phase entering|dwelling|exiting \
  --sample-t 0.0..1.0 \
  --cells all|non-empty|modified \
  [--with-causation]
```

This binary lives in `crates/tui-vfx-probe` and is intended for direct engine scenes rather
than recipe JSON. Additional phase-1.5 flags:
- `--frames N` — emit a `ProbeTimelineReport` sampled evenly across the phase
- `--diff-to T` — emit a `ProbeDiffReport` comparing `--sample-t` against another phase-local time

Use `pipeline-validator` in the sibling `tui-vfx-recipes` repo when you need recipe parse/rules/profile stages.

---

## Probe Types

### `ProbeSceneSpec`

```rust
pub struct ProbeSceneSpec {
    pub source: ProbeGridSpec,
    pub destination: ProbeGridSpec,
    pub widget_offset: ProbePoint,
    pub composition: CompositionSpec,
}
```

Wraps the full direct-engine input seam required for one probe run:
- source grid
- destination frame
- widget placement
- serialized composition config

### `ProbeRequest`

```rust
pub struct ProbeRequest {
    pub phase: ProbePhase,
    pub sample_t: f64,
    pub cells: ProbeCellSelector,
    pub with_causation: bool,
}
```

### `ProbeReport`

Top-level single-frame report containing:
- `request` and `timing`
- `frame` and `widget` geometry
- `pipeline` inventory counts
- `summary` (`total_cells`, `non_empty_cells`, `modified_cells`)
- emitted `cells[]`
- per-cell `last_touch` and optional `trace[]`

For SQL-backed analysis, trace snapshots now persist both foreground and
background before/after data, which is especially important for verifying
background-only effects such as sweeps and glow-band interactions.

### `ProbeTimelineReport`

Wraps `frames: Vec<ProbeReport>` sampled evenly across one phase.

### `ProbeDiffReport`

Emits only changed cells between `from_t` and `to_t`, each with:
- `before` snapshot
- `after` snapshot
- optional `last_touch` / `trace` from the later sample

**Current limitation:** probe traces are now useful for compositor stages, but style/content-stage hooks are still pending.

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
- `.with_runtime_params(Arc<ShaderRuntimeParams>)`
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
    pub runtime_params: ShaderRuntimeParams,
}
```

**Notes:**
- Intended for JSON/TOML-driven configs.
- Uses `ShaderLayerSpec` + `SpatialShaderType` (serializable).
- `shadow` uses the same `ShadowSpec` as runtime `CompositionOptions`.
- `ShadowSpec` is serializable and wraps `ShadowConfig` (style, edges, soft edges).
- `preserve_unfilled` defaults to `true` to match runtime behavior.
- `runtime_params` is runtime-only today (not serialized) and feeds dynamic shader bindings.

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

## Runtime shader params / bindings

Spatial shaders can now read render-time values from `ShaderRuntimeParams`.

Example:

```rust
let runtime_params = [("selected_row", 7_u16)]
    .into_iter()
    .collect::<ShaderRuntimeParams>();

let shader = SpatialShaderType::FocusedRowGradient(FocusedRowGradientShader {
    selected_row: None,
    selected_row_binding: Some("selected_row".to_owned()),
    selected_row_ratio: 0.5,
    selected_row_ratio_binding: None,
    falloff_distance: 4,
    bright_color: ColorConfig::White,
    dim_color: ColorConfig::Black,
    apply_to: ApplyToColor::Background,
});
```

At render time, pass `runtime_params` through `CompositionOptions` / `CompositionSpec`.
This is intended for dynamic widget state such as selected row, scroll ratio, cursor
position, or hover/focus coordinates.

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

### WipeDirection (24 variants)

Defined in `tui-vfx-geometry` and shared by the `Wipe` mask, the
`RevealWipe` shader (`RevealDirection = WipeDirection`), and the V3
grouped reveal family (`VfxRevealDirection = WipeDirection`).

Cardinal:
`LeftToRight`, `RightToLeft`, `TopToBottom`, `BottomToTop`

Diagonal (Manhattan sweep, straight-line wavefront):
`TopLeftToBottomRight`, `TopRightToBottomLeft`, `BottomLeftToTopRight`, `BottomRightToTopLeft`

Aliases:
`FromLeft`, `FromRight`, `FromTop`, `FromBottom`

Center-out (curtains opening):
`HorizontalCenterOut`, `VerticalCenterOut`

Edges-in (curtains closing):
`HorizontalEdgesIn`, `VerticalEdgesIn`

Corner-out (Euclidean quadrant arc expanding from a corner;
authoring aliases `corner_down_top_left` / `corner_down_top_right` /
`corner_up_bottom_left` / `corner_up_bottom_right`):
`CornerOutFromTopLeft`, `CornerOutFromTopRight`,
`CornerOutFromBottomLeft`, `CornerOutFromBottomRight`

Corner-in (Euclidean quadrant arc collapsing toward a corner):
`CornerInToTopLeft`, `CornerInToTopRight`,
`CornerInToBottomLeft`, `CornerInToBottomRight`

Corner-arc variants are **distinct** from the Manhattan-diagonal
variants: diagonal sweep is a slanted-line wavefront; corner arc is
a quarter-circle wavefront. Both are intentionally preserved.

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
| `BrailleDust` | Animated braille dust | `density`, `hz`, `seed`, `pattern: BraillePatternType`, `color: Option<ColorConfig>`, `drift` |
| `CharsetNoise` | Time-varying char replacement (living textures) | `hz`, `seed`, `jitter`, `affect: CharsetNoiseAffect`, `chars: Option<String>` (flat) or `gradient: Option<Vec<CharsetNoiseGradientStop>>` (position-aware) |
| `InterlaceCurtain` | Scanline dimming | `density`, `dim_factor`, `scroll_speed` |
| `MotionBlur` | Directional trail | `trail_length`, `opacity_decay`, `direction: MotionBlurDirection` |
| `ColorBridgedShade` | Shade chars (░▒▓█) | `opacity`, `fg_color`, `bg_color` |
| `SubPixelBar` | 8x progress bar | `progress`, `direction: SubPixelBarDirection`, `filled_color`, `unfilled_color`, `animated: bool` |
| `SubCellShake` | Partial-block vibration | `amplitude`, `frequency`, `seed`, `edge_only`, `filled_color`, `bg_color` |
| `RigidShake` | Damped rigid shake | `shake_period`, `num_shakes`, `pause_duration`, `max_eighths`, `base_eighths`, `damping: Vec<f32>`, `element_color`, `bg_color`, `inner_width`, `margin_width` |
| `HoverBar` | Progress-driven partial bar | `base_eighths`, `max_eighths`, `position: HoverBarPosition`, `bar_color`, `bg_color`, `progress`, `margin_width` |
| `UnderlineWipe` | Horizontal underline wipe-in | `direction: WipeDirection`, `color`, `bg_color`, `line_char`, `row_offset`, `progress`, `gradient: bool`, `glisten: bool` |
| `BracketEmphasis` | Fade-in brackets around content | `left: char`, `right: char`, `color`, `bg_color`, `progress` |
| `DotIndicator` | Dot/bullet marker | `indicator_char: char`, `position: HoverBarPosition`, `color`, `bg_color`, `progress` |
| `PillButton` | Pill button with gradient edges | `button_color`, `bg_color`, `edge_width`, `glisten: bool`, `progress` |
| `GlistenSweep` | Diagonal 45° highlight sweep | `boost: u8`, `band_width: f32`, `speed: f32`, `progress: f32`, `powerline_mode: bool`, `boost_separator_bg: bool` |
| `KittScanner` | Horizontal scanner sweep (ping-pong or one-way wrap) | `boost: u8`, `band_width: f32`, `bpm: Option<f32>`, `bps: f32`, `progress: f32`, `motion_mode`, `apply_to`, `powerline_mode: bool`, `boost_separator_bg: bool` |
| `ShadeScanner` | Ping-pong scanner w/ shade overlay | `shade_color`, `bps: f32`, `progress: f32` |

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

### HoverBarPosition
`Left` (default), `Right`, `Top`, `Bottom` — reused by `HoverBar` and `DotIndicator`.

### WipeDirection (UnderlineWipe)
`LeftToRight` (default), `RightToLeft`, `TopToBottom`, `BottomToTop`, plus diagonals,
center-out/edges-in variants, and `FromLeft`/`FromRight`/`FromTop`/`FromBottom` aliases.
See `cls_mask_spec.rs::WipeDirection`.

### CharsetNoiseAffect
`All` (replace all cells including whitespace), `NonEmpty` (default — skip whitespace).

### Notes
- **RigidShake** requires **margin cells** around the widget area. Apply to an area that
  includes the margins so the partial-block extensions can render.
- **SubCellShake** (filter) uses partial blocks to simulate physical vibration; **SubCellShake**
  (shader) is a color oscillation effect.
- **GlistenSweep / KittScanner** do not have a `color` field — they apply an additive
  `boost` (u8) to existing cell colors, so drive palette through `base_style` and use
  the filter only for temporal motion. `KittScanner` now accepts either
  human-readable `bpm` or raw `bps`; `bpm` takes precedence when both are set, and
  the default cadence is 72 BPM (`bps = 1.2`). Cadence-driven motion uses
  monotonic elapsed time, so recipe loop period only controls how often the
  surrounding recipe repeats. `KittScanner.motion_mode` controls whether the
  scan ping-pongs (`ping_pong`) or wraps one-way (`forward_wrap` /
  `reverse_wrap`).
- **ShadeScanner** is a dimming sweep (no `boost`), not a brightening sweep like its
  `KittScanner` neighbor.
- **Progress-driven filters** (`HoverBar`, `UnderlineWipe`, `BracketEmphasis`,
  `DotIndicator`, `PillButton`, `GlistenSweep`, `KittScanner`, `ShadeScanner`) take a
  static `progress: f32` in their standard form — use `1.0` to fully activate the
  effect during dwell. For animation, drive `progress` via a signal expression.

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
    pub inset_x: Option<u8>,
    pub inset_y: Option<u8>,
    pub inset_x_end: Option<u8>,
    pub inset_y_end: Option<u8>,
    pub falloff_x: Option<u8>,
    pub falloff_y: Option<u8>,
    pub side_coverage_eighths: Option<u8>,
    pub color: Color,
    pub surface_color: Option<Color>,
    pub edges: ShadowEdges,
    pub soft_edges: bool,
    pub composite_mode: ShadowCompositeMode,
    pub grade: Option<ShadowGradeConfig>,
}
```

Shadow compositing is controlled by `composite_mode`: the default `GlyphOverlay` replaces
destination content with shadow glyphs, `GradeUnderlying` preserves destination glyphs with grading, and `BlendUnderlying` preserves glyphs while alpha-blending the shadow onto the background
and applies color grading (desaturate, dim, tint) scaled by shadow coverage.
Use `.with_dramatic_grade()` for a visible preset with stronger background than foreground grading.
`inset_x`/`inset_y` trim the starting side of horizontal/vertical shadow runs; `inset_x_end`/`inset_y_end` trim the ending side. `falloff_x`/`falloff_y` reduce alpha at horizontal/vertical run ends so shadows can taper transparently without replacing destination glyphs. Bottom-only shadows (`ShadowEdges::BOTTOM` with `offset_x = 0`) default to a symmetric two-cell horizontal inset, so the run starts two cells in from the left and ends two cells before the right; use `with_symmetric_inset(...)` only when overriding that default. Add `with_falloff(2, 0)` for a tapered overhead-light shadow. `side_coverage_eighths` optionally renders solid left/right shadow columns as block-fraction foreground glyphs over transparent backgrounds; `6` is a three-quarter optical side edge. Use that with `GlyphOverlay` when the visible sub-cell side edge matters, and use `BlendUnderlying` / `GradeUnderlying` when preserving destination glyphs matters.

## ShadowStyle
- `Solid` (default) — transparent full-cell drop shadow using alpha background cells
- `HalfBlock` — explicit sub-cell shadow texture
- `Braille { density }` — 2x4 subpixel grid, font-dependent
- `Solid` — maximum compatibility
- `Gradient { layers }` — multi-layer soft shadow

## ShadowEdges (bitflags)
`RIGHT`, `BOTTOM`, `LEFT`, `TOP`, plus convenience `BOTTOM_RIGHT`, `TOP_LEFT`, `ALL`.

**Rule:** edges only render when the offset direction matches (e.g., `RIGHT` requires `offset_x > 0`).

## ShadowCompositeMode

- `GlyphOverlay` (default) — shadow glyphs replace destination content
- `GradeUnderlying` — destination glyphs preserved; color grading applied
- `BlendUnderlying` — destination glyph/fg preserved; shadow alpha blends onto background

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

`All`, `TextOnly`, `BorderOnly`, `BackgroundOnly`, `Rows(Vec<u16>)`, `RowRange { start, end }`,
`Cell { x, y }`, `Cells(Vec<CellCoord>)`, `Column(u16)`, `Columns(Vec<u16>)`,
`ColumnRange { start, end }`, `Modulo { axis, modulus, remainder }`

`ModuloAxis`: `Horizontal`, `Vertical`

## Spatial Shaders (`SpatialShaderType`)

These are serializable shader variants for use in `CompositionSpec` and `ShaderLayerSpec`.

| Shader | Parameters |
| --- | --- |
| `LinearGradient` | `gradient: Gradient`, `angle_deg`, `apply_to: LinearGradientApplyTo` (Foreground / Background / Both; default Foreground), `intensity: f32` (0–1, default 1.0). Authoring sugar `gradient_overlay` canonicalises to this shape; `apply_to` and `intensity` survive the canonicalisation since payload-normalize 0.5.0. `angle_deg` is a true projection axis (since linear-gradient-shader 1.0.0); non-cardinal angles like 45° produce diagonal sweeps, not the axis-pick behaviour from the previous version. |
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
    FadeIn  { apply_to: FadeApplyTo, ease: EasingCurve, from: FadeTarget },
    FadeOut { apply_to: FadeApplyTo, ease: EasingCurve, to:   FadeTarget },
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

### Canvas-aware FadeIn / FadeOut (since v0.3)

The `from` field on `FadeIn` and `to` field on `FadeOut` default to
`FadeTarget::Black`. **The render path automatically substitutes the sampled
canvas background color for `Color::BLACK`** right before rendering each
widget, so host applications that paint a non-default canvas (e.g. gt-design)
get canvas-aware fades without touching recipe JSON:

1. Host paints its canvas into the buffer.
2. Host calls `preview.render(area, buf, now)`.
3. For each animated widget, the render path reads `buf.cell(widget_x, widget_y).bg`.
4. If the cell has an explicit `Color::Rgb`, that color replaces any fade
   target whose color is still `Color::BLACK`.
5. Widget fades from/to the sampled canvas color.

Recipes don't need to encode palette colors in their JSON — they use the
`fade_in` / `fade_out` defaults and the canvas follows them at render time.

**Explicit override (for custom effects)** — set `from` (or `to`) to any
`FadeTarget::Color { color }`. Explicit colors are never auto-substituted,
so use this when you want a dramatic red alert fade, a white blowout, etc.:

```rust
use tui_vfx_style::models::{FadeTarget, ColorConfig, StyleEffect, FadeApplyTo};
use tui_vfx_geometry::{easing::EasingType, types::EasingCurve};

let effect = StyleEffect::FadeIn {
    apply_to: FadeApplyTo::Both,
    ease: EasingCurve::Type(EasingType::CubicOut),
    from: FadeTarget::Color {
        color: ColorConfig::Rgb { r: 255, g: 40, b: 0 },
    },
};
```

JSON form (recipe authors):

```json
{
  "type": "fade_in",
  "apply_to": "both",
  "easing": "cubic_out",
  "from": {
    "type": "color",
    "color": { "type": "rgb", "r": 255, "g": 40, "b": 0 }
  }
}
```

See `CAPABILITIES_REFERENCE.md` for the full `FadeTarget` variant table and
the auto-substitution rules.

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

### Cursor primitive

Module: `tui_vfx_content::cursor`. Types: `Cursor`, `CursorBlink`, `GrowIn`, `GrowInMode`, `GrowDirection`, `Wake`, `WakeMode`, `CursorState`, `CursorPaintOps`, `PrimaryOp`, `TrailOp`. Functions: `fnc_advance_cursor`, `fnc_render_cursor`, `fnc_cursor_grow_in_glyph`, `fnc_build_cursor_shader`, `fnc_apply_ghost_glyphs_to_grid`, `fnc_typewriter_cursor_position`, `fnc_splice_cursor_into_text`.

General-purpose cursor primitive with opt-in grow-in and wake animations. Defaults render a static cursor identical to v1.1.0 `TypewriterCursor` output. Pairs with `tui_vfx_style::models::CursorShader` (dispatched via `SpatialShaderType::Cursor`) for compositor integration.

`TypewriterCursor` now composes `Cursor` via `#[serde(flatten)]`, so legacy fields (`character`, `blink_interval`, `show_while_typing`, `show_after_complete`) remain at the top-level JSON key space.

### Supporting types

**TypewriterCursor (struct):**
`character`, `blink_interval`, `show_while_typing`, `show_after_complete`
Since 0.4.0 also exposes the flattened `Cursor` fields (`visibility`, `grow_in`, `wake`) for richer behavior.

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
pub use tui_vfx_shadow::{ShadowCompositeMode, ShadowConfig, ShadowEdges, ShadowGradeConfig, ShadowStyle, render_shadow, render_shadow_simple};
```

---

<!-- <FILE>docs/API_HAND.md</FILE> - <DESC>Hand-maintained TUI-VFX API documentation</DESC> -->
<!-- <VERS>END OF VERSION: 2.15.0</VERS> -->
