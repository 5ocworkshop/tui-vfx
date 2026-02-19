<!-- <FILE>docs/CAPABILITIES_REFERENCE.md</FILE> - <DESC>Hand-maintained capabilities reference</DESC> -->
<!-- <VERS>VERSION: 1.8.1</VERS> -->
<!-- <WCTX>Public release prep</WCTX> -->
<!-- <CLOG>Fix FILE metadata to match actual filename</CLOG> -->

# tui-vfx Capabilities Reference

> **MAINTENANCE NOTE:** This document must be kept in sync with the source code.
> Last verified: 2026-01-22
> When adding new effects, update the relevant section below.

This document provides a complete inventory of visual effects available in tui-vfx,
derived from the actual source code. Use this as a reference when planning visual
designs for terminal applications.

---

## Table of Contents

1. [Masks (Transition Shapes)](#masks-transition-shapes)
2. [Filters (Post-Processing)](#filters-post-processing)
3. [Samplers (Coordinate Distortion)](#samplers-coordinate-distortion)
4. [Spatial Shaders (Per-Cell Styling)](#spatial-shaders-per-cell-styling)
5. [Style Effects (Temporal Animations)](#style-effects-temporal-animations)
6. [Content Transformers (Text Effects)](#content-transformers-text-effects)
7. [Shadows](#shadows)
8. [Composition Pipeline](#composition-pipeline)

---

## Masks (Transition Shapes)

**Source:** `crates/tui-vfx-compositor/src/types/cls_mask_spec.rs`

Masks control the visibility of content during transitions. All masks operate on `t` (0.0→1.0).

| Mask | Description | Key Parameters |
|------|-------------|----------------|
| **None** | Fully visible (no mask) | — |
| **Wipe** | Linear reveal/hide from edges | `reveal`/`hide`: WipeDirection, `soft_edge` |
| **Dissolve** | Random pixel dissolve | `seed`, `chunk_size` |
| **Checkers** | Checkerboard pattern reveal | `cell_size` |
| **Blinds** | Venetian blinds effect | `orientation`, `count` |
| **Iris** | Spotlight/iris from center | `shape`: Circle/Diamond/Box, `soft_edge` |
| **Diamond** | Diamond expand from center | `soft_edge` |
| **NoiseDither** | Dithered noise pattern | `seed`, `matrix`: Bayer4/Bayer8 |
| **PathReveal** | Path-based reveal (spiral, radial sweep) | `path`: Spiral/Radial, `soft_edge` |
| **Radial** | Radial expansion from origin | `origin`: Center/corners/Custom, `soft_edge` |
| **Cellular** | Organic/cellular pattern | `pattern`: Voronoi/Hexagonal/Organic, `seed`, `cell_count` |

### WipeDirection Variants (16 total)

**Cardinal:** `LeftToRight`, `RightToLeft`, `TopToBottom`, `BottomToTop`

**Diagonal:** `TopLeftToBottomRight`, `TopRightToBottomLeft`, `BottomLeftToTopRight`, `BottomRightToTopLeft`

**Aliases:** `FromLeft`, `FromRight`, `FromTop`, `FromBottom`

**Center-Out (Barn Door):** `HorizontalCenterOut`, `VerticalCenterOut`, `HorizontalEdgesIn`, `VerticalEdgesIn`

---

## Filters (Post-Processing)

**Source:** `crates/tui-vfx-compositor/src/types/cls_filter_spec.rs`

Filters apply post-processing effects to the rendered output. Applied in order (left to right).

| Filter | Description | Key Parameters |
|--------|-------------|----------------|
| **None** | No filter effect | — |
| **Dim** | Darken output | `factor`: 0.0=black, 1.0=unchanged; `apply_to` |
| **Invert** | Color inversion | `apply_to` |
| **Tint** | Apply color overlay | `color`, `strength`, `apply_to` |
| **Vignette** | Edge darkening | `strength`, `radius` (signal-driven) |
| **Crt** | CRT monitor post-processing | `scanline_strength`, `glow` |
| **PatternFill** | Background texture patterns | `pattern`, `color`, `only_empty` |
| **Greyscale** | Desaturate (BT.601 luminance) | `strength`, `apply_to` |
| **BrailleDust** | Animated braille particles | `density`, `hz`, `seed`, `pattern`, `color` |
| **InterlaceCurtain** | Scanline/interlace effect | `density`, `dim_factor`, `scroll_speed` |
| **MotionBlur** | Directional blur trail | `trail_length`, `opacity_decay`, `direction` |
| **ColorBridgedShade** | Shade char opacity (░▒▓█) | `opacity`, `fg_color`, `bg_color` |
| **SubPixelBar** | 8x resolution progress bar | `progress`, `direction`, `filled_color`, `unfilled_color`, `animated` |
| **SubCellShake** | Edge vibration using partial blocks | `amplitude`, `frequency`, `seed`, `edge_only`, `filled_color`, `bg_color` |
| **RigidShake** | Ketchup bottle damped oscillation | `shake_period`, `num_shakes`, `pause_duration`, `max_eighths`, `base_eighths`, `damping`, `element_color`, `bg_color`, `inner_width`, `margin_width` |
| **HoverBar** | Progress-driven partial bar indicator | `base_eighths`, `max_eighths`, `position`, `bar_color`, `bg_color`, `progress`, `margin_width` |
| **UnderlineWipe** | Horizontal underline wipe-in | `direction`, `color`, `line_char`, `row_offset`, `progress` |
| **BracketEmphasis** | Fade-in brackets around content | `left`, `right`, `color`, `bg_color`, `progress` |
| **DotIndicator** | Simple dot/bullet marker | `indicator_char`, `position`, `color`, `bg_color`, `progress` |

### PatternType Variants

- **Single** — Repeating character (`char`)
- **Checkerboard** — Alternating characters (`char_a`, `char_b`)
- **HorizontalLines** — Row lines (`line_char`, `spacing`)
- **VerticalLines** — Column lines (`line_char`, `spacing`)

### BraillePatternType Variants

- **SingleDot** — Most subtle (⠁ ⠂ ⠄)
- **OneToTwoDots** — Subtle
- **OneToThreeDots** — Moderate
- **OneToFourDots** — More visible

---

## Samplers (Coordinate Distortion)

**Source:** `crates/tui-vfx-compositor/src/types/cls_sampler_spec.rs`

Samplers distort pixel coordinates before rendering, creating spatial effects.

| Sampler | Description | Key Parameters |
|---------|-------------|----------------|
| **None** | No distortion | — |
| **SineWave** | Sinusoidal wave distortion | `axis`, `amplitude`, `frequency`, `speed`, `phase` |
| **Ripple** | Circular ripple from center | `amplitude`, `wavelength`, `speed`, `center` |
| **Shredder** | Paper shredder strips | `stripe_width`, `odd_speed`, `even_speed` |
| **FaultLine** | Fault line displacement | `seed`, `intensity`, `split_bias` |
| **Crt** | CRT scanlines + curvature | `scanline_strength`, `jitter`, `curvature` |
| **CrtJitter** | CRT crash/jitter effect | `intensity`, `speed_hz`, `decay_ms` |

---

## Spatial Shaders (Per-Cell Styling)

**Source:** `crates/tui-vfx-style/src/models/cls_spatial_shader_type.rs`

Spatial shaders compute per-cell style modifications based on position, time, and animation state.

| Shader | Description | Key Parameters |
|--------|-------------|----------------|
| **LinearGradient** | Gradient fill at angle | `gradient`, `angle_deg` |
| **BarberPole** | Animated diagonal stripes | `speed`, `stripe_width`, `gap_width`, `color` |
| **Radar** | Rotating radar sweep | `speed`, `tail_length`, `color` |
| **BorderSweep** | Border highlight sweep | `speed`, `length`, `color` |
| **Highlighter** | Marker-style text reveal | `color` |
| **Reflect** | Moving reflective glint | `speed`, `color` |
| **GlistenBand** | Moving light band sweep | `speed`, `band_width`, `angle_deg`, `head`, `tail`, `direction`, `repeat_count`, `apply_to`, `blend_strength` |
| **GlitchLines** | Random horizontal glitch | `seed`, `intensity`, `max_lines`, `speed`, `flash_chance`, `pulse_color`, `pulse_speed`, `italic_on_flash`, `flash_hold`, `noise_type` |
| **NeonFlicker** | Flickering neon tube | `stability`, `seed`, `segment`, `dim_amount`, `speed`, `flash_chance`, `decay_rate`, `noise_type` |
| **PulseWave** | Rippling color wave | `frequency`, `speed`, `color`, `direction`, `wavelength` |
| **FocusedRowGradient** | Vertical gradient on selected row | `selected_row`, `selected_row_ratio`, `falloff_distance`, `bright_color`, `dim_color`, `apply_to` |
| **RevealWipe** | Progressive reveal | `direction` |
| **StochasticSparkle** | Film grain / frosted glass | `sparkle_density`, `brightness_boost`, `speed`, `seed`, `apply_to`, `noise_type` |
| **AmbientOcclusion** | Contact shadow at widget edges | `intensity`, `radius`, `edges`, `falloff`, `shadow_color` |
| **Bevel** | 3D embossed edge effect | `light_direction`, `highlight_intensity`, `shadow_intensity`, `edge_width` |
| **Glow** | Multi-cell bloom/halo | `color`, `radius`, `falloff`, `intensity`, `pulse_speed` |
| **SubCellShake** | Micro-jitter color oscillation | `amplitude`, `frequency`, `axis`, `chromatic`, `seed`, `edge_only`, `edge_width` |
| **ChromaticEdge** | RGB edge separation | `intensity`, `edge_width`, `horizontal` |

### StochasticSparkle Details

Premium effect for frosted glass / film grain texture:
- `sparkle_density`: Fraction of cells that sparkle (0.05 = 5%)
- `brightness_boost`: Multiplier (1.2 = 20% brighter)
- `speed`: Update rate (0.25 = shimmer, 1.0 = static)
- `noise_type`: Uniform (even) or Gaussian (clustered)
- `apply_to`: Foreground, Background, or Both

### Detailed Notes

#### Shader Notes

**AmbientOcclusion** — Contact shadow shader that darkens cells near widget edges:
- `edges`: BottomRight (default), TopLeft, All, Inner
- `falloff`: Linear, Quadratic (default), Exponential
- Creates depth by simulating light occlusion at boundaries

**Bevel** — 3D embossed edge effect simulating raised/sunken surfaces:
- `light_direction`: TopLeft (default), TopRight, BottomLeft, BottomRight, Top, Bottom, Left, Right
- Highlights edges facing the light, shadows opposite edges

**Glow** — Multi-cell bloom/halo effect around widget edges:
- `falloff`: Controls intensity curve (Linear, Quadratic, Exponential)
- `pulse_speed`: Optional pulsing animation in Hz

**SubCellShake (Shader)** — Micro-jitter visual effect through rapid color oscillation:
- `axis`: Horizontal, Vertical, Both (default)
- `chromatic`: Enable RGB channel separation for chromatic aberration
- `edge_only`: Limit shake to widget borders

#### Filter Notes

**InterlaceCurtain** — Scanline/interlace dimming effect:
- `density`: Row spacing (1.0 = every other row)
- `scroll_speed`: Animation speed for scrolling scanlines

**MotionBlur** — Directional blur trail effect:
- `direction`: Left (default), Right, Up, Down
- `opacity_decay`: Higher values create sharper falloff

**ColorBridgedShade** — Maps opacity to shade characters with color bridging:
- Uses ░▒▓█ characters based on opacity
- Smooth color transitions at shade boundaries

**SubPixelBar** — High-resolution progress bar using partial block characters:
- Uses ▏▎▍▌▋▊▉█ (horizontal) or ▁▂▃▄▅▆▇█ (vertical) for 8x cell resolution
- `direction`: Horizontal or Vertical fill
- `animated`: Enables automatic progress cycling with `t` parameter
- Ideal for loading indicators with smooth sub-cell precision

**SubCellShake (Filter)** — Physical vibration effect using partial vertical blocks:
- Creates edge oscillation using ▏▎▍▌▋▊▉ characters
- Simulates physical "tactile" feedback like an incorrect password shake
- `edge_only`: Limit shake to widget borders (true) or apply to all cells (false)
- Best for error/rejection feedback, button press responses

**RigidShake** — Ketchup bottle damped oscillation for rigid body motion:
- Creates the classic "ketchup bottle" shake: multiple diminishing oscillations then pause
- Entire widget appears to shift as a rigid body using margin cells
- Uses partial blocks (▏▎▍▌▋▊▉█) to render extensions/gaps outside widget area
- Key parameters:
  - `shake_period`: Duration of one back-and-forth shake (default 0.29s)
  - `num_shakes`: Number of shakes before pause (default 4, max 8)
  - `pause_duration`: Rest period between shake cycles (default 0.52s)
  - `max_eighths`: Maximum extension in 1/8ths of a cell (default 12 = 1.5 cells)
  - `base_eighths`: Base extension always visible at rest (default 3 = 25%)
  - `damping`: Array of amplitude multipliers per shake (e.g., [1.0, 0.7, 0.45, 0.25])
  - `margin_width`: Number of margin cells on each side (default 2)
- IMPORTANT: Apply to an area that includes margin cells around the widget
- Ideal for attention-grabbing notifications, satisfying button feedback

### RigidShake Synchronized Effects Recipe

To create a complete RigidShake experience where text styling (italic, shift) stays
perfectly synchronized with the margin animation, use the shared `RigidShakeTiming`
utility from `tui-vfx-types`:

**Source:** `crates/tui-vfx-types/src/rigid_shake_timing.rs`

```rust
use tui_vfx_types::RigidShakeTiming;

// Create timing with default parameters (matches FilterSpec::RigidShake defaults)
let timing = RigidShakeTiming::default();

// Or customize timing parameters
let timing = RigidShakeTiming::new()
    .with_shake_period(0.29)
    .with_num_shakes(4)
    .with_pause_duration(0.52);

// Calculate current state at elapsed time
let state = timing.calculate(elapsed_secs);

// Use state to drive synchronized effects:
if state.is_shifting_right() {
    // Apply italic style, prepend space to text, etc.
    text_style = text_style.add_modifier(Modifier::ITALIC);
    text_prefix = "  "; // Shift text right visually
}
```

**RigidShakeState** provides:
- `offset_eighths`: Current offset in 1/8ths of a cell (i16)
- `raw_offset`: Raw oscillation value (-1.0 to 1.0)
- `is_shifting_right()`: True when element moving right (use for italic)
- `is_shifting_left()`: True when element moving left
- `is_at_rest()`: True during pause or at oscillation center
- `in_pause`: Whether currently in pause phase
- `shake_num`: Current shake number (0-7) or None if paused

**Coordinated usage:**
1. Apply `FilterSpec::RigidShake` to the widget area with margins
2. Use `RigidShakeTiming::calculate(t)` with same timing parameters
3. Apply italic/shift to text content when `state.is_shifting_right()`

The `StyleEffect::RigidShakeStyle` variant provides the same synchronization
as a style effect that can be applied via the style system.

### Hover Indicator Filters

Four filters designed for hover/focus visual feedback, all driven by `progress` (0.0→1.0):

**HoverBar** — Progress-driven partial bar indicator for hover/focus states:
- Uses partial block characters (▏▎▍▌▋▊▉█) with fg/bg inversion for contiguous appearance
- `base_eighths`: Width at rest (0.0 progress), in 1/8ths of a cell (0-8)
- `max_eighths`: Width when fully active (1.0 progress), in 1/8ths (0-16, can span 2 cells)
- `position`: Left or Right side of content
- `margin_width`: Number of margin cells for bar expansion (default 2)
- Ideal for list item selection, navigation indicators, menu hover states

**UnderlineWipe** — Horizontal underline that wipes in based on progress:
- `direction`: WipeDirection (LeftToRight, RightToLeft, etc.)
- `line_char`: Underline character (default ▁)
- `row_offset`: Distance from bottom (0 = last row)
- Ideal for link hover effects, tab indicators

**BracketEmphasis** — Brackets that fade in at content edges:
- `left`/`right`: Bracket characters (default [ and ])
- Fades from bg_color to color based on progress
- Ideal for selection highlighting, focus indicators

**DotIndicator** — Simple dot/bullet that appears adjacent to content:
- `indicator_char`: Marker character (default •)
- `position`: Left or Right side
- Fades in based on progress
- Ideal for list selection, navigation bullets

---

## Style Effects (Temporal Animations)

**Source:** `crates/tui-vfx-style/src/models/cls_style_effect.rs`

Style effects animate properties over time, driven by `t` (0.0→1.0).

| Effect | Description | Key Parameters |
|--------|-------------|----------------|
| **FadeIn** | Opacity fade in | `apply_to`, `ease` |
| **FadeOut** | Opacity fade out | `apply_to`, `ease` |
| **Pulse** | Color intensity pulsing | `frequency`, `color` |
| **Rainbow** | Continuous hue cycling | `speed` |
| **Glitch** | Glitch-style distortion | `seed`, `intensity`, `italic_start`, `italic_end` |
| **NeonFlicker** | Flickering tube simulation | `stability` (0.0-1.0) |
| **Spatial** | Wraps any SpatialShaderType | `shader` |
| **ItalicWindow** | Italic during time window | `start`, `end` |
| **ColorShift** | HSL color manipulation | `hue_shift`, `saturation_shift`, `lightness_shift` |
| **ColorFade** | Fade toward target color | `target`, `color_space` |
| **RigidShakeStyle** | Italic synced with RigidShake | `shake_period`, `num_shakes`, `pause_duration` |

---

## Content Transformers (Text Effects)

**Source:** `crates/tui-vfx-content/src/transformers/mod.rs`

Content transformers modify text content during animation.

| Transformer | Description |
|-------------|-------------|
| **Typewriter** | Character-by-character reveal |
| **Scramble** | Random character scrambling/unscrambling |
| **GlitchShift** | Glitch-style text distortion |
| **ScrambleGlitchShift** | Combined scramble + glitch |
| **Dissolve** | Pixel dissolve text transition |
| **Marquee** | Scrolling text ticker |
| **SlideShift** | Sliding text with row jump after passing a column span |
| **Mirror** | Mirror/flip text |
| **Morph** | Character morphing/blending |
| **Numeric** | Number transition animations |
| **Odometer** | Odometer-style digit rolling |
| **Redact** | Redaction/censoring effect |
| **SplitFlap** | Split-flap display effect |
| **WrapIndicator** | Prefix/suffix wrapping based on progress |

### WrapIndicator Details

Wraps text with prefix/suffix symbols that appear progressively based on progress:

```
Progress 0.0: "YES"
Progress 0.5: "» YES"
Progress 1.0: "» YES «"
```

- `prefix`: String to prepend (e.g., "» ")
- `suffix`: String to append (e.g., " «")
- Characters appear one at a time as progress increases
- Ideal for hover indicators like "» Selected Item «"

---

## Shadows

**Source:** `crates/tui-vfx-shadow/src/types/`

### ShadowStyle Variants

| Style | Description | Quality |
|-------|-------------|---------|
| **HalfBlock** (default) | Half-block characters (▐▄▌▀) | Best quality, sub-cell precision |
| **Braille** | 2×4 subpixel density grid | Fine-grained, font-dependent |
| **Solid** | Simple background color fill | Maximum compatibility |
| **Gradient** | Multi-layer decreasing intensity | Softer appearance |

### ShadowConfig Properties

| Property | Type | Description |
|----------|------|-------------|
| `style` | ShadowStyle | Rendering technique |
| `offset_x` | i8 | X offset (positive = right) |
| `offset_y` | i8 | Y offset (positive = down) |
| `color` | Color | Shadow color |
| `surface_color` | Option<Color> | Background for half-block blending |
| `edges` | ShadowEdges | Which edges to render (BOTTOM_RIGHT, ALL, etc.) |
| `soft_edges` | bool | Use half-blocks at boundaries |

---

## Composition Pipeline

**Source:** `crates/tui-vfx-compositor/src/pipeline/cls_composition_options.rs`

The compositor orchestrates all effects through `CompositionOptions`:

```rust
CompositionOptions {
    sampler_spec: Option<SamplerSpec>,      // Coordinate distortion
    masks: Cow<[MaskSpec]>,                  // Visibility control
    mask_combine_mode: MaskCombineMode,      // AND/OR mask logic
    filters: Cow<[FilterSpec]>,              // Post-processing chain
    shader_layers: SmallVec<[ShaderWithRegion; 2]>,  // Per-region shaders
    shadow: Option<ShadowSpec>,              // Integrated shadow
    preserve_unfilled: bool,                 // Transparency handling
    t: f64,                                  // Animation progress (0.0-1.0)
    loop_t: Option<f64>,                     // Cyclical time for continuous effects
    phase: Option<Phase>,                    // Entering/Dwelling/Exiting/Finished
}
```

`CompositionSpec` is the serializable counterpart used by `render_pipeline_with_spec`.
It mirrors the same capabilities (including `shadow` and `preserve_unfilled`) for
JSON/TOML-driven configurations.

### Render Order

1. **Sampler** — Coordinate distortion applied first
2. **Shadow** — Rendered beneath element (extended area)
3. **Element** — Main content rendered
4. **Masks** — Applied to shadow + element together
5. **Filters** — Post-processing applied last
6. **Shaders** — Per-cell styling throughout

---

## Quick Reference: Effect Categories

### For Transitions (Enter/Exit)
- Masks: Wipe, Iris, Dissolve, Blinds, Radial, Diamond
- Effects: FadeIn, FadeOut, ColorFade

### For Loading States
- Shaders: BarberPole, StochasticSparkle, GlistenBand
- Filters: BrailleDust, PatternFill, MotionBlur, SubPixelBar
- Transformers: Typewriter

### For Focus/Selection
- Shaders: BorderSweep, PulseWave, FocusedRowGradient, Glow
- Effects: Pulse, NeonFlicker

### For Error/Warning States
- Shaders: GlitchLines, NeonFlicker, SubCellShake (shader)
- Filters: SubCellShake (filter), RigidShake
- Samplers: CrtJitter, FaultLine
- Transformers: Scramble, GlitchShift

### For Premium Surfaces
- Shaders: StochasticSparkle, GlistenBand, LinearGradient, AmbientOcclusion, Bevel, Glow
- Filters: BrailleDust, Vignette, ColorBridgedShade
- Shadows: HalfBlock with soft_edges, Gradient

### For Data Display
- Transformers: Odometer, SplitFlap, Numeric
- Shaders: Highlighter, RevealWipe

### For Retro/CRT Aesthetic
- Filters: Crt, InterlaceCurtain
- Samplers: Crt, CrtJitter
- Effects: NeonFlicker, Glitch

---

## Terminal Viability Notes

**Universal (all TrueColor terminals):**
- All masks, filters, style effects
- LinearGradient, PulseWave, BorderSweep shaders
- All content transformers
- HalfBlock and Solid shadow styles

**Font-dependent (may vary):**
- Braille shadow style
- BrailleDust filter
- StochasticSparkle (when using braille noise)

**Performance considerations:**
- StochasticSparkle: Lower `speed` for smoother shimmer
- BrailleDust: Balance `density` vs `hz` for desired effect
- Multiple filters: Applied sequentially, each has cost

---

<!-- <FILE>docs/CAPABILITIES_REFERENCE.md</FILE> - <DESC>Hand-maintained capabilities reference</DESC> -->
<!-- <VERS>END OF VERSION: 1.8.1</VERS> -->
