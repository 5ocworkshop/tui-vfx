<!-- <FILE>PRD-FLAG-ANIMATION.md</FILE> - <DESC>PRD for recipe-authored braille image compositions (flag / logo / pixel-art class), powered by a mixed-signals signal-spec schema</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>Honest gap report for mixed-signals: it is 1D/temporal and does not cover 2D spatial+temporal compound waves; PRD now explicitly names the missing capability and offers two resolution paths (upstream Signal2d extension vs. in-tree SpatialSignalSpec)</WCTX> -->
<!-- <CLOG>0.3.0: document the 2D-signal gap in mixed-signals and propose the SpatialSignalSpec path as primary with an optional upstream `Signal2d` trait as secondary; include a detailed "What mixed-signals provides / doesn't provide" matrix.
0.2.0: drop fireworks scope per direction; replace FlagWave proposal with a SignalSampler + SignalSpec JSON schema that exposes mixed-signals composition (Sine/Ramp/Add/Multiply/Normalize/Remap) to recipe authors; update phasing, acceptance, and recipe sketch to match.
0.1.0: initial draft — motivation, six proposed primitives, phased delivery, acceptance criteria.</CLOG> -->

# PRD — Recipe-authored braille image compositions

**Drafted:** 2026-04-21
**Target surfaces:**
- `tui-vfx` (compositor, samplers, shaders, signal-spec schema, image format)
- `tui-vfx-recipes` (scene-layer schema, per-layer pipelines)
- `rocketsplash-formats` (optional new braille-image asset format)

**Related:**
- `/usr/projects/madeira-flag/src/lib.rs` — 783-LOC standalone crate; reference implementation of the class this PRD generalises.
- `/usr/projects/rocketsplash/crates/rocketsplash/src/v2/ui/ui_madeira_overlay.rs` — embedded copy used as the "About" easter egg.
- `/usr/projects/mixed-signals/` — signal library already consumed by `tui-vfx` samplers (`use mixed_signals::prelude::{Normalized, Remap, Signal, SignalExt, Sine}` in `crates/tui-vfx-compositor/src/samplers/cls_sine_wave.rs`).
- `gt-design/docs/superpowers/specs/2026-04-20-recipe-scene-composer-design.md` — Sub-plan B.1 scene schema (already in `tui-vfx-recipes/src/recipe_schema/scene/`).
- `tui-vfx/docs/CAPABILITIES_REFERENCE.md` §Samplers, §Filters, §Spatial Shaders.

---

## One-liner

Unlock **any flag, any logo, any braille-resolution image** as a single recipe JSON by finishing the scene-composer story with per-layer pipelines, a JSON-encoded `mixed_signals` graph for samplers, a displacement-aware shader, and a braille-supersampled image format. The Madeira New-Year's-Eve flag easter egg is the forcing-function example; every primitive pays rent against the broader recipe corpus.

---

## Why now

Four independent trend lines have converged:

1. **Scene layers landed in Sub-plan B.1.** `RaSceneConfig.layers: Vec<RaSceneLayer>` with `RaContentSource = Text | Image | Procedural | Card`. The schema is in place, but layers today are **static content sources with no per-layer animation** — they can be composed spatially but not individually modulated.
2. **`mixed-signals` is already the signal backbone.** The `SineWave` sampler is a thin wrapper around `Remap<Normalized<Sine>>`, and the full library — `Sine / Triangle / Ramp / Keyframes / Add / Multiply / Mix / Normalize / Remap / ADSR / DampedSpring` — is directly accessible inside the compositor crate. Today it's used privately; we propose to expose its compositional power to recipe authors via a JSON `SignalSpec`.
3. **The recipe corpus is maturing** (~630 recipes), but the recipe format cannot carry a **composed scene with differently-animated elements**.
4. **The madeira flag is the clean forcing-function.** It combines a colored sub-cell bitmap, a position-varying compound waveform, and a delayed text reveal — each on independent timing. Anything that recreates the flag portion as a recipe necessarily unlocks a much larger authoring surface.

---

## What "any braille image of anything" actually means

The flag-portion of the Madeira code distills to three reusable moves. Generalised:

| Move in the flag code | Generic capability it proves out |
|---|---|
| Tribands + cross pattée + Greek cross drawn into a 2×4-per-cell canvas | **Sub-cell RGB imagery** — any braille-resolution picture: flags, heraldry, emoji-ish portraits, pixel-art sprites, scannable symbols, product logos |
| Compound sine (two harmonics) with `amplitude = norm_x · 0.15` and waves blended | **Arbitrary position-varying displacement** — any sampler whose displacement is described by a `mixed_signals` graph, not a single scalar |
| Wave-phase shading `clamp(wave·0.25 + 0.75, 0.65, 1.0)` | **Displacement-aware lighting** — the 3D "cloth lit by the sun" look, ripple sheen, heat-haze highlights, reflective surface wobble |

Three moves, four primitives (one of them — per-layer pipelines — is the keystone that makes the others expressible). After the PRD ships, the library moves from "text-plus-effects" to "composed scenes with signal-driven per-layer motion."

---

## Users and what they gain

| Persona | Today | After this PRD |
|---|---|---|
| Recipe author | Writes text-centric toasts and banners; static images are limited to cell-coarse `.rss`; samplers take a single scalar amplitude. | Authors flag / logo / pixel-art recipes with position-varying displacement and displacement-lit shading, driven by an explicit signal-graph JSON. |
| Engine contributor | Samplers hard-code a single signal type; compositional mixed-signals power is only available behind private `SamplerSpec` variants. | Has a single public `SignalSampler` driven by a `SignalSpec`, plus a per-layer pipeline surface that doesn't require new top-level enum variants for each motion style. |
| Consumer app (rocketsplash, gt-design) | Ships bespoke Rust crates per dramatic overlay (`madeira-flag` is one of several). | Swaps bespoke crates for recipe JSON + one engine dep; visual regressions caught by `pipeline-validator --debug-recipes-qc`. |
| QA / CI | Visual sign-off is manual for bespoke overlays. | Probe reports and `--debug-recipes-qc` fingerprints cover the new surface natively. |

---

## Goals

1. **Recreate the flag portion of the madeira-flag crate as a single recipe JSON** that visually matches the Rust implementation to within human-perception tolerance (backdrop + waving flag + staggered text; fireworks are explicitly out of scope).
2. **Each proposed primitive is general-purpose** — its motivation must be defensible on its own, independent of the flag use case.
3. **No regression on the ~630-recipe corpus.** All existing recipes continue to parse, render, and pass `--debug-recipes-qc` unchanged.
4. **Per-layer pipelines are the keystone** — after they land, every subsequent primitive can be added without further schema churn.
5. **`mixed-signals` is promoted from implementation detail to first-class recipe vocabulary** via a serde-friendly `SignalSpec`.
6. **Observable, debuggable by default.** Every new primitive emits a `TraceEvent` so the existing `InspectionSink` / `TraceSink` surface (see `tui-vfx-debug::inspection` and `TRACE_EVENT_SCHEMA.md`) can reason about it.

## Non-goals

- **Fireworks / particle bursts.** Explicitly dropped from scope — the `madeira-flag` crate covers them with ~110 LOC of bespoke particle physics; recipifying them is a separate conversation. The flag alone is the target.
- Replacing the standalone `madeira-flag` crate — it's a zero-dep ratatui widget; the recipe version is a parallel artefact.
- Becoming a 2D graphics editor. Braille-image assets are authored outside the engine.
- Video playback, streaming, third-party image codecs.

---

## The four primitives (plus one ergonomic nicety)

Ordered by dependency: **(1) is the keystone — nothing else is expressible without it.** (2)–(4) are independently shippable behind (1). (5) is a nice-to-have that can slip.

### 1 · Per-layer pipelines on `RaSceneLayer` — the keystone

**What it is.** Extend `RaSceneLayer` with an optional `pipeline: Option<RaLayerPipeline>` plus an optional `schedule: Option<RaLayerSchedule>` (per-layer enter/exit delays). Layers without a `pipeline` field continue to use the recipe-level global pipeline — fully back-compat.

**Why it's the keystone.** The single biggest structural gap today is that `RaPipelineConfig` is global to the recipe. Applying `SineWave` to warp a flag also warps the background behind it and the text below it. Every other primitive in this PRD is dead-on-arrival until layers can own their own distortion, filtering, and styling.

**What it unlocks beyond the flag.**
- **Toast composition.** Author a toast where the border sweeps, the title glitches, and the body fades in — three motions, three layers, one recipe. Today this takes three stacked recipes.
- **Splash screens.** A logo layer with glisten, a background layer with ambient braille dust, a tagline layer with typewriter reveal — all in one recipe.
- **Tutorial overlays.** A pulsing highlight on the active affordance (one layer), a caption fading in below it (another), a dim wash everywhere else (a third). Today handled with bespoke consumer code.
- **HUD-style game UI.** Health bar flashing on damage, cooldown icon radial-wiping, XP bar gradient-sweeping — three simultaneous motions on three distinct layers.

**API shape.**
```rust
#[non_exhaustive]
pub struct RaSceneLayer {
    pub id: LayerId,
    pub z: i16,
    pub placement: RaLayerPlacement,
    pub source: RaContentSource,
    pub role_tag: RoleTag,
    pub overflow: RaLayerOverflow,
    pub visibility: RaLayerVisibility,

    // NEW — all additive, Option-wrapped, back-compat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<RaLayerPipeline>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<RaLayerSchedule>,
}

pub struct RaLayerPipeline {
    pub mask: Option<MaskSpec>,
    pub sampler: Option<SamplerSpec>,
    pub filter: Option<FilterSpec>,
    pub shader: Option<Box<SpatialShaderType>>,
    pub style: Option<StyleLayer>,
}

pub struct RaLayerSchedule {
    #[serde(default)] pub enter_delay_ms: u64,
    #[serde(default)] pub enter_duration_ms: Option<u64>, // else inherit
    #[serde(default)] pub exit_start_at_ms: Option<u64>,
    #[serde(default)] pub exit_duration_ms: Option<u64>,
}
```

**Interaction with the recipe-level pipeline.** Layer pipeline overrides recipe pipeline *for that layer's composed cells*. Cells outside any layer still take the recipe pipeline. This is the same "layer wins where it applies" rule the compositor already uses for `Shadow` role tagging.

**Risks / costs.**
- Compositor work per frame scales with number of layers × stages. Needs the existing per-layer cache (the tests in `tui-vfx-recipes/tests/layers/test_per_layer_cache.rs` already anticipate this).
- Trace taxonomy: emit new `TraceEvent::LayerPipelineApplied { layer_id, stage }` so `pipeline-validator --probe` can diff per-layer stages.

**Effort estimate.** Medium-large. The hardest part is the cache-key story; the schema change itself is mechanical.

---

### 2 · `SpatialSignalSampler` driven by a `SpatialSignalSpec` (built on, and extending, `mixed-signals`)

**What it is.** A new `Sampler::SpatialSignalSampler` variant whose displacement is computed from a JSON-encoded spatial signal graph. Internally the graph reuses `mixed-signals` for all strictly-1D composition and processing (Add, Multiply, Clamp, Remap, …) but carries per-leaf **spatial-frequency metadata** (`x_freq`, `y_freq`, `t_freq`) that `mixed-signals` itself does not currently support.

`tui-vfx-compositor` constructs the spatial graph at recipe-parse time from the `SpatialSignalSpec`, then evaluates it per-cell per-frame by projecting `(x_norm, y_norm, t)` to each leaf's scalar phase and composing with `mixed-signals` operators. The existing `SineWave` sampler is preserved unchanged for back-compat.

**Why this framing** (instead of either a bespoke `AmplitudeCurve` + `Harmonic` API on `SineWave`, or a naive JSON mirror of `mixed-signals`). `mixed-signals` is already a dependency and has exactly the 1D composition primitives we want to reuse. Inventing parallel `Add`/`Multiply`/`Clamp` would duplicate well-tested code. But `mixed-signals::Signal::sample(t)` is **strictly 1D** — there is no native way to evaluate a graph at `(x, y, t)` where each leaf responds at its own spatial-frequency ratio (see §"What mixed-signals provides / doesn't provide" below). So this PRD's `SpatialSignalSpec` lives in `tui-vfx` and adds spatial awareness **at the leaves**, delegating everything else to `mixed-signals`.

The Madeira flag's `amplitude · (sin(x·8 − t·2.4) + 0.3·sin(x·15 − t·4))` with `amplitude = norm_x · 0.15` becomes, in the proposed spatial graph:

```json
{
  "type": "multiply",
  "a": { "type": "spatial_ramp", "from": 0.0, "to": 0.15, "axis": "x_norm" },
  "b": {
    "type": "add",
    "a": { "type": "spatial_sine", "x_freq": 8.0, "y_freq": 0.0, "t_freq": -2.4 },
    "b": {
      "type": "multiply",
      "a": { "type": "spatial_sine", "x_freq": 15.0, "y_freq": 0.0, "t_freq": -4.0 },
      "b": { "type": "constant", "value": 0.3 }
    }
  }
}
```

Each `spatial_*` leaf carries the per-axis frequencies the compositor needs to project `(x_norm, y_norm, t)` to a scalar phase before delegating to a `mixed-signals` generator (or, for the `spatial_sine` leaf, inlining `sin(2π · phase)` directly). The composition nodes (`add`, `multiply`, `clamp`, `remap`) are pure 1D scalar-to-scalar and **can** map directly onto `mixed-signals::{Add, Multiply, Clamp, Remap}` as wrappers over the spatial leaves.

**What it unlocks beyond the flag.**
- **Cloth / banner motion** — any pinned-edge or pinned-corner wave, with authored amplitude curves.
- **Water / ripple surfaces** — pond ripples viewed top-down via `Ramp` × radial-distance composed with `Sine`.
- **Heat haze** — amplitude rising with height (multiply displacement by `Ramp(y_norm)`) above a hot element.
- **Holographic interference / beats** — two nearly-equal-frequency sines added.
- **Paper / card physics** — a `DampedSpring` in place of the sine: the flag flutters on initial deploy and settles.
- **Scripted wave shapes** — `Keyframes` for authored-not-procedural motion (storyboarded waves for cinematic intros).
- **Noise-modulated wave** — `sine.mix(perlin, 0.2)` for organic, imperfect motion.

**API shape (sketch).** Leaves carry `(x_freq, y_freq, t_freq)`; composition / processing nodes are 1D pass-throughs that delegate to `mixed-signals`.

```rust
/// Additive new sampler variant in `SamplerSpec`.
pub struct SpatialSignalSampler {
    pub axis: Axis,                   // which spatial axis is displaced
    pub signal: SpatialSignalSpec,    // serde-friendly spatial graph
    pub speed: f32,                   // global time multiplier; default 1.0
    pub amplitude_cells: f32,         // graph output in [-1,1] remaps to [-amp, +amp] cells
}

/// Serde-friendly spatial graph. Leaves are spatial-aware; nodes are 1D.
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SpatialSignalSpec {
    // Spatial-aware LEAVES: each carries per-axis frequencies. Compositor computes
    // phase = x_norm * x_freq + y_norm * y_freq + t * t_freq  (+ phase_offset)
    // and feeds that to the 1D generator.
    SpatialSine     { #[serde(default)] x_freq: f32, #[serde(default)] y_freq: f32, #[serde(default)] t_freq: f32, #[serde(default = "one_f32")] amplitude: f32, #[serde(default)] phase_offset: f32 },
    SpatialTriangle { x_freq: f32, y_freq: f32, t_freq: f32, amplitude: f32, phase_offset: f32 },
    SpatialSquare   { x_freq: f32, y_freq: f32, t_freq: f32, amplitude: f32, phase_offset: f32 },
    SpatialSawtooth { x_freq: f32, y_freq: f32, t_freq: f32, amplitude: f32, phase_offset: f32 },
    /// Linear ramp along a specific spatial axis or time.
    SpatialRamp     { from: f32, to: f32, axis: RampAxis /* x_norm | y_norm | t_seconds */ },
    /// Authored keyframe curve over a specific axis.
    SpatialKeyframes { points: Vec<(f32, f32)>, axis: RampAxis, #[serde(default)] interp: Interp },
    /// Constant scalar (no spatial dependence).
    Constant { value: f32 },
    /// Spatial Perlin noise (samples at (x_norm, y_norm) scaled, uses `SignalContext.seed`).
    SpatialPerlin   { x_scale: f32, y_scale: f32, t_scale: f32, seed: u64 },

    // 1D NODES: delegate directly to mixed-signals operators on child outputs.
    // The compositor evaluates children at (x, y, t), gets scalar outputs, and applies the 1D op.
    Add       { a: Box<SpatialSignalSpec>, b: Box<SpatialSignalSpec> },                     // -> mixed_signals::Add
    Multiply  { a: Box<SpatialSignalSpec>, b: Box<SpatialSignalSpec> },                     // -> mixed_signals::Multiply
    Mix       { a: Box<SpatialSignalSpec>, b: Box<SpatialSignalSpec>, t: f32 },             // -> mixed_signals::Mix
    WeightedMix { parts: Vec<(f32, SpatialSignalSpec)> },                                    // -> mixed_signals::WeightedMix
    Normalized { inner: Box<SpatialSignalSpec> },                                            // -> mixed_signals::Normalized
    Remap     { inner: Box<SpatialSignalSpec>, from: (f32, f32), to: (f32, f32) },           // -> mixed_signals::Remap
    Clamp     { inner: Box<SpatialSignalSpec>, min: f32, max: f32 },                         // -> mixed_signals::Clamp
    Abs       { inner: Box<SpatialSignalSpec> },                                             // -> mixed_signals::Abs
    Invert    { inner: Box<SpatialSignalSpec> },                                             // -> mixed_signals::Invert
}
```

`mixed-signals::Sine` et al already have `Serialize, Deserialize` (verified in `src/generators/cls_sine.rs`, `cls_ramp.rs`, etc.). The reason we **don't** reuse those types directly in `SpatialSignalSpec` is that they carry only a single `frequency` field — they can't express "this sine runs at x_freq=8 AND t_freq=−2.4" which is exactly what Madeira-style compound waves require.

**Risks / costs.**
- **Combinatorial validation.** A nonsense graph (e.g. `Add { a: Ramp, b: Normalized { inner: Ramp } }`) must parse but produce a predictable runtime output. `mixed-signals` is defensive by construction; we propagate that.
- **`Box<dyn Signal>` vs generic composition.** `mixed-signals` is generics-heavy; a runtime graph loses a little inline-ability. Acceptable — the per-cell cost is already dwarfed by terminal write costs.
- **Serde round-trip.** The schema must round-trip losslessly. Covered by standard snapshot tests.
- **Version drift.** As `mixed-signals` adds generators, the schema needs to follow. Additive-only; `SignalSpec` is `#[non_exhaustive]` so downstream doesn't have to match exhaustively.
- **Authoring ergonomics.** Deep nesting is ugly in JSON. Follow-up (out of scope): a short-form "expression" string like `"ramp(0,0.15,x) * (sine(8,2.4) + 0.3*sine(15,4))"` that parses to the graph. Not part of this PRD but mentioned so reviewers know the ergonomic ceiling.

**Effort estimate.** Medium. Schema + `build()` + per-variant tests. The runtime impl is largely delegation to `mixed-signals`.

---

### 3 · `DisplacementShade` spatial shader

**What it is.** A shader that reads the **per-cell sampler offset** (the `(dx, dy)` the sampler produced) and uses it as a lighting signal — darkening troughs, brightening peaks. Parameters: `strength: f32`, `bias: f32`, `clamp: (f32, f32)`, `apply_to: ShaderApplyTarget`.

**Why.** Wave displacement without shading looks flat — a wobble rather than a wave. The Madeira code computes `shade = clamp(wave·0.25 + 0.75, 0.65, 1.0)` inline while rasterising; this shader lifts the idea out of the renderer and into the pipeline stage, where it composes with any sampler (not just cloth).

**What it unlocks beyond the flag.**
- **Every `SineWave`/`Ripple`/`SignalSampler` recipe gets free 3D feel** without any author effort. `dream_state.json` benefits instantly.
- **Reflective surfaces** — pair a tight amplitude curve with shade to make metal/glass seem to ripple under light.
- **Terrain suggestion** — a static terrain heightfield (computed once via `SignalSpec::Keyframes`) plus shade produces instant relief maps.
- **Reveal energy** — during a mask-driven reveal, a subtle displacement-shade pass makes the wavefront feel embodied rather than cosmetic.

**Implementation shape.** Samplers need to report their offset alongside the remapped coordinate. Proposal: extend the `Sampler` trait with `sample_with_hint(&self, ...) -> Option<(u16, u16, SamplerOffsetHint)>` as a default method that wraps `sample()`. The hint is a normalised `(dx, dy)` in cells, `None` for non-displacement samplers like `CrtJitter`.

**Risks.** The offset hint must be consistent across samplers — a design-review item. `SignalSampler` returns the signal output as the hint directly; existing samplers get retrofitted one at a time.

**Effort estimate.** Small-to-medium. The shader is trivial; the plumbing to surface the displacement hint is the real work.

---

### 4 · Braille-supersampled image source (`.rsb` format)

**What it is.** A new file format in `rocketsplash-formats` parallel to `.rss`, carrying sub-cell RGB at 2×4-per-cell density:

```rust
pub struct BrailleImage {
    pub version: u8,          // 1
    pub meta: BrailleImageMeta,
    pub width_cells: u32,
    pub height_cells: u32,
    pub dots: Vec<DotCell>,   // width_cells * height_cells entries
}
pub struct DotCell {
    pub fg: Option<Rgb>,      // averaged foreground colour
    pub bg: Option<Rgb>,      // optional background (avg of "off" sub-pixels)
    pub lit: u8,              // bitmask matching braille dot ordering (U+2800 + lit)
    pub style: CellStyle,
}
```

At render time, the compositor emits each cell as a braille glyph (`char::from_u32(0x2800 + lit)`) with the per-cell fg/bg.

The recipe schema gets a new scene-source variant or an additive flag on `RaImageSource`:
```rust
pub struct RaImageSource {
    pub image_name: String,
    pub tint: Option<Color>,
    pub aspect: RaImageAspect,
    #[serde(default)] pub kind: RaImageKind, // Rss (default) | Rsb
}
```

We recommend the **additive `kind` field** path rather than a new `RaBrailleImageSource` variant — keeps the scene schema surface lean and mirrors the `RaContentConfig::image_name` precedence story already in `RaContentConfig`.

**Why.** `.rss` is 1 char + colours per cell. That's enough for pixel-art logos at cell resolution but throws away 8× the density. Braille supersampling is what makes the Madeira flag legible at 30×10 cells.

**What it unlocks beyond the flag.**
- **Any national flag.** The world has ~200 of them, many already authored in braille ASCII demos.
- **Logos and wordmarks** — corporate logos, open-source project marks, anniversary seals.
- **Heraldry and ornament** — medieval-scroll recipes, seal-of-approval stamps, certificates.
- **Pixel-art splash content** — 8-bit game splashes, chiptune-era logos.
- **Fine indicators** — braille progress bars and dials that don't want the cell-coarse look of `.rss`.

**Tooling implication.** A `.rsb` authoring path is needed — at minimum a CLI (`rocketsplash png-to-rsb` or similar) that takes a PNG, optionally a palette, and emits `.rsb`. The tooling is **scoped out of this PRD**; the PRD delivers the format + loader + renderer.

**Effort estimate.** Medium. Format + loader is straightforward; the braille rasteriser is ~50 LOC; wiring into the compositor scene-composer ~200 LOC; tests and golden-image fixtures are the bulk of the work.

---

### 5 · `StaggeredLines` content effect (nice-to-have, ergonomic)

**What it is.** A new `ContentEffect` variant taking `lines: Vec<LineSpec>` and an optional `hint: LineSpec`, each with its own delay / colour / style / ease.

**Why.** After per-layer pipelines ship, three independently-timed text lines can already be expressed as three separate `Text` layers. But a single `StaggeredLines` effect is ~5× less verbose to author for the common case of "stacked captions with staggered fade-in." Same reason `Typewriter` exists rather than forcing authors to express it as a mask + reveal pair.

**What it unlocks beyond the flag.**
- **Splash taglines** that build one line at a time.
- **Tutorial step-by-step callouts.**
- **NPC dialogue sequences.**
- **Marketing-copy beats** — release-notes toast with staggered feature bullets.

**Effort estimate.** Small. One `ContentEffect` variant + renderer + schema entry; no compositor impact.

---

### Cross-cutting: trace taxonomy extensions

Each new primitive adds a taxonomy entry so `InspectionSink` / `TraceSink` (see `TRACE_EVENT_SCHEMA.md`) stays authoritative:

- `TraceEvent::LayerPipelineApplied { layer_id, stage }`
- `TraceEvent::SpatialSignalEvaluated { sampler_id, mean_offset, peak_offset }` (DEBUG verbosity only — sampled across the layer area, not per-cell)
- `TraceEvent::DisplacementShaderApplied { layer_id, mean_shade }`
- `TraceEvent::BrailleImageComposed { layer_id, asset, cells_lit, cells_dark }`

---

## What `mixed-signals` provides / doesn't provide

`tui-vfx-compositor` already depends on `mixed-signals` (`use mixed_signals::prelude::{Normalized, Remap, Signal, SignalExt, Sine}` in the existing `SineWave` sampler). Today it's used privately; the PRD pushes more of it to recipe authors. But this PRD's biggest honesty correction over v0.2.0 is that **`mixed-signals` is a 1D temporal signal library and does not natively solve the Madeira problem end-to-end**. Here is the full accounting.

### What `mixed-signals` already provides ✅

Verified by direct reads of `/usr/projects/mixed-signals/src/`:

| Capability | Location | Notes |
|---|---|---|
| `Sine`, `Triangle`, `Square`, `Sawtooth`, `Ramp`, `Keyframes`, `Constant`, `Step`, `Pulse` | `src/generators/` | All with `#[derive(Serialize, Deserialize)]` — serde already works |
| Composition: `Add`, `Multiply`, `Mix`, `WeightedMix`, `FrequencyMod`, `VcaCentered` | `src/composition/` | Covers compound arithmetic |
| Processing: `Normalized`, `Abs`, `Invert`, `Clamp`, `Remap`, `Quantize`, `NormalizedFrom` | `src/processing/` + `src/traits/ext_signal.rs` | Covers output remap / clamp |
| Envelopes: `ADSR`, `Linear`, `Impact`, `LinearDecay`, `ExponentialDecay` | `src/envelopes/` | For lifecycle-aware shaping |
| Physics: `DampedSpring`, `BouncingDrop`, `FrictionDecay`, `Pendulum`, `Orbit`, `Projectile`, `Attractor` | `src/physics/` | Useful for "cloth settles" follow-up work |
| Noise: `WhiteNoise`, `PerlinNoise`, `PinkNoise`, `CorrelatedNoise`, `SpatialNoise`, `GaussianNoise` | `src/noise/`, `src/random/` | `SpatialNoise` samples at `(x, y)` — one of the few spatial-aware types already in the crate |
| `SignalContext` with `width`, `height`, `frame`, `seed`, `phase`, `phase_t`, `loop_t`, `absolute_t`, `char_index` | `src/traits/signal.rs` | Already carries render-area metadata, though `sample` itself doesn't use it for spatial input |
| `Phase` enum (`Start`/`Active`/`End`/`Done`/`Custom`) | `src/traits/signal.rs` | Directly usable for layer schedule integration |

### What `mixed-signals` does **not** provide — the real gaps ❌

| Missing capability | Why the Madeira flag needs it | Current state |
|---|---|---|
| **A 2D-signal trait** (`sample(x, y, t) -> f32`) or equivalent | Madeira's compound wave `sin(x·8 − t·2.4) + 0.3·sin(x·15 − t·4)` requires each harmonic to see its own `(x_freq, t_freq)` ratio. A single scalar `t` fed to a composed `Add(Sine_A, Sine_B)` makes both Sines share the same input — you can't encode different phase velocities. | `Signal::sample(t: SignalTime) -> f32` is strictly 1D. `SignalContext` carries `width`/`height` as **metadata** only; neither the trait nor any provided op evaluates at `(x, y)`. |
| **Per-leaf axis frequency on periodic generators** | Same. `Sine::new(frequency, amplitude, offset, phase)` takes one `frequency` — there is no `(x_freq, y_freq, t_freq)` triple to project spatial coordinates to phase. | Would require either adding a `frequency: Vec3` parallel constructor or shipping a new `SpatialSine` generator. Both are upstream changes. |
| **A spatial `Ramp` that reads `x_norm` or `y_norm` as its input axis** | Madeira's `amplitude = norm_x · 0.15`. `mixed_signals::Ramp::new(start, end, duration)` interprets input strictly as elapsed time — there is no axis selector. | Current workaround: the sampler pre-projects `x_norm` and feeds it as `t` to `Ramp`. Works for single-leaf cases but breaks for compound graphs where different leaves want different axis inputs. |
| **`FrequencyMod` with both spatial and temporal modulators** | A richer cloth model `carrier(x, t) modulated by slow_sine(t)` needs each side to be spatial-aware. | `mixed_signals::FrequencyMod` is 1D. |
| **Determinism guarantees that compose with the recipe scene's own `seed`/`frame`** | Recipe composer requires deterministic replay (see `TraceEnvelope.frame_no`). | `SignalContext.seed`/`frame` exist but are not universally threaded through the non-noise operators — they're scaffolding rather than contract. |

### Two resolution paths

#### Path A — `SpatialSignalSpec` in-tree (primary, recommended for this PRD)

Put `SpatialSignalSpec` in `tui-vfx-compositor` (or a new `tui-vfx-signal-spec` sub-crate). The variants fall into two groups:

- **Spatial-aware leaves** (`SpatialSine`, `SpatialTriangle`, `SpatialSquare`, `SpatialSawtooth`, `SpatialRamp`, `SpatialKeyframes`, `SpatialPerlin`, `Constant`): carry per-axis frequencies and evaluate themselves. Internally they either inline the math (for `sin`, `triangle`, etc.) or delegate to a pre-projected scalar `mixed_signals` generator.
- **1D composition / processing nodes** (`Add`, `Multiply`, `Mix`, `WeightedMix`, `Normalized`, `Remap`, `Clamp`, `Abs`, `Invert`): evaluate children at `(x, y, t)`, reduce to scalar outputs, and delegate to the corresponding `mixed-signals` operator on those scalars.

Pros: no upstream dependency, fully under our control, ships immediately. Reuses the parts of `mixed-signals` that are unambiguously correct (the 1D ops).

Cons: we're re-spelling a subset of `mixed-signals` leaves with spatial frequency triples. Some duplication.

#### Path B — `Signal2d` / `SpatialSignal` upstream in `mixed-signals` (secondary, optional)

Propose a new trait in `mixed-signals`:
```rust
pub trait Signal2d {
    fn sample_2d(&self, x_norm: f32, y_norm: f32, t: SignalTime, ctx: &SignalContext) -> f32;
}
```
…and have `Sine`/`Triangle`/`Square`/`Sawtooth`/`Ramp`/`Keyframes` implement it with per-leaf `(x_freq, y_freq, t_freq)` constructors. Composition ops (`Add`, etc.) would impl both `Signal` and `Signal2d`, fanning out to children's `sample_2d` when available.

Pros: single-crate story for signal authoring; the 2D surface grows organically.
Cons: requires upstream design work on `mixed-signals`; ripples through existing consumers; requires a `mixed-signals` major version.

**Recommendation.** Ship Path A in this PRD. Path B can be a follow-up that, if it lands, lets `SpatialSignalSpec` thin down to a serde mirror of the upstream types — a nice cleanup but not load-bearing. Path A is what we can deliver now without being blocked on another crate's release cycle.

### What upstream `mixed-signals` changes would still be nice (out of scope for this PRD)

Even without Path B, two small upstream additions would strengthen the story:

1. **`axis_input: TimeInput | SpatialXInput | SpatialYInput` on `Ramp` and `Keyframes`.** Additive enum; defaults to `TimeInput`. Would let the sampler feed spatial coordinates without the current "pretend x_norm is t" workaround.
2. **`Signal::sample_with_context(&self, t, ctx)` default method** that falls back to `sample(t)`. Existing `SignalContext` is underutilised; this gives noise and lifecycle ops a contract-clean way to reach it.

Neither is a blocker — both are improvements that would flow back into `tui-vfx` if they land.

---

## Phased delivery

Each phase ships independently; each leaves the library in a better place than it found it.

### Phase 1 — Keystone: per-layer pipelines (primitive 1)

- Schema changes in `tui-vfx-recipes/src/recipe_schema/scene/`.
- Compositor plumbing to apply layer pipelines before the global pipeline.
- Per-layer cache keys (tests already anticipate this).
- New `TraceEvent::LayerPipelineApplied` variant.
- Golden-image tests for a "three-layer toast" showcase.

**Acceptance.** An existing monolithic recipe can be refactored into layers with equivalent rendering, and a new recipe can apply a sampler to exactly one layer without affecting others.

### Phase 2 — Spatial signal composition: `SpatialSignalSampler` + `SpatialSignalSpec` (primitive 2)

- `SpatialSignalSpec` enum in `tui-vfx-compositor` (or new `tui-vfx-signal-spec` sub-crate).
- Per-variant evaluator: spatial leaves project `(x_norm, y_norm, t)` to scalar phase; 1D nodes delegate to `mixed-signals` operators.
- New `SpatialSignalSampler` variant of `SamplerSpec`.
- Parse/round-trip tests for every `SpatialSignalSpec` variant.
- At least one recipe in the corpus re-authored from `SineWave` to `SpatialSignalSampler` to prove parity.

**Acceptance.** A recipe describing `x_ramp(0→1) · (spatial_sine(x_freq=8, t_freq=−2.4) + 0.3 · spatial_sine(x_freq=15, t_freq=−4))` produces the characteristic pinned-at-left, whipping-at-right wave on a rectangular test layer, matching a reference cell-by-cell at `t = {0.5, 1.0, 2.0}`.

### Phase 3 — Displacement-aware lighting: `DisplacementShade` (primitive 3)

- Sampler trait enhancement (`sample_with_hint` default method).
- `DisplacementShade` shader in `tui-vfx-style/src/models/`.
- Retrofit `SineWave`, `Ripple`, `SignalSampler` to report hints.
- Tests and golden images.

**Acceptance.** A recipe with `SignalSampler` + `DisplacementShade` on a flat-coloured rectangular layer renders as a cleanly-waving banner with visible peaks and troughs.

### Phase 4 — Asset fidelity: `.rsb` braille-supersampled image format (primitive 4)

- New `BrailleImage` type + loader in `rocketsplash-formats`.
- `RaImageSource.kind` additive field (`Rss` | `Rsb`).
- Compositor integration for braille rasterisation.
- At least one reference `.rsb` asset checked in for tests.
- Documentation entry in `CAPABILITIES_REFERENCE.md`.

**Acceptance.** A recipe rendering a `.rsb` image produces a grid of valid U+2800-range glyphs with per-cell averaged colour; the probe report confirms the expected number of cells were lit.

### Phase 5 — Ergonomic cleanup: `StaggeredLines` (primitive 5)

- New `ContentEffect::StaggeredLines` variant + renderer.

**Acceptance.** Reference recipes demonstrating the effect ship in the corpus.

### Phase 6 — The reference recipe

- Commit `recipes/madeira_flag_banner.json` to `tui-vfx-recipes/recipes/` (flag + text only — fireworks intentionally omitted).
- Add a hero-entry to `COMPOSED_CAPABILITIES.md`.
- Write a "porting bespoke overlays to recipes" guide using the madeira crate → recipe diff as the worked example.

**Acceptance.** Side-by-side visual sign-off against the `madeira-flag` crate's `cargo run --example demo` (flag-and-text portion only). Frame hashes at canonical `t = {0.5, 1.0, 2.0, 3.5}` recorded as goldens for CI.

---

## Risks

| Risk | Severity | Mitigation |
|---|---|---|
| Per-layer pipelines multiply per-frame compositor work. | Medium — on very wide terminals with many layers, frame budget tightens. | Per-layer cache (test fixtures already anticipate this). `--debug-recipes-qc` adds a per-layer timing column. |
| `Sampler` trait change breaks downstream consumers. | Low — we control all consumers. | Use an additive `sample_with_hint` default method; existing `sample` keeps working. |
| `SpatialSignalSpec` JSON becomes deeply nested and hard to author. | Medium | Follow-up (out of scope): a short-form expression parser. The JSON graph is always available as the canonical form. |
| `mixed-signals` is 1D and doesn't natively carry 2D composition; we've taken on that surface area ourselves. | Medium | Documented in the dedicated gap section. Path B (upstream `Signal2d`) is a clean follow-up; Path A ships without being blocked. |
| `mixed-signals` ships a new 1D operator; `SpatialSignalSpec` lags. | Low — additive only. | `#[non_exhaustive]` on the spec enum; CI doc note that adding a `mixed-signals` operator comes with a PRD-tracked schema mirror. |
| `.rsb` format fragments the asset ecosystem. | Medium | Ship an "rsb-from-png" tool alongside the format. Treat `.rsb` as an authoring target, not hand-editable. |
| Scope creep pulls fireworks back in. | Low — explicitly out of scope. | Separate PRD if/when the celebration use case returns. |

---

## Acceptance criteria for the PRD as a whole

1. **The flag-and-text portion of the Madeira easter egg ships as a single recipe JSON** that `pipeline-validator` parses clean, `recipe-probe` exports a deterministic NDJSON trace for, and `pipeline-validator --debug-recipes-qc` fingerprints identically across three runs.
2. **At least two non-flag demo recipes** ship alongside it to prove the general framing — candidates: a rippling-water splash using `SignalSampler` + `DisplacementShade`, and a braille logo splash using `.rsb` with a per-layer fade-in.
3. **No existing recipe regresses** — the full corpus passes `--debug-recipes-qc` with no fingerprint drift.
4. **`CAPABILITIES_REFERENCE.md`, `COMPOSED_CAPABILITIES.md`, and `TRACE_EVENT_SCHEMA.md` are updated** in the same PRs that ship the primitives.
5. **The `madeira-flag` crate remains published and buildable** as a zero-dep alternative. The recipe version is a parallel capability.

---

## Open design questions

1. **`SpatialSignalSpec` home crate.** Option A: in `tui-vfx-compositor` alongside the sampler. Option B: a new `tui-vfx-signal-spec` sub-crate. Lean: Option B if a second consumer emerges; Option A for now.
2. **Path A only, or Path A + upstream Path B?** Path A is the load-bearing deliverable. Path B (pushing `Signal2d` upstream to `mixed-signals`) is strictly opt-in. Lean: propose Path B upstream **after** Path A has shipped and proven the shape; `mixed-signals` maintainers can then decide.
3. **Short-form expression language.** Out of scope; noted. `"ramp(0,1,x) * (sine(x:8, t:-2.4) + 0.3*sine(x:15, t:-4))"` is an obvious follow-up once the graph form is proven.
4. **Do layer pipelines have their own enter/exit timings?** Yes — `RaLayerSchedule`. Defaults inherit the recipe-level timing.
5. **Where does `DisplacementShade` get the displacement from when no sampler is active?** Skip the shader (noop), don't render neutral output.
6. **`.rsb` vs `.rss` with supersample flag.** Weighed in §4; lean `.rsb` as a distinct format.
7. **Does `SpatialSignalSpec` expose noise generators?** Yes — `SpatialPerlin` is in the first cut; deterministic seeds are a requirement (CI frame-hash stability). `SpatialNoise` already exists in `mixed-signals` and can back it.

---

## Appendix A — Mapping the flag portion of the Madeira code to the proposed primitives

Reference source: `/usr/projects/madeira-flag/src/lib.rs` (783 LOC). Fireworks (~110 LOC) are **out of scope** and remain in the standalone crate.

| Code location | Behaviour | Recipe expression after this PRD |
|---|---|---|
| `fnc_render_backdrop.rs` (34 LOC) | Clear + fade bg to `Rgb(5, 5, 15)` over 0.25 s | A solid-colour background layer (`source: Procedural { source_id: "solid_color" }` or `source: Text` with empty message + background style) with `schedule.enter_duration_ms: 250` |
| `fnc_draw_flag_pattern.rs` (136 LOC) | Tribands + cross pattée + Greek cross on 2×4 canvas | `.rsb` asset checked into `tui-vfx-recipes/assets/`, referenced by `RaImageSource { image_name: "flag.madeira.rsb", kind: Rsb }` |
| `fnc_render_waving_flag.rs` (111 LOC) | Compound sine with x-proportional amplitude + per-pixel shade | Layer pipeline: `sampler: SpatialSignalSampler { axis: y, amplitude_cells: 3.0, signal: Multiply(SpatialRamp(0→1, x_norm), Add(SpatialSine(x_freq=8, t_freq=−2.4), Multiply(SpatialSine(x_freq=15, t_freq=−4), Constant(0.3)))) }` + `shader: DisplacementShade { strength: 0.25, bias: 0.75, clamp: [0.65, 1.0] }` |
| `fnc_render_text.rs` (74 LOC) | 3 lines + hint, staggered reveal, 4 colours | `content.effect: StaggeredLines { lines: [...], hint: {...} }` (or equivalent via 4 text layers with schedules, if Phase 5 slips) |
| Timing constants (`elapsed > 0.2`, `> 1.0`, `> 2.0`) | Per-element delay schedule | `schedule.enter_delay_ms` per layer |

Every piece of the ~670 LOC covered by this PRD's scope maps to **one recipe field**.

---

## Appendix B — What the final recipe looks like (sketch)

```json
{
  "schema_version": 1,
  "id": "celebration.madeira_flag_banner",
  "title": "Funchal New Year's Flag",
  "version": "1.0.0",
  "config": {
    "message": "",
    "layout": { "mode": "fullscreen", "anchor": "center" },
    "lifecycle": { "auto_dismiss_ms": 0, "loop": true },
    "pipeline": { "enter": { "duration_ms": 250, "easing": "linear" } },
    "scene": {
      "default_role": "background",
      "fit_policy": "clip",
      "layers": [
        {
          "id": "backdrop", "z": 0,
          "placement": { "type": "fill" },
          "source": { "type": "procedural", "spec": { "source_id": "solid_color",
            "params": { "color": "#05050F" } } },
          "role_tag": "background",
          "schedule": { "enter_duration_ms": 250 }
        },
        {
          "id": "flag", "z": 1,
          "placement": { "type": "anchor", "anchor": "center", "width_cells": 60, "height_cells": 20 },
          "source": { "type": "image", "spec": { "image_name": "flag.madeira.rsb", "kind": "rsb" } },
          "role_tag": "image",
          "pipeline": {
            "sampler": {
              "type": "spatial_signal",
              "axis": "y",
              "speed": 1.0,
              "amplitude_cells": 3.0,
              "signal": {
                "type": "multiply",
                "a": { "type": "spatial_ramp", "from": 0.0, "to": 1.0, "axis": "x_norm" },
                "b": {
                  "type": "add",
                  "a": { "type": "spatial_sine", "x_freq": 8.0, "t_freq": -2.4 },
                  "b": {
                    "type": "multiply",
                    "a": { "type": "spatial_sine", "x_freq": 15.0, "t_freq": -4.0 },
                    "b": { "type": "constant", "value": 0.3 }
                  }
                }
              }
            },
            "shader": { "type": "displacement_shade",
              "strength": 0.25, "bias": 0.75, "clamp": [0.65, 1.0] }
          },
          "schedule": { "enter_delay_ms": 200, "enter_duration_ms": 250 }
        },
        {
          "id": "text", "z": 2,
          "placement": { "type": "anchor", "anchor": "bottom_center", "y_offset": -4 },
          "source": { "type": "text", "spec": {} },
          "role_tag": "text",
          "schedule": { "enter_delay_ms": 1000 }
        }
      ]
    },
    "content": {
      "effect": {
        "type": "staggered_lines",
        "lines": [
          { "text": "✨ Feliz Ano Novo! ✨", "color": { "type": "rgb", "r": 255, "g": 215, "b": 0 }, "delay_ms": 0,   "style": { "bold": true } },
          { "text": "Happy New Year From",  "color": { "type": "rgb", "r": 200, "g": 200, "b": 200 }, "delay_ms": 150 },
          { "text": "Funchal, Madeira",     "color": { "type": "rgb", "r": 100, "g": 220, "b": 255 }, "delay_ms": 300, "style": { "bold": true } }
        ],
        "hint": { "text": "Press Esc to return", "delay_ms": 2000,
          "color": { "type": "rgb", "r": 120, "g": 120, "b": 120 } }
      }
    },
    "requires_primitives": [
      "scene.layer_pipeline",
      "sampler.spatial_signal",
      "shader.displacement_shade",
      "image.rsb",
      "content.staggered_lines"
    ]
  }
}
```

Each entry in `requires_primitives` maps to a primitive in this PRD. `pipeline-validator` uses that list for capability checking and graceful degradation.

---

## Summary

The Madeira flag banner (fireworks excluded) is a compact, decidable test of whether `tui-vfx` is a pipeline for **text-plus-effects** or a pipeline for **composed scenes driven by authored spatial signal graphs**. Today it's the former. With the four primitives in this PRD — keystoned by per-layer pipelines and anchored on a new `SpatialSignalSpec` that extends, rather than simply mirrors, `mixed-signals` — it becomes the latter. The ~630-recipe corpus inherits a new authoring vocabulary: **any flag, any braille image, any signal-driven wave, any displacement-lit surface**, each expressible as a single recipe JSON that `pipeline-validator` and `recipe-probe` can reason about end-to-end.

The lift is real. Critically, **`mixed-signals` does not solve the whole problem by itself** — it's 1D/temporal, and Madeira-style compound waves need per-leaf spatial-frequency metadata that `mixed-signals::Sine` (and peers) don't carry. This PRD handles that gap honestly with a `SpatialSignalSpec` that lives in `tui-vfx`, reuses the 1D composition / processing operators from `mixed-signals` where they apply, and adds spatial-aware leaves where the library stops short. Path A is the deliverable; Path B (pushing `Signal2d` upstream) is an opt-in follow-up once the shape is proven.

Every primitive defends itself independently of the flag, the scene-composer foundation is already in the tree (Sub-plan B.1/B.3), `mixed-signals` is already a dependency being used privately, and the payoff compounds: once per-layer pipelines ship, `SpatialSignalSampler` slots in behind the same schema, `DisplacementShade` composes with it, and `.rsb` is the only piece that touches a new format.

<!-- <FILE>PRD-FLAG-ANIMATION.md</FILE> - <DESC>PRD for recipe-authored braille image compositions</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
