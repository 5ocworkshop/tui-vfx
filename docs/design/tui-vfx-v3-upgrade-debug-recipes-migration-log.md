<!-- <FILE>docs/design/tui-vfx-v3-upgrade-debug-recipes-migration-log.md</FILE> - <DESC>Running decision log for the V2→V3 conceptual migration of tui-vfx-recipes/recipes/debug_recipes/**/*. Captures Workflow A classification per shader, metadata choices, and schema questions flagged for plan refinement. Conceptual only — V3 loader does not exist yet; these files will not validate at runtime.</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>All six migration stages complete plus final audit. 202 V3 debug recipes + 66 V3 wargames files + madeira-flag + extended barber_pole = 270 V3 conceptual files produced. Schema questions Q1–Q34 tracked with evolving draft resolutions; running drift summary + major gap list closes the migration exercise.</WCTX> -->
<!-- <CLOG>0.3.0: close the migration. Append Stage 3-6 classifications, Stage 4 madeira-flag recipe (Q30-Q33), Stage 5 wargames hierarchy (extends preserves), Stage 6 fractional barber_pole (Q34), and the comprehensive V2-vs-V3 schema coverage audit with the major gap list (motion_path, signal-graph shape).
0.2.0: absorb the schema-journal section with per-question draft resolutions and a running drift summary. Schema questions now carry current proposed resolution + revision history inline rather than just a status.
0.1.0: initial log; Stage 1 (Tier 1 shader) classifications and schema questions captured.</CLOG> -->

# V2 → V3 Debug-Recipes Migration — Decision Log

> **Status: draft, conceptual only.** The V3 loader does not exist. Migrated files match the proposed V3 shape but do not round-trip through any validator. This log exists so the user and a peer reviewer can see *why* each choice was made and *what* the exercise surfaced as a schema question.

## Scope

Source corpus: `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/**/*` (156 recipes across `shaders/`, `baseline.json`, `complex/`, `content/`, `filters/`, `masks/`, `samplers/`, `styles/`).

The debug corpus is deliberately chosen because every recipe is a **single-capability preview** with minimal envelope — the ideal surface to pressure-test schema decisions without being distracted by composition noise. Per Intention 51 / Principle 4, debug recipes stay as individual files throughout the migration; no consolidation mechanisms are applied here.

## Log-home note

The task assignment specified `docs/internal/plans/...-migration-log.md`. `plans/` is gitignored repo-wide (.gitignore line 13) so content there would not reach the peer reviewer. The V3 plan itself was relocated out of `plans/` into `docs/design/` for the same reason (commit 71350c4). This log lands in `docs/design/` to honor that precedent.

## Output directory structure — proposal

V3 Decision 2 splits the shader surface into two layers:

1. **Primitives** — `ColoredOverlay { color, pattern, intensity, scope }` (and sibling primitive shader kinds where needed). Default authoring surface.
2. **Named compositions** — Tier 1 Rust factories in `tui-vfx-style` (e.g. `Diffusion`, `ConcealedLight`) that earn their place by encoding design judgment. JSON surface: `{"type": "diffusion", ...}` — deserializer delegates to the factory.

Debug recipes exercise **both surfaces**; each surface earns a dedicated subdirectory so authors can tell at a glance which category a given example belongs to, without reading the JSON:

```
recipes/debug_recipes/shaders/
├── primitives/     # exercises primitive form (colored_overlay + pattern, gradient_overlay, etc.)
├── compositions/   # exercises library-stable named compositions (Tier 1 factories)
└── _DEPRECATED_<v2>.json   # V2 source, left in place per easing_family precedent
```

Rationale:

- Matches the authoring mental model the V3 plan introduces: *"is this a primitive, or a named composition?"*
- Preserves individual-item addressability (Intention 51) — each file is still one-preview-per-file; demo app pickers still walk the tree.
- Makes the Workflow A classification visible in the filesystem itself, which doubles as a reviewable artifact.
- Leaves `_DEPRECATED_<v2>.json` next to the migrated variants (not in an archive subtree) so file-picker users can see both versions during the cutover window — lighter-touch version of the `_DEPRECATED_easing_family.json` pattern.

## Global schema conventions adopted for this exercise

These are provisional — every one is up for plan refinement based on what the corpus surfaces.

| Concept | V2 form | V3 form used here | Rationale |
|---|---|---|---|
| Schema version | `"schema_version": 1` | `"schema_version": 3` | Clean break per Why Now |
| Base fg/bg style | `pipeline.style.base_style` | `config.base_style` (sibling of `config.pipeline`) | The `style` slot no longer exists in the tree schema; base glyph style is a recipe-level default, not a pipeline step |
| Enter/exit motion curve | `pipeline.enter`/`pipeline.exit` | `pipeline.timing: {enter_ms, exit_ms, enter_ease, exit_ease}` | Matches simple-fade-toast shape in plan §Shape sketches |
| Single step | `pipeline.{mask, sampler, filter, style}` slots | `pipeline.step: {kind, scope, phase, payload}` | Decision 3 |
| Multiple steps | multiple flat slots | `pipeline.step: {kind: "parallel" or "sequence", children: [...]}` | Decision 3 |
| `apply_to: "background"\|"foreground"\|"both"` | field on the shader | lifted to the step-level `scope: {kind: "channel", value: "..."}` or `{kind: "all"}` | Decision 1 (unified Scope) subsumes the V2 per-shader apply_to |
| Shader named factory | `{"type": "diffusion", ...}` (payload on style.spatial_shader) | same `{"type": "diffusion", ...}` inside the Step payload — preserved sugar for earned names | Decision 2 |
| Shader primitive form | (did not exist in V2) | `{"type": "colored_overlay", "pattern": {"kind": "...", ...}, "color": ..., "intensity": ...}` | Decision 2 |
| Multi-color gradient shader | `{"type": "linear_gradient", "gradient": {...}, "angle_deg": ...}` | `{"type": "gradient_overlay", "gradient": {...}, "angle_deg": ...}` as a sibling primitive to `colored_overlay` — see Schema Q6 | ColoredOverlay carries a single color; gradients are a separate shape |
| Continuous clock (loop) | `time: {loop: true, loop_period_ms}` | `clock: {loop: true, period_ms}` (tentative) — see Schema Q4 | Notification archaeology; name drop candidate |
| Runtime / signal modulation of params | shader-specific `mode: "breath"\|"drift"`, `drift_speed`, `pulse_speed` (encoded as flags on specific shaders) | `"intensity": {"signal": {"kind": "sine", "period_ms": N, "amplitude": A, "offset": O}}` — `ParamValue::SignalGraph` per Decision 6 — see Schema Q3 | Replaces per-shader mode flags with a uniform signal mechanism |

**Vocabulary holds (conservative):** `lifecycle.auto_dismiss_ms`, `layout.anchor` kept as-is for this exercise. They are on Open Q #15's rename shortlist but the plan defers the final call to Workflow C. Flagging them here (Schema Q7) rather than pre-empting a rename that's still open.

## Metadata block applied per recipe

Per Open Q #21, every migrated recipe carries:

```json
"metadata": {
  "aesthetic_tags": [...],         // "warm", "soft", "premium", etc.
  "mood": "...",                   // single word
  "related_themes": ["theme-neutral"],   // debug recipes are not theme-bound
  "use_cases": ["debug_preview", "primitive_reference" | "named_composition_reference"],
  "maturity_era": "mature",        // debug fixtures are later-era library instrumentation
  "authoring_notes": "...",        // short intent line
  "last_reviewed": "2026-04-21"
}
```

## Schema journal — draft resolutions, evolving

> Each entry is my current best guess, not final. When a resolution changes, I update *Current resolution* and add a dated entry to the *Revisions* sub-list below it with the reason, so the arc stays visible. Draft status uses `exploring` / `settling` / `settled` — nothing is `settled` during this exercise.

### Q1 — Where does `base_style` live in V3?

**Source pressure:** Stage 1 — every recipe carries fg/bg defaults that used to hang off `pipeline.style.base_style`. **Stage 3 complex/ recipes force a revision** — `complex_layered_shaders`, `complex_cellular_faultline`, `complex_crt_retro`, `complex_neon_barber`, `complex_diamond_highlight`, `complex_radar_ripple` all carry V2's `styles: [...]` array where **each scope gets its own base_style** (different fg/bg for border cells vs text cells vs specific rows/columns). A single `config.base_style` can't express this.

**Candidates:**
- (A) `config.base_style` sibling of `config.pipeline` — recipe-level default the pipeline operates on.
- (B) A first step in the pipeline tree with kind `StyleEffect` — uniform but makes "base" a weird first-step convention.
- (C) A field inside `RaSceneLayer` / scene layer — right for multi-layer recipes; awkward for single-surface ones.
- (D) Hybrid: `config.base_style` as the recipe-wide default AND `StyleEffect` steps can override per-scope at the head of the pipeline. The StyleEffect steps take scope like any other step.

**Current resolution (exploring, revised):** **(D) hybrid.** `config.base_style` carries the recipe-wide default; multi-scope recipes add `kind: "style_effect"` steps at the head of the pipeline tree with appropriate scope. The V2 multi-element `styles: [...]` array decomposes cleanly to parallel StyleEffect steps + parallel Shader steps sharing the same scopes.

**Revisions:**
- **2026-04-21 (Stage 3 open):** originally (A) only; revised to (D) hybrid after complex/ recipes demonstrated that V2's `styles: [...]` carried per-scope base styles that (A) alone can't express. The sibling + StyleEffect-step hybrid covers both the single-scope (most recipes) and multi-scope (complex recipes) cases cleanly. Why: forcing every recipe into StyleEffect-at-head (option B) adds ceremony to the 90% single-scope case for no authoring gain; forcing `config.base_style` to grow into an array conflates two authoring concerns (default vs. per-scope override).

### Q2 — Enter/exit timing: recipe-level metadata or first-class steps?

**Source pressure:** Stage 1 — every recipe has an enter/exit motion envelope.

**Candidates:**
- (A) `pipeline.timing: {enter_ms, exit_ms, enter_ease, exit_ease}` at pipeline root — matches the simple-fade-toast shape in the plan.
- (B) First-class steps with `phase: enter/exit` carrying a motion-curve payload — uniform but verbose.

**Current resolution (exploring):** **(A) pipeline.timing as pipeline-level metadata.** The envelope is whole-recipe, not per-cell; a step is per-cell. Collapsing them would force every recipe to carry explicit enter/exit steps for the default curve. Holding until Stage 3 complex/ recipes surface a case where phase-scoped steps genuinely need the uniformity win.

**Revisions:** none yet.

### Q3 — `ParamValue::SignalGraph` JSON shape for scalar modulation

**Source pressure:** Stage 1 — `diffusion_breath` (sine on intensity), `concealed_light_drift` (sine on spread), `glow` pulse, `edge_sheen`. V2 encoded these as per-shader `mode: "breath"|"drift"` flags; Decision 6 lifts to uniform SignalGraph. Expected more pressure in Stage 2 bindings.

**Tension:** Decision 6 says V3 consumes `mixed-signals` rather than inventing its own signal surface. The JSON needs to deserialize into something mixed-signals already knows how to build.

**Candidates:**
- (A) Flat per-kind: `{"kind": "sine", "period_ms", "amplitude", "offset"}`.
- (B) Fully compositional tree: `{"kind": "add", "children": [...]}` always.
- (C) Hybrid: flat for canonical leaves, compositional fallback when the author wants Add/Multiply/etc.

**Current resolution (exploring):** **(C) hybrid, currently expressed as `{"signal": {"kind": "sine", "clock_ref": "config.clock", "amplitude", "offset"}}`.** Pure (B) blows up debug-recipe ergonomics; pure (A) can't express the flag-animation PRD's compound signals. (C) needs two refinements the Stage 1 shape doesn't pin down: **clock inheritance** (implicit by default) and **compositional reach** (scalar fields in a leaf signal can themselves be signal refs). Stage 2 binding recipes should force those.

**Revisions:** none yet.

### Q4 — V3 name for V2's continuous-clock envelope

**Source pressure:** Stage 1 — `time: {loop, loop_period_ms}` block at recipe-config level in 3 recipes.

**Candidates:** `clock`, `loop`, `continuous` (V2 archaeology), fold-into-signal.

**Current resolution (exploring):** **`clock: {loop, period_ms}`.** Named what it is (temporal basis); drops notification archaeology (Principle 3); preserves the "one clock, many signals phase-synced" case that fold-into-signal would lose. Signals can still carry their own period when they want different cycles — the clock is the default temporal basis, not the only one.

**Known friction:** if mixed-signals already names this concept (`TimeBase`, `Phase`), align with that vocabulary instead of inventing new.

**Revisions:** none yet.

### Q5 — Multi-edge selector shape in `Pattern::EdgeShadow`

**Source pressure:** Stage 1 — `shader_ambient_occlusion`, V2 `edges: "bottom_right"` (discriminated-string enum spanning single + compound values).

**Candidates:** (A) list `["bottom", "right"]`, (B) combined-enum string, (C) bitmask int.

**Current resolution (exploring):** **(A) list form.** Readable; composes with V3 unified-Scope list idioms; extensible beyond V2's 9-value enum without combinatorial explosion (4-edge combinations already blow up the V2 enum when you need 3-of-4 sets).

**Revisions:** none yet.

### Q6 — `ColoredOverlay` vs `GradientOverlay`: one primitive or two?

**Source pressure:** Stage 1 — `shader_linear_gradient` has multi-stop color data + angle, no single `color` field.

**Candidates:** (A) two sibling primitives, (B) one with `color: ParamValue<Color>` that can be a gradient sampler, (C) no overlay wrapper — `linear_gradient` is its own top-level primitive kind.

**Current resolution (exploring):** **(A) two sibling primitives — `colored_overlay` and `gradient_overlay`.** Single-color + spatial pattern vs multi-stop color + direction are orthogonal dimensions; collapsing them entangles the dimensions.

**Known friction:** later stages may surface shaders that want both (radial diffusion whose color is itself a gradient). Those need either a third primitive or a generalization. Holding (A) until Stage 3 surfaces the case.

**Revisions:** none yet.

### Q7 — Notification-era vocabulary (`auto_dismiss_ms`, `anchor`)

**Source pressure:** every Stage 1 recipe. Open Q #15 defers to Workflow C.

**Current resolution (exploring):** **Hold V2 spellings during this migration; sweep rename later.** Applying a provisional rename now would be re-done in the sweep. Better to migrate structure and sweep vocabulary in one atomic pass.

**Revisions:** none yet.

### Q8 — `Pattern::PerimeterHalo` — new variant or earned name?

**Source pressure:** Stage 1 — `shader_glow` predicted trivial but the plan doesn't enumerate a Pattern that covers it.

**Candidates:** (A) add `PerimeterHalo` to the Pattern catalog, (B) promote glow to `compositions/` as earned name, (C) unify with `EdgeShadow` via an `outward: true` flag.

**Current resolution (exploring):** **(A) new `PerimeterHalo` variant.** (C) entangles contact-shadow and bloom-halo which are semantically different. (B) fails the earn-its-place bar — glow is "halo around a thing," exactly what a generic Pattern is for.

**Revisions:** none yet.

### Q9 — Pattern-vs-overlay ownership of color-adjacent params

**Source pressure:** Stage 1 — `concealed_light` has `source_cutoff`, `edge_width` that look like light-model params.

**Current resolution (exploring):** **Not applicable to concealed_light** — classified earned-name, so the Tier 1 factory owns its full param set as an opaque payload. For primitive forms: **pattern carries its spatial params only; overlay carries color + intensity + scope.** Clean separation — overlay is always "a color at intensity over scope"; pattern is always "this cell's weight".

**Revisions:** none yet.

### Q10 — Scope-all and dual-channel tinting for `ColoredOverlay`

**Source pressure:** Stage 1 — V2 `apply_to: "both"`.

**Candidates:** (A) scope is sufficient (channel is encoded in scope or omission means all channels), (B) scope + separate `target_channels` orthogonal axes.

**Current resolution (exploring):** **(A) scope is sufficient.** V2 `apply_to` was always a channel selector, not a cell selector; V3 unified Scope covers the same vocabulary without a second axis. (B)'s extra flexibility ("these cells, but only FG") is a case V2 doesn't exercise.

**Known friction:** a later recipe that wants "scope to text cells, but tint only their BG behind the glyph" would force (B). Holding (A).

**Revisions:** none yet.

### Q11 — RuntimeBinding vs SignalGraph at the JSON surface

**Source pressure:** Stage 2 forced the concrete shape across 13 binding-variant recipes.

**Current resolution (exploring):** **`{"binding": "<name>", "default": <T>}` and `{"signal": {...}}` as the two variants of `ParamValue<T>` besides the constant scalar form.** The `default` field preserves V2's offline-validator fallback behavior; without it, every binding recipe fails offline validation — a pointless regression. The binding name is a free string resolved at runtime from the app's `Substitutions`-equivalent context.

**Known friction:** if V3 wants bounds enforcement (e.g., glisten_band.speed clamps to 0.1..=10.0), bounds could go on ParamValue (`{"binding": "...", "default": 1.0, "min": 0.1, "max": 10.0}`). Probably better to leave bounds in the factory's post-resolution clamp — keeps ParamValue lean.

**Revisions:** none yet.

### Q12 — Enum-typed bindings

**Source pressure:** Stage 2 — `glisten_band.direction_binding` binds to a u16-coded enum (0=Forward, 1=Reverse, 2=PingPong); `highlighter.direction_binding` similar for its 6-direction enum.

**Candidates:** (A) app sends ordinal integer that the factory decodes; (B) app sends enum-variant string (`"forward"`, `"reverse"`, `"ping_pong"`) that the factory deserializes.

**Current resolution (exploring):** **(B) variant-string.** Self-documenting at the app boundary; V3 ParamValue<EnumType> deserializes the string into the right variant. V2's u16 ordinals worked because wire-format integers were cheaper, but V3's tree-schema audience includes AI-assisted authoring that benefits from readable values.

**Revisions:** none yet.

### Q13 — Tagged-union payloads inside named-factory shaders

**Source pressure:** Stage 2 — `focus_field.shape = "rect" | "ellipse"` discriminator, similarly `highlighter.mode`, `bevel.light_direction`.

**Current resolution (exploring):** **No special V3 handling needed.** The factory's Rust deserializer owns the tagged-union dispatch (via `#[serde(tag = "...")]` or equivalent). Intention 39 merge semantics apply at the factory-payload level automatically when `deny_unknown_fields` is set. V3 schema sees one factory; authors see multiple modes; no schema plumbing required.

**Revisions:** none yet.

### Q14 — Dual-color shaders

**Source pressure:** Stage 2 — `glisten_band.head` + `glisten_band.tail`; Stage 2 — `focused_row_gradient.bright_color` + `dim_color`.

**Candidates:** (A) two parallel ParamValue<Color> fields (current); (B) unify with a "color ramp" concept that also covers gradient_overlay.stops.

**Current resolution (exploring):** **(A) two parallel fields per factory.** No unification pressure until a future shader wants three-or-more color stops (rule of three not met). If a third emerges, reconsider.

**Revisions:** none yet.

### Q15 — Factory-internal scope sub-objects (`row_mask`, etc.)

**Source pressure:** Stage 2 — `highlighter.row_mask: { mode: "last_row" }` is scope-encoding inside the factory payload.

**Candidates:** (A) keep as factory-internal; (B) lift to step-level `scope: {kind: "row", selector: "last"}` and remove from the factory.

**Current resolution (exploring):** **(B) lift, eventually.** `row_mask` is encoding a Row variant of Scope inside the factory because V2's Scope primitive didn't cover rows. V3's unified Scope can and should cover row/column selection at the step level. Kept V2 shape in the migrated files because this is a structural change worth deciding in the plan, not applied piecemeal during mechanical migration. Flag for plan resolution: **Scope needs a Row/Column variant** — and when it does, `row_mask` can leave highlighter's payload.

**Revisions:** none yet.

### Q16 — Accessibility hints in factory payloads

**Source pressure:** Stage 2 — `highlighter.text_contrast: { mode: "preserve" }`.

**Current resolution (exploring):** **Stay in factory until rule-of-three hits.** Only highlighter exercises this. If a second and third shader want text-contrast policies, lift to step-level metadata (e.g., `step.accessibility_policy: {text_contrast: "preserve", reduced_motion: "preserve"}`). Coordinates with Open Q #18 (role tags) — role tags could carry accessibility intent for tooling dispatch.

**Revisions:** none yet.

### Q17 — Positional primitives and widget-layout coupling

**Source pressure:** Stage 2 — `cursor.primary.position`, `wayfinding_node.nodes[].{x,y}`, `trace_path.paths[].points[].{x,y}`.

**Current resolution (exploring):** **Positions stay widget-local in the JSON; runtime-binding handles dynamic cases.** If a cursor recipe is hosted in a widget whose layout changes, the author either (i) runtime-binds the positions via ParamValue, or (ii) authors a recipe that expects a specific layout. Both are legitimate. V3 doesn't need a "relative-to-widget-center" coord system because either widget-local or runtime-bound covers the cases.

**Known friction:** scene-layer composition (Decision 5) may introduce relative-to-layer coordinates; reconsider then.

**Revisions:** none yet.

### Q18 — Primitive payloads carrying internally-composed sub-animations

**Source pressure:** Stage 2 — `trace_path.paths[]` with per-path `delay` is a micro-composition inside one primitive.

**Candidates:** (A) keep the internal composition (factory owns the multi-path coordination); (B) split to parallel steps with phase-scoped delays.

**Current resolution (exploring):** **(A) factory owns internal composition.** The two paths share `tail_length`, `vertical_weight`, `color`, `speed` — splitting to parallel steps would duplicate all those params per step. The factory's compression of "N paths with staggered starts sharing all other params" is authoring-valuable.

**Known friction:** if per-path params become meaningful (each path wants its own color), (A) is no longer clearly better than (B). Reconsider when that case emerges.

**Revisions:** none yet.

### Q19 — Clock + bindings coexistence

**Source pressure:** Stage 2 — `focus_field_center_binding` has both `clock` (drives pulse) and bindings (center). `wayfinding_node_current_index_binding` same pattern.

**Current resolution (exploring):** **Coexist cleanly, no special treatment needed.** Clock is the default temporal basis for signals inside the recipe (Q4); bindings bypass the clock (they are resolved from app state, which has its own temporal characteristics). Orthogonal concerns.

**Revisions:** none yet.

### Q20 — *(folded into Q16)*

Accessibility hints / role routing — subsumed into Q16 treatment. Role tags (Open Q #18 in the plan) carry tooling-dispatch intent that overlaps with accessibility; the two should resolve together.

### Q21 — Offline-default for runtime bindings

**Source pressure:** Stage 2 — V2 pattern of static field + `_binding` field carried an implicit offline default (the static value).

**Current resolution (exploring):** **Include `default` in ParamValue::RuntimeBinding.** `{"binding": "name", "default": <T>}`. Offline validators render the default; runtime hosts override. Matches V2 behavior exactly and avoids making every binding recipe fail offline validation.

**Revisions:** none yet.

### Q23 — Scope primitive must cover V2's full `StyleRegion` vocabulary

**Source pressure:** Stage 3 complex/ recipes use V2 region forms that aren't enumerated in Decision 1: `BorderOnly`, `{"Cell": {x, y}}`, `{"Cells": [...]}`, `{"Rows": [...]}`, `{"RowRange": {start, end}}`, `{"Columns": [...]}`, `{"ColumnRange": {start, end}}`. Decision 1 lists only `All`, `Outer(margins)`, `Inner(margins)`, `Rect`, `RectExclude`, channel variants, content variants, `Role`, `Predicate`, `And/Or/Not`.

**Gap:** to port complex/ recipes cleanly, V3 Scope needs these additional variants — or needs to argue the existing vocabulary covers them (unlikely; row/column selectors are a categorically different selection axis from rect-area selectors).

**Current resolution (exploring):** **Extend Decision 1's Scope enum with axis-selector variants.** Proposed:
- `{kind: "border"}` — covers V2 `"BorderOnly"`. Distinct from `Outer(margins)` because "border" is the recipe's chrome border, not a generic outer margin.
- `{kind: "cell", x, y}` — single cell target.
- `{kind: "cells", cells: [{x,y},...]}` — list of cells.
- `{kind: "rows", rows: [0, 1, ...]}` — list of row indices.
- `{kind: "row_range", start, end}` — half-open row range.
- `{kind: "columns", columns: [0, 1, ...]}` — list of column indices.
- `{kind: "column_range", start, end}` — half-open column range.

These compose cleanly with the existing `And/Or/Not` combinators per Decision 1.

**Known friction:** the list-of-cells form for `Cells` overlaps semantically with an `Or` combinator of `Cell` variants. `{"kind": "cells", "cells": [...]}` is ergonomic for debug recipes but arguably redundant with `{"kind": "or", "children": [{kind: "cell", ...}, ...]}`. Kept the compact form because the V2 corpus uses it; the expanded Or form is always available for authors who need it.

**Revisions:** none yet.

### Q24 — Phase vocabulary includes `dwell` as a first-class phase

**Source pressure:** Stage 3 — `complex_cellular_faultline` has `pipeline.sampler.enter/dwell/exit` — a three-phase sampler. `complex_diamond_highlight` has a `dwell` mask. V3 phase vocabulary must include `dwell` alongside `enter` and `exit`.

**Current resolution (exploring):** **Phase vocabulary is `enter | dwell | exit | all`.** `all` is the default (step applies across all phases). `dwell` is the middle phase where the recipe is presented at rest — distinct from the continuous post-enter state because dwell is bounded by whatever ends the recipe (lifecycle.auto_dismiss or user dismissal). This matches V2's implicit phase vocabulary once you notice V2 uses the `dwell` key.

**Revisions:** none yet.

### Q25 — `config.content` effect home in V3

**Source pressure:** Stage 3 — `complex_full_pipeline`, `complex_cinematic_reveal`, `complex_crt_retro`, `complex_radar_ripple`, `complex_content_shader_combo`, `complex_diamond_highlight`, `complex_neon_barber`, `complex_cellular_faultline` all carry a `config.content: { mode, effect: {...} }` block that describes how the text renders (typewriter, scramble, marquee, split_flap, mirror).

**Candidates:**
- (A) Keep `config.content` as a sibling of `config.pipeline` — content effect is a source-authoring concern (how the message becomes cells); pipeline operates on those cells.
- (B) Lift content effect to a pipeline step with kind `content_effect` — uniform tree shape.
- (C) Absorb into Decision 5's scene-layer source taxonomy — a Text source carries its content effect as part of its source spec.

**Current resolution (exploring):** **(A) keep as sibling for root-level recipes; (C) per-layer for scene-layer recipes.** Root-level recipes have an implicit single-text-source-with-optional-effect; the content block is how the author configures that source. Scene-layer recipes (Decision 5) expand this to per-layer content per source.

**Known friction:** `content.mode` values (`enter_only`, `dwell_only`, `loop`, `all`) span phase-like semantics that the tree schema already handles via step phases. Reconsider after scene-layer recipes land — they may subsume this.

**Revisions:** none yet.

### Q26 — Sampler center: symbolic vs coordinate

**Source pressure:** Stage 3 — `complex_radar_ripple.sampler.enter.center: "center"` (symbolic string) vs `complex_radar_ripple.sampler.exit.center: {x: 30, y: 4}` (coordinate object).

**Current resolution (exploring):** **Tagged-union enum variant.** `center` can be `{kind: "center"}` (symbolic, widget-center-aware) or `{kind: "cell", x, y}` (coordinate-based). V2's bare-string form and bare-object form both become tagged objects in V3.

**Revisions:** none yet.

### Q27 — V2 `{r,g,b}` without `"type": "rgb"` discriminator

**Source pressure:** Stage 3 — multiple complex recipes have shader color fields as `{"r": 255, "g": 200, "b": 100}` without the `"type": "rgb"` tag (e.g., `complex_layered_shaders.styles[0].spatial_shader.color`, `complex_full_pipeline.glisten_band.head/tail`, many others).

**Cause:** V2 serde probably handles this via an untagged deserializer that falls back to RGB when fields match. Inconsistency within the V2 corpus.

**Current resolution (exploring):** **V3 normalizes to tagged form.** All color fields carry `{"type": "rgb", "r, g, b"}` explicitly. Migration consistently adds the tag. This is a cleanliness win — no more "is this a color or something else?" ambiguity at the deserializer.

**Revisions:** none yet.

### Q22 — Is `pipeline.step` optional for effect-free recipes?

**Source pressure:** Stage 3 — `baseline.json` has no pipeline effects; V2 used `{type: "none"}` sentinels everywhere.

**Candidates:** (A) `pipeline.step` optional (omit when no effects); (B) require explicit `{kind: "none"}` sentinel.

**Current resolution (exploring):** **(A) optional.** V2's `{type: "none"}` sentinels were a workaround for the flat schema's slot-presence requirement. V3's tree shape treats absence as first-class: no step means no cell-level operation, but the pipeline still has meaning via `timing`. Cleaner than ceremonial sentinels.

**Revisions:** none yet.

---

## Running schema drift summary

Deviations from the pristine V3 plan shape, with rationale. New drifts append here as they appear.

| # | Plan assumed | Actual used | Why |
|---|---|---|---|
| D1 | clock block name unspecified | `clock: {loop, period_ms}` | Needed a name; dropped `time` archaeology; preserves "one clock, many phase-synced signals" |
| D2 | SignalGraph JSON skin unspecified | `{"signal": {"kind": "sine", "clock_ref", "amplitude", "offset"}}` hybrid | Needed concrete shape to migrate breath/drift; will re-align with mixed-signals once its JSON surface is settled |
| D3 | gradient possibly a Pattern variant of ColoredOverlay | Sibling primitive `gradient_overlay` | Multi-stop color is orthogonal to single-color + spatial pattern |
| D4 | `Pattern::EdgeShadow` edge selector unspecified | list form `["bottom", "right"]` | Composes with V3 unified-Scope list idioms; extensible beyond V2's 9-value enum |
| D5 | `Pattern::PerimeterHalo` not in plan's named list | Proposed as new variant | Trivial-composition classification for glow needed a Pattern name |
| D6 | `config.base_style` not explicit in plan | Used as sibling of `config.pipeline` | Homeless after `pipeline.style` slot removal; recipe-level default is the natural home |
| D7 | `pipeline.timing` keys (`enter_ms`, `exit_ms`, `enter_ease`, `exit_ease`) not enumerated in plan | Used as shown | Matches simple-fade-toast sketch in plan; concrete key names needed for authoring |

---

## Stage 1 — Tier 1 shaders (calibration)

### Classification summary

| Source file | Classification | V3 destination | Rationale |
|---|---|---|---|
| `shader_diffusion.json` | Trivial composition | `primitives/shader_diffusion_center_bg.json` | `ColoredOverlay + Pattern::RadialFromCorner` — no earned tuning beyond defaults |
| `shader_diffusion_both.json` | Trivial composition | `primitives/shader_diffusion_top_left_all.json` | Same primitive; scope=all; variant just changes corner and channel scope |
| `shader_diffusion_foreground.json` | Trivial composition | `primitives/shader_diffusion_center_fg.json` | Same primitive; scope=foreground |
| `shader_diffusion_breath.json` | Trivial composition with signal | `primitives/shader_diffusion_breath.json` | Same primitive; `intensity` bound to a sine SignalGraph — this *is* the V3 replacement for V2's `mode: "breath"` flag. Surfaces Schema Q3 |
| `shader_concealed_light.json` | Earned name | `compositions/shader_concealed_light.json` | `source_cutoff + edge_width` combo encodes "concealed emitter" design judgment; the light emerging *past* the source edge (rather than at it) is the distinctive tuning worth a Tier 1 library factory |
| `shader_concealed_light_both.json` | Earned name | `compositions/shader_concealed_light_both.json` | Same factory; scope=all |
| `shader_concealed_light_drift.json` | Earned name + signal | `compositions/shader_concealed_light_drift.json` | Same factory; spread bound to a low-amplitude sine (replaces V2 `mode: "drift"` flag). Also surfaces Schema Q3 |
| `shader_concealed_light_foreground.json` | Earned name | `compositions/shader_concealed_light_foreground.json` | Same factory; scope=foreground |
| `shader_glow.json` | Trivial composition (tentative) | `primitives/shader_glow.json` | `ColoredOverlay + Pattern::PerimeterHalo { radius, falloff }` + sine-bound intensity for pulse. Surfaces Schema Q8 — depends on whether Pattern::PerimeterHalo is in the catalog |
| `shader_linear_gradient.json` | Primitive itself (sibling) | `primitives/shader_linear_gradient.json` | Uses `gradient_overlay` rather than `colored_overlay` — gradients are a separate primitive shape, not a Pattern over ColoredOverlay. Surfaces Schema Q6 |
| `shader_edge_sheen.json` | Earned name | `compositions/shader_edge_sheen.json` | Moving perimeter sheen with `corner_boost` is distinctive design judgment — not a generic Pattern. Belongs as a Tier 1 library factory |
| `shader_ambient_occlusion.json` | Trivial composition | `primitives/shader_ambient_occlusion.json` | `ColoredOverlay + Pattern::EdgeShadow { edges, radius, falloff }` with a black color — AO is the colored-overlay primitive with shadow color and edge-distance falloff. Surfaces Schema Q5 |

### Totals

- Tier 1 source files: 12
- Classified as primitives: 8
- Classified as earned names: 4
- New Pattern variants proposed: `RadialFromCorner`, `PerimeterHalo` (Q8), `EdgeShadow` (Q5)
- New primitive kinds proposed (sibling to `colored_overlay`): `gradient_overlay` (Q6)
- Schema questions surfaced: Q1–Q10

---

## Stage 2 — Tier 2 and Tier 3 shaders (remaining shaders)

### Classification summary

**Tier 2 (23 files, all earned-name → `compositions/`):**

| Shader family | Count | V3 destination | Rationale |
|---|---|---|---|
| `highlighter` (10 variants) | 10 | `compositions/` | Marker-style sweep with curated mode (fill/band), direction (6-way), soft_edge, blend_strength, row_mask, text_contrast, runtime bindings. Distinctive multi-mode design judgment that doesn't decompose to ColoredOverlay+Pattern cleanly. |
| `focus_field` (6 variants) | 6 | `compositions/` | Tagged-union shape discriminator (rect\|ellipse) + feather + optional pulse. Candidate for future primitive decomposition (Pattern::RectField, Pattern::EllipseField) — kept earned-name for now because the shared \"focus field\" vocabulary is authoring-helpful. |
| `border_sweep` (2 variants) | 2 | `compositions/` | Discrete bead of N cells travelling the perimeter. Candidate for Pattern::PerimeterSweep decomposition; earned-name for now because the bead-metaphor is a useful authoring concept. |
| `pulse_wave` (2 variants) | 2 | `compositions/` | Oscillating wave with frequency/wavelength/speed. Candidate for Pattern::TravellingWave decomposition; earned-name for now because the param tuning encodes \"what a pulse wave should feel like\" design judgment. |
| `glisten_band` (3 variants) | 3 | `compositions/` | Two-color (head + tail) diagonal sweep with angle_deg. The dual-color API is the distinctive design judgment — a generic single-color primitive can't express the head/tail contrast. |

**Tier 3 (18 files, mixed primitives and compositions):**

| Shader | Classification | V3 destination | Rationale |
|---|---|---|---|
| `radar` | Earned name | `compositions/shader_radar.json` | Polar-coordinate tail-length sweep. On the edge — could be Pattern::PolarSweep but the factory surface is cleaner. |
| `wayfinding_node` (2 variants) | Earned name | `compositions/shader_wayfinding_node*.json` | Node-list with three-state (previous/current/future) emphasis. List-of-things shader with per-node state policy — not decomposable to primitive. |
| `barber_pole` | Earned name | `compositions/shader_barber_pole.json` | Diagonal translating stripes. Candidate for Pattern::DiagonalStripes; earned-name because barber-pole metaphor = named \"indeterminate progress\" authoring intent. |
| `bevel` | Earned name | `compositions/shader_bevel.json` | light_direction → edge-pair lookup compression. Decomposes to parallel ColoredOverlay+EdgeShadow but the compression is earned. |
| `reflect` | Earned name | `compositions/shader_reflect.json` | Moving glint — minimal API (color + speed), but the \"reflective surface\" metaphor is earned. |
| `affordance_wake` (2 variants) | Earned name | `compositions/shader_affordance_wake*.json` | zone + progress compression. Decomposes to EdgeShadow + intensity envelope but the factory surface is cleaner. |
| `focused_row_gradient` | Earned name | `compositions/shader_focused_row_gradient.json` | Two-color row-distance gradient. Candidate for Pattern::RowDistance primitive; earned-name for now. |
| `sub_cell_shake` | Primitive itself | `primitives/shader_sub_cell_shake.json` | Chromatic-RGB noise with seeded jitter. Per-channel per-cell operation — doesn't fit ColoredOverlay. |
| `stochastic_sparkle` | Primitive itself | `primitives/shader_stochastic_sparkle.json` | Density-driven stochastic brightening with Uniform/Gaussian noise. Generator, not overlay. |
| `chromatic_edge` | Primitive itself | `primitives/shader_chromatic_edge.json` | Per-channel RGB offset at edges. Channel-displacement primitive. |
| `glitch_lines` | Primitive itself | `primitives/shader_glitch_lines.json` | Stochastic horizontal bands + flash_chance. Generator. |
| `neon_flicker` | Primitive itself | `primitives/shader_neon_flicker.json` | Segment-level stochastic dimming. Segmented generator. |
| `trace_path` | Primitive itself | `primitives/shader_trace_path.json` | Authored polyline rendering with multi-path composition in payload. Positional. |
| `trace_propagation` | Primitive itself | `primitives/shader_trace_propagation.json` | Manhattan-distance grid pulse from origin. Novel spatial function. |
| `orbit` | Primitive itself | `primitives/shader_orbit.json` | Dots on circle — positional, not weight-based. |
| `cursor` | Primitive itself | `primitives/shader_cursor.json` | Primary + trail positional shader. |
| `reveal_wipe` | Primitive itself | `primitives/shader_reveal_wipe.json` | Progressive directional reveal — arguably a mask dressed as shader; flagged for possible reclassification. |

### New schema questions surfaced in Stage 2

Added to the Schema Journal above:

- **Q11 — RuntimeBinding vs SignalGraph at the JSON surface** — Stage 2 forced the concrete shape. Used `{"binding": "name", "default": <T>}` uniformly; default preserves V2's offline-validator fallback behavior. The collapse from V2's `field` + `field_binding` pair to V3's single `field: ParamValue<T>` is the clearest authoring win of the migration.
- **Q12 — Enum-typed bindings** — `direction_binding: "hover_direction"` binds to a u16-coded enum (0=Forward, 1=Reverse, 2=PingPong). V3 needs to decide: does the app send ordinal integers (closer port) or enum-variant strings (more self-documenting)? Lean variant-strings.
- **Q13 — Tagged-union payloads inside named-factory shaders** — `focus_field.shape = "rect" | "ellipse"` discriminator gates which other fields apply. Intention 39 merge semantics apply at the factory-payload level. Factory's deserializer handles dispatch; V3 doesn't need to surface the tag at the schema level.
- **Q14 — Dual-color shaders** — `glisten_band.head` + `glisten_band.tail` are two parallel ParamValue<Color> fields. No unification pressure at the factory level; flag for future if a third-color shader emerges.
- **Q15 — `row_mask` and other factory-internal scope sub-objects** — `highlighter.row_mask` is scope-encoding inside the factory. V3 could lift to step.scope = {kind: "row", selector: "last"}. Kept V2 shape; flag for plan review.
- **Q16 — Accessibility hints inside factory payloads** — `highlighter.text_contrast` is a legibility affordance that could generalize cross-shader. Stays inside highlighter (rule-of-three not met).
- **Q17 — Positional primitives and widget-layout coupling** — `cursor.primary.position`, `wayfinding_node.nodes[].{x,y}`, `trace_path.paths[].points[].{x,y}` all embed widget-local cell coords. Production use will typically runtime-bind these; the static coords are offline defaults.
- **Q18 — Primitive payloads that carry internally-composed sub-animations** — `trace_path.paths[]` with per-path `delay` is a micro-composition inside one primitive. Could split to parallel trace_path steps with phase-scoped delays, but the factory's shared params argue for staying bundled.
- **Q19 — Clock + bindings coexistence** — `focus_field_center_binding` has both a clock (pulse_speed) AND bindings (center). Orthogonal drivers; clock is the default temporal basis for signals inside the recipe, bindings bypass the clock.
- **Q20 — Accessibility hints** (duplicates Q16; folded into Q16).
- **Q21 — Offline-default for runtime bindings** — V2 pattern: static field = offline default; _binding field = runtime override. V3 `ParamValue::RuntimeBinding` carries an explicit `default` so offline validators have something to render and the binding-provided value at runtime wins. Lean toward including `default` because the V2 corpus relies on offline validators.

### Totals

- Stage 2 source files: 41 (Tier 2: 23, Tier 3: 18)
- Stage 2 classified as primitives (primitive-itself): 10 (all Tier 3 generator-class or positional shaders)
- Stage 2 classified as earned names: 31 (all Tier 2 plus 7 Tier 3 compositions)
- New primitive kinds proposed: `sub_cell_shake`, `stochastic_sparkle`, `chromatic_edge`, `glitch_lines`, `neon_flicker`, `trace_path`, `trace_propagation`, `orbit`, `cursor`, `reveal_wipe` (10 kinds, none fitting ColoredOverlay + Pattern)
- Schema questions surfaced: Q11–Q21 (added to journal above)

### Stage 2 observations

**The RuntimeBinding collapse is the clearest V3 win.** Every `_binding` variant in V2 could be read as \"this version of the recipe is app-driven.\" In V3 that becomes `field: {"binding": "name", "default": X}` uniformly — one authoring mechanism per param, not two parallel fields. This single change has the largest read-improvement per byte in the migrated corpus; it's visible in 13 of the 41 Stage 2 recipes.

**Tagged-union factory payloads are invisible to V3 schema.** `focus_field.shape`, `highlighter.mode`, `bevel.light_direction` all dispatch inside the factory's deserializer. V3 schema doesn't need to surface these; authors see one factory, many modes.

**Primitive-itself vs earned-name is a judgment call at the middle.** radar, wayfinding_node, affordance_wake, bevel, reflect all live in the gap between \"new Pattern variant\" and \"new factory kind.\" I erred toward earned-name when the factory's *compression* of multiple decisions (light_direction → edge-pair, zone → edge-set, current_index → three-state emphasis) is the authoring win. If a future Pattern catalog grows to cover those primitives, the factories become trivially-decomposable and could be retired.

**Generator-class shaders are unambiguously primitive-itself.** sub_cell_shake, stochastic_sparkle, chromatic_edge, glitch_lines, neon_flicker all share \"per-cell stochastic or per-channel logic with seed\" as their identity. These are clearly new base kinds in the primitive catalog, not ColoredOverlay-derived.

**The migration has not surfaced a true V3-inexpressible recipe yet.** Every Stage 2 recipe ports cleanly (modulo the schema questions flagged). The \"primitive itself\" classifications are about catalog growth, not schema gaps. This is a quiet vote of confidence in Decision 1 (unified Scope), Decision 2 (Pattern-as-axis with earned names), Decision 3 (tree schema), and Decision 6 (ParamValue union).

---

## Stage 3 — Non-shader debug subdirs (145 files)

| Subdir | Count | Directory policy | Notes |
|---|---|---|---|
| `baseline.json` | 1 | Root-level, unchanged | Surfaces Q22 (pipeline.step optional) |
| `complex/` | 10 | Same dir | Scope-differentiated multi-layer compositions; surfaces Q23–Q27, revises Q1 |
| `content/` | 46 | Same dir | 12 basic + 14 split_flap + 14 typewriter_cursor + 2 glyph_cascade + 4 assorted; no new schema questions |
| `filters/` | 45 | Same dir | All earned-name factories; includes 13 binding-variants collapsed to V3 ParamValue |
| `masks/` | 14 | Same dir | All earned-name factories; materialize_center+corner are enter-only |
| `samplers/` | 6 | Same dir | crt, crt_jitter, faultline, ripple, shredder, sinewave; surfaces Q29 (sampler payload's `phase` field renamed to `phase_offset` to avoid collision with step-level `phase`) |
| `styles/` | 14 | Same dir | StyleEffect step kind canonically exercised; canvas-aware fade variants; `style_spatial_effect` showcases the V3 cleanup win (no need for the Spatial-wrapped-shader StyleEffect — every step is phase-scopable directly) |

### Stage 3 totals

- Stage 3 source files: 145 (1 + 10 + 46 + 45 + 14 + 6 + 14; wargames counted separately in Stage 5)
- Schema questions added: Q22–Q29 (see journal entries above)
- Q1 revised to hybrid resolution (config.base_style + StyleEffect steps)

---

## Stage 4 — madeira-flag as V3 recipe

**Output:** `recipes/madeira_flag/madeira_flag.json`

**Scene composition:** four scene layers (backdrop, fireworks, flag, text_stack) composed per Decision 5 scene-layer taxonomy. Each layer has its own content source + pipeline:

- `backdrop` — procedural `solid_color_fade` source, 250ms fade-in to 0.85 alpha
- `fireworks` — procedural `ballistic_fireworks` source with 12 slots, ballistic trajectories, gravity, palette array
- `flag` — .rsb braille image (PRD primitive 4) + spatial_signal sampler (compound sine emitting `displacement` hint) + displacement_shade shader (reads hint, applies wave-correlated 3D shading). **This layer is the canonical V3 flag-animation PRD scenario.**
- `text_stack` — card source with title/body/footer + `below_sibling` placement anchored to the flag layer

### Stage 4 new schema questions

- **Q30** — procedural generators with large param surfaces (fireworks has 10+ nested config keys including palette arrays and spawn_zones). V3 Decision 5's `params: serde_json::Value` is opaque; at this complexity, typed per-generator schemas would help authoring + validation. Proposal: generator registry includes schema-per-generator-id; loader validates.
- **Q31** — mixed-signals signal-graph vocabulary needs spatial-coordinate leaves (`sample_norm_x`, `sample_norm_y`) and spatial_frequency tuples on `sine` nodes. Otherwise compound-spatial-temporal signals can't be expressed as ParamValue::SignalGraph without per-cell closures.
- **Q32** — `wave_correlated` shading needs a primitive or must be expressible as a SignalGraph on shader.intensity with per-cell spatial inputs. Decision 7 step-output-hints is the mechanism if the sampler emits a `wave` hint; the shader reads it via `binds: { shade_input: wave }`.
- **Q33** — card placement relative to sibling layers (`anchor: below_sibling, sibling_id: flag`). New V3 capability — siblings need stable `id`s and scene-level placement vocabulary.

---

## Stage 5 — wargames/ hierarchy (66 files)

**Output:** 10 V3 templates in `recipes/wargames/themes/` + 56 V3 child recipes in `recipes/wargames/`. V2 originals → `_DEPRECATED_`.

### Templates (10 migrated fully)

- `computer_base`, `enhanced_crt_computer`, `human_input`, `new_computer_typing_steady`, `new_human_input_smooth`, `new_rapid_sequence_base`, `new_scrolling_output`, `new_wopr_fullscreen_cyan`, `wopr_cyan`, `wopr_green`

### Children (56 batch-transformed)

**Justification for batch-transform:** the 56 child recipes are mechanically uniform — each carries `extends: themes/<base>.json` + a small config override (message + lifecycle, occasionally content.effect=null for maps). The substantive schema work is in the templates; children are content overrides. Batch transformation via Python script preserves each child's V2 content verbatim while applying the uniform V3 envelope changes (schema_version, version, last_updated, metadata block). This is the one place in the migration where batch transformation is appropriate because the per-file reflection *is* "this is mechanical."

### Stage 5 schema observations

- V3 `extends` preserves V2 semantics (deep-merge + tagged-union replacement per Intention 39). No schema changes needed to support the wargames hierarchy — V3 `extends` just works.
- The `extends` path still points at `themes/<base>.json` — no V3-specific rewriting of the path needed.
- Children's config overrides naturally deep-merge into the V3 resolved template config. The V3 tree schema (pipeline.step) does not appear in any child — inherited from template.

---

## Stage 6 — extended barber_pole with fractional fg glyphs

**Output:** `recipes/debug_recipes/shaders/compositions/shader_barber_pole_fractional_third_color.json`

Thought-experiment recipe exploring whether V3 can express sub-cell stripe width and tri-color composition via primitive composition rather than a dedicated primitive.

**Composition:** parallel of two Shader steps —
1. Base barber_pole (two-color, integer-cell stripes)
2. Hypothetical `fractional_stripe_overlay` primitive painting partial-block glyphs at stripe boundaries, emitting a third color via the fg-glyph's own color

### Stage 6 schema questions

- **Q34** — does V3 want a dedicated `fractional_stripe_overlay` / `sub_cell_bar` primitive, or should authors compose this via `pattern_fill` + predicate scope? The recipe proposes the new primitive because the boundary-aware placement is non-trivial to express via pattern_fill alone — it depends on the sibling barber_pole's stripe boundaries, which suggests Decision 7 step-output-hints (barber_pole emits `stripe_boundary_distance` hint; overlay reads it).
- Demonstrates V3's ability to compose tri-color compositions without a built-in tri-color primitive, via stacked Shader steps where the fg-glyph's color becomes perceptually-blended third color.

---

## Final audit — V2 vs V3 schema coverage

Below is the comprehensive map of every V2 field/concept encountered across the full debug-recipes corpus (shaders, filters, masks, samplers, styles, content effects, complex compositions, wargames templates+extends) against its V3 mapping. Anything marked **Unmapped** or **Gap** needs plan-level decision before the V3 loader can be written.

### Core envelope

| V2 field | V3 mapping | Status |
|---|---|---|
| `schema_version: 1` | `schema_version: 3` | Mapped |
| `id`, `title`, `description`, `version`, `last_updated` | Same names, same semantics | Mapped |
| `extends: <path>` | Same (supports `themes/<path>.json` and `recipes/<path>.json`) | Mapped |
| `config.message` | Same | Mapped |
| `config.layout.width`/`height`/`anchor` | Same | Mapped (Q7 — vocab refresh for `anchor` → `placement` still deferred) |
| `config.layout.mode: "fullscreen"` | Same | Mapped |
| `config.layout.mode: "fixed"` | Same | Mapped |
| `config.lifecycle.auto_dismiss_ms` | Same | Mapped (Q7 — `auto_dismiss_ms` → `duration_ms` still deferred) |
| `config.border.{type, trim}` | Same | Mapped |
| `config.time: {loop, loop_period_ms}` | `config.clock: {loop, period_ms}` | **D1** (documented drift) |
| `config.content: {mode, effect}` | Same (Q25 — sibling of pipeline for root recipes; per-layer for scene recipes) | Mapped |

### Base style

| V2 | V3 | Status |
|---|---|---|
| `pipeline.style.base_style: {foreground, background}` (single scope) | `config.base_style: {foreground, background}` | Mapped |
| `pipeline.styles: [{region, base_style, ...}]` (multi-scope) | `pipeline.step = Parallel(StyleEffect with scope, ...)` | Mapped (Q1 hybrid resolution) |
| `base_style.added_modifiers` / `removed_modifiers` | **Unmapped** — V2 supported Bold/Italic/Underline modifiers. V3 should preserve but the migration didn't explicitly test. Needs `modifiers: [...]` channel in V3 base_style or a dedicated Modifier step | **Gap — minor** |

### Enter/exit/dwell phase vocabulary

| V2 | V3 | Status |
|---|---|---|
| `pipeline.enter.{duration_ms, easing}` | `pipeline.timing.{enter_ms, enter_ease}` | Mapped |
| `pipeline.exit.{duration_ms, easing}` | `pipeline.timing.{exit_ms, exit_ease}` | Mapped |
| `pipeline.enter.snapping: {type}` | **Unmapped** — V2 had snapping (round/floor/ceil) for motion path quantization. V3 should preserve as `timing.enter_snap` or fold into motion-path payloads | **Gap — minor** |
| `pipeline.enter.motion_path: {type}` | **Unmapped** — V2 supported motion_path {type: linear, arc, bezier, spring, ...}. V3 needs a pipeline.timing.motion_path or a first-class MotionPath step. Related to `tui-vfx-geometry::PathType` per Intention 38 | **Gap — MAJOR** |
| `pipeline.enter.from: {type: offscreen, margin_cells, direction}` / `pipeline.exit.to: ...` | **Unmapped** — V2 had from/to offscreen for slide-in/slide-out. V3 needs these as motion-source/destination params on timing or motion-path | **Gap — MAJOR** |
| Phase vocabulary: `enter`, `dwell`, `exit`, `all` | Same (Q24 confirmed) | Mapped |

### Pipeline step kinds (Decision 3)

| V2 field | V3 step | Status |
|---|---|---|
| `pipeline.mask.enter/dwell/exit` | `Step {kind: mask, phase, payload}` | Mapped |
| `pipeline.sampler.enter/dwell/exit` | `Step {kind: sampler, phase, payload}` | Mapped (Q29 — sampler `phase` payload field renamed to `phase_offset`) |
| `pipeline.filter.enter/dwell/exit` | `Step {kind: filter, phase, payload}` | Mapped |
| `pipeline.filter.<phase>: [{...}, {...}]` (list) | Multiple parallel Filter steps with same phase (Q28) | Mapped |
| `pipeline.style.spatial_shader` | `Step {kind: shader, payload}` | Mapped |
| `pipeline.style.enter_effect/dwell_effect/exit_effect` | `Step {kind: style_effect, phase, payload}` | Mapped |
| `pipeline.style.region` | `Step.scope` | Mapped |
| `StyleEffect::Spatial { shader }` | Directly `Step {kind: shader, phase: dwell/enter/exit}` — V3 cleanup win | Mapped |
| `animation_type` (V2 legacy field) | **Unmapped** — likely vestigial; wargames templates carry `animation_type: none`. No observable V2 behavior tied to it. Likely safe to drop | **Drop recommendation** |

### Scope primitive (Decision 1)

| V2 StyleRegion | V3 Scope | Status |
|---|---|---|
| `"All"` | `{kind: "all"}` | Mapped |
| `"BorderOnly"` | `{kind: "border"}` (Q23) | Mapped |
| `"TextOnly"` | `{kind: "content", value: "text"}` | Mapped |
| `"BackgroundOnly"` | `{kind: "channel", value: "background"}` | Mapped |
| `{Cell: {x, y}}` | `{kind: "cell", x, y}` (Q23) | Mapped |
| `{Cells: [{x,y},...]}` | `{kind: "cells", cells: [...]}` (Q23) | Mapped |
| `{Rows: [0, 1, ...]}` | `{kind: "rows", rows: [...]}` (Q23) | Mapped |
| `{RowRange: {start, end}}` | `{kind: "row_range", start, end}` (Q23) | Mapped |
| `{Columns: [0, 1, ...]}` | `{kind: "columns", columns: [...]}` (Q23) | Mapped |
| `{ColumnRange: {start, end}}` | `{kind: "column_range", start, end}` (Q23) | Mapped |
| `apply_to: "background"/"foreground"/"both"` | Lifted to step.scope (Q10) | Mapped |
| Scope coords as `{x: {binding}, y: 3}` (style_cell_position_binding) | `{kind: "cell", x: ParamValue<u16>, y: u16}` | **Partially mapped** — Scope variants must accept ParamValue-typed coord fields explicitly |

### Runtime bindings (V2 → V3 ParamValue)

| V2 pattern | V3 ParamValue | Status |
|---|---|---|
| `{field, field_binding}` parallel pair | `field: {binding: "name", default: T}` (Q11, Q21) | Mapped |
| Enum-typed bindings (direction, u16-coded) | `field: {binding, default: "variant_string"}` (Q12) | Mapped — lean variant-string over ordinal |
| `BindableU16` for coords | Same ParamValue mechanism (Q29) | Mapped — requires Scope variants to accept ParamValue coords |
| `speed_binding`, `direction_binding`, `progress_binding`, `rect_x_binding` ... | All collapse to ParamValue<T> on the target field | Mapped |

### Signal-driven parameters

| V2 pattern | V3 | Status |
|---|---|---|
| `mode: "breath"`/`"drift"` + shader-specific modulation flags (drift_speed, pulse_speed, drift_amount) | `field: {signal: {...}}` SignalGraph (Q3) | **Partially mapped** — V3 concept exists but shape unsettled |
| `temporal_dither_hz` / `speed_hz` clock-bound params | Currently preserved as factory-internal clock values; could lift to SignalGraph | Mapped (as factory-internal) |
| `sine_wave.phase` (V2 field on sampler payload) | Renamed to `phase_offset` to avoid collision with step.phase (Q29) | Mapped |

### Content effects

| V2 | V3 | Status |
|---|---|---|
| `content.mode: enter_only/loop/...` | Same | Mapped |
| `content.effect: {type, ...params}` | Same factory payload | Mapped |
| All ~16 content effect types (typewriter, scramble, marquee, mirror, morph, dissolve, glitch_shift, scramble_glitch_shift, slide_shift, redact, numeric, odometer, split_flap, glyph_cascade, wrap_indicator) | Same — factory payloads preserved verbatim | Mapped |

### Colors and normalization

| V2 | V3 | Status |
|---|---|---|
| `{type: "rgb", r, g, b}` | Same | Mapped |
| `{r, g, b}` bare (no type tag) | V3 normalizes to tagged form (Q27) | Mapped — migration did normalize |
| `{type: "cyan"/"white"/"black"/"red"/"green"/"blue"/"yellow"/"magenta"/"light_cyan"/"light_blue"/"light_red"/"light_green"/"light_yellow"}` | Same named-color vocabulary | Mapped |
| `{type: "reset"}` | Same | Mapped |

### Scene-layer and multi-layer (Decision 5)

| V2 | V3 | Status |
|---|---|---|
| `config.scene.layers: [...]` (V2 Sub-plan B.1 introduction) | V3 Decision 5 extends with per-layer pipelines, content sources (Text, Image, Procedural, Card), role_tags, placement | Mapped — but scene-layer recipes were not in debug_recipes/; madeira-flag Stage 4 is the first concrete exercise |
| Per-layer content sources (rsb image, procedural, card) | See Stage 4 — Q30 (procedural generator params), Q33 (sibling-anchored placement) | **Partially mapped** |

### Gaps requiring plan-level decision before loader implementation

1. **`base_style.added_modifiers` / `removed_modifiers`** (Bold/Italic/Underline text modifiers) — V3 needs a channel for these. Proposal: `base_style.modifiers: ["bold", "italic"]` alongside fg/bg.
2. **`pipeline.<phase>.snapping`** — round/floor/ceil for motion-path quantization. Needs a timing-level snapping field.
3. **`pipeline.<phase>.motion_path: {type}`** — V2 motion_path (linear, arc, bezier, spring, ...) via `tui-vfx-geometry::PathType`. V3 needs either a first-class MotionPath step kind or a motion-path field on timing. **Significant gap.** Per Intention 38, V3 should trial-deserialize motion_path values against upstream PathType for validation.
4. **`pipeline.<phase>.from`/`to: {type: "offscreen"}`** — V2's slide-in/slide-out origin/destination. V3 needs these paired with motion_path. **Significant gap.**
5. **`animation_type` legacy field** — safe to drop.
6. **Signal-graph shape (Q3)** — still tentative; needs mixed-signals alignment.
7. **Procedural-generator typed params (Q30)** — V3 Decision 5's `params: serde_json::Value` is opaque; large generators (fireworks) push for typed schemas per generator_id.
8. **Sibling-anchored placement in scene layers (Q33)** — new V3 concept.
9. **Spatial-coordinate signal-graph leaves (Q31)** — mixed-signals extension required.
10. **`wave_correlated` shading kind (Q32)** — either a primitive or a ParamValue SignalGraph pattern.
11. **Row/Column/Cell scope on ParamValue-typed coords** — style_cell_position_binding demonstrates it but V3 Scope needs to explicitly accept ParamValue<u16> for cell coords.
12. **`fractional_stripe_overlay` / sub-cell composition primitives (Q34)** — Stage 6 explores the tri-color sub-cell case; V3 needs to decide whether the overlay primitive exists or composition+predicate-scope is sufficient.

### Gaps marked major — **user attention required**

The two most significant gaps for V3 to resolve before loader implementation:

- **Motion paths (#3, #4 above).** V2 motion_path + offscreen from/to are how recipes arrive and depart with geometry-aware trajectories (arc, spring, bezier). V3's `pipeline.timing` is just enter_ms/exit_ms/ease — no path concept yet. Options: (a) extend pipeline.timing with a motion_path field; (b) introduce a MotionPath step kind; (c) use existing Decision 5 scene-layer placement to drive entry trajectories. Lean (a) — keeps motion_path coupled with timing, avoids a new step kind.
- **Signal-graph JSON shape (#6, Q3 resolution).** Every breath/drift/pulse variant in the debug corpus uses this. Without a settled shape V3 can't ship. Needs mixed-signals JSON-surface alignment.

---

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-debug-recipes-migration-log.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.3.0</VERS> -->
