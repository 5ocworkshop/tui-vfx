<!-- <FILE>docs/design/tui-vfx-effect-composition-model.md</FILE> - <DESC>Architectural framing: named stages vs arbitrary chaining for effect composition; honest assessment under terminal constraints; recommendation + implications for the signal facade</DESC> -->
<!-- <VERS>VERSION: 0.4.0</VERS> -->
<!-- <WCTX>2026-04-27: extend Model B with pre/post-pass slots around the four-stage element pipeline. Generalizes today's hardcoded shadow path; opens a clean home for glow, outline, vignette, scanline, backdrop blur, and scene-layer underlays/overlays. Resolves §9.4.</WCTX> -->
<!-- <CLOG>0.4.0: add §12 enumerating the full deliverables list — trait surface, compositor port, V3 schema (draft + annotated), validator slot enforcement, debug/probe/trace tooling, authoring docs, rustdoc + capabilities autogen, V2→V3 lowering, corpus migration, tests, and release gates.</CLOG> -->

# tui-vfx effect composition model

> **Companion to:** `tui-vfx-mixed-signals-recipe-surface-proposal.md` (the signal facade) and `tui-vfx-v3-upgrade-plan/00_INDEX.md` (V3 schema redesign).
>
> **Trigger:** before locking the signal facade shape, decide what "composition" means in tui-vfx — fixed stages with internal flexibility, or a free-form effect graph. The facade fits both, but in *different* shapes; locking the facade without locking the model is premature.

## The question, in one paragraph

Do we want to allow **arbitrary chaining** of effects in any order with typed I/O between them (like a shader-graph editor), or do we want to keep the **named-stage pipeline** (sample → mask → shade → filter) where the order is fixed and authors compose by stacking layers *within* each stage? The choice has real consequences for recipe ergonomics, performance under terminal constraints, the shape of the effect-trait surface, and where the signal facade sits.

---

## 1. The two models

### 1.1 Model A — Free-form effect graph (node-graph composition)

```
                 ┌────────┐         ┌────────┐
   input cell ──▶│ Sampler│──coord─▶│ Filter │──cell──┐
                 └────────┘         └────────┘        │
                      │                               │
                      └──coord──▶┌────────┐           ▼
                                 │  Mask  │──gate─▶ ┌──────┐
                                 └────────┘         │ out  │
                 ┌────────┐                  ┌────▶│ cell │
                 │Shader A│──style───────────┘      └──────┘
                 └────────┘
                      ▲
                 ┌────┴────┐
                 │Shader B │  (chained to A)
                 └─────────┘
```

Recipe authors wire effects as nodes; output of one feeds input of another. Order is recipe-defined per layer. Plugin-style effect libraries can ship arbitrary node types.

**What this model assumes:**
- Typed ports between effects (e.g. a Sampler outputs `coord`, can feed any node that takes `coord`).
- The runtime resolves the graph per cell per frame.
- The recipe schema describes nodes + edges, not stages + layers.

### 1.2 Model B — Named stages with within-stage flexibility (status quo, gently extended)

```
        ┌──────────────────────────────────────────────────┐
        │  Per-cell pipeline  (one VfxCellContext per cell)│
        └──────────────────────────────────────────────────┘
                              │
                              ▼
   ┌────────────────────────────────────────────────────────┐
   │ STAGE 1: SAMPLER chain   (output coord ⇨ next sampler) │
   │   sampler[0] ⇨ sampler[1] ⇨ sampler[2] ⇨ resolved coord│
   └────────────────────────────────────────────────────────┘
                              │
                              ▼
   ┌────────────────────────────────────────────────────────┐
   │ STAGE 2: MASK chain      (combined via AND/OR)         │
   │   mask[0] AND mask[1] AND mask[2] ⇨ visible? bool      │
   └────────────────────────────────────────────────────────┘
                              │
                              ▼ (skip if !visible)
   ┌────────────────────────────────────────────────────────┐
   │ STAGE 3: STYLE/SHADER chain  (output style ⇨ next)     │
   │   shader[0] ⇨ shader[1] ⇨ shader[2] ⇨ final style      │
   └────────────────────────────────────────────────────────┘
                              │
                              ▼
   ┌────────────────────────────────────────────────────────┐
   │ STAGE 4: FILTER chain    (mutates &mut Cell sequentially)│
   │   filter[0] ⇨ filter[1] ⇨ filter[2] ⇨ final cell       │
   └────────────────────────────────────────────────────────┘
                              │
                              ▼
                          out_cell
```

The four stages are fixed. Within each stage, recipe authors stack layers in any order they want; output of one layer feeds input of the next within the same stage. Cross-stage I/O happens at well-defined seams (mask gate, sampler-resolved coord, shader-style chain).

**This is what tui-vfx has today.** Phase F (just landed) makes the per-cell `VfxCellContext` uniform across all four stages, so effect impls in different stages can share parameter shapes without forcing identical traits.

---

## 2. Honest assessment against terminal constraints

GPU shader graphs work because the arithmetic is favorable: millions of fragments, massive parallelism, cheap memory bandwidth, mature optimization compilers. Terminal rendering does not have that arithmetic.

| Constraint | GPU pipeline | Terminal pipeline |
|---|---|---|
| Cells per frame | 1M–10M+ fragments | 2K–10K cells (typical TUI) |
| Parallelism | massive (thousands of cores) | rayon at best; mostly sequential |
| Memory model | dedicated VRAM, cache hierarchy | host RAM, no special locality |
| Frame budget | 16ms for 1M ops | 16ms for 10K ops + flush |
| Per-cell payload | float4 RGBA + depth + auxiliary | char + style (24-bit RGBA × 2 + mods) |
| Optimizer maturity | decades of GLSL/HLSL/SPIR-V tooling | single-binary Rust compiler, no graph-level opt |

**The implication:** the per-cell *cost* in terminal rendering is concentrated in a few-millisecond window where the pipeline does ~10K cell visits. A node-graph model that resolves edges per cell per frame adds:
- Per-cell graph traversal overhead (real ms tax on a 16ms budget).
- Per-effect dynamic dispatch (already a thing today, but bounded by stage count; node graphs would multiply it).
- Type-check overhead at recipe load (acceptable) and per-frame port-compat checks (not acceptable).

The four-stage model already gives us:
- Constant-time per-cell pipeline traversal (4 stages, fixed).
- Each stage can early-exit (mask=false short-circuits shader+filter).
- Per-cell `VfxCellContext` constructed once and shared across all four stages (Phase F's whole point).

A free-form graph would force us to either (a) compile recipes to a flat per-cell program at recipe-load time (real engineering effort, ~Slice-scale) or (b) pay the traversal tax per cell per frame.

---

## 3. What the four stages actually encode

The fixed order is not arbitrary — it encodes data dependencies that are inherent to terminal cell composition:

| Stage | What it answers | Output | Why it's at this position |
|---|---|---|---|
| **Sampler** | "What source cell does this dest cell read from?" | redirected `(x, y)` coord | Must come first; downstream stages need to know what cell they're operating on. |
| **Mask** | "Is this cell visible at all?" | `bool` gate | After sampling (mask may depend on resolved coord); before shading (no point styling an invisible cell). |
| **Shader** | "What style does this cell take?" | `Style` (fg, bg, mods) | After masking (skip invisible cells); before filtering (filters operate on the styled cell). |
| **Filter** | "Post-process the rendered cell" | mutated `Cell` | Last; filters see the final styled cell and tweak it (dim, blur, scan-line, etc.). |

Re-ordering breaks the dataflow. "Filter then mask" means filtering invisible cells. "Shade then sample" means shading the wrong source. The order is the type system writ visually.

**Within-stage chaining is different.** Within a stage, all effects produce values of the same type, so chaining is pure composition:
- Sampler chain: each sampler outputs a coord, next sampler reads that coord. Already works in `cls_prepared_sampler.rs::sample_sampler_chain`.
- Mask chain: each mask outputs a bool; combined via AND (default) or OR (declarable). Already works in `fnc_check_masks.rs`.
- Shader chain: each shader takes the previous shader's `Style` output as `base`. Already works in `orc_render_pipeline.rs::apply_shaders` (and Phase F.0 just cleaned up the struct-literal sites that were the friction point).
- Filter chain: each filter mutates `&mut Cell` in turn. Already works.

So the question reduces to: do we want **inter-stage flexibility** (Model A) or are we content with **intra-stage flexibility** (Model B)?

---

## 4. Where Model B falls short today (the honest list)

Three real cases where Model B is awkward:

1. **Boundary-crossing effects.** A "ripple" effect is naturally a sampler (coord redirect) + mask (only visible during the wave) + shader (color shift in the wake). Today an author writes three layers and threads parameters through `runtime_params` to keep them coordinated. Phase F made this much cleaner (uniform `VfxCellContext`), but it's still three declarations for one conceptual effect.

2. **Filter-driven masking.** Some effects want a Filter to *also* gate visibility ("dim this cell, but skip it entirely if its luminance fell below X"). Today filters can't return "skip"; they can only mutate the cell. Workaround: write a Mask that runs the same luminance computation, knowing it'll re-execute downstream in the Filter — which violates "no double work."

3. **Sampler-chain output as Mask input.** "Mask the area the sampler-chain redirected away from." Today the mask sees the original coord, not the post-sampler coord. To get post-sampler masking, you have to bake the mask into the sampler logic — a Sampler that returns `None` for masked cells.

**Are these worth a node graph?** No. They're worth surgical seams in Model B:
- (1) is a recipe-authoring ergonomics issue → solve with **composite-effect templates** at the recipe layer (e.g. `"effect": "ripple"` expands to the three-layer triple at recipe-load time). Doesn't change the pipeline.
- (2) is a Filter-trait shape issue → could let Filter return `Option<()>` or carry a "discard" bit. One-line trait change.
- (3) is a Mask dataflow issue → could give Mask access to the post-sampler coord via a new field on `VfxCellContext` (e.g. `resolved_x`, `resolved_y`). One field bump on `VfxCellContext`, free with Phase F's design.

All three are surgical fixes within Model B. None require a graph engine.

---

## 5. Recommendation

**Stay with Model B (named stages + within-stage chaining). Reject Model A for tui-vfx's lifetime.**

Rationale:

1. **Terminal constraints make the graph engine a net loss.** The per-cell traversal tax and the recipe-load compilation effort would buy us flexibility we mostly don't need (see §4 — three real cases, all solvable inside Model B).
2. **The fixed order encodes real dataflow.** Authors don't get less power; they get a predictable mental model that matches the underlying constraints.
3. **Phase F already paid the dividend for Model B.** Uniform `VfxCellContext` across all four traits means within-stage chaining is consistent, future field additions are cheap, and the per-cell hot path is a constant-time four-stage walk.
4. **Composite-effect templates cover the "ripple is one effect, not three" ergonomics issue** without changing pipeline semantics. This is a recipe-layer move, exactly where it belongs.

**Three explicit follow-on moves to make Model B a complete answer:**

| Move | Cost | Earns its place via |
|---|---|---|
| **Composite-effect templates** in the recipe schema (V3): `"type": "ripple"` declares a name + parameter schema; recipe-load expands it into the three layers (sampler + mask + shader) using the parameter values. Like SCSS mixins. | medium; one expansion pass at recipe-load | covers the boundary-crossing ergonomics (§4 case 1) without runtime cost |
| **Filter-discard bit** on `VfxCellContext` (or a return type change on `Filter::apply`) | small | covers §4 case 2; closes a real gap |
| **Resolved-coord fields** on `VfxCellContext` (`resolved_x`, `resolved_y`) populated after sampler stage | small; one field bump on the type Phase F just introduced | covers §4 case 3; cheap given the bundle pattern |

None of these are required to ship the signal facade. They are the natural next moves *after* the facade lands.

---

## 6. Implications for the signal facade

The recommendation is Model B → **signals fill parameter slots, signals are not graph nodes**. The facade in `tui-vfx-mixed-signals-recipe-surface-proposal.md` v0.2.0 is the right shape for Model B. Specifically:

| Question | Model A answer | Model B answer (= what we're building) |
|---|---|---|
| What is a signal? | A node in a graph with output ports. | A value-producing expression that fills a parameter slot on an effect. |
| Where does it appear in JSON? | As a node with edges to other nodes. | As a `{"signal": {"type": "...", ...}}` value inside a `BindableValue` / `BindableU16` / `BindableColor` field. |
| Can a signal's output drive another signal's parameter? | Yes, via graph edges. | Yes, via SignalSpec's composition operators (`add`, `multiply`, `mix`, `frequency_mod`) — already supported. |
| Can a signal drive an effect's *structural* shape (e.g. number of layers)? | Possibly. | No. Recipes stay structurally static; signals only drive parameter values. |
| Can effects in different stages share signal output? | Yes, via shared edges. | Yes, via `runtime_params` (host-bound) or via inline expression duplication (recipe-bound). |
| Where does the facade live? | At the node-registry level. | At the recipe-deserialization level — `tui_vfx_recipes::signals::*`. |

So the signal facade proposal can land as-is. The Model B decision is what *justifies* the facade's narrow scope (just deserialization + curation + autogen reference, not a graph engine).

---

## 7. What this means for V3 schema

The V3 schema redesign (`docs/design/tui-vfx-v3-upgrade-plan/`) is the right moment to lock Model B explicitly:

- The recipe schema continues to declare effects under stage-named keys (`samplers: []`, `masks: []`, `shaders: []`, `filters: []`) — already its shape.
- Within-stage layered ordering stays recipe-author-controlled (already true).
- Composite-effect templates (`"type": "ripple"`) get a dedicated section in the schema with explicit expansion semantics.
- The signal facade gets a dedicated reference (`SIGNALS_REFERENCE.md`, autogen) so authors know what signal types they can drop into parameter slots in any stage.

If V3 is also the moment to add the §5 follow-on moves (filter-discard bit, resolved-coord fields), they can land as schema-additive fields on the relevant trait surfaces.

---

## 8. What we are NOT building

To prevent over-baking:

- **No node-graph runtime.** No node registry, no port-compat checker, no edge resolver, no per-cell graph traversal.
- **No cross-stage reordering** (e.g. "filters run before masks in this recipe"). The four element stages stay in fixed order.
- **No structural signals.** Signals don't drive layer count, effect type selection, or recipe topology.
- **No plugin loader.** Effects continue to be Rust impls compiled into the binary; recipes don't load `.so`/`.dll` plugins.

These are explicit non-goals. Future re-evaluation requires a fresh proposal with concrete user demand, not speculative flexibility.

**What is in scope (clarified by §11):** pre-pass and post-pass slots around the four-stage element pipeline. These are not "more steps" added to the per-cell loop — they are buffer-level operations with their own trait family (`PrePass`, `PostPass`). Generalizing today's hardcoded shadow path into a pre-pass slot is in scope and decided in §11; it is not a node-graph runtime.

---

## 9. Open questions

1. **Composite-effect template syntax.** SCSS-mixin-style (`"type": "ripple", "amplitude": 4` expands at recipe load) is one option; macro-style (`"@ripple": {...}`) is another. Pick one as part of the V3 schema discussion.
2. **Where do composite templates live?** Probably alongside the signal facade — a sibling module in `tui_vfx_recipes`. Could even share the autogen pipeline.
3. **Is the §5 follow-on work part of V3 or a separate Slice?** Argument for V3: schema cohesion. Argument for separate: V3 is already large; new trait surfaces deserve their own packet.
4. ~~**Does the four-stage taxonomy need a fifth slot?**~~ **Resolved (2026-04-27).** The honest answer is *two* slots, not one, and they sit *around* the per-cell pipeline rather than inside it. See §11. Today's hardcoded shadow path is the canary; glow, outline, vignette, scanline, backdrop blur, and scene-layer underlays/overlays are the same shape and would otherwise each require their own fork.

## 10. Decision (2026-04-26)

**Accepted:** Model B (named stages: sample → mask → shade → filter, with within-stage layered chaining). Model A (free-form graph) rejected for tui-vfx's lifetime per the §3 cost analysis — the per-cell traversal tax doesn't pay for itself on a 16ms terminal budget at ~10K cells/frame.

**Consequences:**
- §5 follow-on moves are ready to schedule. Per `docs/design/tui-vfx-2026-04-26-handoff-outstanding.md` §8.3, the order is: resolved-coord fields on `VfxCellContext` first (smallest, demonstrates the bundle pattern is the right place to grow per-cell context), filter-discard bit second, composite-effect templates third.
- V3 schema vocabulary locks Model B's stage names per §7. The composite-effect template syntax (open question §9.1) becomes part of the V3 schema discussion.
- The four element stages (Sampler / Mask / StyleShader / Filter) stay in fixed order. The "fifth slot" question (§9.4) is resolved separately by §11 — *around* the per-cell pipeline, not inside it.

---

## 11. Decision (2026-04-27) — Pre-pass / post-pass slots around the four-stage core

**Accepted:** Generalize today's hardcoded shadow path into a closed two-slot framework around the canonical four-stage element pipeline. Shadow becomes the first pre-pass; glow, vignette, scanline overlays, backdrop blur, motion trails, and scene-layer underlays/overlays land naturally in the same framework.

### 11.1 The pipeline shape

```
  recipe.pre_passes[]   →  Shadow, Outline, Reflection, BackdropBlur, …
            ▼
  ┌─────────────────────────────────────┐
  │ ELEMENT PIPELINE (canonical 4)      │
  │   Sampler → Mask → Shader → Filter  │   (per-cell, unchanged)
  └─────────────────────────────────────┘
            ▼
  recipe.post_passes[]  →  Glow, Vignette, Scanline, MotionTrail, …
            ▼
                       dest
```

Pre-passes and post-passes operate on *buffers*, not per-cell. They have a different shape from the four element stages — `buffer in → buffer out + blend mode + canvas extent` — and live in their own trait family.

### 11.2 The closed slot taxonomy

Six slots, total. Closed vocabulary, in the same discipline as `Scope`:

```
  pre_pass
  element.sampler
  element.mask
  element.shader
  element.filter
  post_pass
```

Adding a seventh slot requires a fresh proposal with a concrete driver, the same way §9.4 was treated. AI authors and contract-discovery tooling can hold the full set in head.

### 11.3 Why pre/post and not a fifth-step-in-the-loop

Three properties make these operations structurally different from the per-cell pipeline:

1. **Generated content.** A pre-pass can synthesize source from nothing (shadow has no source cell to sample). The four-stage pipeline assumes you start from a source cell.
2. **Extended canvas.** Pre/post-passes may operate on a canvas larger than `element_rect` (shadow extrudes beyond it; backdrop blur reads dest beyond it).
3. **Destination-aware blending.** Composite modes read the existing dest cell to mix under or over it. The four-stage pipeline only writes; it never reads dest.

A "fifth step" in the per-cell loop cannot honestly carry any of these. Forcing it would either (a) leak buffer-level concepts into a per-cell trait, or (b) silently grow the loop into a graph runtime, which §10 already rejected.

### 11.4 Trait families

The four element stages keep their existing traits (`Sampler`, `Mask`, `Shader`, `Filter`). Two new trait families land:

| Trait | Slot | Shape |
|---|---|---|
| `PrePass` | `pre_pass` | `(canvas_extent, generate(buffer)) → buffer; declares blend mode for composition under element` |
| `PostPass` | `post_pass` | `(canvas_extent, transform(buffer, dest)) → buffer; declares blend mode for composition over element` |

The shadow code in `crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs::render_pipeline_with_shadow` is the working sketch of the `PrePass` shape; formalize it into the trait, then port shadow onto it as the first instance.

### 11.5 Slot applicability — trait shape implies the slot for free

Most primitives are slot-locked by their trait shape. No metadata is required for the common case:

```
  impl PrePass  ⇒ valid only in pre_pass
  impl Sampler  ⇒ valid only in element.sampler
  impl Mask     ⇒ valid only in element.mask
  impl Shader   ⇒ valid only in element.shader
  impl Filter   ⇒ valid only in element.filter
  impl PostPass ⇒ valid only in post_pass
```

(Listed in pipeline-execution order, top to bottom — pre-pass first, four element stages, post-pass last. Same convention applies everywhere in this doc and downstream authoring docs: never list slots in alphabetical or trait-introduction order, because pre-pass gets missed when it's not at the top.)

The schema validator gets slot-correctness as a side-effect of typing.

### 11.6 Multi-slot primitives — explicit declaration where the trait is ambiguous

A small subset of primitives legitimately occupy more than one slot. They declare their applicability in the primitive registry:

```
  applicable_slots: ["element.shader", "post_pass"]
```

Examples in this category: `ColoredOverlay` (per-cell shader vs. whole-frame tint), `Pattern` (sampler redirect vs. backdrop generator), `Noise` (shader modulator vs. pre-pass texture).

### 11.7 Naming discipline — family-named distinct primitives over slot-modulated semantics

When the same conceptual operation has meaningfully different semantics per slot, prefer **distinct family-named primitives** (`CellTint` for shader, `FrameTint` for post-pass) over a single name with slot-dependent behavior. Reasons:

- Flat contract discovery — each name has exactly one semantic.
- Easier corpus search across the 500+ recipe library.
- AI-authoring friendly — no slot-context reasoning required to predict behavior.

Reserve same-name slot-modulated primitives for cases where the operation is genuinely identical across slots (rare).

### 11.8 Slot validation lives on the contract-discovery surface

The validator already reports recipe substitution and binding requirements (MARKETING tertiary 19). Slot occupancy and primitive applicability extend the same surface — one more table, same authoring guide, same introspection API. Do not build slot validation as a separate subsystem.

### 11.9 Consequences

- The shadow pathway in `render_pipeline_with_shadow` is the canary, not the contract. After the `PrePass` trait lands, shadow ports onto it and `render_pipeline` loses its hardcoded shadow fork.
- Pipeline observability (Unit A) gets a uniform per-pass entered/finished pair instead of shadow-specific wiring.
- The V3 schema gains two new top-level recipe fields (`pre_passes`, `post_passes`), each an ordered list. Default is empty for both — recipes that don't need passes are unaffected.
- Open question §9.4 is resolved.
- Effects that have been waiting for a home (glow, outline, vignette, scanline overlay, backdrop blur, motion trails) become schedulable.

### 11.10 Non-goals (carried forward from §8 and refined)

- The four element stages remain in fixed order. Pre/post passes do not unlock cross-stage reordering inside the element pipeline.
- No node-graph runtime. Pre/post passes are an ordered list, not a DAG with typed ports between passes.
- Pre/post passes do not introduce structural signals. The list of passes is recipe-static; signals continue to fill parameter slots within passes.
- No plugin loader. Pre-pass and post-pass primitives are Rust impls compiled into the binary, like every other primitive.

---

## 12. Deliverables to land §11

§11 is not done when the trait compiles. It is done when every consumer surface — schema, validator, debug tooling, authoring docs, autogen reference, and the recipe corpus — agrees that pre/post passes are first-class. The list below is the closed set of deliverables; nothing here is optional.

### 12.1 Trait surface (tui-vfx-types or tui-vfx-compositor)

- Define `PrePass` trait: `(canvas_extent, generate(buffer)) → buffer + blend mode`.
- Define `PostPass` trait: `(canvas_extent, transform(buffer, dest)) → buffer + blend mode`.
- Define `BlendMode` enum (initially: `GlyphOverlay`, `GradeUnderlying`, `BlendUnderlying`, `Additive`, `Screen`; lifted from existing shadow `ShadowCompositeMode` and extended).
- Define `CanvasExtent` shape (`Element` for same-rect, `Extruded { extra_w, extra_h, offset_x, offset_y }` for shadow-style passes).

### 12.2 Compositor pipeline port (tui-vfx-compositor)

- `render_pipeline` gains an ordered driver: `pre_passes → element pipeline → post_passes`.
- Port `Shadow` onto `PrePass` as the first concrete pre-pass primitive.
- Delete the hardcoded shadow fork in `render_pipeline_with_shadow` once the port reaches rendering-equivalence with the legacy path (see §12.10 release gate).
- The four-stage element pipeline is unchanged.

### 12.3 V3 draft schema (`docs/design/tui-vfx-v3-schema-draft.json`)

- Add `pre_passes: []` and `post_passes: []` as top-level recipe fields with documented annotations.
- Add a worked-example pre-pass entry (shadow, since it's the canary).
- Add a worked-example post-pass entry (vignette or scanline overlay — pick one and ship it).
- Per-pass entry shape: `{ kind, payload, blend_mode, canvas_extent }`.
- Update the document's TOP-DOWN AUTHORING MODEL block to name pre/post passes as recipe-level slots.
- Document the six-slot taxonomy in the comment header.

### 12.4 Annotated schema (the same draft file's `#` annotations)

- Per-field rationale comments on `pre_passes` / `post_passes`: why they exist, when to use them, and the closed slot taxonomy.
- Cross-reference §11 of this doc by path.
- Document the `applicable_slots` registry surface where a primitive declaration lives (multi-slot primitives only).

### 12.5 Recipe schema validator (tui-vfx-recipes / pipeline-validator)

- Reject element-stage primitives placed in `pre_passes` / `post_passes`.
- Reject pre/post-pass primitives placed in element-stage arrays.
- Honour `applicable_slots` for multi-slot primitives; reject misplacement.
- Reject empty / malformed canvas extents on extruded passes (e.g. zero offset, negative extras).
- Extend the contract-discovery output: report which slots a recipe occupies and which primitive families.
- Strict-contracts mode: reject recipes that reference a primitive name without a registered slot affinity (no silent default).

### 12.6 Debug tooling (tui-vfx-debug, tui-vfx-probe, pipeline-validator)

- `PipelineStageKind` gains `PrePass` and `PostPass` variants. Existing `Shadow` variant deprecates and aliases to `PrePass { kind: "shadow" }` during the migration window, then is removed.
- Pipeline observability (Unit A): pre-pass and post-pass entered/finished pairs emitted via the shared per-stage helper. The shadow-specific stage emit folds into this.
- `CompositorInspector` callbacks: `on_pre_pass_entered`, `on_pre_pass_finished`, `on_post_pass_entered`, `on_post_pass_finished`. Per-cell `on_shadow_cell_applied` generalizes to `on_pre_pass_cell_applied` (or stays specialised — pick one and document the choice in the rustdoc).
- `pipeline-validator --probe` dumps per-pass buffers at named stages alongside per-element-stage probes.
- `--debug-recipes-qc` fingerprints include pre/post-pass output so drift detection covers passes too.
- Trace events: per-pass blocks emitted with the same step_id discipline existing stages use. No special-casing.

### 12.7 Authoring docs

- New top-level section in the V3 authoring guide (sibling of "Effects", "Scopes", "Signals"): **Passes** — explains the six-slot model, when to reach for a pre-pass vs. a post-pass, and how passes interact with masks (writeback gate semantic).
- Per-primitive authoring guide entries gain a `Slot:` line listing the slot(s) the primitive is valid in. Single-slot primitives quote one; multi-slot primitives quote all and explain the per-slot semantic.
- Document the family-named distinct primitive convention (`CellTint` vs `FrameTint`) and when to apply it.
- Migration note: V2 `shadow:` field becomes V3 `pre_passes: [{ kind: "shadow", ... }]`.

### 12.8 Rustdoc + autogen pipeline

- `PrePass`, `PostPass`, `BlendMode`, `CanvasExtent` get full rustdoc on the trait/type and every public item: purpose, contract, expected canvas semantics, blend-mode interaction, examples.
- Every new primitive impl carries rustdoc on its public methods describing slot semantic and any per-slot behavior differences.
- `docs/templates/capabilities.toml` gains:
  - The two new trait families as capability categories.
  - The six-slot taxonomy as a vocabulary entry.
  - Per-primitive slot-applicability metadata for the capabilities manifest.
- `cargo xtask docs generate` regenerates `docs/generated/` so the capability manifest, schema reference, and authoring reference all reflect §11. CI freshness check (`cargo xtask docs check`) catches drift.
- The signal facade autogen (`docs/templates/signals.toml`) is unaffected; passes consume signals through the same `ParamValue<T>` surface.

### 12.9 V2 → V3 lowering and corpus migration

- `docs/design/tui-vfx-v3-upgrade-plan/57_v2_to_v3_lowering_rules.md` gains a rule: V2 top-level `shadow:` lowers to a single-entry `pre_passes:` array in V3.
- The migration script (xtask or in-tree tooling) mechanizes the lowering across the recipe corpus.
- Recipes that use shadow-shaped effects ad-hoc (e.g. inlined scene-layer underlays) get migrated to pre/post-pass form as part of the cutover. The migration log records every change.

### 12.10 Tests and release gates (`60_testing_release_gates.md`)

- Unit tests for `PrePass` / `PostPass` traits (round-trip, canvas extent, blend mode contract).
- Pipeline integration tests confirming the three-block ordering: `pre_passes → element pipeline → post_passes`.
- Validator tests for slot-misplacement rejection (every misplacement category gets one test).
- Probe/trace fixture tests covering pre-pass and post-pass entered/finished emission.
- **Rendering-equivalence release gate (Concern F discipline):** the pre-pass shadow port must produce a rendered output identical to the legacy `render_pipeline_with_shadow` fork across the full debug-recipes corpus, fingerprint-checked. The legacy fork stays in-tree until the gate is green; only then does it delete.
- Probe-fidelity gate: pre/post-pass probe events must round-trip through the trace recorder with the same step_id / payload discipline as element-stage events.
- Performance gate: `bench_full_trace_60fps` continues to pass with one shadow pre-pass + one post-pass primitive (vignette) active. The 16.7 ms / 60 fps budget is non-negotiable.

### 12.11 Memory hygiene

- No partial implementation. The slot taxonomy is closed and curated; every slot must be wired through the full stack (trait → schema → validator → debug → authoring → autogen → tests). Partial coverage is stop-and-ask, not silent default.
- No "accepted but inert" schema fields. `pre_passes` and `post_passes` ship fully wired in the same V3 phase that introduces them, or they don't ship.
- Rustdoc is updated in the same change that adds or modifies any public item touched by §11; autogen runs and freshness check passes before the change merges.

<!-- <FILE>docs/design/tui-vfx-effect-composition-model.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.4.0</VERS> -->
