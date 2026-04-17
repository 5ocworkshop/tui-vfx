<!-- <FILE>docs/CAPABILITIES_REFERENCE.md</FILE> - <DESC>Hand-maintained capabilities reference</DESC> -->
<!-- <VERS>VERSION: 1.18.0</VERS> -->
<!-- <WCTX>feat/cursor-braille: document the new braille cursor additions — 4 static row-stacked convenience constructors (braille_2/4/6/8 → ⠉ ⠛ ⠿ ⣿) and 2 scan modes (BraillePulse, BrailleRowFlip) that override the base glyph with braille fills.</WCTX> -->
<!-- <CLOG>MINOR: Add "Braille cursors" subsection listing the 4 static ctors; extend "Scan" subsection with BraillePulse and BrailleRowFlip rows noting base-glyph override behavior; extend the debug recipe catalog with 6 new braille recipe entries.</CLOG>

# tui-vfx Capabilities Reference

> **MAINTENANCE NOTE:** This document must be kept in sync with the source code.
> Last verified: 2026-04-13
> When adding new effects, update the relevant section below.

This document provides a complete inventory of visual effects available in tui-vfx,
derived from the actual source code. Use this as a reference when planning visual
designs for terminal applications.

Before authoring new effects or recipes, read
[`TERMINAL_MOTION_HEURISTICS.md`](TERMINAL_MOTION_HEURISTICS.md) for the
terminal-specific perception and compositing constraints that should shape the
design.

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
9. [Observability & Debugging](#observability--debugging)

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
| **Materialize** | Organic resolve/materialize reveal | `origin`, `seed`, `chunk_size`, `noise`, `soft_edge` |
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
| **BrailleDust** | Animated braille particles | `density`, `hz`, `seed`, `pattern`, `color`, `drift` |
| **CharsetNoise** | Time-varying character replacement (living textures) | `hz`, `seed`, `jitter`, `affect`, `chars` (flat) or `gradient` (position-aware) |
| **InterlaceCurtain** | Scanline/interlace effect | `density`, `dim_factor`, `scroll_speed` |
| **MotionBlur** | Directional blur trail | `trail_length`, `opacity_decay`, `direction` |
| **ColorBridgedShade** | Shade char opacity (░▒▓█) | `opacity`, `fg_color`, `bg_color` |
| **SubPixelBar** | 8x resolution progress bar | `progress`, `direction`, `filled_color`, `unfilled_color`, `animated` |
| **SubCellShake** | Edge vibration using partial blocks | `amplitude`, `frequency`, `seed`, `edge_only`, `filled_color`, `bg_color` |
| **RigidShake** | Ketchup bottle damped oscillation | `shake_period`, `num_shakes`, `pause_duration`, `max_eighths`, `base_eighths`, `damping`, `element_color`, `bg_color`, `inner_width`, `margin_width` |
| **HoverBar** | Progress-driven partial bar indicator | `base_eighths`, `max_eighths`, `position`, `bar_color`, `bg_color`, `progress`, `margin_width` |
| **UnderlineWipe** | Horizontal underline wipe-in | `direction`, `color`, `bg_color`, `line_char`, `row_offset`, `progress`, `gradient`, `glisten` |
| **BracketEmphasis** | Fade-in brackets around content | `left`, `right`, `color`, `bg_color`, `progress` |
| **DotIndicator** | Simple dot/bullet marker | `indicator_char`, `position`, `color`, `bg_color`, `progress` |
| **EdgeGrow** | Generalized edge growth/stretch indicator | `rest_eighths`, `peak_eighths`, `edge`, `fill_color`, `bg_color`, `progress`, `margin_width` |
| **PillButton** | Pill-shaped button with gradient edges | `button_color`, `bg_color`, `edge_width`, `glisten`, `progress` |
| **GlistenSweep** | Diagonal 45° brightness sweep (hover shine) | `boost` (u8, additive), `band_width` (f32, diagonal fraction), `speed`, `progress`, `powerline_mode`, `boost_separator_bg` |
| **KittScanner** | Horizontal scanner sweep (KITT/Larson or one-way lighthouse wrap) | `boost` (u8), `band_width`, `bps`, `progress`, `motion_mode`, `apply_to`, `powerline_mode`, `boost_separator_bg` |
| **ShadeScanner** | Ping-pong scanner that dims text with shade overlay | `shade_color`, `bps`, `progress` |

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
| **FocusedRowGradient** | Vertical gradient on selected row | `selected_row`, `selected_row_binding`, `selected_row_ratio`, `selected_row_ratio_binding`, `falloff_distance`, `bright_color`, `dim_color`, `apply_to` |
| **RevealWipe** | Progressive reveal | `direction` |
| **StochasticSparkle** | Film grain / frosted glass | `sparkle_density`, `brightness_boost`, `speed`, `seed`, `apply_to`, `noise_type` |
| **AmbientOcclusion** | Contact shadow at widget edges | `intensity`, `radius`, `edges`, `falloff`, `shadow_color` |
| **Bevel** | 3D embossed edge effect | `light_direction`, `highlight_intensity`, `shadow_intensity`, `edge_width` |
| **Glow** | Multi-cell bloom/halo | `color`, `radius`, `falloff`, `intensity`, `pulse_speed` |
| **ConcealedLight** | Hidden-source architectural light wash | `source`, `spread`, `edge_width`, `source_cutoff`, `intensity`, `mode`, `apply_to` |
| **Diffusion** | Soft material-light diffusion | `source`, `radius`, `softness`, `edge_firmness`, `intensity`, `mode`, `apply_to` |
| **AffordanceWake** | Dormant secondary affordance wake | `zone`, `radius`, `progress`, `progress_binding`, `rest_intensity`, `peak_intensity`, `apply_to` |
| **WayfindingNode** | Calm node emphasis for breadcrumbs/steps | `nodes`, `radius`, `current_index`, `current_index_binding`, `previous_strength`, `future_strength`, `pulse_speed` |
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

**ConcealedLight** — Hidden-source architectural light for thresholds, seams, and shell hierarchy:
- `source`: Which edge hides the light source (Top, Bottom, Left, Right)
- `spread`: How far the light reaches inward from the source
- `edge_width`: Width of the strongest lit band just inside the concealed lip
- `source_cutoff`: Reserves a dark lip before the light visibly emerges
- Use for panel shells, drawers, thresholds, headers, and subtle structural depth

**Diffusion** — Soft material-light response for paper, textile, frosted, and lantern-like surfaces:
- `source`: Center, edge, or corner source geometry
- `softness`: Broadens/tightens the diffusion response
- `edge_firmness`: Preserves a disciplined frame around the perimeter
- `mode`: Static by default; WarmDrift, CoolDrift, and Breath stay intentionally subtle
- Best on shell-owned or background-heavy surfaces, not dense text blocks

**AffordanceWake** — Low-noise dormant-to-active affordance emphasis:
- `zone`: AllEdges, Corners, LeftRail, RightRail, TopRail, BottomRail
- `progress` / `progress_binding`: Activation amount for reveal-on-need affordances
- `rest_intensity`: Optional dormant baseline, often 0.0
- `peak_intensity`: Full wake strength
- Intended to complement explicit hover/focus indicators rather than replace them

**WayfindingNode** — Calm node/junction emphasis for breadcrumbs, progress steps, and route hints:
- `nodes`: Explicit x/y node positions
- `current_index` / `current_index_binding`: Current active node
- `previous_strength`: Emphasis strength for already-passed nodes
- `future_strength`: Optional preview of upcoming nodes
- More practical and application-oriented than the routed signal-trace shaders

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

### Newer Hover & Feedback Filters

Four filters added after the original hover-indicator family. These work by
**boosting existing cell colors** with additive brightness, so the widget's
`base_style` drives the palette and the filter drives the temporal/spatial
motion on top of it. None of them take a `color` field — if prose descriptions
elsewhere refer to "the red KITT color" or "the warm-white glisten", that's
aesthetic intent, not a recipe field. Drive color through `base_style`.

**PillButton** — Pill-shaped button appearance with horizontal gradient edges:
- `button_color`: Solid fill color for the pill interior
- `bg_color`: Background at the extreme edges (gradients fade `button_color → bg_color`)
- `edge_width`: Width of the left/right gradient edge in cells (default 3)
- `glisten`: Enable glisten/shimmer sweep on hover (default true)
- `progress`: Hover progress 0.0..1.0 (default 0.0 — set to a non-zero value to activate)
- Ideal for interactive buttons, CTA primitives, rounded interactive surfaces

**GlistenSweep** — Diagonal 45° highlight band that boosts existing colors:
- `boost`: Additive u8 brightness boost applied under the band (default 40)
- `band_width`: Width of the highlight band as a fraction of the diagonal (default 0.2)
- `speed`: Animation speed; when 0, the band is positioned by `progress` only (default 0.5)
- `progress`: Hover progress 0.0..1.0 (set to 1.0 to keep the filter active)
- `powerline_mode`: Smart powerline rendering (bg on text, fg only on separator glyphs)
- `boost_separator_bg`: Additionally boost separator backgrounds when `powerline_mode` is true — needed for powerlines with a continuous bg rather than terminal default
- Ideal for hover shine, button press feedback, polished CTAs

**KittScanner** — Horizontal scanner sweep:
- `boost`: Additive u8 brightness boost under the band (default 50)
- `band_width`: Width of the scanner band as fraction of total width (default 0.15, typical 0.0..0.5)
- `bps`: Beats per second for the scanner cycle (default 1.0) — **not** `speed`
- `progress`: 0.0..1.0, set to 1.0 to activate
- `motion_mode`: `ping_pong` (classic KITT/Larson), `forward_wrap`, or `reverse_wrap`
- `apply_to`: Which color component to boost (fg / bg / both, default Both)
- `powerline_mode` / `boost_separator_bg`: See GlistenSweep
- Use a red base style for the classic KITT/Larson or lighthouse look — the boost is additive, not replacement
- Ideal for status bars, alert indicators, ambient attention-getters

**ShadeScanner** — Ping-pong scanner that dims text with a shade overlay:
- `shade_color`: The dimming overlay color applied as the band sweeps (default dark grey)
- `bps`: Beats per second for the ping-pong cycle (default 1.0)
- `progress`: 0.0..1.0, set to 1.0 to activate
- Simpler than KittScanner (no boost, no band_width, no powerline options) — this is a dimming sweep, not a brightening sweep
- Ideal for "reading" effects, progressive-reveal teases, subtle attention cues

### Living-Texture Filters

**CharsetNoise** — Non-converging time-varying character replacement:
- `hz`: Pattern changes per second (default 8.0)
- `seed`: Deterministic random seed for reproducible patterns
- `jitter`: Per-cell random offset to gradient position (0.0..1.0)
- `affect`: `"all"` or `"non_empty"` (default `non_empty`) — whether whitespace cells are replaced
- `chars`: Flat charset used for every cell (ignored if `gradient` is set)
- `gradient`: Position-aware charsets — `Vec<{ at: f32, chars: String }>` — overrides `chars`
- Use the flat form for uniform textures (static noise, smoke); use the gradient form for fire-like shapes where sparse/flickering characters sit above dense solid characters
- Chains naturally: `charset_noise` → `braille_dust` → `tint`

---

## Style Effects (Temporal Animations)

**Source:** `crates/tui-vfx-style/src/models/cls_style_effect.rs`

Style effects animate properties over time, driven by `t` (0.0→1.0).

| Effect | Description | Key Parameters |
|--------|-------------|----------------|
| **FadeIn** | Fade toward base from a configurable start color (default black) | `apply_to`, `ease`, `from` |
| **FadeOut** | Fade from base toward a configurable end color (default black) | `apply_to`, `ease`, `to` |
| **Pulse** | Color intensity pulsing | `frequency`, `color` |
| **Rainbow** | Continuous hue cycling | `speed` |
| **Glitch** | Glitch-style distortion | `seed`, `intensity`, `italic_start`, `italic_end` |
| **NeonFlicker** | Flickering tube simulation | `stability` (0.0-1.0) |
| **Spatial** | Wraps any SpatialShaderType | `shader` |
| **ItalicWindow** | Italic during time window | `start`, `end` |
| **ColorShift** | HSL color manipulation | `hue_shift`, `saturation_shift`, `lightness_shift` |
| **ColorFade** | Fade toward target color | `target`, `color_space` |
| **RigidShakeStyle** | Italic synced with RigidShake | `shake_period`, `num_shakes`, `pause_duration` |

### Canvas-aware FadeIn / FadeOut (since v0.3)

`FadeIn` and `FadeOut` fade between the widget's base color and a configurable
`FadeTarget`. Historically these were hard-coded to fade to/from `Black`; as of
v0.3 the renderer now supports **automatic canvas substitution** plus optional
explicit `from` (on FadeIn) / `to` (on FadeOut) fields on the recipe.

**Automatic canvas substitution (the normal case).** Right before rendering
each animated widget, the render path samples the destination buffer's
background color at the widget's top-left cell. If the host has painted a
non-default RGB background there (e.g. a gt-design canvas surface) **and the
recipe's fade is still defaulting to `Color::BLACK`**, the render path
substitutes the sampled canvas color into the fade's target. The widget
then appears to grow *out of* the canvas on enter and dissolve *into* it on
exit — no black flash, no recipe changes needed.

The substitution rules:

1. Sample succeeds only when the destination cell has an explicit `Rgb(r, g, b)`
   background. Terminal-default / named / indexed backgrounds are ignored.
2. Substitution only replaces `Color::BLACK`. Any fade that already has an
   explicit color (see "explicit override" below) is passed through untouched.
3. If the host paints nothing before calling render, the legacy fade-from-black
   behavior is preserved bit-for-bit.

This means `gt-design`-style applications that paint a canvas color into the
buffer before calling `preview.render(...)` get canvas-aware fades for free,
and recipes don't need to encode palette-specific colors in their JSON.

**Explicit override (for custom effects).** When a recipe wants to fade from
something other than the canvas — a dramatic red alert flash, a white blowout,
whatever — set `from` (or `to` on FadeOut) to a concrete color. Explicit
colors are never replaced:

```json
{
  "type": "fade_in",
  "apply_to": "both",
  "easing": "back_out",
  "from": {
    "type": "color",
    "color": { "type": "rgb", "r": 255, "g": 40, "b": 0 }
  }
}
```

**`FadeTarget` variants:**

| Variant | JSON | Behavior |
|---|---|---|
| `Black` (default) | `{"type": "black"}` or omit | Fade from/to `rgb(0,0,0)`; **auto-replaced with sampled canvas color if host painted one** |
| `White` | `{"type": "white"}` | Fade from/to `rgb(255,255,255)` |
| `Transparent` | `{"type": "transparent"}` | Snap threshold (no smooth blend) |
| `Base` | `{"type": "base"}` | Use the widget's base color (no-op in chains) |
| `Color` | `{"type": "color", "color": {...}}` | Fade from/to an explicit `ColorConfig` — never auto-substituted |

For fully custom color-to-color fades that aren't anchored at the base color
on one end, use `ColorFade` instead — it takes both endpoints and an
interpolation color space.

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
| **GlyphCascade** | Glyph alphabet cascade / symbol evolution |

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

### Applying a content effect (since 0.3.0)

For the common case where you have a `progress: f64` from an animation
loop and want the transformed text back, call `ContentEffect::apply`
directly instead of going through the dispatcher:

```rust
use tui_vfx_content::prelude::*;

let effect = ContentEffect::Typewriter {
    speed_variance: SignalOrFloat::Static(0.0),
    cursor: None,
};
let revealed: String = effect.apply("Hello World", 0.5);
```

The full method set:

| Method | Returns | When to use |
|--------|---------|-------------|
| `apply(target, progress)` | `String` | The 95% case — owned result, default `SignalContext`. |
| `apply_to_borrowed(target, progress)` | `Cow<'_, str>` | When you want to preserve the zero-allocation fast path (e.g. Typewriter at progress `1.0`). |
| `apply_with_context(target, progress, &ctx)` | `Cow<'_, str>` | Advanced — signal-driven pacing with a custom `SignalContext`. |

The existing `get_transformer(&effect).transform(target, progress, &ctx)`
path is unchanged and remains the canonical advanced API.

### TypewriterCursor presets (since 0.3.0)

`TypewriterCursor` ships with one-line constructors for the canonical
terminal cursor glyphs:

```rust
use tui_vfx_content::types::TypewriterCursor;

let block = TypewriterCursor::block();        // █
let underscore = TypewriterCursor::underscore(); // _
let pipe = TypewriterCursor::pipe();          // |
let caret = TypewriterCursor::caret();        // ▌
let custom = TypewriterCursor::simple('◆');   // any single glyph
```

Each preset uses `Default::default()` for `blink_interval`,
`show_while_typing`, and `show_after_complete`, so consumers can opt in
to a glyph without writing the full struct literal.

Since 0.4.0, `TypewriterCursor` composes the general [`Cursor`](#cursor-primitive-since-040)
primitive via `#[serde(flatten)]`. The legacy fields (`character`,
`blink_interval`, `show_while_typing`, `show_after_complete`) continue to
work unchanged at both the Rust and JSON layers, but richer cursor
behavior — grow-in animations and fading wake trails — is authored on
the nested `Cursor` directly (e.g. `tcursor.cursor.grow_in = ...`,
`tcursor.cursor.wake = ...`).

### Cursor primitive (since 0.4.0)

`tui_vfx_content::cursor::Cursor` is a general-purpose cursor primitive. Any
effect, transformer, or overlay that wants a cursor — with optional grow-in
animation and a fading wake trail — can own one.

Defaults render a plain static block cursor. Animations opt in:

```rust
use tui_vfx_content::cursor::Cursor;

// Static (v1.1.0-compatible behavior)
let plain = Cursor::block();

// 200ms bottom-up grow on show
let greeting = Cursor::block().with_grow_in(200.0);

// Fading warmth trail, 1.5s decay, 8-cell cap
let editor = Cursor::block().with_wake_tint(1.5, 8);
```

#### Braille cursors

Row-stacked braille fills give a denser, sub-cell cursor aesthetic. The four convenience constructors each produce a static `Cursor` with the matching glyph and otherwise-default config:

| Ctor | Glyph | Dot count | Rows filled |
|------|-------|-----------|-------------|
| `Cursor::braille_2()` | `⠉` | 2 | Row 1 (top only) |
| `Cursor::braille_4()` | `⠛` | 4 | Rows 1–2 |
| `Cursor::braille_6()` | `⠿` | 6 | Rows 1–3 |
| `Cursor::braille_8()` | `⣿` | 8 | Rows 1–4 (all) |

Braille characters in `U+2800..=U+28FF` encode an 8-dot 2×4 grid. The four glyphs above are the "row-stacked fills" — dots accumulate from the top row downward, so they read as a discrete 4-step density ramp. `braille_8` (`⣿`) is visually comparable to a solid block but at braille's sub-cell density; `braille_2` (`⠉`) is a light two-dot indicator.

```rust
use tui_vfx_content::cursor::Cursor;

let light = Cursor::braille_2(); // ⠉
let dense = Cursor::braille_8(); // ⣿
```

**Subsystems:**

- `CursorBlink { interval_ms }` — `0` disables blinking (accepts legacy alias `blink_interval`).
- `GrowIn { mode, direction, duration_ms, grow_out_ms, curve }` — `mode = Never` (default) disables.
- `Wake { mode, decay_seconds, max_cells, curve, tint }` — `mode = Off` (default) disables.
- `CursorScan { mode, period_ms, curve }` — `mode = Off` (default) disables. Cycles the cursor glyph while parked (grow-in wins during its window).

#### Wake

Tail length configurability:

- `max_cells` caps the trail length (hard cap, number of cells). `0` means no cap — the trail is time-bounded only, running as long as `decay_seconds` allows.
- `decay_seconds` controls the fade speed. A larger value keeps the trail visible for longer; `0` disables the trail regardless of `mode`.
- Non-block cursors (e.g. `|`, `_`, `▌`) cannot be shape-ramped, so they use alpha-only animation for wake **and** grow-in. The 1/8th-block ramp used by grow-in and by `ScanMode::Pulse` is block-only (`█`); any other base glyph is passed through unchanged.

#### Scan

`CursorScan` (since 0.6.0 of the cursor module) cycles the cursor glyph through a bounded shape set while the cursor is parked, adding life to a steady cursor without moving it. All fields default to a no-op:

```rust
use tui_vfx_content::cursor::{Cursor, CursorScan, ScanMode};
use mixed_signals::prelude::SignalOrFloat;

let breathing = Cursor {
    scan: CursorScan {
        mode: ScanMode::Pulse,
        period_ms: SignalOrFloat::Static(1500.0),
        ..CursorScan::default()
    },
    ..Cursor::default()
};
```

| Mode | Glyph sequence | Reads as |
|------|----------------|----------|
| `Off` (default) | (base glyph unchanged) | Static cursor |
| `Pulse` | Triangle wave over `▁▂▃▄▅▆▇█` (up for half the period, back down for half) | Soft breath |
| `HalfBlockBounce` | Three-step cycle: `▀` (upper), `█` (full — brief "both"), `▄` (lower), at thirds of the period | Mechanical scanner bar |
| `BraillePulse` | Sine-eased cycle through the four row-stacked braille fills: `⣿ → ⠿ → ⠛ → ⠉ → ⠛ → ⠿ → ⣿` (phase 0/1 = densest, phase 0.5 = sparsest) | Sub-cell braille breath |
| `BrailleRowFlip` | Square-wave alternation: `⠉` (phase `<0.5`) ↔ `⠛` (phase `≥0.5`) | Calm 1-row / 2-row indicator |

**Precedence:** `grow_in` (during its active window) > `scan` > base character. Scan never overrides the grow-in ramp. `period_ms = 0` disables scan regardless of `mode`.

**Scope:**

- `Pulse` and `HalfBlockBounce` only affect block (`█`) cursors — non-block cursors (`|`, `_`, `▌`, `◆`, …) are passed through unchanged because the ramps operate on the 1/8th-block Unicode set.
- `BraillePulse` and `BrailleRowFlip` **override the base `character` unconditionally** — the output is always one of the four (resp. two) row-stacked braille glyphs. This is different from `Pulse` / `HalfBlockBounce`, which pass through non-block base chars. Set the cursor's `character` to match your intended "resting" frame (e.g. `⣿` for `BraillePulse`) so the static and animated states agree visually.

**Runtime:**

- `CursorState` holds the per-frame bookkeeping (current/previous position,
  grow-in phase, wake history). Consumers own one per rendered cursor.
- `fnc_advance_cursor(&mut state, &cursor, pos, now, dt, ctx)` advances state.
- `fnc_render_cursor(&state, &cursor, now, ctx)` produces a
  [`CursorPaintOps`](#consumer-bridging-pattern) snapshot with a primary-cell
  op and a trail of fading cells.

See rustdoc and the design spec at
`docs/superpowers/specs/2026-04-17-cursor-primitive-design.md` for full field
reference.

#### Consumer bridging pattern

The content crate produces paint ops; the style crate's `CursorShader`
consumes a flattened snapshot and renders it through the compositor. The
four-line bridge is:

```rust
// Per frame:
fnc_advance_cursor(&mut state, &cursor, pos, now, dt, ctx);
let (text, ops) = typewriter.transform_with_cursor(
    target, progress, ctx, &cursor, &mut state, now, dt,
);
// Paint text into your source grid as usual, then install the shader:
let shader = fnc_build_cursor_shader(&ops, &cursor.wake);
comp_spec.shader_layers.push(ShaderLayerSpec {
    shader: SpatialShaderType::Cursor(shader),
    region: full_area,
});
// Optional: for Ghost-mode wake, also call
//   fnc_apply_ghost_glyphs_to_grid(&mut source, &ops);
// before composition so the ghost glyphs appear where the shader paints tint.
```

#### Debug recipe catalog

Ten curated recipes under `tui-vfx-recipes/recipes/debug_recipes/content/` exercise every cursor sub-feature. All use a short 200ms slide-in so the widget lands before any cursor animation plays (earlier recipes had the cursor painting while the widget was still off-screen). Grow-in recipes use `content.mode=loop` + `every_show` grow-in + slow blink so the effect replays each loop cycle; scan recipes use `enter_only` because scan keys off `absolute_t` and cycles continuously during dwell regardless.

| Recipe file | What it shows |
|-------------|---------------|
| `content_typewriter_cursor_grow_in_up.json` | Block cursor grows in from the baseline (1/8th block → full) on each un-blink |
| `content_typewriter_cursor_grow_in_down.json` | Block cursor drops in from the ceiling on each un-blink |
| `content_typewriter_cursor_grow_in_center.json` | Block cursor expands outward from the middle row on each un-blink |
| `content_typewriter_cursor_caret.json` | Non-block caret (▌) fading in via alpha (direction is ignored for non-block glyphs) |
| `content_typewriter_cursor_wake_tint.json` | Connected tint trail behind the cursor; tint matches cursor fg (fading echo look) |
| `content_typewriter_cursor_wake_ghost.json` | Fading copies of the cursor glyph trailing the reveal head (Ghost mode) |
| `content_typewriter_cursor_wake_gap.json` | Detached meteor-tail trail — `gap_cells=3` leaves an unpainted gap between cursor and trail |
| `content_typewriter_cursor_scan_pulse.json` | Steady cursor breathing through the 1/8th-block ramp (Pulse scan, 1.5s period) |
| `content_typewriter_cursor_scan_bounce.json` | Steady cursor ping-ponging through ▀→█→▄ (Half-Block Bounce scan, 900ms period) |
| `content_typewriter_cursor_braille_2.json` | Static 2-dot row-stacked braille cursor (⠉, top row only) |
| `content_typewriter_cursor_braille_4.json` | Static 4-dot row-stacked braille cursor (⠛, top two rows) |
| `content_typewriter_cursor_braille_6.json` | Static 6-dot row-stacked braille cursor (⠿, top three rows) |
| `content_typewriter_cursor_braille_8.json` | Static 8-dot row-stacked braille cursor (⣿, all four rows — densest) |
| `content_typewriter_cursor_braille_pulse.json` | Steady cursor sine-breathing through ⣿→⠿→⠛→⠉→⠛→⠿→⣿ (BraillePulse, ~2.8s period) |
| `content_typewriter_cursor_braille_flip.json` | Steady cursor gently alternating ⠉ ↔ ⠛ on a slow 2s square wave (BrailleRowFlip) |
| `content_typewriter_cursor_full.json` | Kitchen sink: grow-in + wake tint + scan + blink all composed |

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

## Observability & Debugging

**Source:** `crates/tui-vfx-probe/*`, `docs/PIPELINE_PROBE_LLM_GUIDE.md`

`tui-vfx` now includes an engine-owned direct-scene probe surface for structured debugging.

### Phase-1 probe capabilities

| Capability | Status | Notes |
| --- | --- | --- |
| Single-frame JSON dump | ✅ | `run_probe()` / `pipeline-probe` |
| NDJSON output | ✅ | One report per line; useful for tooling |
| Selectors | ✅ | `all`, `non-empty`, `modified` |
| Widget/frame geometry | ✅ | `frame.size`, `widget.abs_origin`, `widget.size` |
| Summary counts | ✅ | `total_cells`, `non_empty_cells`, `modified_cells` |
| Compositor-stage last-touch attribution | ✅ | sampler, mask, shader, filter callbacks only |
| Rich trace emission | ✅ | sampler source coords, mask visibility, shader/filter before-after snapshots with full fg/bg state |
| Multi-frame timeline | ✅ | `collect_timeline()` / `pipeline-probe --frames N` |
| Frame diff mode | ✅ | `run_probe_diff()` / `pipeline-probe --diff-to T` |
| Probe-side diagnostics helpers | ✅ | border alpha leakage + underline-on-border detection |
| Style/content-stage hooks | ❌ | planned follow-up |
| Full engine-wide causation chain | ❌ | planned follow-up |

### Probe inputs and outputs

- **Input:** `ProbeSceneSpec`
  - `source` grid
  - `destination` frame
  - `widget_offset`
  - serialized `CompositionSpec`
- **Outputs:**
  - `ProbeReport` for one frame
  - `ProbeTimelineReport` for evenly sampled multi-frame playback
  - `ProbeDiffReport` for changed cells between two samples
  - `collect_basic_diagnostics(&ProbeReport)` for typed border/text-integrity warnings and errors

### Recommended debugging split

- Use **`pipeline-probe`** for direct engine scenes and machine-parseable frame inspection.
- Use **`pipeline-validator`** (sibling `tui-vfx-recipes` repo) when you need recipe parsing, rules, profile construction, and recipe-corpus checks.

For a concrete LLM-ready workflow, see `docs/PIPELINE_PROBE_LLM_GUIDE.md`. The guide now documents frame dumps, timelines, and diff-based debugging flows.

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
<!-- <VERS>END OF VERSION: 1.14.0</VERS> -->
