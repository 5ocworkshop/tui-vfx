<!-- <FILE>docs/design/tui-vfx-glyph-rendering-framework-plan.md</FILE> - <DESC>Implementation plan for the shared scalar-field-to-glyph framework consumed by water, fire, and future field-driven effects. Covers the upstream math/noise lift into mixed-signals, the encoder/subcell helper layer in tui-vfx-types, the unifying ScalarFieldGlyphFilter in tui-vfx-compositor, the WaterFieldSignal wrapper, and the first water-glyph debug recipe.</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>Phase 7 doc closure landed: water plan §13/§21.1 marked Implemented; fire plan §9.0/§9.1/§9.10 reference upstream primitives + FireFieldSignal pattern; INDEX entry added.</WCTX> -->
<!-- <CLOG>0.3.0: Phase 7 doc closure complete — all seven phases shipped; framework is the canonical scalar-field-to-glyph surface for water (live) and fire (next consumer).</CLOG> -->

# tui-vfx glyph rendering framework — plan

## 0. One-paragraph goal

Stand up a shared three-layer pipeline so that any 2D scalar field — water now, fire next, future terrain/audio/noise visualizers — can render to subcell-positioned braille or block-bar glyphs through one canonical filter. Lift the renderer-agnostic math primitives (`smoothstep`, `lerp`, `fade`, `saturate`) and the noise primitives (`hash01`, `hash3`, `value_noise3`, `fbm3`) upstream into `mixed-signals` so water, fire, and future consumers all import the same code. Expose `GlyphEncoder` and `sample_eight_subcells` in `tui-vfx-types` so the encoding vocabulary is shared. Land `ScalarFieldGlyphFilter<S: Signal>` in `tui-vfx-compositor` as the unifying filter. Wrap the existing `TerminalWaterShader` math in a `WaterFieldSignal` so the framework's first real consumer ships behind one debug recipe. Coordinate fire's plan in lockstep so fire's first implementation commit consumes upstream from day one.

## 0.5. Read order if you are implementing

Read top-to-bottom in the order below. Do not start Phase 1 until you have done all of this.

1. `/usr/projects/tui-vfx/CLAUDE.md` — project orientation, load-bearing don'ts, OFPF prefixes.
2. `/usr/projects/tui-vfx/steering/INTENTIONS.md` — the durable rules. Pay attention to **Intention 9** (mixed-signals boundary), **Intention 23** (rule of three), **Intention 24** (additive changes earn their place), **Intention 26** (single source of truth), **Intention 34** (pipeline-touch family obligations), **Intention 37** (loopback required for bindings).
3. `/usr/projects/mixed-signals/steering/INTENTIONS.md` — substrate-side rules for what does and does not earn upstream placement.
4. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md` — recipe-side rules that govern Phase 6.
5. This plan, end to end. Note section numbering.
6. `/usr/projects/tui-vfx/docs/design/tui-vfx-terminal-water-shader-plan.md` §13 and §21.1 — the original water context. The math being lifted lives in `crates/tui-vfx-style/src/models/cls_terminal_water_shader.rs`.
7. `/usr/projects/tui-vfx/docs/design/tui-vfx-terminal-fire-shader-plan.md` §9.0, §9.1, §9.10 — the parallel fire consumer. These three sections will be rewritten during Phase 1 of this plan.
8. `/usr/projects/mixed-signals/src/traits/signal.rs` — read in full. The plan rests on the `Signal` trait, `SignalContext::cell_x/cell_y/width/height/absolute_t`, `SignalRange`, and `SignalTime`.
9. `/usr/projects/tui-vfx/crates/tui-vfx-compositor/src/traits/filter.rs` — the `Filter` trait shape your new filter implements.
10. `/usr/projects/tui-vfx/crates/tui-vfx-types/src/braille.rs` — read in full. You will not duplicate any of these constants.
11. `/usr/projects/tui-vfx/crates/tui-vfx-compositor/src/filters/cls_subcell_light.rs` — the canonical "before" filter you are generalizing.

If you find yourself unsure which file owns a behavior, run `ofpf-orientation --root /usr/projects/tui-vfx` and `ofpf-orientation --root /usr/projects/mixed-signals` before continuing.

## 1. The three-consumer story (rule-of-three justification)

Per Intention 23 (rule of three) and Intention 9 (mixed-signals boundary rule for renderer-agnostic substrate with three or more callers), every upstream lift in this plan must name three concrete callers.

| Symbol | Caller 1 | Caller 2 | Caller 3 |
|---|---|---|---|
| `smoothstep(edge0, edge1, x)` | water shader (currently private at `crates/tui-vfx-style/src/models/cls_terminal_water_shader.rs:888`) | fire shader (planned, `cls_terminal_fire_shader.rs` §9.0) | noise/perlin (currently private `smoothstep(t)` at `mixed-signals/src/noise/cls_perlin.rs:114`) |
| `lerp(a, b, t)` | noise/perlin (currently private at `mixed-signals/src/noise/cls_perlin.rs:119`) | fire shader (planned §9.0) | future audio/animation linear interpolation (already implicit; the public surface formalizes it) |
| `fade(t)` | fire shader value-noise (planned §9.1) | future Perlin gradient noise (commonly paired) | future terrain/cloud noise |
| `saturate(x)` | fire shader (planned §9.0) | future filter math | future signal-graph clamps; pairs with `smoothstep` |
| `finite_or(value, fallback)` (already exists, currently `pub(crate)`) | water shader (call sites at lines 283-294, 472, 518, 538-540, 593, 596, 599, etc.) | fire shader (planned §9.0) | already used inside mixed-signals' own perlin and other modules |
| `hash01(seed)` | water shader rain emitters (`cls_terminal_water_shader.rs:895` and call sites 538-540) | fire shader (planned §9.1) | future deterministic-randomness consumers (debug visualizers) |
| `hash3(seed, x, y, z)` | fire shader (planned §9.1, the foundation for value_noise3) | future Perlin gradient noise | future terrain/cloud noise |
| `value_noise3(seed, x, y, z)` | fire shader (planned §9.1, the central primitive) | future terrain/cloud renderers | future debug field visualizers |
| `fbm3(seed, x, y, z, octaves, gain, lacunarity)` | fire shader (planned §9.1) | future cloud/terrain | future hot-spot heatmap |

Encoder layer (lifted into `tui-vfx-types`, not `mixed-signals`, because braille is terminal-rendering-specific):

| Symbol | Caller 1 | Caller 2 | Caller 3 |
|---|---|---|---|
| `GlyphEncoder` enum | `SubcellLight` (existing — refactored to call into encoders) | `ScalarFieldGlyphFilter<WaterFieldSignal>` (Phase 4 + Phase 5) | future fire glyph filter |
| `sample_eight_subcells` | `ScalarFieldGlyphFilter` (Phase 4) | future fire glyph filter | future cell-color-intensity wrapper if any consumer ever needs eight subcell color samples |

The fire shader plan today expects these helpers to be either inlined or upstreamed once water and fire both need them; see fire plan §9.0 ("Move them to `utils` only after water/fire both need them") and §9.1 ("Add a small dependency-free value noise helper... or keep a private helper until both fire and water need it"). Both conditions are now scheduled to be met. This plan exists so fire begins implementation against upstream primitives instead of shipping a third copy and migrating later.

## 2. Three-layer architecture

### Layer A — Field samplers as `Signal` implementations (no new trait)

Decision: re-use `mixed_signals::traits::Signal::sample_with_context` directly. **Do not invent a `ScalarField2d` wrapper trait.** The existing `SignalContext` already carries `cell_x`, `cell_y`, `width`, `height`, and `absolute_t` (see `mixed-signals/src/traits/signal.rs:48-84`). That is exactly the per-sample input shape a 2D scalar field needs. `SpatialCoordinateSignal` (`mixed-signals/src/generators/cls_spatial_coordinate.rs`) is the precedent — same context, same pattern.

Rejected alternative: a new `ScalarField2d` trait with a typed `(cell_x, cell_y, t)` method. Rejected because (a) it duplicates a contract already covered, (b) it forks the substrate surface, (c) Intention 24 — a wrapper that does not reduce call-site work or improve readability versus `Signal` does not earn its place. We add concrete impls, not a new trait.

What we add are concrete `Signal` impls per effect, living next to the effect's math:

- `WaterFieldSignal` in `tui-vfx-style` (Phase 5; wraps the existing `TerminalWaterShader`'s field math).
- `FireFieldSignal` in `tui-vfx-style` (added when the fire shader lands; not in this plan's scope).
- `CellColorIntensitySignal` in `tui-vfx-compositor` (Phase 4; wraps the existing `SubcellLight::sample_color` + `project_intensity` path so the legacy "sample current cell color" use-case is expressible inside the new framework).

Each impl declares its `output_range()` so downstream code can reason about whether values are unit (`0..1`) or bipolar (`-1..1`).

### Layer B — Subcell sampling helper

A small free function, co-located in a new `tui-vfx-types/src/glyph/` module sibling to the existing `braille.rs` primitives:

```rust
// crates/tui-vfx-types/src/glyph/fnc_sample_eight_subcells.rs
use mixed_signals::traits::{Signal, SignalContext, SignalTime};

/// Sample any `Signal` at the eight subcell positions inside one terminal cell.
///
/// Subcell layout matches the braille dot map already exposed by
/// `tui_vfx_types::braille`:
///   index 0 = dot 1 (col 0, row 0)
///   index 1 = dot 2 (col 0, row 1)
///   index 2 = dot 3 (col 0, row 2)
///   index 3 = dot 4 (col 1, row 0)
///   index 4 = dot 5 (col 1, row 1)
///   index 5 = dot 6 (col 1, row 2)
///   index 6 = dot 7 (col 0, row 3)
///   index 7 = dot 8 (col 1, row 3)
///
/// Per-subcell offsets are applied in-cell as fractional `(dx, dy)` where
/// `dx ∈ {0.25, 0.75}` (half-column centers) and
/// `dy ∈ {0.125, 0.375, 0.625, 0.875}` (quarter-row centers).
///
/// The caller provides `(cell_x, cell_y, width, height)` via `ctx`; this
/// helper synthesizes a per-subcell context using fractional position offsets
/// in cell-local coordinates by re-using `SignalContext::with_cell_position`
/// with the integer `(cell_x, cell_y)` and forwarding fractional offsets via
/// `SignalContext::with_subcell_offset` (see Phase 1 §4.4 below for the
/// `SignalContext` extension this requires).
pub fn sample_eight_subcells<S: Signal + ?Sized>(
    signal: &S,
    ctx: &SignalContext,
    t: SignalTime,
) -> [f32; 8];
```

Implementation note (you write this in Phase 3): for samplers that don't read `subcell_offset`, the result is eight identical samples — that's correct behaviour for a coarse sampler. The slope-derivative shortcut (§8) is the production path for high-frequency samplers like water.

### Layer B (slope shortcut) — opt-in trait extension

Decision (resolves §13 Q-B): the slope shortcut is a **separate trait** `SignalWithSlope: Signal`, not a default-implemented method on `Signal`. We do not extend the universal `Signal` trait for one renderer's optimization. Adding methods to `Signal` would force every downstream consumer in mixed-signals' growing ecosystem to think about slopes; a separate trait keeps the substrate clean.

```rust
// mixed-signals/src/traits/cls_signal_with_slope.rs (Phase 1)
use crate::traits::{Signal, SignalContext, SignalTime};

/// Optional companion trait for samplers that can return analytic slopes
/// `(value, ∂x, ∂y)` cheaply. Renderers that step through subcells benefit
/// from this when one center sample plus two derivatives is cheaper than
/// eight full samples (the water shader's case).
///
/// Default implementation falls back to numeric differencing using
/// `Signal::sample_with_context` at `(cell_x ± h, cell_y)` etc.
pub trait SignalWithSlope: Signal {
    /// Sample at `(cell_x, cell_y)` plus return ∂value/∂cell_x and
    /// ∂value/∂cell_y in cell-local units.
    fn sample_with_slope(&self, t: SignalTime, ctx: &SignalContext) -> SlopeSample {
        // Default: numeric differencing. Concrete impls override for cheap.
        let h: f32 = 1.0;
        let cx = ctx.cell_x.unwrap_or(0);
        let cy = ctx.cell_y.unwrap_or(0);
        let value = self.sample_with_context(t, ctx);
        let mut left = ctx.clone();
        left.cell_x = Some(cx.saturating_sub(1));
        let mut right = ctx.clone();
        right.cell_x = Some(cx.saturating_add(1));
        let mut up = ctx.clone();
        up.cell_y = Some(cy.saturating_sub(1));
        let mut down = ctx.clone();
        down.cell_y = Some(cy.saturating_add(1));
        let dx = (self.sample_with_context(t, &right) - self.sample_with_context(t, &left)) / (2.0 * h);
        let dy = (self.sample_with_context(t, &down) - self.sample_with_context(t, &up)) / (2.0 * h);
        SlopeSample { value, dx, dy }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlopeSample {
    pub value: f32,
    pub dx: f32,
    pub dy: f32,
}
```

`WaterFieldSignal` overrides this in Phase 5 to return cached `slope_x`/`slope_y` from `WaterFieldSample` directly — eight subcell scalars from one full evaluation plus eight cheap MAC ops.

The Phase 3 helper companion `sample_eight_subcells_with_slope` lives next to `sample_eight_subcells`:

```rust
// crates/tui-vfx-types/src/glyph/fnc_sample_eight_subcells_with_slope.rs
use mixed_signals::traits::{SignalContext, SignalTime, SignalWithSlope};

pub fn sample_eight_subcells_with_slope<S: SignalWithSlope + ?Sized>(
    signal: &S,
    ctx: &SignalContext,
    t: SignalTime,
) -> [f32; 8];
```

### Layer C — Glyph encoders

A closed enum in `tui-vfx-types/src/glyph/cls_glyph_encoder.rs`. Decision (resolves §13 Q-C): **enum, not trait.** Reasons: (1) the encoder set is bounded and ours to define; we don't expect third-party consumers to register custom encoders against the wire-format surface, (2) an enum is `Copy` (or near-Copy with one `Cow`) and friendly to `ScalarFieldGlyphFilter<S>`'s hot loop, (3) trait dispatch in the per-cell loop is what we are explicitly avoiding (§8). If a consumer needs a custom encoder later, that is a recipe-vocabulary discussion (§13 Q-R follow-up), not a trait-object proliferation.

```rust
// crates/tui-vfx-types/src/glyph/cls_glyph_encoder.rs
use std::borrow::Cow;

/// Glyph encoder vocabulary shared by every scalar-field renderer.
///
/// `encode_one(intensity, x, y, t)` is the single-scalar path used by
/// `BrailleEighths`, `BlockHorizontal`, `BlockVertical`, and `Ramp`.
///
/// `encode_subcell(subcells, x, y, t)` is the eight-scalar path used by
/// `BrailleSubcell`. Encoders that don't accept eight scalars fall back
/// to averaging into a single intensity so callers can swap encoders
/// without branching on the variant.
#[derive(Debug, Clone, PartialEq)]
pub enum GlyphEncoder {
    /// 8 subcell scalars → 256-pattern braille (true subcell positions).
    /// `threshold` is per-subcell: each dot lights independently when its
    /// scalar exceeds the threshold (decision §13 Q-D).
    BrailleSubcell { threshold: f32 },

    /// 1 scalar → eighths dot count, optionally rotated for spatial hashing.
    /// Replaces the legacy `SubcellLight::rotated_braille_pattern`.
    BrailleEighths { rotated: bool },

    /// 1 scalar → ▏▎▍▌▋▊▉█.
    BlockHorizontal,

    /// 1 scalar → ▁▂▃▄▅▆▇█.
    BlockVertical,

    /// 1 scalar → custom char ramp; `' '` at intensity 0, ramp's last
    /// char at intensity 1.
    Ramp(Cow<'static, [char]>),
}

impl GlyphEncoder {
    /// Encode one scalar in `0.0..=1.0` to a glyph at `(x, y, t)`.
    pub fn encode_one(&self, intensity: f32, x: u16, y: u16, t: f64) -> char { /* impl in Phase 3 */ }

    /// Encode eight subcell scalars in `0.0..=1.0` to a glyph at `(x, y, t)`.
    /// For non-subcell encoders, averages and falls back to `encode_one`.
    pub fn encode_subcell(&self, subcells: [f32; 8], x: u16, y: u16, t: f64) -> char { /* impl in Phase 3 */ }
}
```

Ownership of legacy helpers is explicit:

- `SubcellLight::rotated_braille_pattern` (current `crates/tui-vfx-compositor/src/filters/cls_subcell_light.rs:125`) → delegated to `GlyphEncoder::BrailleEighths { rotated: true }::encode_one`.
- `SubcellLight::horizontal_partial` (current line 97) → delegated to `GlyphEncoder::BlockHorizontal::encode_one`.
- `SubcellLight::vertical_partial` (current line 111) → delegated to `GlyphEncoder::BlockVertical::encode_one`.
- `tui-vfx-types::braille::braille()`, `from_dots()`, `braille_bits()` are unchanged; `BrailleSubcell` builds on top of them.
- `AnimatedGlyphRamp::glyph_index` is **not** migrated — it is a phase/timing-driven index lookup, not a scalar quantization, and does not fit this framework. It stays where it is (`cls_animated_glyph_ramp.rs`).

### Per-subcell threshold — worked example

Decision (resolves §13 Q-D): per-subcell threshold. Each dot lights when its own subcell scalar exceeds the threshold. Cell-wide thresholds were rejected because they collapse the 8-scalar input back into one decision, making the subcell encoder no better than `BrailleEighths`.

Worked example. Given subcells `[0.1, 0.5, 0.7, 0.2, 0.0, 0.9, 0.3, 0.1]` and `threshold = 0.4`:

- Per-subcell interpretation (chosen): dots that exceed threshold are at indices `[1, 2, 5]`. The bit pattern is `BRAILLE_DOTS[1] | BRAILLE_DOTS[2] | BRAILLE_DOTS[5]` = `0x02 | 0x04 | 0x10` = `0x16`. The character is `braille(0x16)` = `'⠖'`.
- Cell-wide interpretation (rejected): mean = `(0.1+0.5+0.7+0.2+0.0+0.9+0.3+0.1)/8 = 0.35` < `0.4`, so the cell stays blank — losing all the subcell information.

## 3. The unifying filter

```rust
// crates/tui-vfx-compositor/src/filters/cls_scalar_field_glyph_filter.rs
use crate::traits::filter::Filter;
use mixed_signals::traits::{Signal, SignalContext};
use tui_vfx_types::glyph::GlyphEncoder;
use tui_vfx_types::{Cell, Color};

/// Generic scalar-field-to-glyph filter.
///
/// `S` is the field sampler (any `Signal`); `recolor` paints lit/unlit
/// cells from the encoder's output, or preserves the producer's color
/// when `None` (the typical water/fire case where the upstream shader
/// already painted the cell).
pub struct ScalarFieldGlyphFilter<S: Signal> {
    pub sampler: S,
    pub encoder: GlyphEncoder,
    /// `Some((lit, unlit))` paints cell.fg/cell.bg keyed by intensity;
    /// `None` preserves whatever the producer step already wrote.
    pub recolor: Option<(Color, Color)>,
    /// Single-scalar gate when the encoder is `BrailleEighths`,
    /// `BlockHorizontal`, `BlockVertical`, or `Ramp`. For
    /// `BrailleSubcell`, see `GlyphEncoder::BrailleSubcell { threshold }`.
    pub threshold: f32,
    /// Skip non-blank cells when `true` (mirrors `SubcellLight::only_blank`).
    pub only_blank: bool,
    /// Extra per-frame jitter for `BrailleEighths { rotated: true }`.
    pub temporal_dither_hz: f32,
    /// Frame and seed propagated into the sampler's context.
    pub frame: u64,
    pub seed: u64,
}

impl<S: Signal> Filter for ScalarFieldGlyphFilter<S> {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, w: u16, h: u16, t: f64) {
        if self.only_blank && cell.ch != ' ' {
            return;
        }
        let ctx = SignalContext::new(self.frame, self.seed)
            .with_dimensions(w, h)
            .with_cell_position(x, y)
            .with_absolute_time(t);
        let new_ch = match &self.encoder {
            GlyphEncoder::BrailleSubcell { .. } => {
                let subcells = tui_vfx_types::glyph::sample_eight_subcells(&self.sampler, &ctx, t);
                self.encoder.encode_subcell(subcells, x, y, t)
            }
            _ => {
                let v = self.sampler.sample_with_context(t, &ctx);
                if v <= self.threshold {
                    return;
                }
                self.encoder.encode_one(v, x, y, t)
            }
        };
        cell.ch = new_ch;
        if let Some((lit, unlit)) = self.recolor {
            cell.fg = lit;
            cell.bg = unlit;
        }
    }
}
```

`SubcellLight` becomes a thin shim: it constructs a `ScalarFieldGlyphFilter<CellColorIntensitySignal>` internally and forwards (Phase 4). The `SubcellLight` public API does not change in this plan. Decision (resolves §13 Q-E): we keep the shim for now and schedule a follow-up plan **trigger condition** — when at least one debug recipe migrates from `SubcellLight` to `ScalarFieldGlyphFilter` voluntarily in tui-vfx-recipes, file a follow-up plan to retire the shim. Until then, additive only.

## 4. Mixed-signals lift — what moves and why

### 4.1 Math primitives → `mixed_signals::math` (public)

Three-or-more-callers test for each: see §1. Renderer-agnostic numeric utility, no rendering semantic, fits with existing `math/` neighbours (`fnc_distance.rs`, `fnc_harmonic.rs`, `fnc_spatial_warps.rs`, etc.).

| Symbol | New file | Existing duplicate(s) being replaced |
|---|---|---|
| `pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32` | `mixed-signals/src/math/fnc_smoothstep.rs` | water shader private at `cls_terminal_water_shader.rs:888-894`; perlin noise private `smoothstep(t)` at `cls_perlin.rs:114-116` |
| `pub fn fade(t: f32) -> f32` | `mixed-signals/src/math/fnc_fade.rs` | none today; lifts in for fire's planned use |
| `pub fn lerp(a: f32, b: f32, t: f32) -> f32` | `mixed-signals/src/math/fnc_lerp.rs` | perlin noise private at `cls_perlin.rs:119-121` |
| `pub fn saturate(x: f32) -> f32` | `mixed-signals/src/math/fnc_saturate.rs` | none today; lifts in for fire's planned use; pairs with `smoothstep` |
| `pub fn finite_or(value: f32, fallback: f32) -> f32` | edits `mixed-signals/src/math/fnc_sanitize.rs` | promote existing `pub(crate)` to `pub`; water shader's private duplicate (`cls_terminal_water_shader.rs:863`) is removed |
| `pub fn finite_or_clamp(value, min, max, fallback)` | same file | promote existing `pub(crate)` to `pub`; water shader's `clamp_finite` is removed in favor of this |

`finite_or_f64` and `finite_or_min` stay `pub(crate)` for now (no external caller demands them; promote on demand per Intention 24).

OFPF prefix: `fnc_` per repo convention. Each gets its own file with metadata header and a peer `test_fnc_*.rs` (TDD red→green per Intention 14). Each function is `< 30 LOC` so the soft `fnc_` limit (75 LOC) is not threatened.

### 4.2 Noise primitives → `mixed_signals::noise` (public)

Three-or-more-callers test for each: see §1. The existing `noise/` module already hosts `PerlinNoise` and `WhiteNoise`; value-noise primitives are deliberately **lower-level free functions**, not new `Signal` impls, because the consumers (fire, water rain) call them inside their own per-cell math rather than wiring a `Signal` graph. A future `ValueNoise3Signal` `Signal` impl can be added later if a recipe ever wants raw value noise as a parameter source; that is deferred until a real consumer asks (Intention 24).

| Symbol | New file |
|---|---|
| `pub fn hash01(x: u32) -> f32` | `mixed-signals/src/noise/fnc_hash01.rs` |
| `pub fn hash3(seed: u32, x: i32, y: i32, z: i32) -> f32` | `mixed-signals/src/noise/fnc_hash3.rs` |
| `pub fn value_noise3(seed: u32, x: f32, y: f32, z: f32) -> f32` | `mixed-signals/src/noise/fnc_value_noise3.rs` |
| `pub fn fbm3(seed: u32, x: f32, y: f32, z: f32, octaves: u8, gain: f32, lacunarity: f32) -> f32` | `mixed-signals/src/noise/fnc_fbm3.rs` |

### 4.3 What does NOT move upstream

Stays in `tui-vfx`:

- `WaterField` math (Gerstner waves, Fresnel, specular, foam, glint) — renderer-semantic, water-specific composition of primitives. Justification: the *primitives* (sin/cos, smoothstep, normalize) are generic; the *recipe* of how they assemble into "water" is rendering policy. Per Intention 9, the policy stays high.
- Fire flame-shape, temperature, sparks (planned) — same reasoning.
- `GlyphEncoder` and `sample_eight_subcells` — terminal-rendering specific. Mixed-signals does not know about braille codepoints or block characters.
- `SubcellLight`'s color-axis-projection (`project_intensity` at `cls_subcell_light.rs:67-95`) — consumes `Color` from `tui-vfx-types`; not generic.

### 4.4 Public-API change to `SignalContext` — `subcell_offset`

The subcell sampler needs to communicate fractional position offsets inside the cell to the underlying `Signal`. The simplest forward-compatible extension is a new optional field on `SignalContext`:

```rust
// Add to mixed-signals/src/traits/signal.rs in Phase 1.
// SignalContext gains:
pub subcell_offset: Option<(f32, f32)>,  // (dx, dy) in cell-local 0..1

// Plus a builder:
impl SignalContext {
    pub fn with_subcell_offset(mut self, dx: f32, dy: f32) -> Self {
        self.subcell_offset = Some((dx.clamp(0.0, 1.0), dy.clamp(0.0, 1.0)));
        self
    }
}
```

Default for samplers that don't read it: ignored. `SignalContext::default()` is unaffected. The framework lift is **additive within the in-flight v0.3.0 release** — no separate version bump (see §4.5).

### 4.5 Cargo dependency wiring and version policy

**Verified state at draft time** (`/usr/projects/mixed-signals/Cargo.toml:9`, `/usr/projects/tui-vfx/Cargo.toml:51`, `/usr/projects/mixed-signals/CHANGELOG.md`):

- Public on crates.io: `mixed-signals` **v0.2.2** (released 2026-02-24).
- Cargo.toml in-tree: `version = "0.3.0"` — the **in-flight, unreleased** version that `[Unreleased]` accumulates into.
- `[Unreleased]` block already contains additions from prior work (route-relative helpers, swirl/attractor primitives, spatial-warp helpers).
- `[0.2.3]` block (dated 2026-04-23) documents the spatial-coordinate `SignalContext` work that introduced `cell_x`/`cell_y`/`with_cell_position(...)` — those are present in-tree on the way to 0.3.0.
- `tui-vfx` workspace pin: `mixed-signals = { path = "../mixed-signals", version = "0.3.0" }` — path-first with a version contract-check.

**Version policy for this lift: no crate-level version bump.** The framework primitives fold into the existing `[Unreleased]` block, releasing as part of 0.3.0 alongside the route/warp/swirl additions already accumulating. Rationale:

- The crate is pre-1.0; the user has explicitly directed *do not run away versions* during early public release.
- Cargo.toml is already at `0.3.0`; the open `[Unreleased]` section is the canonical home for additive changes targeting that release.
- A separate 0.4.0 here would compress the public-release cadence beyond what the project wants and create the same version-runaway pattern the user is correcting elsewhere.

**No diff to `mixed-signals/Cargo.toml`.** Cargo.toml stays at `0.3.0`.

**No diff to `/usr/projects/tui-vfx/Cargo.toml`.** The workspace pin stays at `0.3.0`. Path dep means the local checkout is canonical; the version pin remains a contract-check.

**CHANGELOG edit (the only release-coordination change in Phase 1).** Append to the existing `## [Unreleased]` block in `/usr/projects/mixed-signals/CHANGELOG.md`. Skeleton:

```markdown
## [Unreleased]

- Add route-relative `carrier_orbit_position` and `figure_eight_position` helpers ...    (existing entries — DO NOT TOUCH)

- Add substrate-named spatial warp helpers: ...                                         (existing entries — DO NOT TOUCH)

### Added
- `mixed_signals::math::endpoint_bell` ...                                              (existing entry — DO NOT TOUCH)
- `mixed_signals::math::swirl_offset` and `swirl_position` ...                          (existing entry — DO NOT TOUCH)
- `mixed_signals::math::attractor_pull_factor` and `attract_position` ...               (existing entry — DO NOT TOUCH)

  --- NEW ENTRIES BELOW (Phase 1 of glyph rendering framework lift) ---
- `mixed_signals::math::smoothstep`, `fade`, `lerp`, `saturate` — public scalar helpers
  promoted from private duplicates (`cls_perlin.rs`) and downstream copy-paste targets
  (`tui-vfx-style/src/models/cls_terminal_water_shader.rs`,
  fire shader plan §9.0). Three-or-more callers test met by water + fire + perlin.
- `mixed_signals::noise::hash01`, `hash3`, `value_noise3`, `fbm3` — public coherent
  3D value-noise primitives. Three-or-more callers test met by water (rain drop hash),
  fire (noise foundation per fire plan §9.1), and future field-based effects
  (terrain/audio/signal-graph visualizers).
- `mixed_signals::traits::SignalWithSlope` trait + `SlopeSample` struct — opt-in
  extension for samplers with cheap analytic gradients; default impl falls back
  to numeric differencing of `Signal::sample_with_context`.
- `SignalContext::with_subcell_offset(dx, dy)` builder — supports per-subcell
  position threading for the `tui-vfx-types` subcell sampling helper. The
  underlying field is `pub(crate)`; the builder is the public surface.

### Why
- `tui-vfx`'s glyph rendering framework needs shared scalar/noise math and a
  per-subcell sampling path; the math is renderer-agnostic substrate per Intention 9
  of `/usr/projects/mixed-signals/steering/INTENTIONS.md`.
- Keeping the substrate in `mixed-signals` lets terminal-water, terminal-fire, and
  future field-based effects share one implementation rather than three copies.
```

The exact entry text is the implementer's call; the skeleton is the structural pattern. Do not create a new `[0.x.y]` heading; do not move `[Unreleased]`.

**Why `pub(crate)` for `SignalContext::subcell_offset` instead of `pub`.** Even within a single in-flight version, keeping the field non-public preserves the option to refactor it (e.g., into a small `SubcellSample` struct if a future consumer needs more than `(dx, dy)`) without forcing every downstream `SignalContext { ... }` literal-construction site to update. The builder-only path is the lower-blast-radius choice. Audit by `rg "SignalContext\s*\{" /usr/projects/tui-vfx/crates /usr/projects/mixed-signals/src` to confirm no in-tree caller uses struct-literal construction without `..Default::default()` — if any do, leave them alone (the field is private; they can't see it anyway), but note the audit ran clean.

**Cross-repo lift order (no temporary mismatch):**

1. Land all of Phase 1 in `/usr/projects/mixed-signals`. **No `Cargo.toml` version edit.** Append to `[Unreleased]` in `CHANGELOG.md`. Run mixed-signals tests in isolation. Commit.
2. tui-vfx's workspace pin (`version = "0.3.0"`) already matches; no edit needed when Phase 2 begins. Path dep means the local checkout is canonical; the version pin is a safety check that already passes.
3. Phase 2 onward all build against the same in-tree `0.3.0` plus the new symbols.

When `mixed-signals` eventually publishes 0.3.0 to crates.io (a separate release event the leader schedules independently of this framework), the workflow is identical — `cargo publish` from `/usr/projects/mixed-signals` after `[Unreleased]` is renamed to `[0.3.0] - <date>`. The framework lift's primitives are part of that release; no further coordination required.

## 5. Fire-parallel coordination

Fire's plan today copies primitive math into `cls_terminal_fire_shader.rs`. With this framework, fire's first commit consumes upstream from day one. Concrete edits to fire's plan during Phase 1 (these are **edits to fire's plan document**, not to fire's source — fire has not been implemented yet):

- **Fire plan §9.0 (Copy-paste scalar helpers).** Replace the inline `saturate`, `finite_or`, `smoothstep`, `fade`, `lerp` block with:

  ```rust
  use mixed_signals::math::{saturate, finite_or, smoothstep, fade, lerp};
  ```

  Plus a one-paragraph note: "These primitives are public in the in-flight mixed-signals `[Unreleased]` block (releases as v0.3.0); see `tui-vfx-glyph-rendering-framework-plan.md` §4.1 for the lift rationale."

- **Fire plan §9.1 (Noise foundation).** Replace the inline `hash01`, `hash3`, `value_noise3`, `fbm3` block with:

  ```rust
  use mixed_signals::noise::{hash01, hash3, value_noise3, fbm3};
  ```

  Plus the same one-paragraph note pointing to §4.2.

- **Fire plan §9.10 (Assembled `sample_field_at` skeleton).** Wrap the existing skeleton in a `FireFieldSignal` struct and `Signal` impl whose `sample_with_context` reads `cell_x`/`cell_y`/`width`/`height`/`absolute_t` from `SignalContext`. Add a §9.11 note: "If you implement the slope shortcut, also implement `SignalWithSlope` returning analytic gradients of the temperature/density field." Fire still owns the math; only the entry shape changes.

These three edits are part of Phase 1's deliverables (§7.1 deliverable list) so fire's plan and the mixed-signals lift land in the same commit boundary. CLOG-bump fire's plan; new content is small. Fire's other 21 sections do not move.

Fire-discovered candidates (primitives that fire reveals water did not need) get tracked in §12 of this plan as they appear in fire's implementation.

## 6. Inventory of duplication today (with file:line citations)

Every entry below is a delete-or-route target. Verify each with `rg` before removing.

| Symbol | Today | Action after plan |
|---|---|---|
| `clamp_finite` (private fn) | `crates/tui-vfx-style/src/models/cls_terminal_water_shader.rs:866-868` | Removed; call sites use `mixed_signals::math::finite_or_clamp` |
| `finite_or` (private fn) | `cls_terminal_water_shader.rs:863-865` | Removed; call sites use `mixed_signals::math::finite_or` |
| `smoothstep` (private fn) | `cls_terminal_water_shader.rs:888-894` | Removed; call sites use `mixed_signals::math::smoothstep` |
| `hash01` (private fn) | `cls_terminal_water_shader.rs:895-902` | Removed; call sites use `mixed_signals::noise::hash01` |
| `smoothstep(t)` (private fn) | `mixed-signals/src/noise/cls_perlin.rs:114-116` | Removed; in-crate import `use crate::math::smoothstep;` (note the new public `smoothstep` is `(edge0, edge1, x)`-shaped — perlin's shape is `(t)`-only, equivalent to `smoothstep(0.0, 1.0, t)`. Call site adapts to the public shape.) |
| `lerp(a, b, t)` (private fn) | `mixed-signals/src/noise/cls_perlin.rs:119-121` | Removed; in-crate import `use crate::math::lerp;` (perlin's is `f64`; the public `lerp` is `f32`; either keep an `f64`-flavored private helper for perlin or add `pub fn lerp_f64`. Decision: add `pub fn lerp_f64` to `fnc_lerp.rs` rather than retain a private duplicate — same file, same earned-its-place test, free with the existing call site.) |
| `BRAILLE_BASE`, `BRAILLE_DOTS` (private constants) | `crates/tui-vfx-compositor/src/filters/cls_subcell_light.rs:42-43` | Removed; encoders consume `tui_vfx_types::braille::braille()` and `from_dots` directly |
| `rotated_braille_pattern`, `horizontal_partial`, `vertical_partial` (private methods) | `cls_subcell_light.rs:97-143` | Replaced by `GlyphEncoder` calls (§3) |
| `BRAILLE_BASE` (private constant), `BRAILLE_DOTS` (private constant), `BRAILLE_RIGHT_DOTS` (private constant) | `crates/tui-vfx-shadow/src/renderers/cls_braille.rs:22-34` | Audit (Phase 4): `BRAILLE_BASE = 0x2800` and `BRAILLE_DOTS = [0x01,0x02,0x04,0x40,0x08,0x10,0x20,0x80]` are byte-identical to the constants implicit in `tui_vfx_types::braille`. `BRAILLE_RIGHT_DOTS = [0x08,0x10,0x20,0x80]` is the right column dots — equivalent to filtering `tui_vfx_types::braille::RIGHT_COLUMN`. Action: replace the three private constants with `tui_vfx_types::braille` imports; if any subtle layout difference emerges in test, document it as a load-bearing variation rather than silently routing. |
| `BRAILLE_LEFT_COL = "⡇"`, `BRAILLE_RIGHT_COL = "⣸"` | `crates/tui-vfx-content/src/transformers/fnc_morph_chars.rs:106-109` | Audit (Phase 4): these are *string literals of pre-encoded characters*, not bit constants. They equal `braille(LEFT_COLUMN as u8 | something)` — verify and either replace with `braille(0x47)` / `braille(0xB8)` or keep as-is with a doc-comment cross-referencing `tui_vfx_types::braille::LEFT_COLUMN`/`RIGHT_COLUMN`. Pure-data duplication is cheap; leave with a comment unless it costs nothing to route. |

## 7. Phase ordering

Each phase lands as one or more commits, ends with tests passing, and ends with a phase-end audit per Intention 15. Subsequent phases do not begin until the previous one is green.

Phase ordering revisit (resolves §13 Q-H): mixed-signals lift → water migration → encoders → compositor framework → WaterFieldSignal → recipe → docs. Could encoders (Phase 3) land before water migration (Phase 2)? Yes technically, but no: water migration is the proof that the upstream `smoothstep`/`finite_or` are byte-equivalent to the local copies, and we want that proof landed before adding any new framework code on top. Phases 3 and 4 cannot reasonably swap — Phase 4 imports Phase 3's `GlyphEncoder`. Phases 5 and 6 cannot reasonably swap — Phase 6's recipe imports Phase 5's `WaterFieldSignal`. The ordering is therefore optimal as drafted.

### Phase 1 — Mixed-signals lift

**Crate root:** `/usr/projects/mixed-signals`

**Files added:**

- `src/math/fnc_smoothstep.rs` + `src/math/test_fnc_smoothstep.rs`
- `src/math/fnc_fade.rs` + `src/math/test_fnc_fade.rs`
- `src/math/fnc_lerp.rs` + `src/math/test_fnc_lerp.rs` (contains both `pub fn lerp(a: f32, b: f32, t: f32) -> f32` and `pub fn lerp_f64(a: f64, b: f64, t: f64) -> f64`)
- `src/math/fnc_saturate.rs` + `src/math/test_fnc_saturate.rs`
- `src/noise/fnc_hash01.rs` + `src/noise/test_fnc_hash01.rs`
- `src/noise/fnc_hash3.rs` + `src/noise/test_fnc_hash3.rs`
- `src/noise/fnc_value_noise3.rs` + `src/noise/test_fnc_value_noise3.rs`
- `src/noise/fnc_fbm3.rs` + `src/noise/test_fnc_fbm3.rs`
- `src/traits/cls_signal_with_slope.rs` + `src/traits/test_cls_signal_with_slope.rs`

**Files edited:**

- `src/math/mod.rs` — add eight new module declarations and `pub use` lines for the new public symbols. Keep `pub(crate) use fnc_sanitize::{…}` in place; add `pub use fnc_sanitize::{finite_or, finite_or_clamp};` (the two we promote).
- `src/math/fnc_sanitize.rs` — change `pub(crate) fn finite_or(...)` to `pub fn finite_or(...)` and same for `finite_or_clamp`. Update file's `<VERS>` to `1.1.0` and `<CLOG>` to a one-line note. Other helpers (`finite_or_f64`, `finite_or_min`) stay `pub(crate)`.
- `src/noise/mod.rs` — add four new module declarations and `pub use` lines.
- `src/noise/cls_perlin.rs` — replace private `smoothstep` and `lerp` with `use crate::math::{smoothstep, lerp_f64};` and adapt the call sites (perlin uses `f64`; `lerp` is `f32`; use `lerp_f64`. Perlin's `smoothstep(t)` becomes `smoothstep(0.0, 1.0, t as f32) as f64` — verify against the test golden values; if precision matters, add `smoothstep_f64` rather than fudging the cast). Bump `<VERS>` to `2.1.0`, update `<CLOG>`.
- `src/traits/mod.rs` — add `pub mod cls_signal_with_slope;` and `pub use cls_signal_with_slope::{SignalWithSlope, SlopeSample};`.
- `src/traits/signal.rs` — add `pub(crate) subcell_offset: Option<(f32, f32)>` field to `SignalContext` and the `with_subcell_offset` builder method. Update `<VERS>` to `2.2.0`, update `<CLOG>`.
- `Cargo.toml` — **no edit**. Version stays at `0.3.0` (the in-flight unreleased version). See §4.5.
- `CHANGELOG.md` — append entries to the existing `## [Unreleased]` block alongside the route/warp/swirl/attractor entries already there. Do not create a new `[0.x.y]` heading. See §4.5 for the entry skeleton.

**Audit gate.**

```bash
cd /usr/projects/mixed-signals
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

All three must exit 0. Mechanical pass/fail.

**Expected new public symbols (rg-verifiable):**

```bash
cd /usr/projects/mixed-signals
rg "^pub fn (smoothstep|fade|lerp|lerp_f64|saturate|finite_or|finite_or_clamp|hash01|hash3|value_noise3|fbm3)\b" src/
# Expect 11 hits.
rg "^pub trait SignalWithSlope\b" src/traits/
# Expect 1 hit.
```

**Companion-doc edits required during Phase 1 (write scope: fire plan).** Edit `/usr/projects/tui-vfx/docs/design/tui-vfx-terminal-fire-shader-plan.md`:

- §9.0 lines 682-719: replace the entire scalar-helpers block with the import line and the one-paragraph upstream-note (see §5 of *this* plan).
- §9.1 lines 721-814: replace the inline noise-foundation impl with the import line and a note pointing to mixed-signals `[Unreleased]` block (releases as v0.3.0). Keep the "good defaults" and "fire-upward time" guidance — those are renderer-semantic, not lift candidates.
- §9.10 lines 1049-…: wrap in `FireFieldSignal: Signal` and `impl SignalWithSlope for FireFieldSignal { ... }` once gradients are decided. Update the section to reflect that fire registers as a `Signal` impl.
- Bump fire plan `<VERS>` to `0.3.0`. Update `<CLOG>` to one line.

### Phase 2 — Water shader migration to upstream

**Crate root:** `/usr/projects/tui-vfx`

**File:** `crates/tui-vfx-style/src/models/cls_terminal_water_shader.rs`

**Edits:**

Add at top:

```rust
use mixed_signals::math::{finite_or, finite_or_clamp, smoothstep};
use mixed_signals::noise::hash01;
```

Remove the four private fns at lines 863-902 (`finite_or`, `clamp_finite`, `smoothstep`, `hash01`).

For every `clamp_finite(value, fallback, min, max)` call site (lines 283-294, 472, 518, 596, 633-637, 646-647, 732, 833-843, 866 — verify with `rg "clamp_finite" crates/tui-vfx-style/`), replace with `finite_or_clamp(value, min, max, fallback)` — note the **argument order changes**. The mixed-signals helper is `(value, min, max, fallback)`; the local helper was `(value, fallback, min, max)`. Get this right or the tests will tell you. Example:

```diff
-let amplitude = clamp_finite(self.amplitude, DEFAULT_AMPLITUDE, 0.0, 2.0);
+let amplitude = finite_or_clamp(self.amplitude, 0.0, 2.0, DEFAULT_AMPLITUDE);
```

For every `finite_or` call site (lines 285-286, 593, 598, 645, 648, 653, 733-734, 827, 863): no argument-order change; just the import.

For `smoothstep` call sites (lines 407, 736 twice): no signature change; just the import.

For `hash01` call sites (lines 538-540): no signature change; just the import.

Bump file `<VERS>` to `0.2.0`, update `<CLOG>`:

```diff
-// <VERS>VERSION: 0.1.0</VERS>
-// <WCTX>Add motion-field primitive for layered water lighting with foam, ripples, rain, flow, and wakes.</WCTX>
-// <CLOG>Initial terminal water shader model with deterministic field sampling and style application.</CLOG>
+// <VERS>VERSION: 0.2.0</VERS>
+// <WCTX>Migrate water shader's private math/noise helpers to mixed-signals upstream primitives (in-flight 0.3.0).</WCTX>
+// <CLOG>0.2.0: replace private smoothstep/finite_or/clamp_finite/hash01 with mixed_signals::math + mixed_signals::noise imports.</CLOG>
```

Match the same edit at the end-of-file footer line.

**Tests.** No new tests in this phase. The migration is behaviour-preserving: existing water shader tests in the same file (`mod tests`) must stay green. If any test breaks, the upstream primitive does not match the local one — fix the upstream primitive in mixed-signals (return to Phase 1), do not paper over with a local wrapper.

**Audit gate.**

```bash
cd /usr/projects/tui-vfx
cargo fmt --all -- --check
cargo clippy -p tui-vfx-style --all-targets -- -D warnings
cargo test -p tui-vfx-style
cargo xtask docs generate
git diff --stat crates/tui-vfx-style/src/models/cls_terminal_water_shader.rs
# Expect: deletions in helper functions; additions only in import block.
```

### Phase 3 — Glyph encoders + subcell helper

**Crate root:** `/usr/projects/tui-vfx`

**Files added:**

- `crates/tui-vfx-types/src/glyph/mod.rs` — module root, `pub use` re-exports.
- `crates/tui-vfx-types/src/glyph/cls_glyph_encoder.rs` — the `GlyphEncoder` enum and impls (target ≤ 180 LOC, comfortably inside the `cls_` 200-LOC hard limit).
- `crates/tui-vfx-types/src/glyph/test_cls_glyph_encoder.rs`
- `crates/tui-vfx-types/src/glyph/fnc_sample_eight_subcells.rs` (target ≤ 50 LOC)
- `crates/tui-vfx-types/src/glyph/test_fnc_sample_eight_subcells.rs`
- `crates/tui-vfx-types/src/glyph/fnc_sample_eight_subcells_with_slope.rs` (target ≤ 50 LOC)
- `crates/tui-vfx-types/src/glyph/test_fnc_sample_eight_subcells_with_slope.rs`

**Files edited:**

- `crates/tui-vfx-types/src/lib.rs` — add `pub mod glyph;` near the other `pub mod` declarations (line 73 region). Add `pub use glyph::{GlyphEncoder, sample_eight_subcells, sample_eight_subcells_with_slope};` near line 92. Bump `<VERS>` to `0.5.0` and update `<CLOG>` (one line).
- `crates/tui-vfx-types/Cargo.toml` — add `mixed-signals.workspace = true` to `[dependencies]` (the helper consumes `Signal`/`SignalContext` from mixed-signals). Bump file `<VERS>`.

**Tests (named, with what-this-proves).**

- `test_braille_subcell_per_dot_threshold_lights_independently` — given subcells `[0.1,0.5,0.7,0.2,0.0,0.9,0.3,0.1]` and `threshold = 0.4`, encoder emits `'⠖'` (= `braille(0x16)`). Proves the per-subcell threshold semantics from §2.
- `test_braille_subcell_all_zeros_emits_blank_braille` — `[0.0; 8]` → `braille(0x00) = '⠀'`. Proves the empty case.
- `test_braille_subcell_all_above_threshold_emits_full` — `[1.0; 8]` and `threshold = 0.0` → `braille(0xFF) = '⣿'`.
- `test_braille_eighths_matches_legacy_subcell_light` — for each `intensity` in `0.125, 0.25, 0.375, …, 1.0` and fixed `(x, y, t) = (3, 2, 0.0)`, `GlyphEncoder::BrailleEighths { rotated: true }::encode_one(intensity, 3, 2, 0.0)` matches `SubcellLight::rotated_braille_pattern((intensity*8.0).round() as usize, 3, 2, 0.0)`. Proves migration parity with the existing filter.
- `test_block_horizontal_table` — for each of nine intensity buckets, returns `' ▏▎▍▌▋▊▉█'`. Proves byte-equivalence with the legacy `horizontal_partial` table.
- `test_block_vertical_table` — same for `' ▁▂▃▄▅▆▇█'`.
- `test_ramp_encoder_picks_last_at_intensity_one` — for ramp `['.', ':', '*', '#']` and `intensity = 1.0`, returns `'#'`.
- `test_ramp_encoder_returns_space_below_threshold` — for `intensity = 0.0`, returns `' '`.
- `test_encode_subcell_falls_back_to_average_for_non_subcell_encoder` — `BlockHorizontal::encode_subcell([0.5; 8], 0, 0, 0.0)` equals `BlockHorizontal::encode_one(0.5, 0, 0, 0.0)`. Proves the fallback rule.
- `test_sample_eight_subcells_returns_eight_in_dot_order` — for a synthetic `Signal` whose `sample_with_context` returns `subcell_offset.0 + subcell_offset.1 * 0.1`, the eight returned scalars are in the dot-index order documented in §2 Layer B (i.e., index 0 = `(0.25, 0.125)`, index 7 = `(0.75, 0.875)`).
- `test_sample_eight_subcells_with_slope_default_implementation_numerically_differences` — for a `Signal` impl that *also* implements default-only `SignalWithSlope`, the slope returned matches `(sample(x+1) - sample(x-1)) / 2.0` to within `1e-3`.

**Audit gate.**

```bash
cd /usr/projects/tui-vfx
cargo fmt --all -- --check
cargo clippy -p tui-vfx-types --all-targets -- -D warnings
cargo test -p tui-vfx-types
```

### Phase 4 — Compositor framework

**Crate root:** `/usr/projects/tui-vfx`

**Files added:**

- `crates/tui-vfx-compositor/src/filters/cls_scalar_field_glyph_filter.rs` — the generic filter (target ≤ 130 LOC). Code skeleton in §3.
- `crates/tui-vfx-compositor/src/filters/test_cls_scalar_field_glyph_filter.rs`
- `crates/tui-vfx-compositor/src/filters/cls_cell_color_intensity_signal.rs` — `Signal` wrapper whose `sample_with_context` implements the legacy `SubcellLight::sample_color → project_intensity` path (target ≤ 90 LOC).
- `crates/tui-vfx-compositor/src/filters/test_cls_cell_color_intensity_signal.rs`

**Files edited:**

- `crates/tui-vfx-compositor/src/filters/mod.rs` — add `pub mod cls_scalar_field_glyph_filter;` and `pub mod cls_cell_color_intensity_signal;` plus `pub use` lines.
- `crates/tui-vfx-compositor/src/filters/cls_subcell_light.rs` — refactor to delete the three private helper methods (`rotated_braille_pattern`, `horizontal_partial`, `vertical_partial`) and the two private constants (`BRAILLE_BASE`, `BRAILLE_DOTS`). The `apply` method now constructs a `GlyphEncoder` per `render_mode` and calls `encode_one`. The public API (`SubcellLight` struct, its fields, its `Filter` impl) stays identical. Bump file `<VERS>` to `0.2.0`, update `<CLOG>`. Existing tests in `mod tests` must stay green.

  Migration before/after for the apply method:

  ```diff
   impl Filter for SubcellLight {
       fn apply(&self, cell: &mut Cell, x: u16, y: u16, _width: u16, _height: u16, t: f64) {
           if self.only_blank && cell.ch != ' ' { return; }
           let sampled = self.sample_color(cell);
           if sampled.a == 0 { return; }
           let intensity = self.project_intensity(sampled);
           if intensity <= self.threshold { return; }
  -        let eighths = ((intensity * 8.0).round().clamp(0.0, 8.0) as u8).max(1);
  -        cell.ch = match self.render_mode {
  -            SubcellLightRenderMode::Braille => self.rotated_braille_pattern(eighths as usize, x, y, t),
  -            SubcellLightRenderMode::Horizontal => Self::horizontal_partial(eighths),
  -            SubcellLightRenderMode::Vertical => Self::vertical_partial(eighths),
  -        };
  +        let encoder = match self.render_mode {
  +            SubcellLightRenderMode::Braille =>
  +                tui_vfx_types::glyph::GlyphEncoder::BrailleEighths { rotated: true },
  +            SubcellLightRenderMode::Horizontal =>
  +                tui_vfx_types::glyph::GlyphEncoder::BlockHorizontal,
  +            SubcellLightRenderMode::Vertical =>
  +                tui_vfx_types::glyph::GlyphEncoder::BlockVertical,
  +        };
  +        cell.ch = encoder.encode_one(intensity, x, y, t);
           cell.fg = self.lit_color;
           cell.bg = self.unlit_color;
       }
   }
  ```

- `crates/tui-vfx-compositor/src/types/cls_filter_spec.rs` — **no edit in Phase 4.** The `ScalarFieldGlyphFilter` filter-spec variant lands in Phase 6 with the first recipe that uses it (per "no parse-and-inert schema fields" rule and Intention 37 loopback discipline). This phase only ships the Rust filter.
- `crates/tui-vfx-shadow/src/renderers/cls_braille.rs` — audit per §6 inventory. Replace the three private constants (`BRAILLE_BASE`, `BRAILLE_DOTS`, `BRAILLE_RIGHT_DOTS`) with `tui_vfx_types::braille` imports. Keep behaviour identical; tests stay green.
- `crates/tui-vfx-content/src/transformers/fnc_morph_chars.rs` — audit per §6. Add a doc-comment cross-reference to `tui_vfx_types::braille::LEFT_COLUMN`/`RIGHT_COLUMN`. The string literals stay (they're pre-encoded chars).

**Tests (named, with what-this-proves).**

- `test_subcell_light_regression_all_three_render_modes_unchanged` — for each of `Braille`, `Horizontal`, `Vertical`, the existing `SubcellLight` test fixtures from `cls_subcell_light.rs:175-273` produce byte-identical output. Proves the refactor is invisible to existing recipes.
- `test_scalar_field_glyph_filter_blank_cell_with_synthetic_signal` — for a `Signal` whose `sample_with_context` returns `(cell_x as f32) / (width as f32)`, applying `ScalarFieldGlyphFilter` with `BlockHorizontal` to a blank cell at `(x=4, y=0, w=8)` produces `'▌'` (intensity 0.5). Proves end-to-end signal-driven encoding.
- `test_scalar_field_glyph_filter_only_blank_skips_text` — same filter applied to a cell with `ch = 'X'` and `only_blank = true` leaves the cell untouched.
- `test_scalar_field_glyph_filter_braille_subcell_uses_eight_samples` — for a `Signal` that varies in y, `BrailleSubcell { threshold: 0.5 }` produces a glyph with `dot_count > 0` when the cell straddles the threshold.
- `test_cell_color_intensity_signal_matches_legacy_subcell_light_projection` — `CellColorIntensitySignal::sample_with_context` returns the same value as `SubcellLight::project_intensity` for a representative grid of color triples. Proves the wrapper preserves semantics.

**Audit gate.**

```bash
cd /usr/projects/tui-vfx
cargo fmt --all -- --check
cargo clippy -p tui-vfx-compositor --all-targets -- -D warnings
cargo clippy -p tui-vfx-shadow --all-targets -- -D warnings
cargo clippy -p tui-vfx-content --all-targets -- -D warnings
cargo test -p tui-vfx-compositor -p tui-vfx-shadow -p tui-vfx-content
```

### Phase 5 — `WaterFieldSignal`

**Crate root:** `/usr/projects/tui-vfx`

**Files added:**

- `crates/tui-vfx-style/src/models/cls_water_field_signal.rs` — wrapper struct holding `&TerminalWaterShader` (or `Arc<TerminalWaterShader>` if cloning is required for filter ownership; decide during implementation based on Phase 4's filter signature). Implements `Signal` and `SignalWithSlope`. Target ≤ 100 LOC.
- `crates/tui-vfx-style/src/models/test_cls_water_field_signal.rs`

**Files edited:**

- `crates/tui-vfx-style/src/models/cls_terminal_water_shader.rs` — promote `WaterFieldSample` and `sample_field_at` from `pub(crate)` to `pub` so the wrapper can call them. Decision: only promote what the wrapper actually needs (`sample_field_at` returning `WaterFieldSample`; `WaterFieldSample` itself becomes `pub` because it's the return type, but its fields stay `pub(crate)` until a probe/trace consumer asks). Bump file `<VERS>` to `0.3.0`, update `<CLOG>`.
- `crates/tui-vfx-style/src/models/mod.rs` — add `pub mod cls_water_field_signal;` and `pub use cls_water_field_signal::WaterFieldSignal;`.

**Code skeleton:**

```rust
// crates/tui-vfx-style/src/models/cls_water_field_signal.rs
use crate::models::TerminalWaterShader;
use mixed_signals::traits::{Signal, SignalContext, SignalRange, SignalTime, SignalWithSlope, SlopeSample};

pub struct WaterFieldSignal<'a> {
    pub shader: &'a TerminalWaterShader,
}

impl<'a> Signal for WaterFieldSignal<'a> {
    fn output_range(&self) -> SignalRange { SignalRange::UNIT }

    fn sample(&self, _t: SignalTime) -> f32 { 0.0 }

    fn sample_with_context(&self, t: SignalTime, ctx: &SignalContext) -> f32 {
        let cx = ctx.cell_x.unwrap_or(0) as f32;
        let cy = ctx.cell_y.unwrap_or(0) as f32 * 2.0;
        // Subcell offset (Phase 1 §4.4) folds into x/y in cell-local space.
        let (dx, dy) = ctx.subcell_offset.unwrap_or((0.5, 0.5));
        let x = cx + dx;
        let y = cy + dy * 2.0;
        self.shader.sample_field_at(x, y, ctx.width, ctx.height, t as f32).light_scalar
    }
}

impl<'a> SignalWithSlope for WaterFieldSignal<'a> {
    fn sample_with_slope(&self, t: SignalTime, ctx: &SignalContext) -> SlopeSample {
        // Use the shader's already-cached slope_x/slope_y from sample_field_at
        // by exposing them in WaterFieldSample (Phase 5 promotion). If
        // sample_field_at currently consumes its slopes internally without
        // returning them, expose them on WaterFieldSample as part of this
        // phase's promotion (pub fields slope_x / slope_y).
        let cx = ctx.cell_x.unwrap_or(0) as f32;
        let cy = ctx.cell_y.unwrap_or(0) as f32 * 2.0;
        let sample = self.shader.sample_field_at(cx, cy, ctx.width, ctx.height, t as f32);
        SlopeSample {
            value: sample.light_scalar,
            dx: sample.slope_x,  // requires Phase 5 promotion
            dy: sample.slope_y,  // requires Phase 5 promotion
        }
    }
}
```

If `slope_x`/`slope_y` are not currently fields of `WaterFieldSample` (verify against `cls_terminal_water_shader.rs:262-274` — at draft time they are computed as locals inside `sample_field_at` and used to compute `light_scalar` but not retained), Phase 5 adds them to the struct. That is a `pub` API addition; bump shader file MINOR.

**Tests (named, with what-this-proves).**

- `test_water_field_signal_sample_with_context_matches_in_shader_path` — for a fixed `TerminalWaterShader::default()` and 100 random `(cell_x, cell_y, t)` triples, `WaterFieldSignal::sample_with_context` returns the same value to within `1e-5` as constructing the same `ShaderContext` and calling `sample_field_at` directly.
- `test_water_field_signal_output_range_is_unit` — `WaterFieldSignal::output_range() == SignalRange::UNIT`.
- `test_water_field_signal_subcell_offset_changes_output_continuously` — for `(dx, dy) ∈ {(0.0, 0.0), (0.5, 0.5), (1.0, 1.0)}`, the three returned values differ but vary smoothly (no jumps > 0.5).
- `test_water_field_signal_with_slope_returns_cached_gradients` — `SlopeSample::dx` and `SlopeSample::dy` equal `WaterFieldSample::slope_x` and `slope_y` exactly.

**Audit gate.**

```bash
cd /usr/projects/tui-vfx
cargo fmt --all -- --check
cargo clippy -p tui-vfx-style --all-targets -- -D warnings
cargo test -p tui-vfx-style
```

### Phase 6 — Water glyph debug recipe

**Crate root:** `/usr/projects/tui-vfx-recipes`

**File added:**

- `recipes/debug_recipes/shaders/primitives/shader_terminal_water_glyph_v3.json` — exercises `ScalarFieldGlyphFilter<WaterFieldSignal>` over an Ocean-mode water shader.

**Filter-spec wiring (lands in Phase 6, not earlier).**

- `crates/tui-vfx-compositor/src/types/cls_filter_spec.rs` — add a new variant `ScalarFieldGlyph { encoder: GlyphEncoderSpec, threshold: BindableValue, only_blank: bool, temporal_dither_hz: BindableValue, recolor: Option<(ColorConfig, ColorConfig)>, sampler: SamplerRef }`. Decisions: `encoder` is a small enum mirroring `GlyphEncoder` but in spec-shape (Cow's `Ramp` becomes `Vec<char>` for serde); `sampler` is a typed reference whose first variant is `SamplerRef::TerminalWater` (the only sampler this phase wires up). Subsequent samplers extend the enum. `BindableValue` keeps `threshold` and `temporal_dither_hz` runtime-bindable per Intention 37 with **explicit loopbacks** in `requires_bindings` if the recipe consumes them. The Phase 6 recipe declares no bindings (it is a static fixture).
- The corresponding V3 normalization, lowering, and schema-emission code paths live in adjacent files inside `crates/tui-vfx-compositor/src/types/` and are touched in the same commit. Run `cargo xtask docs generate` to verify the schema/manifest pick the new variant up.

Code skeleton for the spec variant:

```rust
// crates/tui-vfx-compositor/src/types/cls_filter_spec.rs (excerpt)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum GlyphEncoderSpec {
    BrailleSubcell { threshold: f32 },
    BrailleEighths { rotated: bool },
    BlockHorizontal,
    BlockVertical,
    Ramp { chars: Vec<char> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SamplerRef {
    /// References a TerminalWaterShader produced earlier in the pipeline.
    TerminalWater,
}

// In FilterSpec enum (adjacent variants):
ScalarFieldGlyph {
    sampler: SamplerRef,
    encoder: GlyphEncoderSpec,
    #[serde(default)]
    threshold: BindableValue,
    #[serde(default = "default_only_blank")]
    only_blank: bool,
    #[serde(default)]
    temporal_dither_hz: BindableValue,
    #[serde(default)]
    recolor: Option<(ColorConfig, ColorConfig)>,
},
```

**Recipe (full JSON, no stubs):**

```json
{
  "schema_version": 3,
  "id": "debug.shader.primitive.terminal_water_glyph.v3",
  "title": "Debug: Primitive - TerminalWater + Glyph",
  "description": "TerminalWater Ocean shader plus a ScalarFieldGlyphFilter using BrailleSubcell encoding. Expect a layered blue ocean field where each cell's eight subcell positions are sampled from the water field and rendered as 256-pattern braille on top of the shader's lit color. The glyphs reveal subcell wave structure that the shader's color alone cannot show.",
  "version": "3.0.0",
  "last_updated": "2026-04-26",
  "metadata": {
    "aesthetic_tags": ["terminal_water", "water", "ocean", "glyph", "braille_subcell", "scalar_field_glyph", "shader", "filter", "procedural_field"],
    "mood": "calm",
    "related_themes": ["theme-neutral"],
    "use_cases": ["debug_preview", "primitive_reference", "scalar_field_glyph_reference", "subcell_braille_reference"],
    "maturity_era": "experimental",
    "authoring_notes": "First consumer of ScalarFieldGlyphFilter. Pipeline: shader (terminal_water Ocean) then filter (scalar_field_glyph with BrailleSubcell encoder, threshold 0.45, recolor None so the shader's water color shows through). The filter samples WaterFieldSignal at eight subcell positions per cell.",
    "last_reviewed": "2026-04-26"
  },
  "config": {
    "message": "Terminal Water + Glyph\nOcean field, braille subcell",
    "layout": { "width": 48, "height": 14, "anchor": "center" },
    "lifecycle": { "auto_dismiss_ms": 7500, "loop": true },
    "clock": { "loop": true, "period_ms": 2800 },
    "border": { "type": "rounded", "trim": "none" },
    "base_style": {
      "foreground": { "type": "rgb", "r": 220, "g": 244, "b": 255 },
      "background": { "type": "rgb", "r": 3, "g": 18, "b": 40 }
    },
    "pipeline": {
      "steps": [
        {
          "kind": "shader",
          "scope": { "kind": "all" },
          "payload": {
            "type": "terminal_water",
            "mode": { "mode": "ocean" },
            "layers": 3,
            "amplitude": 0.36,
            "wavelength": 12.0,
            "speed": 1.0,
            "direction_deg": 25.0,
            "steepness": 0.48,
            "normal_strength": 1.45,
            "diffuse": 0.68,
            "specular": 0.62,
            "shininess": 26.0,
            "fresnel": 0.38,
            "foam": 0.58,
            "deep_color": { "type": "rgb", "r": 5, "g": 32, "b": 64 },
            "shallow_color": { "type": "rgb", "r": 40, "g": 170, "b": 210 },
            "foam_color": { "type": "white" },
            "glint_strength": 0.28,
            "glint_angle_deg": -18.0,
            "glint_width": 8.0,
            "glint_speed": 1.0,
            "apply_to": "both"
          }
        },
        {
          "kind": "filter",
          "scope": { "kind": "all" },
          "payload": {
            "type": "scalar_field_glyph",
            "sampler": { "kind": "terminal_water" },
            "encoder": { "type": "braille_subcell", "threshold": 0.45 },
            "threshold": 0.0,
            "only_blank": false,
            "temporal_dither_hz": 0.0,
            "recolor": null
          }
        }
      ]
    },
    "motion": {
      "enter": { "duration_ms": 600, "easing": "quad_out", "route": { "type": "linear" }, "dynamics": [] },
      "exit": { "duration_ms": 500, "easing": "quad_in", "route": { "type": "linear" }, "dynamics": [] }
    }
  }
}
```

**Loopback / bindings (Intention 37).** This recipe declares no `requires_bindings` block. Every authoring parameter is a static literal. Therefore strict-contracts validation has nothing to enforce. The recipe is preview-playable by construction. Future recipes that bind `threshold` or `temporal_dither_hz` must declare loopbacks per Intention 37 — that is a property of the consumer recipe, not of this filter's schema.

**Audit gate.**

```bash
cd /usr/projects/tui-vfx-recipes
just validate
# Or, more granular:
cargo run -p pipeline-validator -- recipes/debug_recipes/shaders/primitives/shader_terminal_water_glyph_v3.json
cargo run -p pipeline-validator -- --rules --strict-contracts recipes/debug_recipes/shaders/primitives/shader_terminal_water_glyph_v3.json
cargo run -p pipeline-validator -- --debug-recipes-qc recipes/debug_recipes/shaders/primitives/shader_terminal_water_glyph_v3.json
```

(Replace the precise validator binary path with whatever the repo's `just validate` target invokes — the validator is canonical.) Plus visual QA per Intention 31 rule 11: render the recipe in the demo player, confirm the description text reads true.

### Phase 7 — Documentation closure

**Files edited:**

- `/usr/projects/tui-vfx/docs/design/tui-vfx-terminal-water-shader-plan.md` — §13 ("256-braille follow-up design") becomes "Implemented; see `tui-vfx-glyph-rendering-framework-plan.md` and `shader_terminal_water_glyph_v3.json`." §21.1 ("Glyph-capable 256-braille water") becomes "Implemented." Bump `<VERS>` and update `<CLOG>` (one line).
- `/usr/projects/tui-vfx/docs/design/tui-vfx-terminal-fire-shader-plan.md` — already edited in Phase 1 (§5 of this plan); confirm in this phase that the edits are present and the `<VERS>` was bumped.
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-INDEX.md` (or whichever INDEX file lists framework-level design docs — verify with `ls /usr/projects/tui-vfx/docs/design/*INDEX* 2>/dev/null`) — add link to this plan if it tracks framework-level design docs.
- Generated docs: regenerate via `cargo xtask docs generate` and commit any drift in `docs/api/` or equivalent.

**Audit gate.**

```bash
cd /usr/projects/tui-vfx
cargo xtask docs generate
git diff --stat docs/
# Expect only intentional doc deltas (water schema unchanged, fire schema unchanged, new filter-spec docs).
```

## 8. Performance design

### 8.1 Quantified concern

`ScalarFieldGlyphFilter<S>::apply` runs per cell per frame. A representative deployment:

- Grid: **80 columns × 24 rows = 1920 cells**.
- Frame rate: **60 fps**.
- Sampling: when encoder is `BrailleSubcell`, eight subcell evaluations per cell.

Without the slope shortcut: `1920 × 8 × 60 = 921,600 sampler calls/sec`. For a `WaterFieldSignal` whose `sample_field_at` does roughly 20 sin/cos per layer × 3 layers = 60 trig ops, that's **55M trig ops/sec** in the steady state. On a modern CPU, sin/cos is ~10 ns, so 55M × 10 ns = 550 ms/sec. That's ~33% of one core spent in water sampling alone. Unacceptable.

### 8.2 Mitigations, in priority order

1. **Slope-derivative shortcut (the production path for water/fire-style samplers).** `SignalWithSlope::sample_with_slope` returns `(value, ∂x, ∂y)` from a single full evaluation. `sample_eight_subcells_with_slope` then applies `value + ∂x*dx + ∂y*dy` per subcell — eight cheap MACs. Cost: `1920 × 1 × 60 = 115,200` full sampler calls/sec ≈ 6.9M trig ops/sec ≈ **70 ms/sec ≈ 4% of one core**. That's the budget.

2. **Encoder selection.** `BrailleEighths` and the block encoders take a single intensity, not eight scalars. For samplers without slope support and full-frame deployment, recommend `BrailleEighths { rotated: true }` as the production default; recommend `BrailleSubcell` for hero surfaces or static screenshots.

3. **No allocations in `apply`.** Every code path in `cls_scalar_field_glyph_filter.rs` and `cls_glyph_encoder.rs` must avoid `Vec`, `String`, `Box`, etc. The `Cow<'static, [char]>` in `Ramp` is acceptable because it borrows in the static case. Verify with `rg "Vec::|String::|Box::|alloc::" crates/tui-vfx-compositor/src/filters/cls_scalar_field_glyph_filter.rs crates/tui-vfx-types/src/glyph/` after Phase 4 lands.

4. **Monomorphization where possible.** `ScalarFieldGlyphFilter<S: Signal>` is generic; the filter's call sites typically know `S` concretely (`ScalarFieldGlyphFilter<WaterFieldSignal>`). Avoid `Box<dyn Signal>` in the hot loop. The recipe layer (Phase 6) constructs concrete instances at load time.

### 8.3 Bench plan

Existing bench at `/usr/projects/tui-vfx/crates/tui-vfx-debug/benches/bench_full_trace_60fps.rs` (verified extant). Phase 4 extends it with one new `criterion` group:

```rust
// In bench_full_trace_60fps.rs (Phase 4 addition):
fn bench_scalar_field_glyph_filter_water_braille_subcell(c: &mut Criterion) {
    let shader = TerminalWaterShader::default();
    let signal = WaterFieldSignal { shader: &shader };
    let filter = ScalarFieldGlyphFilter {
        sampler: signal,
        encoder: GlyphEncoder::BrailleSubcell { threshold: 0.45 },
        recolor: None,
        threshold: 0.0,
        only_blank: false,
        temporal_dither_hz: 0.0,
        frame: 0,
        seed: 0,
    };
    let mut grid = OwnedGrid::new(80, 24);  // 80x24 representative
    c.bench_function("scalar_field_glyph_filter_water_braille_subcell_80x24", |b| {
        b.iter(|| {
            for y in 0..24u16 {
                for x in 0..80u16 {
                    let mut cell = Cell::default();
                    filter.apply(&mut cell, x, y, 80, 24, 0.5);
                }
            }
        })
    });
}
```

**Frame budget assertion.** Target: ≤ **4 ms per 80×24 frame** (single-threaded). At 60 fps that leaves 12.7 ms for the rest of the pipeline. Criterion does not assert latency directly; the gate is "the bench reports a mean below 4_000_000 ns per iteration." Document this as the watermark in the bench file's doc-comment and check it manually during Phase 4 audit. Add an automatic assertion in Phase 7 if Intention 25 (mechanical validation) earns its place.

## 9. Loopback and binding contracts

Per Intention 37, every recipe field that accepts a runtime binding must declare a loopback in `requires_bindings`. The new `ScalarFieldGlyph` filter-spec has these candidate bindable fields:

- `threshold` (single-scalar gate) — bindable; loopback default `0.0` if a future recipe binds it.
- `temporal_dither_hz` — bindable; loopback default `0.0`.
- `encoder.braille_subcell.threshold` — bindable; loopback default `0.4`.
- `recolor.lit` and `recolor.unlit` — bindable colors (likely v2; not in this plan's scope).

The `WaterFieldSignal`'s underlying water-shader parameters are already bindable through the existing terminal-water schema; this plan does not introduce new bindings on the water-shader path. `BrailleEighths { rotated }` is not bindable in v1 — it's a load-time choice that selects one of two render branches.

The Phase 6 recipe declares **no** `requires_bindings` block (it is a static fixture). Strict-contracts validation has no bindings to enforce; it passes by virtue of having nothing to check. Forward-compat: if a future recipe binds `threshold`, that recipe author must add the binding declaration plus a `loopback` (per Intention 37). The validator enforces this at recipe load time, not at compositor compile time, so the framework is unchanged.

## 10. No-dangling-threads checklist

Every item must be true before the plan is declared done. Each is mechanically verifiable.

- [ ] `mixed-signals` exposes the eleven new public symbols (eight free fns + `SignalWithSlope` + `SlopeSample` + `SignalContext::with_subcell_offset`); `cargo test --all-features` is green; `Cargo.toml` is unchanged at `version = "0.3.0"`; `CHANGELOG.md` `[Unreleased]` block contains the new entries appended after the existing route/warp/swirl/attractor entries.
- [ ] `mixed-signals/src/noise/cls_perlin.rs` no longer defines its private `smoothstep` or `lerp` (`rg "^fn (smoothstep|lerp)\b" /usr/projects/mixed-signals/src/noise/cls_perlin.rs` returns 0 hits).
- [ ] `crates/tui-vfx-style/src/models/cls_terminal_water_shader.rs` no longer defines `clamp_finite`, `finite_or`, `smoothstep`, or `hash01` (`rg "^fn (clamp_finite|finite_or|smoothstep|hash01)\b" crates/tui-vfx-style/src/models/cls_terminal_water_shader.rs` returns 0 hits).
- [ ] Fire shader plan §9.0/§9.1/§9.10 rewritten to import upstream; `<VERS>` of fire plan bumped to `0.3.0`.
- [ ] `tui-vfx-types::glyph::GlyphEncoder` covers the four glyph generators (rotated braille, horizontal block, vertical block, plus the new subcell braille); `Ramp` covers the future char-ramp use case (`rg "GlyphEncoder::(BrailleSubcell|BrailleEighths|BlockHorizontal|BlockVertical|Ramp)" crates/tui-vfx-types/` returns ≥ 5 hits).
- [ ] `SubcellLight` private helpers deleted in favor of `GlyphEncoder` calls; public API of `SubcellLight` unchanged (verify by diff: only `impl SubcellLight` block shrinks; `pub struct SubcellLight` and `impl Filter for SubcellLight` keep the same surface).
- [ ] Shadow audit: `crates/tui-vfx-shadow/src/renderers/cls_braille.rs` no longer defines `BRAILLE_BASE`, `BRAILLE_DOTS`, `BRAILLE_RIGHT_DOTS` as private constants — they are imported from `tui_vfx_types::braille`. (Or, if a load-bearing variation was discovered, a doc-comment explaining it is in the file.)
- [ ] Content audit: `crates/tui-vfx-content/src/transformers/fnc_morph_chars.rs` has a doc-comment cross-reference to `tui_vfx_types::braille::LEFT_COLUMN`/`RIGHT_COLUMN`.
- [ ] `WaterFieldSignal` available; `sample_with_context` returns the same `light_scalar` as the in-shader path (Phase 5 test enforces).
- [ ] `WaterFieldSample` exposes `slope_x` / `slope_y` as `pub` fields (Phase 5 promotion).
- [ ] `ScalarFieldGlyphFilter` available; `SubcellLight` shim is decided (kept or replaced — current plan: keep, retire when first recipe migrates voluntarily; record the trigger condition in §3).
- [ ] Phase 6 recipe passes basic, `--rules --strict-contracts`, and `--debug-recipes-qc` validator flags.
- [ ] Water shader plan §13 / §21.1 marked implemented with pointers; water plan `<VERS>` bumped.
- [ ] Bench `bench_full_trace_60fps` extended with `bench_scalar_field_glyph_filter_water_braille_subcell_80x24`; mean per iteration ≤ 4 ms documented in bench output.
- [ ] Generated docs regenerated (`cargo xtask docs generate`); no unintended drift in `docs/api/` etc.
- [ ] Filter-spec touch (`cls_filter_spec.rs`, normalization, lowering, schema-emission paths) lands in Phase 6 with the recipe; `cargo xtask docs generate` picks up `ScalarFieldGlyph` variant.
- [ ] Fire's first implementation commit consumes upstream from day one (verified in fire's PR review when fire ships; not blocking this plan's completion).
- [ ] Recyclebin protocol: no source files become orphaned by this plan (only modifications and additions). Confirm by `git status` showing no `D ` deletions of whole files; only modifications. If §6's audit identifies a file that becomes empty (e.g. if `cls_subcell_light.rs` shrinks to nothing — it won't, but verify), move to `recyclebin/` per `/usr/projects/global_prompts/standards/90_recycle_bin.md`.

## 11. Test strategy by phase

| Phase | Layer under test | Test approach |
|---|---|---|
| 1 | Math/noise primitives + `SignalWithSlope` default impl + `SignalContext::with_subcell_offset` | Golden-value tests; determinism (same input → same output across runs); NaN/Inf handling at the boundary; `value_noise3` continuity (nearby inputs → close outputs); `fbm3` octave/gain/lacunarity clamping; `SignalContext` builder roundtrip. |
| 2 | Water migration | Existing water shader tests stay green with no parameter changes; the `cargo test -p tui-vfx-style` baseline is the gate. Behaviour is preserved. |
| 3 | Encoders + subcell helper | Per-encoder unit tests for known intensities; subcell helper produces dot-ordered 8-element arrays for a synthetic `Signal`; round-trip with `tui-vfx-types::braille::from_dots`; subcell threshold worked example (§2). |
| 4 | Framework filter + signal wrapper + `SubcellLight` refactor | `SubcellLight` regression suite (existing tests stay green); `ScalarFieldGlyphFilter` over a synthetic `Signal`; `CellColorIntensitySignal` matches old `SubcellLight::project_intensity`. |
| 5 | `WaterFieldSignal` | Field equivalence between in-shader and `Signal` paths; output range; slope-shortcut returns cached gradients; subcell offset varies output continuously. |
| 6 | Recipe | Validator passes basic / `--rules --strict-contracts` / `--debug-recipes-qc` flags; manual visual QA per Intention 31. |
| 7 | Doc and bench | Docs generate without drift; bench under 4 ms target. |

## 12. Fire-discovered candidates (tracking)

Empty at draft time. As fire's implementation surfaces primitives that water didn't reveal, this section grows. Each entry records: symbol, three-or-more-callers test, where it should live, whether it lifted upstream, and the deciding rationale. Kept here so we never lose track of pending consolidation work between water and fire shipping.

## 13. Open questions — resolved

All open questions from the prior draft are decided. Remaining truly-open items are escalated as numbered "leader-sign-off" lines.

1. **`mixed-signals` version bump.** Resolved: **no crate-level bump**. Cargo.toml stays at `0.3.0` (the in-flight, unreleased version). New entries append to the existing `## [Unreleased]` block in `CHANGELOG.md`. The new `subcell_offset` field on `SignalContext` is `pub(crate)` with a public builder, keeping the struct extensible without committing to a literal-construction-stable surface. (See §4.5 for full rationale and the verified state of the public crates.io v0.2.2 release vs. in-tree 0.3.0.)

2. **`ScalarFieldGlyph` filter-spec timing.** Resolved: filter-spec entry lands in **Phase 6** with the first recipe, not Phase 4. Per "no parse-and-inert schema fields" — schema and consumer ship together. (See §7 Phase 6.)

3. **`SubcellLight` shim retirement.** Resolved: keep the shim indefinitely; trigger condition for follow-up plan is "at least one debug recipe migrates from `SubcellLight` to `ScalarFieldGlyphFilter` voluntarily." Until then, additive-only per Intention 24. (See §3.)

4. **`BrailleSubcell { threshold }` semantics.** Resolved: per-subcell threshold (worked example in §2 Layer C). Cell-wide rejected.

5. **Shadow/content braille-constant audit.** Resolved with action: replace shadow's three private constants with `tui_vfx_types::braille` imports during Phase 4; leave content's pre-encoded string literals with a doc-comment cross-reference. Both audits are line items in §10 No-Dangling-Threads.

6. **Wrapper trait vs `Signal` direct.** Resolved: use `Signal` directly; no `ScalarField2d` wrapper. (See §2 Layer A.)

7. **Slope shortcut placement.** Resolved: separate trait `SignalWithSlope: Signal` in `mixed-signals`, not a default-method extension of `Signal`. (See §2 Layer B.)

8. **`GlyphEncoder` enum vs trait.** Resolved: enum, with rationale documented. (See §2 Layer C.)

**Leader sign-off needed before Phase 1 begins** (these are framing decisions, not implementation choices; defaults are recorded so a junior can proceed without a leader meeting if the leader is unavailable):

L1. **Promote `finite_or_f64` and `finite_or_min` to `pub` now or wait?** Default: wait. Promote on demand per Intention 24. If leader prefers eager promotion (one fewer cross-repo handoff later), promote in Phase 1 — additive, no extra version bump.

L2. **Add `pub fn smoothstep_f64(edge0, edge1, x)` to `fnc_smoothstep.rs` for perlin's f64 path?** Default: yes — perlin's existing test golden values use f64 and a cast through f32 risks drift. Adds one extra `pub fn` to the lift list (twelve total instead of eleven).

L3. **Should `SignalContext::subcell_offset` be `pub` from day one?** Default: `pub(crate)` plus `with_subcell_offset` builder, per §4.5 SemVer correctness analysis. Promote to `pub` only when a third caller needs to read it directly without the builder.

## 14. Acceptance criteria

The plan is implemented when every No-Dangling-Threads checklist item (§10) is checked, every phase audit gate (§7) has passed, the bench is within budget (§8), and the framework's first two consumers (water glyph mode shipped, fire about to consume) are visible in the codebase as living evidence.

Mechanical pass/fail for the rule-of-three threshold:

- After Phase 6: `rg "GlyphEncoder::" crates/` returns ≥ 4 distinct call sites (encoder constructors in `SubcellLight` for each of `Braille`/`Horizontal`/`Vertical` plus one in `ScalarFieldGlyphFilter`'s tests at minimum).
- After fire ships: `rg "use mixed_signals::math::(smoothstep|lerp|fade|saturate)" crates/` returns ≥ 2 named consumers (water shader, fire shader). `rg "use mixed_signals::noise::(hash01|hash3|value_noise3|fbm3)" crates/` returns ≥ 2 named consumers.
- `rg "impl Signal for" crates/` returns ≥ 3 distinct impls outside mixed-signals (`WaterFieldSignal`, `CellColorIntensitySignal`, `FireFieldSignal` once fire ships).

The threshold is paid before fire ships by water + cell-color-intensity + planned-fire (the third caller is concrete in the fire shader plan, not speculative). The moment fire ships, every public symbol in this plan has at least three concrete callers in the tree.

## 15. OFPF compliance reference

Every new file follows OFPF prefixes and limits. Soft/hard limits per `~/.claude/rules/ofpf.md`. All files take the metadata header/footer template below.

| Phase | New file | Prefix | Expected LOC | Soft/Hard |
|---|---|---|---|---|
| 1 | `mixed-signals/src/math/fnc_smoothstep.rs` | `fnc_` | ~12 | 75/120 |
| 1 | `mixed-signals/src/math/fnc_fade.rs` | `fnc_` | ~10 | 75/120 |
| 1 | `mixed-signals/src/math/fnc_lerp.rs` | `fnc_` | ~15 | 75/120 |
| 1 | `mixed-signals/src/math/fnc_saturate.rs` | `fnc_` | ~10 | 75/120 |
| 1 | `mixed-signals/src/noise/fnc_hash01.rs` | `fnc_` | ~15 | 75/120 |
| 1 | `mixed-signals/src/noise/fnc_hash3.rs` | `fnc_` | ~20 | 75/120 |
| 1 | `mixed-signals/src/noise/fnc_value_noise3.rs` | `fnc_` | ~50 | 75/120 |
| 1 | `mixed-signals/src/noise/fnc_fbm3.rs` | `fnc_` | ~30 | 75/120 |
| 1 | `mixed-signals/src/traits/cls_signal_with_slope.rs` | `cls_` | ~40 | 150/200 |
| 3 | `crates/tui-vfx-types/src/glyph/cls_glyph_encoder.rs` | `cls_` | ~180 | 150/200 (under hard) |
| 3 | `crates/tui-vfx-types/src/glyph/fnc_sample_eight_subcells.rs` | `fnc_` | ~50 | 75/120 |
| 3 | `crates/tui-vfx-types/src/glyph/fnc_sample_eight_subcells_with_slope.rs` | `fnc_` | ~50 | 75/120 |
| 4 | `crates/tui-vfx-compositor/src/filters/cls_scalar_field_glyph_filter.rs` | `cls_` | ~130 | 150/200 |
| 4 | `crates/tui-vfx-compositor/src/filters/cls_cell_color_intensity_signal.rs` | `cls_` | ~90 | 150/200 |
| 5 | `crates/tui-vfx-style/src/models/cls_water_field_signal.rs` | `cls_` | ~100 | 150/200 |

Test files follow `test_<peer-filename>.rs` naming. Test files have no LOC limit (per OFPF).

### Metadata header/footer template (Rust source)

```rust
// <FILE>relative/path/from/repo-root/file.rs</FILE> - <DESC>One-line file role; static across versions</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Transient one-line context for this work session</WCTX>
// <CLOG>0.1.0: one-line note about THIS version's change. Git holds prior history.</CLOG>

// ... file contents ...

// <FILE>relative/path/from/repo-root/file.rs</FILE> - <DESC>One-line file role; static across versions</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
```

Markdown files use `<!-- ... -->` instead of `//`. TOML uses `# ...`. Per memory note: `<CLOG>` is the latest change only — do not accumulate history.

## 16. Read-the-room don'ts (project-specific guardrails)

Pulled forward from `/usr/projects/tui-vfx/CLAUDE.md` "load-bearing don'ts" so the implementer cannot miss them:

- **Don't import internal crate types from consumer examples.** Use the public surface; if a consumer needs something, expand the surface. (Intention 2.)
- **Don't build signal primitives inside tui-vfx.** Extend `mixed-signals` upstream. This entire plan is the operationalization of that rule. (Intention 9.)
- **Don't break individual-unit addressability.** The Phase 6 recipe is one file (one preview, one demo entry). Do not consolidate it with other water primitives via `template + variants` until/unless tooling support is in place. (Intention 26 / Principle 4.)
- **Don't leak ratatui types into the compositor.** The new filter and signal wrapper consume `tui_vfx_types::Cell` and `mixed_signals::traits::Signal` only. Verify with `rg "ratatui" crates/tui-vfx-compositor/src/filters/cls_scalar_field_glyph_filter.rs` returning 0 hits. (Intention 1.)
- **Don't apply the `Vfx*` prefix to internal types.** `GlyphEncoder`, `WaterFieldSignal`, `CellColorIntensitySignal`, `ScalarFieldGlyphFilter`, `SignalWithSlope`, `SlopeSample` are all internal helpers / engine types — no prefix. The wire-format surface (recipe-spec types like `GlyphEncoderSpec`, `SamplerRef`, the `ScalarFieldGlyph` variant of `FilterSpec`) lives inside existing `Vfx`-free recipe schema modules; no new wire-format types added by this plan. (Intention 8.)
- **Don't accumulate `<CLOG>` history across versions.** One-line summary of the latest change only. Git is the running history. (User memory.)

<!-- <FILE>docs/design/tui-vfx-glyph-rendering-framework-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
