<!-- <FILE>docs/CAPABILITIES_REFERENCE.md</FILE> - <DESC>Hand-maintained capabilities reference</DESC> -->
<!-- <VERS>VERSION: 1.30.0</VERS> -->
<!-- <WCTX>Phase 3b: lift BindableU16 into RowRange/ColumnRange/Modulo coordinates. Capabilities reference must surface the new bindable contract for SynthGrid expand/collapse, animated stripe density, and scan-down reveals.</WCTX> -->
<!-- <CLOG>StyleRegion::Modulo block now lists modulus/remainder as BindableU16 with bound-form authoring example; new StyleRegion::RowRange/ColumnRange block documents the bindable endpoints and resolved() contract.</CLOG> -->
# tui-vfx Capabilities Reference

> **MAINTENANCE NOTE:** This document must be kept in sync with the source code.
> Last verified: 2026-04-26
> When adding new effects, update the relevant section below.

This document is the **parameter reference** for every visual effect primitive available in tui-vfx, derived from the actual source code. Use it when authoring recipes and you need exact field names, types, defaults, and ranges.

> **Looking for "what should I use for X?"** That decision belongs in the gt-design `/recipe-author` skill, which carries the intent → primitive selection guidance, the 17 foundational composed patterns, the render-order/conflict matrix, and the quality bar. This document is the parts catalog the skill reaches into.

Before authoring new effects or recipes, read [`TERMINAL_MOTION_HEURISTICS.md`](TERMINAL_MOTION_HEURISTICS.md) for the terminal-specific perception and compositing constraints that should shape the design.

---

## Quick scan — primitive inventory by category

One-screen overview. Names link to the per-section detail below.

**Masks** (transition shapes, 12 types) — [`None`](#masks-transition-shapes), [`Wipe`](#masks-transition-shapes) (16 directions), [`Dissolve`](#masks-transition-shapes), [`Checkers`](#masks-transition-shapes), [`Blinds`](#masks-transition-shapes), [`Iris`](#masks-transition-shapes), [`Diamond`](#masks-transition-shapes), [`NoiseDither`](#masks-transition-shapes), [`Materialize`](#masks-transition-shapes), [`PathReveal`](#masks-transition-shapes), [`Radial`](#masks-transition-shapes), [`Cellular`](#masks-transition-shapes)

**Filters** (post-processing, 28 types) — basic: [`None`](#filters-post-processing), [`Dim`](#filters-post-processing), [`Invert`](#filters-post-processing), [`Tint`](#filters-post-processing), [`Greyscale`](#filters-post-processing) · ambient: [`Vignette`](#filters-post-processing), [`PatternFill`](#filters-post-processing), [`BrailleDust`](#filters-post-processing), [`CharsetNoise`](#filters-post-processing) · retro: [`Crt`](#filters-post-processing), [`InterlaceCurtain`](#filters-post-processing) · motion: [`MotionBlur`](#filters-post-processing) · sub-cell: [`ColorBridgedShade`](#filters-post-processing), [`SubPixelBar`](#filters-post-processing), [`SubcellLight`](#filters-post-processing), [`SubCellShake`](#filters-post-processing), [`RigidShake`](#filters-post-processing) · indicators: [`HoverBar`](#filters-post-processing), [`UnderlineWipe`](#filters-post-processing), [`BracketEmphasis`](#filters-post-processing), [`DotIndicator`](#filters-post-processing), [`EdgeGrow`](#filters-post-processing) · stylistic: [`PillButton`](#filters-post-processing) · animated: [`GlistenSweep`](#filters-post-processing), [`KittScanner`](#filters-post-processing), [`ShadeScanner`](#filters-post-processing) · content-aware: [`GlyphStyle`](#filters-post-processing)

**Samplers** (coordinate distortion, 8 types) — [`None`](#samplers-coordinate-distortion), [`SineWave`](#samplers-coordinate-distortion), [`Ripple`](#samplers-coordinate-distortion), [`Shredder`](#samplers-coordinate-distortion), [`FaultLine`](#samplers-coordinate-distortion), [`RadialTwist`](#samplers-coordinate-distortion), [`Crt`](#samplers-coordinate-distortion), [`CrtJitter`](#samplers-coordinate-distortion)

**Spatial Shaders** (per-cell styling, 24 types) — structural: [`AmbientOcclusion`](#spatial-shaders-per-cell-styling), [`Bevel`](#spatial-shaders-per-cell-styling), [`Glow`](#spatial-shaders-per-cell-styling), [`ConcealedLight`](#spatial-shaders-per-cell-styling), [`Diffusion`](#spatial-shaders-per-cell-styling), [`FocusField`](#spatial-shaders-per-cell-styling), [`LinearGradient`](#spatial-shaders-per-cell-styling) · contextual: [`AffordanceWake`](#spatial-shaders-per-cell-styling), [`WayfindingNode`](#spatial-shaders-per-cell-styling), [`BorderSweep`](#spatial-shaders-per-cell-styling), [`GlistenBand`](#spatial-shaders-per-cell-styling), [`PulseWave`](#spatial-shaders-per-cell-styling), [`FocusedRowGradient`](#spatial-shaders-per-cell-styling), [`RevealWipe`](#spatial-shaders-per-cell-styling) · expressive: [`BarberPole`](#spatial-shaders-per-cell-styling), [`Radar`](#spatial-shaders-per-cell-styling), [`RadialSpiral`](#spatial-shaders-per-cell-styling), [`Highlighter`](#spatial-shaders-per-cell-styling), [`Reflect`](#spatial-shaders-per-cell-styling), [`StochasticSparkle`](#spatial-shaders-per-cell-styling), [`GlitchLines`](#spatial-shaders-per-cell-styling), [`NeonFlicker`](#spatial-shaders-per-cell-styling) · tactile: [`SubCellShake`](#spatial-shaders-per-cell-styling), [`ChromaticEdge`](#spatial-shaders-per-cell-styling)

**Style Effects** (temporal, 11 types) — [`FadeIn`](#style-effects-temporal-animations), [`FadeOut`](#style-effects-temporal-animations), [`Pulse`](#style-effects-temporal-animations), [`Rainbow`](#style-effects-temporal-animations), [`Glitch`](#style-effects-temporal-animations), [`NeonFlicker`](#style-effects-temporal-animations), [`Spatial`](#style-effects-temporal-animations), [`ItalicWindow`](#style-effects-temporal-animations), [`ColorShift`](#style-effects-temporal-animations), [`ColorFade`](#style-effects-temporal-animations), [`RigidShakeStyle`](#style-effects-temporal-animations)

**Content Transformers** (text effects, 15 types) — [`Typewriter`](#content-transformers-text-effects), [`Scramble`](#content-transformers-text-effects), [`GlitchShift`](#content-transformers-text-effects), [`ScrambleGlitchShift`](#content-transformers-text-effects), [`Dissolve`](#content-transformers-text-effects), [`Marquee`](#content-transformers-text-effects), [`SlideShift`](#content-transformers-text-effects), [`Mirror`](#content-transformers-text-effects), [`Morph`](#content-transformers-text-effects), [`Numeric`](#content-transformers-text-effects), [`Odometer`](#content-transformers-text-effects), [`Redact`](#content-transformers-text-effects), [`SplitFlap`](#content-transformers-text-effects), [`WrapIndicator`](#content-transformers-text-effects), [`GlyphCascade`](#content-transformers-text-effects)

**Shadows** (5 styles) — [`Solid`](#shadows) (default transparent full-cell), [`HalfBlock`](#shadows), [`Braille`](#shadows), [`MediumShade`](#shadows), [`Gradient`](#shadows) — all support `source_region` role-aware extrusion since v0.8.0

**Composition order** — `Sampler → Shadow → Element → Masks → Filters → Shaders` (per-cell, every frame). Shadow output carries `RoleTag::Shadow` (v0.8.0+).

**For intent → primitive guidance**, the per-shader posture, the 17 composed patterns, and the quality bar, invoke `/recipe-author` in any GT-Design session. This document is its parameter reference.

---

## Table of Contents

1. [Masks (Transition Shapes)](#masks-transition-shapes)
2. [Filters (Post-Processing)](#filters-post-processing)
3. [Samplers (Coordinate Distortion)](#samplers-coordinate-distortion)
4. [Spatial Shaders (Per-Cell Styling)](#spatial-shaders-per-cell-styling)
5. [Style Effects (Temporal Animations)](#style-effects-temporal-animations)
6. [Content Transformers (Text Effects)](#content-transformers-text-effects)
7. [Shadows](#shadows)
8. [V3 Recipe Pathway Capabilities](#v3-recipe-pathway-capabilities)
9. [Composition Pipeline](#composition-pipeline)
10. [Observability & Debugging](#observability--debugging)

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
| **Vignette** | Edge darkening | `strength`, `radius` (signal-driven), `sides`, `dither_amount`, `temporal_dither_hz` |
| **Crt** | CRT monitor post-processing | `scanline_strength`, `glow` |
| **PatternFill** | Background texture patterns | `pattern`, `color`, `only_empty` |
| **Greyscale** | Desaturate (BT.601 luminance) | `strength`, `apply_to` |
| **BrailleDust** | Animated braille particles | `density`, `hz`, `seed`, `pattern`, `color`, `drift` |
| **CharsetNoise** | Time-varying character replacement (living textures) | `hz`, `seed`, `jitter`, `affect`, `chars` (flat) or `gradient` (position-aware) |
| **InterlaceCurtain** | Scanline/interlace effect | `density`, `dim_factor`, `scroll_speed` |
| **MotionBlur** | Directional blur trail | `trail_length`, `opacity_decay`, `direction` |
| **ColorBridgedShade** | Shade char opacity (░▒▓█) | `opacity`, `fg_color`, `bg_color` |
| **SubPixelBar** | 8x resolution progress bar | `progress`, `direction`, `filled_color`, `unfilled_color`, `animated` |
| **SubcellLight** | Sub-cell light renderer for blank shell-owned cells | `lit_color`, `unlit_color`, `render_mode`, `sample_from`, `threshold`, `temporal_dither_hz`, `only_blank` |
| **SubCellShake** | Edge vibration using partial blocks | `amplitude`, `frequency`, `seed`, `edge_only`, `filled_color`, `bg_color` |
| **RigidShake** | Ketchup bottle damped oscillation | `shake_period`, `num_shakes`, `pause_duration`, `max_eighths`, `base_eighths`, `damping`, `element_color`, `bg_color`, `inner_width`, `margin_width` |
| **HoverBar** | Progress-driven partial bar indicator | `base_eighths`, `max_eighths`, `position`, `bar_color`, `bg_color`, `progress`, `margin_width` |
| **UnderlineWipe** | Horizontal underline wipe-in | `direction`, `color`, `bg_color`, `line_char`, `row_offset`, `progress`, `gradient`, `glisten` |
| **BracketEmphasis** | Fade-in brackets around content | `left`, `right`, `color`, `bg_color`, `progress` |
| **DotIndicator** | Simple dot/bullet marker | `indicator_char`, `position`, `color`, `bg_color`, `progress` |
| **EdgeGrow** | Generalized edge growth/stretch indicator | `rest_eighths`, `peak_eighths`, `edge`, `fill_color`, `bg_color`, `progress`, `margin_width` |
| **PillButton** | Pill-shaped button with gradient edges | `button_color`, `bg_color`, `edge_width`, `glisten`, `progress` |
| **GlistenSweep** | Diagonal 45° brightness sweep (hover shine) | `boost` (u8, additive), `band_width` (f32, diagonal fraction), `speed`, `progress`, `powerline_mode`, `boost_separator_bg` |
| **KittScanner** | Scanner sweep (KITT/Larson or one-way lighthouse wrap), horizontal or vertical | `boost` (u8), `band_width`, `bpm?`, `bps`, `progress`, `motion_mode`, `axis` (`horizontal` default / `vertical`), `apply_to`, `powerline_mode`, `boost_separator_bg` |
| **ShadeScanner** | Ping-pong scanner that dims text with shade overlay | `shade_color`, `bps`, `progress` |
| **GlyphStyle** | Per-glyph-category fg/bg overrides via char-membership rules | `rules`: `[{chars, fg, bg, bg_alternate?}]` first-match-wins; unmatched cells unchanged. `bg_alternate` enables a coordinate-checkerboard bg modulation bounded by char match (subtle card-edge perception). |

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
| **RadialTwist** | Center-weighted vortex/maelstrom coordinate warp | `twist`, `center`, `radius_floor` |
| **Crt** | CRT scanlines + curvature | `scanline_strength`, `jitter`, `curvature` |
| **CrtJitter** | CRT crash/jitter effect | `intensity`, `speed_hz`, `decay_ms` |

---

## Spatial Shaders (Per-Cell Styling)

**Source:** `crates/tui-vfx-style/src/models/cls_spatial_shader_type.rs`

Spatial shaders compute per-cell style modifications based on position, time, and animation state.

| Shader | Description | Key Parameters |
|--------|-------------|----------------|
| **LinearGradient** | Gradient fill at any angle | `gradient`, `angle_deg`, `apply_to` (Foreground / Background / Both, default Foreground), `intensity` (0–1, default 1) |
| **BarberPole** | Animated diagonal stripes | `speed`, `stripe_width`, `gap_width`, `color` |
| **Radar** | Rotating radar sweep | `speed`, `tail_length`, `color` |
| **RadialSpiral** | Procedural radial spiral density field for portal/background motion | `arms`, `radial_frequency`, `radial_power`, `speed`, `blend_strength`, `color` |
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
| **FocusField** | Point or pane-following focus field | `shape`, `center_x/y`, `radius_x/y`, `rect_*`, `feather`, `intensity`, `pulse_speed` |
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

**LinearGradient** — Static directional gradient fill at any angle:
- `gradient`: multi-stop gradient (stops + colour space).
- `angle_deg`: gradient axis angle in degrees, measured CCW from the positive X axis. `0` = left→right, `90` = top→bottom, `45` = TL→BR diagonal, `135` = TR→BL. Any angle is supported via projection of the cell's normalised position onto the gradient axis (since linear-gradient-shader 1.0.0); previous releases ignored the magnitude and picked an axis.
- `apply_to`: Foreground (default, back-compat), Background, or Both.
- `intensity`: blend strength `0.0..=1.0`. `1.0` (default) fully replaces the target channel; lower values blend toward base.
- Authoring alias `gradient_overlay` canonicalises to this shape; `apply_to` and `intensity` survive the canonicalisation since payload-normalize 0.5.0.
- Recipes can also use a `channel:background` scope; the lowering layer translates that into `apply_to: "background"` on the payload.

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
- When used on blank shell-owned surfaces, pairing `Glow` with `SubcellLight` can soften square cell boundaries and make the field feel less blocky

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
- `SubcellLight` can be layered afterward in blank cells when the resulting field feels too square

**FocusField** — A subtle attention field that can follow either a hotspot or a pane:
- `shape`: `Ellipse` for spotlight-like center fields, `Rect` for pane-following emphasis
- Ellipse mode uses `center_x` / `center_y` and `radius_x` / `radius_y`
- Rect mode uses `rect_x`, `rect_y`, `rect_width`, `rect_height`, with `feather` controlling soft spill outside the pane
- All geometry can be bound at runtime, making the field follow changing focus targets
- Best for active-pane emphasis, subtle hotspot guidance, and attention shaping that should be felt more than seen
- In multi-effect recipes, treat it as a **background/shell support layer** rather than the primary effect users consciously notice
- It pairs well with `ConcealedLight`, `AffordanceWake`, and `WayfindingNode`
- Avoid letting it compete with strong foreground shaders, dense text effects, or loud animated sweeps on the same cells

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

**SubcellLight** — A companion filter for light fields rather than a standalone light source:
- best used after `Glow`, `ConcealedLight`, `Diffusion`, or `FocusField`
- interprets the existing light field and re-renders blank cells in the targeted region with braille or partial blocks
- useful when a background light field feels too obviously square on the cell grid
- should usually stay on shell-owned cells, not dense body text

**SubCellShake (Shader)** — Micro-jitter visual effect through rapid color oscillation:
- `axis`: Horizontal, Vertical, Both (default)
- `chromatic`: Enable RGB channel separation for chromatic aberration
- `edge_only`: Limit shake to widget borders

#### Filter Notes

**InterlaceCurtain** — Scanline/interlace dimming effect:
- `density`: Row spacing (1.0 = every other row)
- `scroll_speed`: Animation speed for scrolling scanlines

**Vignette** — Edge darkening that draws focus inward:
- `strength`: overall dimming amount
- `radius`: how far the vignette reaches inward
- `sides`: optional list of directional edges (`top`, `bottom`, `left`, `right`) for single-side or two-side light/falloff impressions
- `dither_amount`: optional low-amplitude contour softening on large flat fields
- `temporal_dither_hz`: optional low-rate temporal refresh for the dither
- Use subtle dither when the vignette reads too obviously as a square cell-grid gradient rather than a soft optical falloff

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

**SubcellLight** — Companion filter that renders an existing light field into partial-block or braille glyphs in blank cells:
- `render_mode`: `braille`, `horizontal`, or `vertical`
- `sample_from`: `foreground` or `background` light field to interpret
- `lit_color` / `unlit_color`: endpoint colors for the rendered glyph
- `threshold`: minimum normalized intensity before a blank cell is converted
- `temporal_dither_hz`: optional low-rate temporal braille variation (0 = static)
- Best when paired with `ConcealedLight`, `Diffusion`, or `FocusField` on shell-owned blank cells to make light feel less square

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

**KittScanner** — Scanner sweep, horizontal or vertical:
- `boost`: Additive u8 brightness boost under the band (default 50)
- `band_width`: Width of the scanner band as fraction of total extent along the active axis (default 0.15, typical 0.0..0.5)
- `bpm`: Optional human-readable beats-per-minute cadence. When present, it overrides `bps`
- `bps`: Beats per second for the scanner cycle (default 1.2 = 72 BPM) — **not** `speed`
- `progress`: 0.0..1.0, set to 1.0 to activate
- `motion_mode`: `ping_pong` (classic KITT/Larson), `forward_wrap`, or `reverse_wrap`
- `axis`: `horizontal` (default — band sweeps left↔right) or `vertical` (band sweeps top↔bottom). Vertical reuses the same oscillator; only the coordinate fed in changes (`y/height` instead of `x/width`). Use vertical for column-wise reveals (TTE Beams), staff lights, scanline-down effects.
- `apply_to`: Which color component to boost (fg / bg / both, default Both)
- `powerline_mode` / `boost_separator_bg`: See GlistenSweep
- Cadence-driven motion uses monotonic elapsed time. In `ping_pong` mode, one full return cycle is `120 / bpm` ms (or `2 / bps` seconds), but recipe loop period only controls how often the surrounding recipe repeats.
- Use a red base style for the classic KITT/Larson or lighthouse look — the boost is additive, not replacement
- Ideal for status bars, alert indicators, ambient attention-getters; vertical axis covers reveal/scan effects that previously needed a sampler chain

**ShadeScanner** — Ping-pong scanner that dims text with a shade overlay:
- `shade_color`: The dimming overlay color applied as the band sweeps (default dark grey)
- `bps`: Beats per second for the ping-pong cycle (default 1.0)
- `progress`: 0.0..1.0, set to 1.0 to activate
- Simpler than KittScanner (no boost, no band_width, no powerline options) — this is a dimming sweep, not a brightening sweep
- Ideal for "reading" effects, progressive-reveal teases, subtle attention cues

### Content-Aware Filter

**GlyphStyle** — Per-glyph-category fg/bg overrides via char-membership rules.
For each cell, evaluates rules in declaration order and applies the first
rule whose `chars` set contains the cell's character. Cells that match no
rule pass through unchanged. Designed for content transformers that emit
mixed glyph categories in a single output stream — color each category
independently without per-cell `RoleMap` plumbing.

- `rules`: `Vec<GlyphStyleRule>` — each rule is `{ chars, fg?, bg?, bg_alternate? }`
- `chars`: a `String` of characters this rule matches (any match triggers); order inside the string doesn't matter
- `fg` / `bg`: optional `ColorConfig` overrides; omit to leave that channel untouched
- `bg_alternate`: optional `ColorConfig` — when set, cells with odd `(x + y)` parity use this color instead of `bg`, producing a coordinate-checkerboard bg modulation. Bounded by the rule's char match — only matched cells alternate, so applying it to a "letter cells" rule gives just-perceptible per-card edges without painting the whole board. Real-Solari bezel suggestion at zero compute cost.
- First-match-wins: order rules from most-specific to least-specific
- Unmatched cells are unchanged (no implicit catch-all; add a final rule with the chars you want as a fallback if you need one)

Primary motivation: `SplitFlap` boards emit block (`█▓▒░`), hinge
(`▀▔—▁▄`), letter (A-Z, 0-9, …), and turned-preview glyphs (`Ⱡꓭ⊥∩…`)
in one stream. A 2-3-rule `GlyphStyle` filter lets each category have
its own color — block/hinge get a slightly-lighter card-face grey,
letters get a darker board-card grey, turned previews get dimmer fg —
producing the depth-tone Solari-board aesthetic without any role
tagging. Useful equally for `GlyphCascade`, `Scramble`, `Redact`, or
any other transformer that mixes character categories.

```json
{
  "type": "glyph_style",
  "rules": [
    { "chars": "█▓▒░▀▔—▁▄",
      "fg": {"type": "rgb", "r": 180, "g": 180, "b": 180},
      "bg": {"type": "rgb", "r":  48, "g":  48, "b":  48} },
    { "chars": "ⱯꓭƆꓷƎℲ⅁ꓘ⅂Ԁꓤ⊥∩Λ⅄Ɛ",
      "fg": {"type": "rgb", "r": 150, "g": 150, "b": 150} },
    { "chars": "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,!?·→:-/",
      "bg":           {"type": "rgb", "r": 28, "g": 28, "b": 28},
      "bg_alternate": {"type": "rgb", "r": 36, "g": 36, "b": 36} }
  ]
}
```

The `bg_alternate` on the third rule gives the letter cells a subtle
checkerboard alternation — adjacent cards read as distinct without
the alternation reading as a visible pattern.

Composes naturally with the existing pipeline order
(`Sampler → Shadow → Element → Masks → Filters → Shaders`): GlyphStyle
runs in the Filters stage and modifies fg/bg before any per-region
shaders apply, so a downstream `BorderSweep` or `LinearGradient`
shader still sees and overlays its effect on whatever fg/bg
GlyphStyle settled on.

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
| **Odometer** | Mechanical tile-grid roll; replaces old digit interpolation |
| **Redact** | Redaction/censoring effect |
| **SplitFlap** | Split-flap/Solari display with 1x1 legacy and even 2/4/6/8 center-hinged tiles |
| **WrapIndicator** | Prefix/suffix wrapping based on progress |
| **GlyphCascade** | Glyph alphabet cascade / symbol evolution |

### Mechanical Display Notes

- **Odometer** is a mechanical cell-grid/tile roll. It rolls old tile cells out of a fixed viewport while target tile cells enter from the opposite edge; the old vertical digit interpolation behavior is intentionally replaced. Key fields: `direction`, tagged `travel` (`{ "type": "axis" }`, `{ "type": "full_clear" }`, or `{ "type": "cells", "cells": N }`), `tile_width`, `tile_height`, and optional `from_message`.
- **SplitFlap** keeps `1x1` legacy character flips by default. Larger Solari-style cards use `tile_width` plus even `tile_height` values `2`, `4`, `6`, or `8` for center-hinged rendering; invalid multi-cell heights are rejected by transformer validation.

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
| **Solid** (default) | Space cells with alpha background | Clean transparent full-cell drop shadow |
| **HalfBlock** | Half-block characters (▐▄▌▀) | Explicit sub-cell texture |
| **Braille** | 2×4 subpixel density grid | Fine-grained, font-dependent |
| **MediumShade** | Medium shade glyph (`▒`) | Textured full-cell shadow |
| **Gradient** | Multi-layer decreasing intensity | Softer appearance |

### ShadowConfig Properties

| Property | Type | Description |
|----------|------|-------------|
| `style` | ShadowStyle | Rendering technique |
| `offset_x` | i8 | X offset (positive = right) |
| `offset_y` | i8 | Y offset (positive = down) |
| `inset_x` | Option<u8> | Trim the start of top/bottom shadow runs, e.g. left side of a bottom shadow; bottom-only defaults to 2 when unset |
| `inset_x_end` | Option<u8> | Trim the end of top/bottom shadow runs, e.g. right side of a bottom shadow; bottom-only defaults to 2 when unset |
| `inset_y` | Option<u8> | Trim the start of left/right shadow runs, e.g. top of a right shadow |
| `inset_y_end` | Option<u8> | Trim the end of left/right shadow runs, e.g. bottom of a right shadow |
| `falloff_x` | Option<u8> | Alpha falloff cells at the start/end of top/bottom runs |
| `falloff_y` | Option<u8> | Alpha falloff cells at the start/end of left/right runs |
| `side_coverage_eighths` | Option<u8> | Optional sub-cell foreground block coverage for left/right solid shadow columns; `6` gives a three-quarter side edge |
| `color` | Color | Shadow color |
| `surface_color` | Option<Color> | Background for half-block blending |
| `edges` | ShadowEdges | Which edges to render. Builder-facing names include `BOTTOM_RIGHT` / `ALL`; recipe JSON currently uses bitflag strings such as `"RIGHT | BOTTOM"`. |
| `soft_edges` | bool | Use half-blocks at boundaries |
| `composite_mode` | ShadowCompositeMode | `GlyphOverlay` (default), `GradeUnderlying`, or `BlendUnderlying` |
| `grade` | Option<ShadowGradeConfig> | Grade parameters for `GradeUnderlying`; ignored by `GlyphOverlay` and `BlendUnderlying` |
| `source_region` | Option<RoleTag> | v0.8.0+ — restrict extrusion to cells whose role matches the tag. `None` (default) = rectangular extrusion as before. `Some(role)` = extrude from the tight bounding rectangle of role-matched source cells (see `extract_shadow_envelope`) and tag produced cells with `RoleTag::Shadow` in the destination role map. Fixes the "shadow on text rect" splash bug. Builder: `with_source_region(role)`; accessor: `source_region()`. Recipe JSON currently uses enum names like `"Border"`. |

`side_coverage_eighths` is an optical tuning control for side shadows. With
`GlyphOverlay`, a value such as `6` renders the side as a three-quarter block
glyph over transparent background so the vertical edge feels lighter than a
full-cell column. Prefer `BlendUnderlying` / `GradeUnderlying` for shadows that
must preserve existing destination glyphs; a terminal cell cannot display the
destination glyph and a separate sub-cell shadow glyph at once.

Note: as of v0.8.0, `ShadowConfig` is no longer `Copy` — `RoleTag::Custom` carries an `Arc<str>`. Call-sites that relied on implicit copy must add `.clone()`.

Since Hotfix H2 (2026-04-20), `tui-vfx-recipes` exposes this upstream surface directly as top-level `config.shadow`, and both `pipeline-validator` / `recipe-probe` now preserve the authored `ShadowSpec` instead of silently dropping it.

### New in v0.8.0 — Role-aware shadow entrypoints

| Item | Description |
|------|-------------|
| `render_shadow_into_scene(source_grid, source_roles, &mut SemanticScene, element_rect, config, progress)` | High-level entrypoint: honours `config.source_region` and writes `RoleTag::Shadow` into destination roles for produced cells. |
| `extract_shadow_envelope(source_grid, source_roles, source_region)` | Pure function returning a `CellMask` of cells the shadow stage should extrude from. |
| `CellMask` | Dense row-major boolean mask with `get((x,y))`, `set`, `count`, `bounding_rect` accessors. |

---


## V3 Recipe Pathway Capabilities

V3 recipes are the compositional authoring pathway for `tui-vfx-recipes` and
GT-Design. The primitive inventory above still matters, but V3 adds a higher
level of control: recipes can declare scene layers, source surfaces, runtime
contracts, and explicit data flow between effects. Treat this section as the
capability map for deciding whether a recipe should be a simple single-effect
preview, a chained effect system, or a scene-level composition.

### What V3 adds beyond the primitive catalog

| Capability | What it means for authors | Use when |
|------------|---------------------------|----------|
| **First-class effect I/O** | Pipeline steps can declare `io.outputs` and `io.inputs` so one effect produces a named hint/signal and later effects consume it. | A sampler/filter/shader/style effect should be driven by another effect instead of duplicating timing math. |
| **Ordered step trees** | `sequence` preserves authored order; later siblings can consume earlier outputs. `parallel` snapshots sibling inputs and exposes outputs after the join. | You need predictable data dependencies or safe batching. |
| **Cross-family chaining** | Samplers, masks, filters, shaders, style effects, and content steps can pass hints across family boundaries. | A field, scalar, mask, or style decision should influence a different visual lane. |
| **Scene layers** | Recipes can define multiple layers, each with `source`, `placement`, `surface`, `visibility`, and layer-local `pipeline`. | Build cards, overlays, procedural backdrops, badges, captions, or composed hero scenes. |
| **Runtime contracts** | `requires_bindings`, `requires_tokens`, and `requires_assets` document host-supplied inputs and file-backed assets. | The host should drive focus, hover, progress, text, colors, or asset selection without code changes. |
| **Binding-resolved params** | Any procedural/source param can contain `{ "binding": "key", "default": value }`; the compiled scene path resolves it once per frame before rendering. | The same recipe needs to react to host state while keeping procedural sources deterministic. |
| **Procedural sources** | Scene layers can render deterministic stock or host-registered generators such as spinners, breathe/pulse fields, fireworks, or file-backed braille dotfields. | Content should be generated from parameters/assets rather than hand-coded as an app-specific widget. |
| **Canonical inspection** | `pipeline-validator --dump-normalized`, `--strict-contracts`, and `--lowering-report` expose normalized IR, contract usage, and migration/lowering evidence. | You need to debug or review what the recipe actually means after normalization. |

### First-class I/O: the central V3 superpower

V3 I/O turns effects from isolated decorations into a signal graph. A producer
step names an output; a later consumer references that output. The runtime can
then preserve the dependency, batch safe independent branches, and reject invalid
cross-feeds.

Minimal shape:

```json
{
  "kind": "sequence",
  "children": [
    {
      "kind": "sampler",
      "scope": { "kind": "all" },
      "io": {
        "outputs": [{ "hint": "wave_field", "kind": "scalar" }]
      },
      "payload": { "type": "spatial_signal", "signal": { "type": "sample_norm_x" } }
    },
    {
      "kind": "sampler",
      "scope": { "kind": "all" },
      "io": {
        "inputs": [{ "input": "amplitude", "hint": "wave_field", "kind": "scalar" }]
      },
      "payload": { "type": "sine_wave", "axis": "y", "frequency": 1.5 }
    },
    {
      "kind": "shader",
      "scope": { "kind": "all" },
      "io": {
        "inputs": [{ "input": "intensity", "hint": "wave_field", "kind": "scalar" }]
      },
      "payload": { "type": "diffusion", "source": "right" }
    }
  ]
}
```

Authoring rule of thumb:

- Use **outputs** for reusable values: scalar progress, spatial fields, masks,
  displacement maps, focus/falloff fields, or semantic activity flags.
- Use **inputs** when another step should react to that value.
- Use `sequence` when step B must see step A's output.
- Use `parallel` when siblings are independent; outputs become visible after the
  parallel join.
- Keep names semantic (`wave_field`, `focus_falloff`, `hover_progress`) rather
  than implementation-shaped (`sampler_1_output`).

### New V3 edge/pathway primitives from adjacent-library review

The whoa/cellophane review added a small set of reusable, substrate-named
capabilities. The borrowed demo names are intentionally not the API surface; the
public names describe the underlying motion or field so recipe authors can reuse
them across flags, dashboards, loaders, and design-system surfaces.

| Capability | Recipe/API name | What it does | Authoring notes |
|------------|-----------------|--------------|-----------------|
| Projected corkscrew path | `carrier_orbit` (`helix` alias) | Moves content around a carrier route while preserving start/end positions. | Use for orbiting badges, corkscrew arrivals, or attention carriers. `helix` is accepted for author convenience; `carrier_orbit` is the substrate name. |
| Figure-eight path | `figure_eight` (`infinity`, `infinity_symbol`, `lemniscate` aliases) | Produces a 2D harmonic figure-eight/∞ route. | Use when a focus item should continually cross the center without implying 3D depth. This is not a sideways helix. |
| Vortex source remap | `radial_twist` sampler | Re-samples content through a center-weighted twist field. | Use for portal, maelstrom, and pull-into-center distortions; keep text large because coordinate warps can reduce legibility. |
| Spiral density field | `radial_spiral` shader / V3 `motion_field.radial_spiral` | Blends style through a radial/angle spiral field. | Use as a procedural loading/background texture or as a shader paired with sampler/mask output hints. |

Shared math for these route and field treatments lives in `mixed-signals` so it
can be reused consistently by `tui-vfx`, recipes, and downstream applications.

### Chaining examples that V2 could not express cleanly

#### 1. One spatial field drives both displacement and light

A sampler emits `wave_field`; a downstream sampler uses it to displace cells;
a shader uses the same field to shade the wave crest. This creates a coherent
“one force, two manifestations” effect: the text moves and brightens from the
same signal.

```text
sampler(spatial_signal) -> wave_field
  -> sampler(sine_wave, amplitude=wave_field)
  -> shader(diffusion, intensity=wave_field)
```

Use for water ripples, heat shimmer, sonar sweeps, magical materialization, or
music-reactive UI.

Concrete fixture family: `complex_field_hint_displace_shade.json` and related
field-hint debug recipes.

#### 2. A filter derives a mask-like scalar that drives a shader

A filter can re-emit a computed scalar (`dim_factor`, `edge_progress`,
`scan_position`) and a later shader can use that value for color, glow, or
highlight intensity. This lets an operational effect become an aesthetic driver.

```text
filter(kitt_scanner) -> scan_position
  -> shader(border_sweep, input=scan_position)
```

Use for scanner bars that also light the border, hover rails that also wake
secondary affordances, or error shakes that also italicize/highlight text.

#### 3. Parallel branches compute independent signals, then a post-join consumer combines them

Parallel is safe for independent producers: each branch sees the same input
snapshot, then outputs are joined. A later step can consume both.

```text
parallel {
  sampler(focus_field) -> focus_falloff
  filter(hover_bar) -> hover_progress
}
shader(affordance_wake, inputs=[focus_falloff, hover_progress])
```

Use for rich focus states where cursor position, hover progress, and selection
state each contribute without one branch accidentally reading another branch's
partial output.

Concrete fixture family: `v3_scheduler_parallel_join_sampler_style.json` and
post-join sampler/style tests.

#### 4. Content output drives downstream visual treatment

A content transformer can expose progress or activity, then downstream filters
or shaders can react. For example, a typewriter reveal can emit current reveal
progress; a highlighter shader can follow the revealed edge and a cursor/wake
filter can trail behind it.

```text
content(typewriter) -> reveal_progress
  -> shader(highlighter, input=reveal_progress)
  -> filter(cursor_wake, input=reveal_progress)
```

Use for typing assistants, command palettes, onboarding copy, terminal tutorial
flows, and “decrypting” dashboards where the visual treatment follows text
semantics instead of a separate clock.

Concrete fixture family: `content_typewriter_io_filter_shader.json`.

### Runtime bindings, tokens, and assets

V3 recipes can be host-driven without requiring new Rust code for each variant.
Declare external dependencies at the top level:

```json
{
  "requires_bindings": {
    "hover_progress": { "type": "number", "default": 0.0 },
    "selected_row": { "type": "number", "default": 0 },
    "show_hint": { "type": "bool", "default": true },
    "wave_speed": { "type": "number", "default": 1.0, "range": [0.0, 4.0] }
  },
  "requires_tokens": {
    "headline": { "type": "string", "default": "READY" }
  },
  "requires_assets": {
    "madeira_flag_base": {
      "type": "braille_dotfield",
      "format": "tui-vfx.braille_flag_asset.v1",
      "canonical_path": "recipes/madeira_flag/assets/base_flag_dots.json"
    }
  }
}
```

Use bindings for live host state (hover, selection, progress, time-varying
controls). Use tokens for text or theme substitutions. Use assets for external
visual data such as a braille dotfield, icon grid, image-derived source, or
procedural source seed data.

The current binding rules authors should remember:

- Declare every host-supplied key in `requires_bindings`; `--strict-contracts`
  rejects runtime binding use that is not declared.
- Use binding leaves inside source/procedural params when the host should drive a
  value: `{ "binding": "wave_speed", "default": 1.0 }`. The compiled V3 scene
  path resolves these leaves once per frame, before the procedural source or
  effect receives its payload.
- Use `visibility.predicate` with either a declared bare key (`"show_hint"`) or
  an explicit `"binding:show_hint"` when a layer should appear/disappear from
  host state.
- Use `{{token_name}}` for text/theme template substitutions and `{{ asset_key
  }}` for contract-backed asset paths. `asset.ref` / `asset.key` may name a
  `requires_assets` entry directly when a source supports it, but the placeholder
  form makes the dependency visible where it is used.
- Keep runtime bindings distinct from step I/O hints: bindings come from the
  host/contract boundary; I/O hints are produced by effects inside the same
  per-frame pipeline.

Example: a scene layer can hide/show from host state while its procedural source
is still driven by a separate runtime binding:

```json
{
  "id": "flag",
  "visibility": { "predicate": "show_hint" },
  "source": {
    "type": "procedural",
    "spec": {
      "source_id": "braille_flag_field",
      "params": {
        "asset": {
          "path": "{{ madeira_flag_base }}",
          "format": "tui-vfx.braille_flag_asset.v1"
        },
        "wave": {
          "speed": { "binding": "wave_speed", "default": 1.0 }
        }
      }
    }
  }
}
```

### Procedural sources and asset-backed content

Procedural scene sources let a recipe generate content from parameters, host
bindings, and external assets. They are the V3 escape from one-off demo code:
write or register a deterministic source once, then vary it through recipe data.

Stock source ids currently include:

| Source id | Capability | Typical use |
|-----------|------------|-------------|
| `braille_spinner` / `dots_spinner` / `line_spinner` | Small deterministic loading indicators. | Inline waits, badges, status affordances. |
| `breathe` / `pulse` | Ambient clock-driven fills/highlights. | Low-motion background life, selection emphasis. |
| `solid_color_fade` | Full-canvas color underlay/backdrop helper used by scene recipes that own their canvas. | Backdrops, recipe-owned black/blue surfaces, fade-in underlays. |
| `ballistic_fireworks` | Deterministic particle-style fireworks from palette, spawn zones, timing, gravity, and seed params. | Celebratory scenes and hero moments. |
| `braille_flag_field` | File-backed braille dotfield resampled into a waving, shaded flag-like surface. | Asset-agnostic image/flag/banner demos. |
| `fallback_procedural` | Visible deterministic fallback for unknown ids. | Debugging missing registrations; not intended as authored output. |

Procedural authoring rules:

- Sources must be deterministic for the same `clock`, target rect, params,
  assets, and bindings; do not hide wall-clock or mutable state in them.
- Sources should be tiny-rect safe (`1×1` must not panic) and tag painted cells
  as procedural/semantic roles so downstream scopes can target them.
- Transparent overlay sources should leave unpainted cells empty and visible
  glyphs transparent-background unless the recipe deliberately owns the canvas.
- Recipe-owned canvas mode is explicit: add a backdrop/base surface so
  transparent procedural cells reveal the intended black/blue/etc. underlay
  rather than the host preview substrate.
- Prefer file-backed assets for reusable visual material. The Madeira flag base
  art lives in `recipes/madeira_flag/assets/base_flag_dots.json`, is declared in
  `requires_assets.madeira_flag_base`, and is loaded through the generic
  `braille_flag_field` path rather than embedded in Rust.
- Hosts can install additional sources through a procedural registry; authored
  recipes should still expose their variable inputs as params/bindings/assets so
  humans and AI tools can inspect and mutate them.

The Madeira flag recipe is the current reference for asset-backed procedural
scene work: the base flag dotfield is file-backed, the flag wave speed and
fireworks enablement are runtime-bound, text lines are tokenized, and the same
V3 pathway can load a different compatible dotfield asset without adding
Madeira-specific Rust code.

### Scene layers and layer-local pipelines

A V3 scene recipe can compose multiple sources:

```json
{
  "config": {
    "scene": {
      "layers": [
      {
        "id": "backdrop",
        "source": { "type": "procedural", "spec": { "source_id": "solid_color_fade" } },
        "placement": { "type": "anchor", "spec": { "anchor": "center" } }
      },
        {
          "id": "flag",
          "source": { "type": "procedural", "spec": { "source_id": "braille_flag_field" } },
          "placement": { "type": "absolute", "spec": { "rect": { "x": 1, "y": 0, "width": 40, "height": 17 } } },
          "pipeline": { "step": { "kind": "shader", "payload": { "type": "glisten_band" } } }
        }
      ]
    }
  }
}
```

Layer capabilities:

- `source`: text, card, image-like, or procedural content.
- `placement`: anchor or absolute geometry, optionally sibling-relative through
  `sibling_id`, `offset_rows`, and `offset_cols`.
- `surface`: base style and attached shadow owned by the layer.
- `visibility`: phase or binding-driven layer visibility.
- `role_tag`: semantic tagging (`content`, `background`, `decoration`, etc.) for
  role scopes, shadows, and downstream reasoning.
- `pipeline`: layer-local sampler/mask/filter/shader/style/content chain; hints
  produced here are local to that layer pipeline rather than automatically
  shared across sibling layers.
- `timing`: optional layer-local enter/exit duration, delay, and easing so a
  scene can choreograph surfaces without moving logic back into application
  code.

Use scene layers when the design has independently meaningful surfaces:
background firework field + flag + label text; modal card + badge + tooltip;
status panel + animated border + inline command hint.

### Validation and inspection commands for authors

Run these while authoring V3 recipes:

```bash
cd /usr/projects/tui-vfx-recipes

# See canonical normalized V3 IR and discovered contract usage.
cargo run -q -p pipeline-validator -- \
  --dump-normalized --format json recipes/path/to/recipe.json

# Enforce declared runtime bindings/tokens/assets.
cargo run -q -p pipeline-validator -- \
  --rules --strict-contracts recipes/path/to/recipe.json

# Inspect lowering/migration evidence and human-review flags.
cargo run -q -p pipeline-validator -- \
  --lowering-report --format json recipes/path/to/recipe.json

# Probe rendered behavior and timeline/output summaries.
cargo run -q -p pipeline-validator -- \
  --probe --format json recipes/path/to/recipe.json
```

### Choosing the right V3 shape

- **Single primitive preview:** one leaf step, no I/O. Best for documenting a
  mask/filter/shader/style/content primitive.
- **Simple chain:** one `sequence`; producer before consumer. Best for “this
  thing drives that thing.”
- **Parallel producers + post-join consumer:** one `parallel` followed by a
  consumer. Best for independent field/scalar generation.
- **Scene recipe:** `scene.layers[]` with optional layer-local pipelines. Best
  for composed UI surfaces and asset/procedural demos.
- **Host-driven recipe:** declare `requires_bindings`, `requires_tokens`, and
  `requires_assets`. Best for downstream design-system integration.
- **Procedural/asset recipe:** use a deterministic `source_id` plus file-backed
  assets and binding-resolved params. Best when the authored visual should be
  reusable with different artwork, palettes, speeds, or enablement gates.
- **Semantic surface recipe:** use role-tagged scene layers, role scopes, and
  role-aware shadows when the visual depends on meaning (`border`, `content`,
  `decoration`) rather than raw rectangles.

### Coverage reminders for human and AI authors

Before inventing new code, check whether the desired result can be expressed by
combining these existing V3 surfaces:

- source layers for separate semantic surfaces
- bindings for host state
- tokens for text/theme substitutions
- assets for visual material that should live outside Rust
- procedural sources for deterministic generated content
- step I/O for effect-to-effect signal flow
- scopes/roles for targeting
- timing/lifecycle fields for choreography
- validator/probe/lowering reports for proof

If one of those surfaces almost fits but lacks a narrow primitive, add the
smallest reusable primitive/source and document its contract. Avoid returning to
demo-specific Rust for things that can be recipe data, a file-backed asset, or a
parameterized procedural source.

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

### Role-Aware Pipeline Signature (v0.7.0+)

Since v0.7.0 all four public entrypoints (`render_pipeline`,
`render_pipeline_with_area`, `render_pipeline_with_spec`,
`render_pipeline_with_spec_area`) require a `source_roles: &RoleMap` and
a destination `&mut SemanticScene`. Call-sites without semantic role
information construct `RoleMap::all_background(w, h)` and
`SemanticScene::from_grid_with_default_role(grid, RoleTag::Background)`
as the no-info default.

### `StyleRegion` Targeting

`StyleRegion::Role(RoleTag)` is the canonical role-based targeting
variant. Available role tags include `Background`, `Text`, `Title`,
`Caption`, `Border`, `Image`, `Icon`, `Indicator`, `Highlight`,
`Shadow`, `Decoration`, `Procedural`, and `Custom(InternedRoleName)`.

Legacy JSON fixtures that still write `"BorderOnly"`, `"TextOnly"`, or
`"BackgroundOnly"` continue to parse — a custom `Deserialize` lowers
them to `Role(RoleTag::Border / Text / Background)` respectively.
Serialization always emits the canonical form; the schema converges
on round-trip.

#### `StyleRegion::Modulo`

```rust
StyleRegion::Modulo {
    axis: ModuloAxis,          // Horizontal | Vertical
    modulus: BindableU16,      // period (0 produces no matches; bindable since 5.2.0)
    remainder: BindableU16,    // offset within the period (bindable since 5.2.0)
}
```

`ModuloAxis::Horizontal` makes the rule scan **row by row**: cell
`(x, y)` matches iff `y % modulus == remainder`, so one matched row
becomes one full-row stripe. `ModuloAxis::Vertical` scans
**column by column**: `(x, y)` matches iff `x % modulus == remainder`,
producing full-column stripes. The axis name describes the direction
the rule iterates, not the orientation of the stripes it draws.

Use cases: CRT scanlines (`Horizontal`, modulus 2, remainder 0),
ledger paper (`Horizontal`, modulus 3, remainder 0), alternating
column highlights (`Vertical`, modulus 2, remainder 1), animated
stripe-density sweeps (Horizontal, `modulus: {"binding": "stripe_period"}`).

Authored from V3 recipes via:

```json
"scope": {
  "kind": "modulo",
  "axis": "horizontal",
  "modulus": 3,
  "remainder": 0
}
```

`modulus` and `remainder` accept either a bare integer (back-compat),
the tagged `{"literal": N}` form, or `{"binding": "name"}`. The recipe
compiler folds literal-only inputs into `CompiledScope::StaticModulo`
for the compact static path; bound inputs flow through the dynamic
path and emit `StyleRegion::Modulo { modulus: BindableU16::Binding,
remainder: BindableU16::Binding }`, which `StyleRegion::resolved` then
lowers once per layer per frame against `ShaderRuntimeParams`.
Authors should keep `modulus >= 1`; a value of `0` would produce
no matches at runtime (the rule never fires).

Debug recipes:
- `recipes/debug_recipes/styles/style_modulo_horizontal_every_third_row.json`
- `recipes/debug_recipes/styles/style_modulo_vertical_every_fourth_column_offset.json`

#### `StyleRegion::RowRange` / `StyleRegion::ColumnRange`

```rust
StyleRegion::RowRange    { start: BindableU16, end: BindableU16 }  // [start, end)
StyleRegion::ColumnRange { start: BindableU16, end: BindableU16 }  // [start, end)
```

`start` and `end` accept the same `BindableU16` shapes as `Modulo`'s
numeric fields (bare integer / `{"literal": N}` / `{"binding": "name"}`).
Use the bindable forms to drive SynthGrid-style expand/collapse
animation, scan-down reveals, or any range whose endpoints come from a
runtime parameter. The `should_style` predicate silently matches
nothing when either endpoint is still a `Binding`; the render pipeline
calls `StyleRegion::resolved` once per layer per frame to lower
bindings to literals before the hot loop sees them.

Authored from V3 recipes via:

```json
"scope": {
  "kind": "row_range",
  "start": { "binding": "synth_grid_start_row" },
  "end": { "binding": "synth_grid_end_row" }
}
```

Literal-only inputs collapse to `CompiledScope::StaticRowRange`
(`StaticColumnRange`) at compile time and emit literal-typed
`StyleRegion::RowRange` (`ColumnRange`) without any per-frame work.

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

### Unified Inspection Foundation (v0.9.0+)

Since v0.9.0 (Sub-plan A Phase A.4) the workspace also ships a canonical AI-consumable trace surface in `tui-vfx-debug::inspection`:

| Capability | Type / Function | Purpose |
|------------|-----------------|---------|
| Canonical event taxonomy | `TraceEvent` (18 variants, `#[non_exhaustive]`) | One enum covering lifecycle / resolution / composition / pipeline stages. |
| Event wrapper | `TraceEnvelope` | Event + `frame_no` + `t_ms` + optional `recipe_id` + monotonic `seq_in_frame`. |
| Declarative filter | `TraceFilter` + `StageMask` bitflags | Selectors (OR) + stage mask (AND) + frame range + time range; emit-site short-circuit. |
| Sink trait | `InspectionSink` (object-safe, `Send + Sync`) | `fn report(&self, envelope: TraceEnvelope)`. |
| Default sink impl | `TraceSink` + `with_capacity(filter, n)` | Thread-safe, optionally-bounded (drop-oldest with `dropped` counter). |
| Report + NDJSON | `TraceReport::to_ndjson(writer)` / `from_ndjson(reader)` | AI / CLI round-trip. |
| Compositor bridge | `tui-vfx-compositor::traits::cls_inspection_sink_bridge::InspectionSinkBridge` | Additive: implements `CompositorInspector` and forwards callbacks into any `InspectionSink`. `CompositorInspector` itself stays put — existing `ProbeInspector` / `StageInspector` / `TraceInspector` impls are unchanged. |

Full schema (every variant field) lives in [docs/TRACE_EVENT_SCHEMA.md](TRACE_EVENT_SCHEMA.md). The Sub-plan B CLI consumer (`tui-vfx-trace`) will ingest NDJSON produced here.

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
- Shadows: Solid transparent full-cell with alpha, Gradient

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
- Solid, HalfBlock, and textured shadow styles

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
<!-- <VERS>END OF VERSION: 1.28.0</VERS> -->
