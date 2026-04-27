<!-- <FILE>docs/design/tui-vfx-effect-composition-model.md</FILE> - <DESC>Architectural framing: named stages vs arbitrary chaining for effect composition; honest assessment under terminal constraints; recommendation + implications for the signal facade</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>User direction 2026-04-26: signal facade decision is downstream of a larger question about effect-composition model. Articulate the two models, evaluate against terminal constraints, recommend stance, then make the signal facade fit the chosen model.</WCTX> -->
<!-- <CLOG>0.1.0: initial draft — two models with ANSI diagrams, terminal-constraint analysis, recommendation (keep named stages + expand within-stage chaining + treat signals as parameter values not graph nodes), implications for the recipe-signals facade.</CLOG> -->

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
- **No cross-stage reordering** (e.g. "filters run before masks in this recipe"). The four stages stay in fixed order.
- **No structural signals.** Signals don't drive layer count, effect type selection, or recipe topology.
- **No plugin loader.** Effects continue to be Rust impls compiled into the binary; recipes don't load `.so`/`.dll` plugins.

These are explicit non-goals. Future re-evaluation requires a fresh proposal with concrete user demand, not speculative flexibility.

---

## 9. Open questions

1. **Composite-effect template syntax.** SCSS-mixin-style (`"type": "ripple", "amplitude": 4` expands at recipe load) is one option; macro-style (`"@ripple": {...}`) is another. Pick one as part of the V3 schema discussion.
2. **Where do composite templates live?** Probably alongside the signal facade — a sibling module in `tui_vfx_recipes`. Could even share the autogen pipeline.
3. **Is the §5 follow-on work part of V3 or a separate Slice?** Argument for V3: schema cohesion. Argument for separate: V3 is already large; new trait surfaces deserve their own packet.
4. **Does the four-stage taxonomy need a fifth slot?** Today's pipeline has Sampler / Mask / StyleShader / Filter. Some effects feel like they want a "compositor" or "blend" stage at the very end. Probably not, but flag for review during V3 design.

## 10. Decision (2026-04-26)

**Accepted:** Model B (named stages: sample → mask → shade → filter, with within-stage layered chaining). Model A (free-form graph) rejected for tui-vfx's lifetime per the §3 cost analysis — the per-cell traversal tax doesn't pay for itself on a 16ms terminal budget at ~10K cells/frame.

**Consequences:**
- §5 follow-on moves are ready to schedule. Per `docs/design/tui-vfx-2026-04-26-handoff-outstanding.md` §8.3, the order is: resolved-coord fields on `VfxCellContext` first (smallest, demonstrates the bundle pattern is the right place to grow per-cell context), filter-discard bit second, composite-effect templates third.
- V3 schema vocabulary locks Model B's stage names per §7. The composite-effect template syntax (open question §9.1) becomes part of the V3 schema discussion.
- The four-stage taxonomy (Sampler / Mask / StyleShader / Filter) stays. A fifth "compositor"/"blend" slot (open question §9.4) is not added without a concrete driver.

<!-- <FILE>docs/design/tui-vfx-effect-composition-model.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
