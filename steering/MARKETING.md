<!-- <FILE>steering/MARKETING.md</FILE> - <DESC>Marketing positioning and feature hierarchy for tui-vfx — 30s/60s/90s descriptions, primary/secondary/tertiary feature layers, callouts for uniquely powerful capabilities. The project's north star for how we describe ourselves.</DESC> -->
<!-- <VERS>VERSION: 0.3.7</VERS> -->
<!-- <WCTX>Keep marketing positioning aligned with V3's scene-renderer, VFX compositor, and recipe-runtime scope without over-promising beyond implementation evidence.</WCTX> -->
<!-- <CLOG>0.3.7: record the suite-framing tension and gt-design-first product truth without turning it into a public marketing push.</CLOG> -->

# Marketing — tui-vfx positioning and feature hierarchy

## North star

**tui-vfx is a visual effects library for terminal user interfaces.**

The 90-second description below is canonical. The 30- and 60-second versions are compressions of it, useful at different moments (elevator ride, README opening, talk abstract). If the three descriptions ever disagree on fact or framing, the 90s wins and the shorter versions get re-derived.

### TBD positioning refinement

The current "visual effects library" framing undersells the V3 architecture. It
can cause readers — especially AI authors — to treat tui-vfx as a garnish layer
instead of a recipe-driven scene system. V3 is better introduced as a
Rust-native, high-performance scene renderer plus VFX composition plus a recipe
runtime/authoring contract, designed to sustain a 60 fps terminal animation
budget.

Candidate full framing:

> **tui-vfx is a high-performance Rust terminal scene renderer, VFX compositor,
> and recipe-driven animation runtime for grid-based UIs.**

Candidate shorter framing:

> **tui-vfx is a high-performance Rust scene renderer and VFX engine for
> terminal UIs, built for 60 fps recipe-driven animation.**

TBD: re-derive the 30s / 60s / 90s descriptions from this framing once the
pipeline audit confirms the exact V3 as-built capability boundary.

Why this matters operationally: the first sentence sets the reader's search
strategy. "Effects library" primes an author to look for post-processing calls
over existing content (`apply_effect(grid, params)`) and to stop once those APIs
are found. "Scene renderer, VFX compositor, and animation runtime" primes the
right questions: where is the scene graph, what is the composition model, how
does the animation clock work, how do recipes drive the renderer? It also makes
flagship recipes such as Madeira read as primary usage examples rather than
advanced optional references. For future AI-authoring sessions, lead with the
architectural identity before pointing at source or examples; it shapes the map
the model uses to read everything else.

Guardrail for comparisons: this repositioning must not imply that tui-vfx
vacates the effects/compositor lane. Projects such as tachyonfx are strongest
when described as Ratatui-native effects and animation libraries over rendered
buffers. tui-vfx is also feature-competitive in that effects pipeline space:
filters, masks, samplers, shaders, shadows, style/content effects, scope
targeting, sequencing, parallel composition, timing, and probe/trace evidence
are core product surfaces, not incidental garnish. The distinction is that
tui-vfx contains that lane inside a larger recipe-driven scene renderer and
animation runtime. A fair contrast is **shared effects/composition overlap plus
tui-vfx's broader scene/runtime architecture**, not "tachyonfx does effects;
tui-vfx does scenes." They may stack in some applications, but tui-vfx should be
positioned as capable of owning the effects/compositor role itself.

Suite-framing tension: tui-vfx does not split cleanly along the same lines as
many adjacent crates. One honest mental model is a small suite:

1. scene renderer
2. VFX/effects compositor
3. recipe runtime and authoring/validation system

Each piece should be strong enough to stand in its lane; together they form the
ecosystem gt-design needs for advanced experiences. Avoid overcomplicating the
public story, though. The primary product truth is pragmatic: tui-vfx exists to
serve gt-design's palette, theme, motion, animation, and VFX needs. It is open,
documented, extensible, and ecosystem-agnostic because those qualities make the
architecture better and may help others, not because the project needs a broad
marketing campaign. The strongest public demonstration will be gt-design using
tui-vfx as the cornerstone for polished terminal experiences.

---

## The 30-second description

tui-vfx is a visual effects library for terminal UIs — shadows, gradients, motion along paths, multi-layer composition, all declarative. You author effects as JSON recipes; the engine chains them into tree pipelines and targets anything from a full rect down to a single cell, with phase-aware lifecycles (enter / dwell / exit) at 60 fps. The result is terminal applications that feel as polished as modern GUI apps, without leaving the cell grid.

**As bullets:**

- Visual effects library for terminal UIs — shadows, gradients, paths, multi-layer composition.
- Declarative JSON recipes, not imperative render code.
- Effects chain into tree pipelines (sequence + parallel).
- Targets anything from the full grid down to a single cell.
- Phase-aware lifecycles (enter / dwell / exit) at a 60 fps budget.
- Grid-first: works with ratatui; ratatui isn't a dependency of the compositor.

---

## The 60-second description

*Adds: audience, problem it solves, core benefits.*

tui-vfx is a visual effects library for terminal UIs. It brings shader-like capabilities — shadows, gradients, masks, signal-driven motion along composable paths, multi-layer composition — to ratatui and other grid-based renderers.

The compositor sits between your layout pass and the terminal render: cells in, cells out. You author effects as declarative JSON recipes rather than imperative rendering code. The engine chains effects into tree pipelines (sequence and parallel), composes them per-cell with phase-aware lifecycles, and targets anything from the full grid down to a single cell (`Cell(x, y)`, `RowRange`, `Rect`, theme role, channel, glyph match, and algebraic composition of those). Parameters accept constants, app-supplied runtime bindings, or composed signal graphs. Effects like shadows, glows, braille-supersampled images, scene-layer composition, and multi-pass offscreen rendering become declarative library primitives instead of ad-hoc per-application code — rendered against a 60 fps / 16.7 ms frame budget target.

It's built for teams shipping polished terminal applications — design systems (gt-design is the first major consumer), splash and transition surfaces, dashboards and data visualizations with motion, training demos, games, and any tool where terminal UI fidelity matters. Recipe authoring separates visual design from application code: designers iterate in JSON and preview; developers wire recipes into widgets.

**As bullets:**

- **What.** Visual effects library for terminal UIs. Shader-like capabilities — shadows, gradients, masks, signal-driven motion, multi-layer composition — over ratatui and other grid-based renderers.
- **Where it sits.** Compositor lives between your layout pass and the terminal render: cells in, cells out.
- **Authoring.** Declarative JSON recipes, not imperative rendering code. Designers iterate in JSON; developers wire recipes into widgets.
- **Composition.** Effects chain into tree pipelines (sequence + parallel). Parameters accept constants, runtime bindings from app state, or composed signal graphs from `mixed-signals`.
- **Targeting.** Full grid, rect, row range, column range, channel, content, theme role, glyph match, single cell. First-class primitives at every granularity, composable with `And` / `Or` / `Not`.
- **Shipped primitives.** Shadows, glows, braille-supersampled images, scene-layer composition, multi-pass offscreen rendering.
- **Performance.** 60 fps / 16.7 ms frame budget as a release-gate criterion, enforced by CI bench.
- **For.** Design systems, splash and transition surfaces, dashboards with motion, training demos, games, any surface where terminal-UI fidelity matters.

---

## The 90-second description

*Adds: grid-first architecture, AI-authoring focus, crate family.*

tui-vfx is a visual effects library for terminal UIs. It brings shader-like capabilities — shadows, gradients, masks, signal-driven motion along composable paths, multi-layer composition — to ratatui and other grid-based renderers.

The compositor sits between your layout pass and the terminal render: cells in, cells out. You author effects as declarative JSON recipes. Effects chain into tree pipelines (sequence and parallel) where one step's output can feed another via named hints (`displacement`, `sampled_color`, `cell_density`, `alpha_mask`). The engine composes recipes per-cell with phase-aware lifecycles, targets anything from the full grid down to a single cell, and evaluates parameters as constants, runtime-bound values, or signal graphs. Shadows, glows, braille-supersampled images, scene-layer composition, and multi-pass offscreen rendering become declarative primitives instead of per-application code, rendered against a 60 fps / 16.7 ms frame budget target.

**Grid-first, ecosystem-agnostic architecture.** The compositor renders to an abstract cell grid; ratatui is *a* consumer, not *the* consumer. This makes plausible adjacent uses other terminal libraries rule out by construction — movie composers that render recipe timelines without a widget loop, static exporters (SVG / SIXEL / PNG), wasm-embedded terminal demos, CI visual regression via grid diffs.

**Recipes designed for AI-assisted authoring at scale.** Schema regularity, bounded scope vocabularies, and explicit contracts are deliberate choices so that AI-assisted authoring is reliable at library scale. A growing recipe corpus (500+ files, curated under a William Morris "useful or beautiful" filter for V3) plus on-disk authoring guides plus a validator with contract discovery make AI-generated recipes verifiable.

Built for teams shipping polished terminal applications — design systems (gt-design is the first major consumer), splash and transition surfaces, data viz with motion, training demos, games, any tool where terminal UI fidelity matters. Ships as a family of crates (`tui-vfx`, `-recipes`, `-compositor`, `-shadow`, `-style`, `-types`, `-trace`, and siblings), each sized to its responsibility.

**As bullets:**

- **What.** Visual effects library for terminal UIs. Shader-like capabilities over grid-based renderers.
- **Where it sits.** Compositor between layout and terminal render; cells in, cells out.
- **Authoring.** Declarative JSON recipes, not code. Designed for human and AI authoring as co-equal primary audiences.
- **Composition at three levels (V3).**
  - Signals compose into signal graphs via `mixed-signals`.
  - Signal graphs flow into any step parameter (`ParamValue<T>`).
  - Steps feed per-cell data to other steps by named hint (`HintRef<T>`).
  - Every field accepts `StepInput<T> = ParamValue<T> | HintRef<T>`.
- **Targeting.** First-class scope primitives from whole grid to single cell: `All`, `Rect`, `RowRange`, `Cell(x, y)`, `Role("primary")`, plus `Channel` / `Content` / `GlyphMatches` / `Predicate`, composable with `And` / `Or` / `Not`.
- **Motion.** Nine-path library (linear, arc, bezier, spiral, spring, squash, rectilinear, hover, step) × any easing from `mixed-signals`. Paths and easings compose through the signal graph.
- **Scene composition.** Scene layers with their own content source (text, braille image, procedural, card) and z-order. Per-layer pipelines (V3).
- **Grid-first.** Ratatui is one consumer, not the consumer. Architectural targets include movie player, static renderer, wasm embed, SIXEL / SVG export.
- **AI-ready authoring.** Schema regularity, bounded vocabularies, explicit contracts, on-disk authoring guides, validator with contract discovery. 500+ recipe corpus curated under a William Morris "useful or beautiful" filter for V3.
- **Crate family.** `tui-vfx`, `-recipes`, `-compositor`, `-shadow`, `-style`, `-types`, `-content`, `-geometry`, `-probe`, `-trace`, `-debug`, `-core`. Consumers pick what they need.
- **Performance.** 60 fps / 16.7 ms frame budget enforced by `bench_full_trace_60fps` in CI.
- **Observability.** Probe + trace + fingerprint drift detection at every pipeline stage.
- **For.** Design systems, splash / transition surfaces, dashboards with motion, training demos, games, multi-surface products where recipe authoring is separate from application code.
- **Status.** Pre-1.0; schema stabilizes at V3, currently in planning (see `docs/design/tui-vfx-v3-upgrade-plan/`).

---

## Feature hierarchy

### Primary — what tui-vfx is fundamentally about

1. **Recipe-based declarative authoring.** Design intent lives in JSON, separate from application code.
2. **Precision targeting from whole grid to a single cell.** Every step carries a `Scope` — a closed algebraic type with first-class primitives at every granularity. Area: `All`, `Rect`, `RectExclude`, `Outer(margins)`, `Inner(margins)`, `Border`. Line / column: `RowRange(start, end)`, `Rows([0, 1, 7])`, `ColumnRange`, `Columns([...])`. Individual cells: `Cell(x, y)` for exactly one cell, `Cells([{x, y}, ...])` for a set. Predicates: `Channel(Background | Foreground)`, `Content(Text | NonEmpty)`, `GlyphMatches(pattern)`, `Role("primary")`. Escape hatch: `Predicate(<registered-name>)`. Composition: `And` / `Or` / `Not` over any of the above. Cell coordinates accept runtime bindings, so `Cell(x: {binding: "hovered_col"}, y: {binding: "hovered_row"})` is a first-class, per-frame-updating single-cell target. Scope attaches uniformly to every step in the pipeline.
3. **Composition at three levels.** Signals compose into signal graphs via sibling `mixed-signals` (`Add(Sine, Ramp)`, `Multiply(wave, envelope)`). Signal graphs feed into any step parameter — filter strength, shader color, sampler amplitude, even `Cell(x, y)` coordinates — through a uniform `ParamValue<T>` type with three forms (`Constant`, `RuntimeBinding`, `SignalGraph`). Steps feed per-cell data to other steps by named hint (`displacement`, `sampled_color`, `cell_density`, `alpha_mask`), so a `DisplacementShade` reads whichever sampler in its layer emits `displacement` without knowing which one. Every parameter site accepts `StepInput<T> = ParamValue<T> | HintRef<T>` — constants, app state, signal graphs, and other steps' outputs all flow through the same field. Tree pipelines (`Sequence` and `Parallel`) propagate scope and phase to their children. (V3)
4. **Phase-aware lifecycles.** Effects carry `enter` / `dwell` / `exit` timing, composed into pipelines.
5. **Path-driven motion with composable easings.** Content moves along first-class paths — linear, arc, bezier, spiral, spring, squash, rectilinear, hover, step — parameterized by any easing (`EaseInOut`, `BackOut`, `Elastic`, damped springs, ADSR envelopes, custom cubic-bezier) supplied by sibling `mixed-signals`. Paths compose with the full signal library: `Multiply(arc, envelope)` damps the travel, `Add(bezier, noise)` roughs it up. Content-along-path is a primitive, not bespoke animation code.
6. **Multi-layer scene composition.** Scene layers carry their own content source (text, braille-supersampled image, procedural, card) and z-order, composited through the same pipeline primitives.
7. **Theme integration via semantic scopes.** Target cells by theme-resolved role (`"primary"`, `"surface"`) rather than raw colors; effects re-skin when themes switch.

### Secondary — what compounds with the primary set

8. **60 fps / 16.7 ms frame budget.** A release-gate criterion. The full-trace bench (80×24, 4 layers, full pipeline) targets ≤ 2 ms/frame at 60 fps; closed-vocabulary scope predicates cache as bitmasks; zero-allocation hot paths carry samplers and filters through the pipeline.
9. **Grid-first, ecosystem-agnostic architecture.** Not locked to ratatui. Targets include ratatui today, movie player, static renderer, wasm embed, SIXEL / SVG export.
10. **Shadow and offscreen composition.** Multi-pass rendering with offscreen buffers; shadows with depth-based intensity.
11. **Probe / trace observability.** Every pipeline stage is inspectable; probe fixtures diff rendered output across schema changes; trace events surface per-layer composition.
12. **Two-surface substitution API (V3).** Load-time `Substitutions` for text tokens, asset bytes, and one-shot structured values; per-frame `RuntimeBindings` for live app state.
13. **Per-layer pipelines (V3).** Scene layers can carry their own effect pipelines independent of the recipe-global pipeline.
14. **Named-factory + primitive composition.** Curated named shaders (`Diffusion`, `ConcealedLight`, `Glow`) coexist with primitive form (`ColoredOverlay + Pattern`); authors pick the level that fits.

### Tertiary — enabling technology and adjacent capabilities

15. **Braille supersampling (`.rsb` format).** 2×4 subcell density for image-like detail in a standard terminal.
16. **Font atlas format (`.rsf`).** Custom glyph sets for splash / display use without font-file dependencies.
17. **Asset byte-source loaders.** Recipes reference asset names; loaders consume bytes from filesystem, embedded resources, or any byte source.
18. **Fragment composition / `$use` primitive library (V3).** Shared recipe fragments reused across themes and recipes with preserved individual-item addressability.
19. **Validator with contract discovery.** What substitutions and bindings does this recipe require? Introspection APIs surface it for marketplaces, editors, generic players.
20. **Canonical upstream semantic seam (V3).** Single documented entry point from recipe JSON to playback item; downstream consumers wrap rather than reinterpret semantics.

---

## Unique-capability callouts

Capabilities that make tui-vfx distinctive. Each one is a candidate for a talk headline, a README hero section, or a comparison-table row.

### Braille supersampling at 2×4 density

A single terminal cell carries 2×4 = 8 braille dots. tui-vfx's `.rsb` image format stores a per-cell averaged RGB color plus a dot bitmask, letting images render at 2×4 the cell resolution — close to pixel-art density — inside a standard terminal. Combined with scene-layer composition and signal-driven motion, this enables waving logos, portrait splashes, and data visualizations that feel image-native rather than ASCII-art.

### Grid-first architecture

tui-vfx's compositor renders to an abstract cell grid and maps to ratatui at the final stage. The compositor has no ratatui dependency; a movie-player binary or a static exporter or a wasm embed can render recipes without pulling ratatui's widget / event-loop machinery. Ship static binaries at a few hundred KB instead of MB; enable adjacent uses (docs recordings, CI visual regression, SIXEL / SVG export) that a ratatui-only library rules out by construction.

### Recipe-based authoring designed for AI

Schema regularity, bounded scope vocabularies, explicit contracts, and proximity-weighted shape design are deliberate choices so that AI-assisted authoring at library scale (500+ recipes plus third-party extensions) is reliable. The recipe corpus is the teaching corpus; on-disk authoring guides, the validator's contract-discovery surface, and the recipe library together let AI authors generate recipes from design intent with verifiable correctness.

### Theme integration via semantic scopes

`Scope::Role("primary")` targets cells by theme-resolved semantic role, not RGB color. Effects written against role scopes re-skin automatically when themes switch, without per-recipe rewrites. Effects and themes compose because they share the same semantic vocabulary — which is what lets tui-vfx plug into a design system rather than standing alone.

### Precision targeting from whole grid to a single cell

Scope is a closed algebraic type with a uniform vocabulary that attaches to every pipeline step — masks, filters, samplers, shaders, content transformers. Every primitive below is a first-class variant, not a workaround composed out of something else:

- **Whole-region** — `All`, `Border`, `Outer(margins)`, `Inner(margins)`. Cover the full grid or a framing band in one statement.
- **Rectangular** — `Rect(x, y, w, h)`, `RectExclude(x, y, w, h)`. Any rectangle; exclude a rectangle from a larger scope.
- **Lines** — `RowRange(start, end)` for a contiguous line range, `Rows([0, 1, 7, 8])` for a non-contiguous set.
- **Columns** — `ColumnRange(start, end)`, `Columns([10, 27, 45])`. Same pair for columns.
- **Individual cells** — `Cell(x, y)` for exactly one cell (dedicated primitive, not a 1×1 rect); `Cells([{x, y}, ...])` for a list of individual cells in a single statement.
- **Channel** — `Channel(Background)`, `Channel(Foreground)`.
- **Content** — `Content(Text)`, `Content(NonEmpty)`, `GlyphMatches(pattern)`.
- **Theme role** — `Role("primary")`, `Role("surface")` — resolved at load time via the consumer's design system.
- **Custom** — `Predicate(<registered-name>)` as the explicit closed escape hatch for cell-level logic not expressible in the vocabulary.
- **Composition** — `And([...])`, `Or([...])`, `Not(...)` compose any of the above.

Cell coordinates accept `ParamValue<u16>` — raw integers *or* runtime bindings — so `Cell(x: {binding: "hovered_col"}, y: {binding: "hovered_row"})` is a first-class, per-frame-updating single-cell target driven by app state. Examples: `And([Channel(Background), Not(Content(Text))])` targets "background cells that aren't text"; `And([RowRange(5, 8), Role("surface")])` targets "surface-role cells in rows 5–7"; `Or([Cells([{x:3,y:3}, {x:5,y:3}, {x:7,y:3}]), Cell({binding: "cursor_x"}, {binding: "cursor_y"})])` targets three static cells plus one cell tracking the cursor.

The closed vocabulary means the compositor classifies scopes as static (cacheable as bitmasks per area) or dynamic (evaluated per-frame) and skips re-evaluation when a scope hasn't changed. Expressiveness and performance compound rather than trade off.

### Composition at three levels (V3)

Three independent compositional surfaces combine into one uniform authoring model.

**Level 1 — signals compose into signal graphs.** `mixed-signals` ships `Sine`, `Triangle`, `Ramp`, `Keyframes`, `Noise`, `SpatialNoise`, `ADSR`, `DampedSpring`, plus operators `Add`, `Multiply`, `Mix`, `Normalize`, `Remap`, `Clamp`. Any signal composes with any signal. `Multiply(Add(Sine(0.5Hz), Ramp), ADSR(300ms, 200ms, 0.7, 400ms))` is a full expression — a rising oscillation with a fade envelope.

**Level 2 — signal graphs flow into any step parameter.** Every typed parameter on every step is a `ParamValue<T>` with three forms: `Constant` (a literal), `RuntimeBinding` (app-supplied per frame), or `SignalGraph` (a `mixed-signals` expression). Uniform across filters, samplers, shaders, masks, style effects, and content transformers. Scope cell coordinates also accept `ParamValue<u16>`, so `Cell(x, y)` can track app state or a signal.

**Level 3 — steps feed per-cell data to other steps by name.** Steps declare named output hints (`displacement`, `sampled_color`, `cell_density`, `alpha_mask`) alongside their primary payload. Downstream steps bind to hints by name, not by reference to a specific upstream step. A `DisplacementShade` reads whichever sampler in its layer emits `displacement`. Swap the sampler; the shader keeps working.

**They compose because every field accepts either side.** `StepInput<T> = ParamValue<T> | HintRef<T>` at every parameter site. A shader's `intensity` can be a constant, a binding, a signal graph, or a hint from another step — all through the same field.

Concrete example — a waving flag with shading that follows the wave:

```json
{
  "step": {
    "kind": "sequence",
    "children": [
      { "kind": "sampler",
        "payload": {
          "type": "spatial_signal",
          "emits_hint": "displacement",
          "signal": { "kind": "multiply", "children": [
            { "kind": "sine", "spatial_frequency": {"x": 8, "y": 0},
                              "temporal_frequency_hz": 0.5 },
            { "kind": "sample_norm_x" }
          ]}
        } },
      { "kind": "shader",
        "payload": { "type": "displacement_shade",
                     "binds": { "displacement": "displacement" } } }
    ]
  }
}
```

One signal graph (`Multiply(Sine, norm_x)`) drives both the geometry displacement and the shading. The shader doesn't know which sampler produced the hint; the sampler doesn't know the shader consumes it. Add a second shader downstream to shade differently, or swap `Sine` for `Perlin`, without touching the other step.

Level 1 is the signal library. Level 2 is uniform parameter access. Level 3 is inter-step data flow. Together they make composition the default authoring surface and remove the need for custom animation code in common cases.

### Content animated along composable paths

A first-class path library ships with the geometry crate: `Linear`, `Arc`, `Bezier`, `Spiral`, `Spring`, `Squash`, `Rectilinear`, `Hover`, `Step`. Any content — a toast, a card, a braille image, a single glyph — can travel along any path. Paths are parameterized by easings from sibling `mixed-signals`: `EaseInOut`, `BackOut`, `Elastic`, damped springs, ADSR envelopes, custom cubic-beziers.

Because paths and easings are both consumable by the signal graph, they compose. `Multiply(bezier_path, fade_envelope)` damps the travel. `Add(arc_path, spatial_noise)` gives it texture. `Spring` with an overshoot coefficient lands with the right weight. A toast that arcs in on a spring, holds with a subtle hover, and exits along a squash path is declarative parameter composition, not a custom animation loop.

### 60 fps / 16.7 ms frame budget as a release gate

A representative frame — 80×24, four scene layers, full pipeline (sampler + mask + shader + filter + content transform) — targets ≤ 2 ms of trace-emission overhead at 60 fps. The `bench_full_trace_60fps` criterion bench enforces it in CI. Closed-vocabulary scope predicates cache as bitmasks to skip per-frame recomputation. Zero-allocation hot paths in samplers and filters preserve budget. Per-layer caching in the compositor (V3) prevents redundant work when only one layer's parameters change.

The 16.7 ms budget is the full per-frame target consumers allocate against; tui-vfx occupies the fraction of it where terminal rendering happens. The bench infrastructure makes regressions visible before they ship.

### Probe / trace observability

Every pipeline stage is inspectable. `pipeline-validator --probe` dumps per-step outputs (displacement hints, sampled colors, cell density maps) at named stages. `--debug-recipes-qc` fingerprints recipe renders for drift detection across schema changes. Trace events surface per-layer composition order. This infrastructure is what lets V3's release gate include rendering-equivalence checks (Concern F in the V3 upgrade plan).

---

## Where we sit in the ecosystem

tui-vfx isn't the only effects library for terminal UIs. The closest neighbor is [tachyonfx](https://github.com/junkdog/tachyonfx), which has been shipping since 2024 with real production adoption (~1.2k GitHub stars, 28 releases, v0.25 as of early 2026). If you're already using ratatui and want a focused effects pack to drop into an existing app, tachyonfx is a good fit: 50+ built-in effects, a Rust-syntax DSL with a browser-based live editor, and a clean "apply effects to a rendered buffer" mental model.

Design choices diverge from there:

- **Authoring model.** tachyonfx is Rust code plus a Rust-syntax DSL. tui-vfx is declarative JSON recipes. Data over code: inspectable, diffable, substitutable, AI-generatable, distributable without a compile step, validatable against a schema. The cost is a less Rust-native feel for code-first authors.
- **Targeting.** tachyonfx's `CellFilter` covers color (`FgColor`), content (`Text`), and margin (`Outer`). tui-vfx covers those plus `Cell(x, y)`, `Rows`, `RowRange`, `Columns`, `ColumnRange`, `Cells`, `Role("primary")` (theme-resolved semantic role), and algebraic composition (`And` / `Or` / `Not`). Semantic role targeting re-skins automatically when themes switch.
- **Animation.** tachyonfx uses per-effect timers with scalar interpolation. tui-vfx parameters bind to composed signal graphs from `mixed-signals` — `Add`, `Multiply`, `Mix`, plus physics primitives (`ADSR`, `DampedSpring`, spatial noise). Scalar tweens are a special case; the general case is an expression graph.
- **Motion.** tachyonfx has a `Motion` enum for directional sweeps and a `translate` effect. tui-vfx ships a path library (linear, arc, bezier, spiral, spring, squash, rectilinear, hover, step) that content can travel along, with easings from `mixed-signals` parameterizing each path.
- **Scope of the library.** tachyonfx is ratatui-native, single crate, focused on buffer-stage effects. tui-vfx is grid-first (ratatui is one consumer, not the consumer), ships as 12 crates sized to their responsibilities, and includes shadow/offscreen composition, scene layers with their own content sources, probe/trace observability, and a sibling recipe library designed for a 500+ recipe corpus.
- **Audience.** tachyonfx is built for a Rust developer adding effects to a ratatui app. tui-vfx is built for teams shipping a design system, a platform, or a multi-surface product where recipe authoring is a separate concern from application code — and for AI-assisted authoring at library scale.

**One-line version.** tachyonfx is the right choice for a ratatui-only app that wants a focused effects pack with a live DSL editor today. tui-vfx is the right choice for recipe-authored design systems, multi-surface products, AI-assisted authoring workflows, and future consumers beyond ratatui.

Our differentiators are load-bearing for those use cases, not universally better. Both libraries solve real problems.

**Credit where it's due.** Several tui-vfx architecture choices — closed-vocabulary scope for static/dynamic analyzer caching, `parallel` + `sequence` composition containers, a normalized internal form separate from the authoring surface — are shapes tachyonfx validated first. We adopted them deliberately after review.

**Status.** tui-vfx is pre-1.0. The recipe schema stabilizes at V3, currently in planning (see `docs/design/tui-vfx-v3-upgrade-plan/`). gt-design is the first production consumer. Features in this document annotated `(V3)` depend on the schema cutover.

---

## Notes on evolving this document

This document is a north-star positioning artifact. It evolves with the project.

- When strategic direction changes — new primary audience, new headline feature, architectural shift — the 90s description is updated *first*, then the 60s and 30s are re-derived from it, then callouts are refreshed.
- The feature hierarchy is the project's own judgment about what matters most right now; it is not a complete feature list. Features graduate from tertiary → secondary → primary when they earn that placement through real use; features also move down or drop off if they turn out to be less load-bearing than hoped.
- If anything in this document is out of date relative to the code, the code is the source of truth and this doc needs a revision pass.
- Companion doc: `steering/INTENTIONS.md` captures top-down project direction and decisions. Marketing answers *how we describe what we've built*; intentions answer *how we decide what to build*. The two should stay in sync.

<!-- <FILE>steering/MARKETING.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.3.2</VERS> -->
