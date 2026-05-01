<!-- <FILE>docs/design/post-release/indexed-palette-cycling-spec.md</FILE> - <DESC>Post-release specification for an indexed-palette-with-rotation primitive at the tui-vfx-style layer that decouples spatial pattern (per-cell integer index) from color binding (small N-entry palette table) and lets the palette rotate per frame, providing demoscene-style cheap animation, mood reskin, and palette cross-fade as authorable capabilities — composes with the existing ColorRamp / GradientLUT infrastructure rather than replacing it, and dramatically simplifies high-line-count effect shaders (cls_terminal_fire_shader, cls_focus_field_shader, the Madeira flag's shading dispatcher) by reducing per-frame work from per-cell math to a single offset increment.</DESC> -->
<!-- <VERS>VERSION: 0.2.0-draft</VERS> -->
<!-- <WCTX>Capture the indexed-palette-cycling primitive at the tui-vfx-style layer where color computation already lives, so the demoscene technique of "static spatial pattern + rotating palette = cheap animation" becomes a first-class authorable capability across shaders, sources, and samplers — the post-release sibling of the dynamic-light-shadow primitive in the same brainstorm lineage.</WCTX> -->
<!-- <CLOG>0.2.0-draft: insert new §2 "Why this primitive matters — the architectural payoff" leading the spec with a 7-point ranked benefits framing — authoring leverage, coordinated multi-region animation, cheap mood transitions, free reskin, state separation, compute savings (explicitly demoted to bonus rather than headline), and demoscene-style parametric beauty — to ensure reviewers and migrators don't read the spec as a perf optimization when the primary value is architectural. Renumber subsequent sections §2→§3 through §13→§14 and update all internal cross-references accordingly.</CLOG> -->

# Indexed-palette source with runtime palette rotation

**Status: post-release project.** Not release-blocking V3 work. Keep deferred until the core V3 release gate, recipe migration, and as-built docs are stable. The motivating brainstorm lives at `historical-graphics-techniques-addendum.md` §1.2 (Amiga demoscene / palette rotation); a runnable visual reference implementation lives alongside this spec at `palette-cycling-demo.py` (three swappable presets, key-bindable).

## 1. Purpose

Add an **`IndexedPalette`** primitive at the `tui-vfx-style` layer plus a small **`IndexedField`** shader/source family that consumes one. Together they let recipe authors:

1. Compute a static per-cell integer index (the *spatial pattern*, e.g. concentric rings, plasma, diagonal waves) **once**, at scene-build time or first frame;
2. Hold a small N-entry palette of `Color` values (4–64 entries; 16 is the demoscene-typical sweet spot);
3. Rotate the palette by an offset that advances per frame;
4. Resolve the per-cell color at render time as `palette[(index + offset) % N]` — a single integer-add-and-mod plus a Vec lookup, with no per-cell math beyond that.

This decouples *what shape the pattern has* from *what colors it's drawn in* and makes per-frame animation cost `O(1)` (one offset increment) instead of `O(W × H)` (per-cell trig/sin/sqrt evaluation). The post-release `dynamic-light-shadow-primitive-spec` does the same separation for shadow geometry vs. light position; this spec does it for color geometry vs. palette.

## 2. Why this primitive matters — the architectural payoff

The pitch is **not** "this is faster," and getting that ranking right matters for both the design and the migration argument. On a modern terminal, per-cell color computation is rarely the bottleneck — terminal stdout throughput dominates. The headline benefits are architectural. Ranked by what actually matters for tui-vfx's use cases:

1. **Authoring leverage (the lead).** Same pattern × different palette = wildly different effect with zero new code. Same palette × different pattern = visual variety with one coherent mood. Recipes become tunable along two orthogonal axes that today are entangled inside each shader. The runnable `palette-cycling-demo.py` proves this: three completely different visuals from swapping pure-data inputs while the rendering loop is byte-identical.

2. **Coordinated animation across regions.** Multiple `IndexedField` instances sharing one `SharedPalette` rotate in lockstep automatically — a scene with five "water" regions animates as one organism, with no explicit synchronization. The alternative under the per-cell-color shader model is to wire the time signal into every shader and hope they stay in phase across the bind-resolve-render cycle.

3. **Cheap mood transitions.** Palette cross-fade animates an entire scene's color identity from one mood to another with a single parameter (the fade weight). "Diablo torch dimming as dawn breaks" or "workspace transitioning from focus-mode blue to break-mode warm" become one-line scene-level operations instead of per-shader rewrites. The post-release `dynamic-light-shadow-primitive-spec`'s Diablo-mode demo recipe is the natural first consumer of this capability.

4. **Free re-skin.** Same recipe, different palette at theme-switch time = different mood without re-computing geometry. Useful for light/dark themes, accessibility palettes, and contextual states ("this notification looks different in error vs. success") without authoring duplicate recipes. A theme system that owns palettes and routes them into recipes by name (rather than embedding RGB inline) gets an order-of-magnitude expressiveness improvement from this seam alone.

5. **State separation.** The spatial pattern is static data; only the rotation offset and any cross-fade weight are mutable state. Easier to debug, easier to snapshot for replay, easier to reason about determinism. Each existing per-cell-color shader carries its own time-evolving state; the indexed-palette idiom collapses that to one `u32` per palette regardless of how many fields consume it.

6. **Compute savings (the bonus argument, not the lead).** Real but minor on this hardware. The 921-LOC `cls_terminal_fire_shader` shrinking to ~80 LOC under this idiom (§7) is mostly about code clarity, not per-frame perf. Per-frame microseconds saved are dwarfed by ANSI emit throughput, which is the actual ceiling. Savings start to matter when many effects stack — a busy workspace running ten effects at 60 Hz — but for any single shader the win is small. Lead with the architectural gains; bring this argument out at code review when someone asks "why bother?"

7. **Demoscene-style "parametric beauty."** Once palettes and patterns are independent inputs, the parameter space becomes browsable — which is exactly what the in-flight P0.8 work (parametric variants in the recipe schema) is meant to enable. Indexed palettes are the natural unit of variation in that browse space, more than tweaking individual shader knobs across correlated parameter combinations would be. The demoscene spent a decade exploring this parameter space; tui-vfx authors get to inherit the result.

The migration story (§7) and the acceptance criteria (§9) follow from this ranking: the win is making the system *easier to author and reuse*, not faster to execute. Performance is a side-effect that sweetens the pitch but does not justify the primitive on its own. Reviewers approving the implementation should weigh against the authoring-leverage criteria, not the µs-per-cell numbers.

## 3. Non-goals

1. **Replacing `ColorRamp` / `GradientLUT`.** Those remain the canonical way to express continuous-gradient color authoring. `IndexedPalette` consumes a `ColorRamp` (or a list of `ColorConfig` keyframes) at build time to produce its N entries — it is downstream of, not parallel to, the existing color-config infrastructure.
2. **Replacing the existing shader family.** Shaders that *need* per-cell color computation (e.g. shaders that depend on runtime bindings other than time) keep their current shape. `IndexedField` is for the wide class of effects whose per-cell color is a function of *position only*, which on inspection covers a large fraction of `cls_*_shader.rs` (fire, plasma, gradient rings, diffusion fields, terminal_fire's whole geometry path).
3. **Multiple-palette layered effects.** v1 supports one active palette per `IndexedField`. Layered effects that want multiple palettes use multiple `IndexedField` instances; a future `LayeredPalette` (§11) can compose them.
4. **GPU-style per-fragment palette LUT textures.** The terminal renderer is CPU-bound; the win comes from skipping per-cell trig, not from GPU texture sampling. Implementation stays on the CPU side.
5. **Animation curves on the palette offset.** v1 advances the offset by a constant `direction × speed × dt`. Easing and time-bound playback compose naturally with the existing animation infrastructure and do not need to be re-implemented inside the primitive.

## 4. Conceptual model — and where it sits in the existing architecture

The current tui-vfx color path treats every shader as a per-cell color generator: each `cls_*_shader.rs` computes `Color` from `(x, y, t, bindings)`. Most of those shaders' per-cell math is a function of `(x, y)` only, with `t` entering only through a phase offset that affects *all* cells equally. Restated in palette-cycling terms: the spatial pattern is invariant; only the time term changes.

The indexed-palette primitive makes that invariance **explicit and exploitable**:

```text
                ┌──────────────────────────────────────┐
                │  IndexedField (shader-like source)   │
                │                                      │
   (x, y, w, h)─┤  index_field(x, y) → u8 index ───┐   │
                │  (computed once or cheaply)      │   │
                │                                  │   │
                │  IndexedPalette                  │   │
                │  ┌─────────────────────────────┐ │   │
                │  │ entries:    Vec<Color>      │ │   │
                │  │ offset:     u32 (per frame) │ │   │
                │  │ cross_fade: Option<(other,  │ │   │
                │  │             t: f32)>        │ │   │
                │  └─────────────────────────────┘ │   │
                │                                  │   │
                │  resolve(index, palette) → Color─┘   │
                └──────────────────────────────────────┘
                            │
                            ▼
                Color flows into the existing render path
                (compositor, masks, filters, downstream blends)
                exactly as any other shader's output does.
```

`IndexedPalette` is pure data plus a single `resolve` function. Rotation is an integer increment on `offset` per frame; cross-fade is a runtime-set blend weight to a second palette. The `IndexedField` is a small new shader family that owns an `index_field` function (or precomputed `Vec<Vec<u8>>`) and a reference to a palette.

A note on terminology vs. the existing codebase: tui-vfx's `Sampler` trait operates in coordinate space (output is a displaced source `(x, y)`). The indexed-palette feature operates in color space and is therefore a new shader-family member, not a sampler — but the addendum and prior brainstorm used "sampler" loosely. The mental shape is "spatial pattern + parameterized color binding," whichever name lands.

## 5. Data shape

```rust
/// A small palette of colors with a runtime-mutable rotation offset and an
/// optional cross-fade target. Cheap to construct from a ColorRamp or from
/// raw Vec<ColorConfig>.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndexedPalette {
    /// Palette entries. Length is the modulus used for index lookup.
    /// Typical N: 8 (chunky / pixel-art), 16 (demoscene default), 32 (smooth).
    pub entries: Vec<Color>,

    /// Frame-mutable rotation offset. resolve() looks up
    /// entries[(index + offset) % entries.len()].
    /// Skipped on serialize so persisted recipes don't fix the offset.
    #[serde(skip)]
    pub offset: u32,

    /// Optional cross-fade target. When Some, resolve() linearly blends
    /// this palette and `target` at weight `t` (0.0 = self, 1.0 = target).
    /// Used for "mood transitions" — e.g. water palette → ice palette.
    /// Skipped on serialize for the same reason as offset.
    #[serde(skip)]
    pub cross_fade: Option<(Box<IndexedPalette>, f32)>,
}

impl IndexedPalette {
    /// Build an N-entry palette by uniformly sampling a ColorRamp.
    /// Closes the cycle by sampling at t = i/N (so N maps back to 0),
    /// which is what makes rotation seamless.
    pub fn from_ramp(ramp: &ColorRamp, n: usize) -> Self;

    /// Build directly from explicit colors (no interpolation).
    pub fn from_colors(colors: Vec<Color>) -> Self;

    /// Resolve a per-cell index to a final color through the current
    /// rotation and any active cross-fade. Pure; no allocation.
    pub fn resolve(&self, index: u8) -> Color;

    /// Advance the rotation offset by `steps`, wrapping modulo entry count.
    pub fn rotate(&mut self, steps: i32);
}
```

```rust
/// A shader-family source that owns (or references) a static per-cell
/// integer-index field and renders it through an IndexedPalette. Plugs
/// into the same render path as other cls_*_shader sources.
pub struct IndexedField {
    /// Per-cell indices. Computed once at construction (or cached on first
    /// render). For parametric fields built from x/y formulae, this is the
    /// memoized table — the whole point is to not recompute it per frame.
    field: IndexedFieldSource,

    /// The palette this field renders through. May be shared (Rc/Arc) with
    /// other IndexedField instances so a single palette rotation animates
    /// multiple regions in lockstep — the multi-region demoscene trick.
    palette: SharedPalette,
}

pub enum IndexedFieldSource {
    /// Precomputed table — fastest path, used by recipes that build the
    /// field at scene-load (rings, waves, fire-source-pattern).
    Static(Vec<Vec<u8>>),

    /// Closure or function pointer — used when the field cheaply computes
    /// per cell (small W×H, no need to memoize). Returns u8 in [0, palette_n).
    Procedural(Arc<dyn Fn(u16, u16, u16, u16) -> u8 + Send + Sync>),
}

pub type SharedPalette = Arc<RwLock<IndexedPalette>>;
```

`SharedPalette` is the seam that lets a single palette drive many fields (both `Light` consumers in §7 and unrelated effects in different layers) so per-frame rotation work stays `O(1)` regardless of how many regions are subscribing.

## 6. Recipe JSON shape

Two examples, both authorable today against existing `ColorRamp` / `ColorConfig` types.

**Example A — Concentric rings rendered as flowing water:**

```json
{
  "kind": "indexed_field",
  "pattern": {
    "kind": "rings",
    "center": "auto",
    "ring_step": 1.4,
    "aspect_correction": 2.2
  },
  "palette": {
    "from_ramp": {
      "stops": [
        { "position": 0.00, "color": { "rgb": [10, 40, 80] } },
        { "position": 0.33, "color": { "rgb": [40, 100, 180] } },
        { "position": 0.66, "color": { "rgb": [120, 200, 255] } },
        { "position": 1.00, "color": { "rgb": [40, 100, 180] } }
      ],
      "space": "rgb"
    },
    "n": 16
  },
  "rotation": {
    "speed": 12.0,
    "direction": 1,
    "binding": "palette_offset"
  }
}
```

**Example B — Plasma rendered through a fire palette:**

```json
{
  "kind": "indexed_field",
  "pattern": {
    "kind": "plasma",
    "frequencies": [0.30, 0.50, 0.20, 0.25]
  },
  "palette": {
    "from_ramp": "preset:fire",
    "n": 32
  },
  "rotation": { "speed": 8.0 }
}
```

The `pattern` block enumerates a small set of named fields (`rings`, `plasma`, `waves`, `bars`, `spiral`); each accepts a small parametric block. The `palette` block accepts either an inline ramp or a preset (the existing `ColorRamp::fire()`, `ColorRamp::ice()`, `ColorRamp::rainbow()` already qualify). The `rotation` block governs the per-frame offset advance and exposes a `binding` so other recipe parts can drive rotation explicitly.

## 7. Reuse story for existing tui-vfx shaders

This is where the win is concrete:

- **`cls_terminal_fire_shader.rs` (921 LOC)** — the existing fire effect computes per-cell color per frame from a fire-field signal. Re-expressed as an `IndexedField` with pattern `plasma` (or a fire-specific `flame_field`) plus a `from_ramp: "preset:fire"` palette, the per-frame work collapses to one offset increment and the geometry+color paths separate cleanly. Estimated post-migration LOC: ~80, with the existing fire palette reused verbatim through `ColorRamp::fire()`.
- **`cls_focus_field_shader.rs`** — reads `palette` already (per the grep in §4 prep). Migration is straightforward: the existing focus-field gradient becomes the `field`, the existing palette becomes an `IndexedPalette::from_ramp`, rotation is opt-in and defaults to 0 (static).
- **The Madeira flag's shading dispatcher** — currently `mode: lambert | …` with a hand-rolled light-direction signal. Adding `mode: indexed_palette` lets the flag's underlying brightness field render through a flag-themed palette and cycle for celebratory effects (e.g. wave animation that uses palette rotation rather than re-evaluating brightness per frame). Composes with the dynamic-light-shadow primitive: the same `Light` that drives the lambert mode's diffuse term can drive the field's index assignment.
- **Animated dividers, progress bars, banners** — many small recipes that today use `linear_gradient` with a moving offset can be re-expressed as `indexed_field { kind: bars }` with palette rotation. Identical visual; substantially cheaper at 60Hz, and authoring becomes "pick a palette" rather than "tune a gradient and animate the offset by hand."

The migration story is opt-in per shader, never breaking. The new primitive lives alongside the existing color-config types and consumes them; no in-flight recipes need a schema bump.

## 8. Runtime story

Per frame, the renderer:

1. Iterates the active scene's `IndexedField` shaders.
2. For each, consults the shared `IndexedPalette` (one `RwLock` read) and grabs `(entries, offset, cross_fade)`.
3. For each cell in the field's region, looks up `entries[(field[y][x] + offset) % N]`, optionally blending with the cross-fade target.
4. Hands the resolved `Color` into the existing compositor path — masks, filters, blends downstream are all unchanged.

Per-frame *animation* work for one field is a single `palette.rotate(steps)` call: one `u32` add and one modulo. Multiple fields sharing the palette all advance together for that one call. Recomputing the field is unnecessary except on resize or pattern parameter change.

Memory: `entries` is `N × Color` ≈ 16 × 4 bytes = 64 bytes per palette. The static field is `W × H × u8` ≈ 80 × 24 × 1 = 1.9 KB per region — substantially less than the per-frame Color buffer for the same region (4× larger), so the indexed approach reduces both compute and memory pressure for the per-cell-per-frame work.

## 9. Acceptance criteria

A reasonable v1 of this primitive delivers all of:

1. **Pure resolve.** `IndexedPalette::resolve(index) -> Color` performs no allocation, no I/O, and no math beyond integer mod, vec index, and (when `cross_fade.is_some()`) a single `blend_colors` call. Verifiable by `cargo bench` or no_std-style review.
2. **Both field sources verified.** Unit tests cover both `Static(Vec<Vec<u8>>)` and `Procedural(Fn)` field sources, with rotation offsets at 0, mid-cycle, and wraparound, plus a cross-fade test asserting the midpoint matches `blend_colors(a.resolve, b.resolve, 0.5)`.
3. **ColorRamp round-trip.** `IndexedPalette::from_ramp(&ramp, n).resolve(0)` equals `ramp.sample(0.0)` within rounding tolerance for the existing `ColorRamp::fire()`, `ice()`, `rainbow()` presets at n ∈ {8, 16, 32}.
4. **Lockstep rotation.** Two `IndexedField` instances sharing one `SharedPalette` produce visually-coherent animation: the test asserts that after `palette.rotate(k)`, both fields' resolved cells use the same `(field_idx + k) % N` lookup.
5. **Cheap.** `IndexedField` per-frame cost for an 80×24 region with N=16 benchmarks under 50µs on a representative dev machine — including the iteration, lookup, and color emit. The compositor path's per-frame ANSI emit dominates; the palette path adds noise-floor overhead.
6. **Recipe round-trip.** A recipe authored with the JSON shape in §6 loads, runs, and re-serializes byte-identically (modulo skipped runtime fields).
7. **Reference implementation parity.** A small example reproduces the runnable `palette-cycling-demo.py`'s three presets through the Rust path: same field shapes, same palette colors (within RGB rounding tolerance), same observable rotation behavior.

## 10. Open questions

1. **Single global palette registry, or per-recipe?** A global registry lets unrelated recipes share palettes (e.g. all "water" effects use the same palette and rotate together). Per-recipe avoids cross-coupling. Initial preference: per-recipe with explicit "palette ID" references, with a future global-registry overlay.
2. **Where does `IndexedField` live in the crate graph?** Options: (a) new module under `tui-vfx-style/src/models/`, mirroring the other `cls_*_shader.rs`; (b) new submodule `tui-vfx-style::palette` next to the gradient/ramp types. Initial preference: (b), since the primitive is reusable beyond the shader family.
3. **Should `cross_fade` accept more than two palettes?** A `Vec<(IndexedPalette, weight)>` general blend is more powerful but adds complexity. Two-target blend covers the high-value case (sunrise, sunset, mood transition). Initial preference: two-target in v1, generalize later if a consumer needs it.
4. **Index width — `u8` or `u16`?** `u8` covers N ≤ 256, which is generous for the demoscene-cycling use case (typical N ≤ 64). `u16` covers larger palettes for, e.g., 256-entry color tables ported from BBS-era ANSI art. Initial preference: `u8` in v1; revisit if a consumer needs more.
5. **Static vs. procedural field as separate types or a single enum?** The current draft uses `IndexedFieldSource::{Static, Procedural}`. A type-erased `dyn FieldSource` trait is more idiomatic Rust; the enum gives slightly tighter codegen. Decided in implementation.
6. **Should the rotation offset be `f32` for sub-step interpolation?** A fractional offset (e.g. `offset = 3.7`) lets the renderer blend `entries[3]` and `entries[4]` for genuinely smooth rotation. Costs one extra `blend_colors` per cell but eliminates the visible step at low FPS or large N. Worth pre-evaluating: on benchmark runs, does the smoothness gain justify the cost?
7. **Migration story for `cls_terminal_fire_shader.rs`.** Hard cutover (rewrite as `IndexedField`), soft (accept both shapes for one release), or gated (recipe schema bump)? Ties to the broader §11 question of shader-by-shader migration.

## 11. Future extensions (deferred)

- **Sub-cell offset + temporal dithering.** Combine an `f32` rotation offset (§10.6) with the temporal-dithering pattern from `historical-graphics-techniques-addendum.md` §1.3 to fake intermediate palette colors between exact entries. Potentially worth one effective bit of perceived color depth.
- **`LayeredPalette`.** A composition primitive that blends two `IndexedField` outputs with a per-cell mask, for layered effects (e.g. fire on the bottom rows, smoke palette on the top, blended through a vertical mask).
- **`PaletteSwap` transition.** A scene-level transition primitive that cross-fades all `IndexedField` palettes in lockstep over N frames — Diablo-mode mood change in one declaration.
- **HSL/HSV-space rotation.** The current spec interpolates entries in whatever color space the source `ColorRamp` used. A future variant could hue-rotate the entries themselves at runtime — a different effect (the *colors* shift hue, not just their indices), and a long-time demoscene staple.
- **Field-of-fields ("nested cycling").** Two palettes both rotating, with the field selecting between them based on a third low-frequency field. Produces the "two flowing rivers in different colors with interaction" effect that defined late-90s demos. Feasible at ~2× current per-frame cost.
- **Stochastic dither at field boundaries.** Where two regions with different palettes meet, a per-cell jittered selection between them produces the ANSI-art stipple effect from `addendum.md` §1.1. Composes both ideas.

## 12. Relationship to other tui-vfx work

- **`ColorRamp` / `GradientLUT`** — `IndexedPalette::from_ramp` consumes these directly; the existing `fire()`, `ice()`, `rainbow()` presets become palette presets without new authoring.
- **`Sampler` trait** — orthogonal. Samplers handle coordinate displacement; `IndexedField` handles color binding. A spatial-distortion sampler can wrap an `IndexedField`'s output (or its source field) for "rippling palette-cycled water" effects.
- **The `dynamic-light-shadow-primitive-spec`** — the same `Light` that drives shadow projection can drive an `IndexedField`'s index assignment (e.g. cell index = quantized angular distance from the light's azimuth), letting the post-release Light primitive light up a palette-cycled scene through both shadow and color paths from a single source of truth.
- **The existing shader family (`cls_*_shader.rs`)** — overlap with several existing shaders (terminal_fire, focus_field, gradient rings, fault-line ripple). Migration is opt-in; per §7 the LOC reduction is substantial.
- **`historical-graphics-techniques-addendum.md` §1.2** — the brainstorm origin; this spec is the actionable form. The addendum's "high-leverage actions" item 2 ("Palette-index outputs from samplers + palette rotation") is what this spec implements.
- **`palette-cycling-demo.py`** — the runnable reference. Three pattern presets × three palettes × live key switching. Use as a visual-target acceptance test for §9.7.

## 13. Decision boundaries

This spec does **not** decide:

- The exact module path for `IndexedPalette` and `IndexedField` (§10.2).
- The `cross_fade` cardinality beyond two-target (§10.3).
- Whether the rotation offset is `u32` or `f32` in v1 (§10.6).
- Per-shader migration ordering (§10.7).
- Whether to expose a global palette registry in v1 or defer (§10.1).

These belong in the implementation plan that follows this spec.

## 14. Next steps

1. **Review this spec** alongside the dynamic-light-shadow primitive spec; the two share a common architectural pattern (small primitive + pure pure resolve function + recipe-JSON binding) and should validate each other.
2. **Resolve open questions** §10.1 (palette registry) and §10.6 (offset width) — they affect the public surface.
3. **Spike `IndexedPalette` + `from_ramp` in `tui-vfx-style`** behind a feature flag, with unit tests covering §9.1–§9.4. ~half-day task.
4. **Spike `IndexedField` consuming the spike palette**, paired with a single recipe (rings + water palette) that round-trips. ~half-day task.
5. **Migrate `cls_terminal_fire_shader.rs`** as the first proving consumer; verify visual parity against existing fire recipes before retiring the legacy code path.
6. **Author a small Rust example** that mirrors `palette-cycling-demo.py`'s three presets, for the §9.7 visual-parity check.

<!-- <FILE>docs/design/post-release/indexed-palette-cycling-spec.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.2.0-draft</VERS> -->
