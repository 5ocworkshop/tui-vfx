<!-- <FILE>steering/MARKETING.md</FILE> - <DESC>Marketing positioning and feature hierarchy for tui-vfx — 30s/60s/90s descriptions, primary/secondary/tertiary feature layers, callouts for uniquely powerful capabilities. The project's north star for how we describe ourselves.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Initial draft of MARKETING.md to establish positioning. First pass captures what tui-vfx is, who it's for, what matters most, and which capabilities genuinely distinguish it. Intended as a living document that evolves with the project; expected to iterate after review.</WCTX> -->
<!-- <CLOG>0.1.0: initial draft. 30s / 60s / 90s descriptions; primary / secondary / tertiary feature hierarchy; callouts for uniquely powerful capabilities. Authored as north-star alignment artifact, not final copy.</CLOG> -->

# Marketing — tui-vfx positioning and feature hierarchy

## North star

**tui-vfx is a visual effects library for terminal user interfaces.**

The 90-second description below is canonical. The 30- and 60-second versions are compressions of it, useful at different moments (elevator ride, README opening, talk abstract). If the three descriptions ever disagree on fact or framing, the 90s wins and the shorter versions get re-derived.

---

## The 30-second description

tui-vfx is a visual effects library for terminal UIs — shadows, gradients, motion, multi-layer composition, all declarative. You author effects as JSON recipes; the engine composes them per-cell with phase-aware lifecycles (enter / dwell / exit). The result is terminal applications that feel as polished as modern GUI apps, without leaving the cell grid.

---

## The 60-second description

*Adds: audience, problem it solves, core benefits.*

tui-vfx is a visual effects library for terminal UIs. It brings shader-like capabilities — shadows, gradients, masks, signal-driven motion, multi-layer composition — to ratatui and other grid-based renderers.

You author effects as declarative JSON recipes rather than imperative rendering code. The engine composes recipes per-cell with phase-aware lifecycles, smooth signal-driven parameters, and theme-aware semantic targeting. Effects like shadows, glows, braille-supersampled images, scene-layer composition, and multi-pass offscreen rendering become declarative library primitives instead of ad-hoc per-application code.

It's built for teams shipping polished terminal applications — design systems (gt-design is the first major consumer), splash and transition surfaces, dashboards and data visualizations with motion, training demos, games, and any tool where terminal UI fidelity matters. Recipe authoring separates visual design from application code: designers iterate in JSON and preview; developers wire recipes into widgets.

---

## The 90-second description

*Adds: grid-first architecture, AI-authoring focus, crate family.*

tui-vfx is a visual effects library for terminal UIs. It brings shader-like capabilities — shadows, gradients, masks, signal-driven motion, multi-layer composition — to ratatui and other grid-based renderers.

You author effects as declarative JSON recipes. The engine composes them per-cell with phase-aware lifecycles, smooth signal-driven parameters, and theme-aware semantic targeting. Shadows, glows, braille-supersampled images, scene-layer composition, and multi-pass offscreen rendering become declarative primitives instead of per-application code.

**Grid-first, ecosystem-agnostic architecture.** The compositor renders to an abstract cell grid; ratatui is *a* consumer, not *the* consumer. This makes plausible adjacent uses other terminal libraries rule out by construction — movie composers that render recipe timelines without a widget loop, static exporters (SVG / SIXEL / PNG), wasm-embedded terminal demos, CI visual regression via grid diffs.

**Recipes designed for AI-assisted authoring at scale.** Schema regularity, bounded scope vocabularies, and explicit contracts are deliberate choices so that AI-assisted authoring is reliable at library scale. A growing recipe corpus (500+ files, curated under a William Morris "useful or beautiful" filter for V3) plus on-disk authoring guides plus a validator with contract discovery make AI-generated recipes verifiable.

Built for teams shipping polished terminal applications — design systems (gt-design is the first major consumer), splash and transition surfaces, data viz with motion, training demos, games, any tool where terminal UI fidelity matters. Ships as a family of crates (`tui-vfx`, `-recipes`, `-compositor`, `-shadow`, `-style`, `-types`, `-trace`, and siblings), each sized to its responsibility.

---

## Feature hierarchy

### Primary — what tui-vfx is fundamentally about

1. **Recipe-based declarative authoring.** Design intent lives in JSON, separate from application code.
2. **Per-cell compositing with scope algebra.** Target cells by area, channel, content, theme-role, or custom predicate; compose predicates with `And` / `Or` / `Not`.
3. **Phase-aware lifecycles.** Effects carry `enter` / `dwell` / `exit` timing, composed into pipelines.
4. **Signal-driven parameters.** Constants, app-supplied runtime values, or composed signal graphs (sine, triangle, keyframes, spring, noise — via sibling `mixed-signals`).
5. **Multi-layer scene composition.** Scene layers carry their own content source (text, braille-supersampled image, procedural, card) and z-order, composited through the same pipeline primitives.
6. **Theme integration via semantic scopes.** Target cells by theme-resolved role (`"primary"`, `"surface"`) rather than raw colors; effects re-skin when themes switch.

### Secondary — what compounds with the primary set

7. **Grid-first, ecosystem-agnostic architecture.** Not locked to ratatui. Future-proofed for movie player, static renderer, wasm embed, SIXEL / SVG export.
8. **Shadow and offscreen composition.** Multi-pass rendering with proper offscreen buffers; shadows with depth-based intensity.
9. **Probe / trace observability.** Every pipeline stage is inspectable; probe fixtures diff rendered output across schema changes; trace events surface per-layer composition.
10. **Two-surface substitution API (V3).** Load-time `Substitutions` for text tokens, asset bytes, and one-shot structured values; per-frame `RuntimeBindings` for live app state.
11. **Per-layer pipelines (V3).** Scene layers can carry their own effect pipelines independent of the recipe-global pipeline.
12. **Named-factory + primitive composition.** Curated named shaders (`Diffusion`, `ConcealedLight`, `Glow`) coexist with primitive form (`ColoredOverlay + Pattern`); authors pick the level that fits.

### Tertiary — enabling technology and adjacent capabilities

13. **Braille supersampling (`.rsb` format).** 2×4 subcell density for image-like detail in a standard terminal.
14. **Font atlas format (`.rsf`).** Custom glyph sets for splash / display use without font-file dependencies.
15. **Asset byte-source loaders.** Recipes reference asset names; loaders consume bytes from filesystem, embedded resources, or any byte source.
16. **Fragment composition / `$use` primitive library (V3).** Shared recipe fragments reused across themes and recipes with preserved individual-item addressability.
17. **Validator with contract discovery.** What substitutions and bindings does this recipe require? Introspection APIs surface it for marketplaces, editors, generic players.
18. **Canonical upstream semantic seam (V3).** Single documented entry point from recipe JSON to playback item; downstream consumers wrap rather than reinterpret semantics.

---

## Unique-capability callouts

These are the capabilities that materially distinguish tui-vfx from what else is available to terminal-UI developers today. Each one is a candidate headline in a talk, a README hero section, or a comparison-table row.

### Braille supersampling at 2×4 density

A single terminal cell carries 2×4 = 8 braille dots. tui-vfx's `.rsb` image format stores a per-cell averaged RGB color plus a dot bitmask, letting images render at 2×4 the cell resolution — close to pixel-art density — inside a standard terminal. Combined with scene-layer composition and signal-driven motion, this enables waving logos, portrait splashes, and data visualizations that feel image-native rather than ASCII-art.

### Signal-driven animation via mixed-signals

Most terminal libraries that support animation give you tweens. tui-vfx parameters can bind to composed signal graphs from the sibling `mixed-signals` library — sines, triangles, keyframes, ADSR envelopes, damped springs, spatial noise, and any composition of the above. `Add(Sine(0.5Hz), Ramp)` produces a rising oscillation; `Multiply(wave, envelope)` lets animations fade in and out coherently. Nothing else in the terminal-UI ecosystem has this.

### Grid-first architecture

tui-vfx's compositor renders to an abstract cell grid and maps to ratatui at the final stage. The compositor has no ratatui dependency; a movie-player binary or a static exporter or a wasm embed can render recipes without pulling ratatui's widget / event-loop machinery. Ship static binaries at a few hundred KB instead of MB; enable adjacent uses (docs recordings, CI visual regression, SIXEL / SVG export) that a ratatui-only library rules out by construction.

### Recipe-based authoring designed for AI

Schema regularity, bounded scope vocabularies, explicit contracts, and proximity-weighted shape design are deliberate choices so that AI-assisted authoring at library scale (500+ recipes plus third-party extensions) is reliable. The recipe corpus is the teaching corpus; on-disk authoring guides, the validator's contract-discovery surface, and the recipe library together let AI authors generate recipes from design intent with verifiable correctness. This is not an afterthought — it's the primary composition pathway the library is optimized for.

### Theme integration via semantic scopes

`Scope::ThemeRole("primary")` targets cells by theme-resolved semantic role, not RGB color. Effects written against theme-role scopes re-skin automatically when themes switch, without per-recipe rewrites. This is the feature that makes tui-vfx a design-system *primitive* rather than just a rendering library — effects and themes compose because they share the same semantic vocabulary.

### Per-cell scope algebra

`Scope::And([Channel::Background, Not(Content::Text)])` targets "background cells that aren't text." Full `And` / `Or` / `Not` composition over a closed algebraic scope vocabulary gives surgical control. The closed vocabulary means the compositor can cache static-scope predicates as bitmasks and skip dynamic evaluation per frame — performance and expressiveness both.

### Probe / trace observability

Every pipeline stage is inspectable. `pipeline-validator --probe` dumps per-step outputs (displacement hints, sampled colors, cell density maps) at named stages; `--debug-recipes-qc` fingerprints recipe renders for drift detection across schema changes; trace events surface per-layer composition order. This is the infrastructure that makes V3's release gate (Concern F in the V3 upgrade plan) include rendering-equivalence checks — the library already instruments itself for debuggability.

---

## Notes on evolving this document

This document is a north-star positioning artifact. It evolves with the project.

- When strategic direction changes — new primary audience, new headline feature, architectural shift — the 90s description is updated *first*, then the 60s and 30s are re-derived from it, then callouts are refreshed.
- The feature hierarchy is the project's own judgment about what matters most right now; it is not a complete feature list. Features graduate from tertiary → secondary → primary when they earn that placement through real use; features also move down or drop off if they turn out to be less load-bearing than hoped.
- If anything in this document is out of date relative to the code, the code is the source of truth and this doc needs a revision pass.
- Companion doc: `steering/INTENTIONS.md` captures top-down project direction and decisions. Marketing answers *how we describe what we've built*; intentions answer *how we decide what to build*. The two should stay in sync.

<!-- <FILE>steering/MARKETING.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
