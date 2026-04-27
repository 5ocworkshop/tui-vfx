<!-- <FILE>docs/design/tui-vfx-2026-04-26-handoff-outstanding.md</FILE> - <DESC>Handoff capture of outstanding work from the 2026-04-26 Phase F session — single source so the next session can pick items off one at a time</DESC> -->
<!-- <VERS>VERSION: 1.4.0</VERS> -->
<!-- <WCTX>§8.7 probe-fidelity fix — mark item 9 done</WCTX> -->
<!-- <CLOG>1.4.0: mark §8.7 item 9 done — probe-fidelity regression fixed; `infer_roles_from_grid` replaces all-Background placeholder in `run_probe`; `btop_focused_row_demo.json` now returns modified_cells: 180; regression test added</CLOG> -->

# Handoff — outstanding items as of 2026-04-26

> **Scope.** This document captures everything that was *surfaced but not closed* during the 2026-04-26 Phase F session. Each item is independent — pick them off in any order. Use this as the queue for the next session(s).
>
> **What shipped this session (no action needed):** Slice 6.6 Phase F.0–F.8 (TransformContext + VfxCellContext bundle across Filter/Mask/Sampler/StyleShader). All five sibling-trait migrations + cleanup landed at commits `51f5204`, `c883683`, `b628abd`, `5535b0e`, `7821815`, `5249550` on tui-vfx. See `docs/design/tui-vfx-buy-once-architecture-sweep.md` §Top 1 for the closing record. Builds clean across all three repos; tui-vfx and tui-vfx-recipes are 100% green.

## Index

| # | Item | Repo | Severity | Estimated effort | Status |
|---|---|---|---|---|---|
| 1 | `focused_row_btop` recipe doesn't visually distinguish selected row | gt-design | **P1 — real product bug** | half-day to a day | **Deferred — gt-design work; auto-closes after items 7 or 8 land. See §8.6.** |
| 2 | Decide effect-composition model (named stages vs free-form graph) | tui-vfx | architectural decision | one decision + small doc bump | **DECIDED 2026-04-26 — Model B accepted. Acceptance note appended to `tui-vfx-effect-composition-model.md` v0.2.0.** |
| 3 | Decide signal-facade placement + scope (`tui_vfx_recipes::signals`) | tui-vfx-recipes | architectural decision | one decision + Slice α/β packets | **DECIDED 2026-04-26 — Option A accepted; γ Q4 = `VfxRecipeSignalSpec` (Vfx prefix per Intention 8). α/β + γ executors both in flight. Catalog source-of-truth split: γ owns runtime catalog inline in code; α/β owns editorial overlay in `signals.toml` (Core 12 list + enrichment). Phase δ still gated on γ landing. Audit caught two naming inaccuracies in the proposal doc: "Noise" → `PerlinNoise`, "DampedSpring" lives outside `SignalSpec` in the parallel motion-spec channel — γ documents the conceptual collapse without physically removing the motion-spec route.** |
| 4 | Three Model B follow-on moves (composite-effect templates, filter-discard bit, resolved-coord fields) | tui-vfx | future Slice work | one Slice each | **Gated on item 2. Per-move recommendations in §8.3.** |
| 5 | Buy-once sweep findings still queued | tui-vfx | future Slice work | per-finding | **1.2.B + 1.3.A DONE; 1.2.A + 1.7.A DONE 2026-04-26 (one slice — VfxBindable<T, S> in tui-vfx-core, three concrete types aliased, originals recyclebinned); other tier-2 findings tracked in §8.4.** |
| 6 | Pipeline observability bus + `vfx-inspect` tool | tui-vfx | architectural foundation | per-phase | **IN FLIGHT (parallel session). Spec at `tui-vfx-pipeline-observability.md`.** |
| 7 | Migrate gt-design recipe stack to V3 schema awareness | gt-design | unblocks item 1 + future V3-only recipes | multi-day packet | **Packet written 2026-04-26: `tui-vfx-2026-04-26-packet-gt-design-v3-stack-migration.md` (785 lines). Q4 recommends bundling item 8 (producer fix) into this packet. Deferred per leader direction until tui-vfx family is finished. See §8.5. **Sequencing: item 12 (observability Unit B) lands FIRST in this lane** — gives diagnostic truth on gt-design side, aiding everything that follows.** |
| 12 | Observability Unit B — wire gt-design factory into the inspection bus + delete `factory_trace_composition_preview` + swap `GTD_TRACE_RENDER=1` to read the production event stream | gt-design + tui-vfx | trace-fidelity payoff of the observability bus | one slice (per Unit A's deferral note) | **Held 2026-04-26 per leader direction: gt-design lane stays paused until tui-vfx-family executors converge. **When that lane reopens, Unit B is the first slice** — the diagnostic-truth machinery it lands aids item 7's V3 migration audit (every regression caught by the bus instead of by re-deriving from prose trace).** |
| 8 | Producer-side fix: `ContentShell::card` tags every cell `Surface` instead of distinguishing inner content from border | gt-design | architectural debt V3 inherits | half-day | **Folded into item 7 packet per agent recommendation Q4=A. Co-execution rationale documented in the packet.** |
| 9 | Probe-path fidelity regression: `pipeline-validator --probe` reports `modified_cells: 0` for known-working canonical schema_v1 recipes (e.g., `recipes/btop_focused_row_demo.json`) | tui-vfx-recipes + tui-vfx | **silent diagnostic lie** (broader than initially scoped — V3 path also affected) | small fix, no bisect needed | **FIXED 2026-04-26. Root cause: `tui-vfx@ec872a4` A.2 cutover left `RoleMap::all_background` placeholder in `run_probe` (Sub-plan C TODO that never landed). Fix: replaced placeholder with `infer_roles_from_grid` in `orc_run_probe.rs`; new helper `fnc_infer_roles_from_grid.rs`; regression test `tests/test_probe_fidelity_role_scoped.rs`. `btop_focused_row_demo.json` now returns `modified_cells: 180`.** |
| 10 | `--runtime-params-json` silently dropped on schema_v1 recipes (documented as wired only for compiled-V3 paths) | tui-vfx-recipes | small UX trap | small | **FIXED 2026-04-26 evening (executor; ~50 LOC, unstaged for review). Diagnostic `runtime_params_dropped` (severity: warning) emitted at `tools/pipeline-validator/src/fnc_run_probe_mode.rs` post-build mutation; new test `cli_v1_probe_mode_emits_runtime_params_dropped_warning` at `tools/pipeline-validator/tests/test_schema_v1_probe_diagnostics.rs`. Manual reproduce confirmed.** |
| 11 | Legacy `region: TextOnly` lowering produces `Role(Text)` scope, not the content-equivalent V3 produces | tui-vfx-recipes | architectural mismatch | small if standalone, free if folded into V3 cutover | **New finding. Recommended path in §8.9.** |

---

## 1. `focused_row_btop` recipe — selected row not visually distinguished (P1)

**Reproducer:**
```bash
cd /usr/projects/gt-design && cargo test -p gtd-ratatui --test test_content_shell \
  test_content_shell_pipeline_override_without_animation_hints_uses_dwelling_effects \
  test_content_shell_stateful_pipeline_override_without_animation_hints_uses_dwelling_effects
```

Both fail with the same assertion:

```
assertion `left != right` failed: selected row should differ from the previous row when the focused-row recipe is active
  left: Rgb(245, 248, 252)
 right: Rgb(245, 248, 252)
```

at `crates/gtd-ratatui/tests/test_content_shell.rs:336` (the `assert_selected_row_differs_from_neighbors` helper). Row 3 (the selected row) renders the same fg as rows 2 and 4.

**The recipe under test:**
- `themes/eichler/recipes/focused_row_btop.json` — the override key the test sets is `pipeline_override_key("focused_row_btop")` (lines 182 and 211 of `test_content_shell.rs`).
- Runtime params: `selected_row = 3_u16` (via `focused_row_runtime_params()` at `test_content_shell.rs:310`).

**Verified pre-existing.** The test was checked against pre-Phase-F tui-vfx (`git checkout 6554947` for tui-vfx, run the gt-design test) — fails identically. Phase F is not the cause.

**Prior context — DF-010.** `df-tickets/DF-010-focused-row-integrated-render-no-op.json` describes this exact pathology and claims resolution. The ticket's fix had two parts:
1. Stop `gtd-ratatui` from synthesizing an entering animation snapshot from `pipeline_override_key` alone (which made `focused_row_btop` resolve with zero shader layers).
2. Realign the vendored `focused_row_btop` recipe with the canonical btop foreground-brightness reference, preserving `selected_row_binding` for live viewport-local state.

The ticket says the resolution landed. The integration test failing today says either (a) the fix regressed, (b) the fix landed for `gitui` but not for the eichler-themed test path, or (c) the test reproduces a different code path that's still broken.

**Suggested first probes (no edits):**

```bash
# 1. Confirm the recipe is loadable and what shaders it declares
cd /usr/projects/gt-design && cat themes/eichler/recipes/focused_row_btop.json | head -80

# 2. Run pipeline-validator probe on the recipe — does the focused-row shader fire?
cd /usr/projects/tui-vfx-recipes && cargo run -q -p pipeline-validator -- \
  --probe --probe-causation --probe-widget-cell 4,3 --phase dwelling --sample-t 1.0 \
  -- /usr/projects/gt-design/themes/eichler/recipes/focused_row_btop.json

# 3. Use recipe-probe to compare expected vs actual cell color at the selected row
cd /usr/projects/tui-vfx-recipes && cargo run -q -p recipe-probe -- \
  /usr/projects/gt-design/themes/eichler/recipes/focused_row_btop.json \
  --phase dwelling --sample-t 1.0 --with-causation --widget-cell 4,3

# 4. Compare to the canonical btop reference if it still exists
ls /usr/projects/tui-vfx-recipes/recipes/btop_focused_row_demo.json 2>&1
# (DF-010 cited this path as the canonical btop foreground-brightness reference)
```

**Likely root-cause hypotheses, in order:**
1. The vendored `focused_row_btop` regressed back to the background-bar variant DF-010 fixed (look at recent commits touching the recipe).
2. The `selected_row` runtime param is reaching the recipe but not reaching the shader (the binding plumbing broke in some subset of the pipeline override path).
3. The shader is firing but writing to background rather than foreground (the test reads `cell_fg` at `test_content_shell.rs:347`).
4. The `entering`-animation-snapshot fix from DF-010 regressed and the recipe is again resolving with zero shader layers in the test's specific code path.

**Bonus — likely the same bug in the demo viewer.** User flagged 2026-04-26 that the focused-row recipe doesn't render properly in the demo viewer either. Same recipe, same code path; fixing the test will likely fix the demo. Worth checking the demo viewer at `examples/binding_lab/` (which has a comment at `cls_lab_state.rs:63` referencing the `focused_row_btop` shader's HLL coordinate read).

---

## 2. Decide effect-composition model (named stages vs free-form graph)

**Doc:** `docs/design/tui-vfx-effect-composition-model.md` v0.1.0 — companion proposal written this session.

**Recommendation in the doc:** Stay with Model B (named stages: sample → mask → shade → filter, with within-stage layered chaining). Reject Model A (free-form graph) for tui-vfx's lifetime — the per-cell traversal tax doesn't pay for itself on a 16ms terminal budget at ~10K cells/frame. Three surgical follow-on moves cover the real boundary-crossing cases without building a graph engine.

**What's needed:** explicit decision (acceptance, modification, or rejection) so item 4 below knows what to schedule. If accepted, the V3 schema redesign should lock Model B's vocabulary explicitly per §7 of the doc.

---

## 3. Decide signal-facade placement + scope (`tui_vfx_recipes::signals`)

**Doc:** `docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` v0.2.0 — companion proposal written this session.

**Recommendation in the doc:**
- **Placement:** Option A — in-crate module at `tui_vfx_recipes::signals::*` (no new crate; cannot be published independently). If the module ever outgrows reasonable size, promote to a sub-crate `tui-vfx-recipes-signals` (Option B) — mechanical conversion.
- **Scope:** narrow (recipe-deserialization only). Production code keeps importing `mixed_signals::*` directly; the facade does not intercept production paths.
- **Headline maintenance lever (user articulation):** locally-named/scoped interface point to drive recipe inputs; future swaps, plug-ins, exposure-limiting, and rename/remap stay in one place.

**Phasing (from §8.6 of the doc):**
- α: autogen `SIGNALS_REFERENCE.md` from SignalSpec rustdoc + `signals.toml` overlay (smallest; doc-only).
- β: curated "Core 12" cheatsheet (one section of α's autogen output).
- γ: build the `signals` module + collapse the parallel physics channel.
- δ: symmetric `BindableF32` / `BindableColor` family with signal-form variants pointing at `VfxRecipeSignalSpec`.

**What's needed:** explicit decision (yes/no on facade; yes/no on each phase). If accepted, α + β can land as one packet.

---

## 4. Three Model B follow-on moves (gated on item 2 acceptance)

From `docs/design/tui-vfx-effect-composition-model.md` §5. All three address real cases where Model B is awkward today; none require a graph engine.

| Move | Cost | Earns its place via |
|---|---|---|
| **Composite-effect templates** in the recipe schema (V3): `"type": "ripple"` declares a name + parameter schema; recipe-load expands it into the constituent layers (sampler + mask + shader). Like SCSS mixins. | medium; one expansion pass at recipe-load | covers the boundary-crossing ergonomics ("ripple is one effect, not three") without runtime cost |
| **Filter-discard bit** on `VfxCellContext` (or a return-type change on `Filter::apply`) | small | covers "filter wants to also gate visibility" — closes a real gap |
| **Resolved-coord fields** on `VfxCellContext` (`resolved_x`, `resolved_y`) populated after sampler stage | small; one field bump on the type Phase F just introduced | covers "mask wants to react to post-sampler coords" — cheap given the bundle pattern |

Each is a separate Slice. Not blocked by anything except the item 2 decision.

---

## 5. Buy-once sweep findings — re-audited 2026-04-26

OFPF audit during the 2026-04-26 evening session found that two items the morning handoff listed as queued had already shipped in tree. Updated state below.

| Finding | Risk | Recommendation | Status |
|---|---|---|---|
| **1.2.A** Bindable<T> generalization | L | Next slice | **Queued. Junior packet at `docs/design/completed/tui-vfx-2026-04-26-packet-1.2.A-bindable-generic.md` (671 lines). Three open architectural Q's await leader decision — see §6 below.** |
| 1.2.B Pool<T> generalization | S | Do now | **DONE — landed at tui-vfx commit `8cad7a2` "Collapse five sibling pool types into Pool<T> with aliases (1.2.B)". Canonical at `crates/tui-vfx-content/src/pool/cls_pool.rs`; five hand-rolled files retired to `recyclebin/`.** |
| 1.3.A VfxImageSource.image_name → BindableString | S | Do now | **DONE — landed at tui-vfx-recipes commit `e64cf56` "Lift VfxImageSource.image_name to BindableString (1.3.A)". Schema, compile-bridge resolver `resolve_image_source_bindings`, `MissingImageBinding` error variant, runtime `expect()`, and the `_bindable` debug recipe all in tree.** |
| 1.1.B Bindable*::evaluate signature unification | M | Wait for third trigger | Bundled into 1.2.A per packet §1.2.A.4 |
| 1.7.A BindableValue cross-crate home | S | Bundle into 1.2.A | Bundled into 1.2.A |
| (others — see sweep doc tier list) | various | various | Unchanged |

## 6. Open architectural questions — 1.2.A packet

The 1.2.A packet enumerates five questions; three need leader decision before any junior executes. Recommended defaults are in the packet. The most impactful is **Q3 (signal-arm shape)** — chosen carefully, the resulting shape governs the symmetric Bindable family for the project's lifetime.

| Q | Decision | Recommended default | Why it matters |
|---|---|---|---|
| Q1 | Home crate for `VfxBindable<T>` | `tui-vfx-core` (already depends on `mixed-signals`; both downstream crates already pull it) | Determines whether the signal-arm dependency lives in the consolidated home or stays split |
| Q2 | `Vfx*` prefix on the consolidated type | `VfxBindable<T>` per Intention 8 (wire-format crossing crates) | Locks naming policy for the whole symmetric family that follows |
| Q3 | Signal-arm parameterization | `VfxBindable<T, S = std::convert::Infallible>` (Never) | Never makes the unconstructable-Signal-arm a type-system invariant. `S = ()` makes it a footgun-by-discipline. Stop-and-ask trigger flagged in the packet. |

## 8. Recommended low-complexity / high-value next steps

This section excludes the two parallel work streams (1.2.A `VfxBindable<T>` consolidation; observability bus implementation against the shipped `tui-vfx-debug::inspection` surface). Everything below is what remains tracked here.

For each item: the smallest move that earns the most. Cost qualifier (S/M/L per the sweep doc convention). Value qualifier (what it unblocks or prevents).

### 8.1 Item 2 — Effect-composition model decision

**Doc:** `docs/design/tui-vfx-effect-composition-model.md` v0.1.0.
**Recommendation in the doc:** stay Model B (named stages), reject Model A (free-form graph). Per-cell traversal tax of A doesn't pay for itself on a 16ms terminal budget at ~10K cells/frame.
**Cost to decide:** S — one leader call, no audit needed. The doc's recommendation is well-grounded; absent a contrary consideration, accept-as-default is the cheapest move.
**Value:** unblocks item 4 (three follow-on moves) and locks the V3 schema vocabulary per §7 of the proposal doc. Until decided, V3 schema work that touches the composition surface stays on hold.
**Lowest-cost move:** accept the recommendation, append a one-line acceptance note to the proposal doc, mark item 4 ready.

### 8.2 Item 3 — Signal-facade decision

**Doc:** `docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md` v0.2.0.
**Recommendation in the doc:** Option A (in-crate `tui_vfx_recipes::signals` module), narrow scope (recipe-deserialization only). Production code keeps importing `mixed_signals::*` directly.
**Cost to decide:** S — same shape as 8.1.
**Value:** unblocks phase α (autogen `SIGNALS_REFERENCE.md` from SignalSpec rustdoc) and phase β (curated Core 12 cheatsheet) — both small, doc-only, immediately useful for AI-assisted recipe authoring.
**Lowest-cost move:** accept Option A, accept phase α + β as one packet (doc-only), defer γ + δ until the parallel-session 1.2.A lands (δ depends on the consolidated `VfxBindable<T>` shape).

### 8.3 Item 4 — Three Model B follow-on moves (gated on 8.1)

Per `docs/design/tui-vfx-effect-composition-model.md` §5. Ordered cheapest-first, which is also the order their gaps bite hardest:

| Move | Cost | Value | Ranking |
|---|---|---|---|
| **Resolved-coord fields on `VfxCellContext`** (`resolved_x`, `resolved_y` populated post-sampler) | S — one field-bump on the type Phase F just landed; the bundle pattern makes the addition mechanical | covers "mask wants to react to post-sampler coords" — closes a real downstream gap | **Do first** |
| **Filter-discard bit** on `VfxCellContext` (or a return-type change on `Filter::apply`) | S — one boolean field plus a sweep through the 30 Filter impls Phase F.5 already touched | covers "filter wants to also gate visibility" — closes a real gap | **Do second** |
| **Composite-effect templates in V3 schema** (`"type": "ripple"` declares a name + parameter schema; recipe-load expands into constituent layers, SCSS-mixin-style) | M — one expansion pass at recipe-load; needs schema design + template registry | covers boundary-crossing ergonomics ("ripple is one effect, not three") without runtime cost; biggest authoring win | **Do third** |

Each is independently shippable. None depend on the V3 cutover. The first two are particularly cheap because Phase F already established `VfxCellContext` as the bundled context type that flows through every stage.

**Lowest-cost move:** if 8.1 accepts Model B, schedule the resolved-coord fields move as the first slice — it's the smallest demonstration that the bundle is the right place to grow per-cell context.

### 8.4 Other queued sweep findings

From `docs/design/tui-vfx-buy-once-architecture-sweep.md` §4, excluding 1.2.A (in flight) and 1.2.B / 1.3.A (shipped):

| Finding | Risk | Existing recommendation | Low-cost / high-value commentary |
|---|---|---|---|
| **1.1.B** Bindable*::evaluate signature unification | M | Wait for third trigger; bundle into 1.2.A | Auto-resolved by 1.2.A landing. No standalone action. |
| **1.2.C** FontRegistry / AssetRegistry merge | S | Wait for third trigger | Two instances today, threshold is three. **Defer.** Add to a "watch list" for the next loader-shape change. |
| **1.6.A** `cls_filter_spec.rs` (2193 LOC) split-up | L | Next slice, but timing tied to V3 cross-family work that adds 5+ variants | **Defer until V3 cutover decisions** clarify which variants are landing. Splitting first risks the wrong cut lines; splitting after lets the additions inform the structure. |
| **1.7.A** BindableValue cross-crate home | S | Bundle into 1.2.A | Auto-resolved by 1.2.A landing. No standalone action. |
| **1.8.A** tui-vfx-core/schema cycles | S | Wait for third trigger | Two cycle pairs known today; defer per the recommendation. |
| **1.9.A** Hand-written ConfigSchema audit | M | Next slice (as validation infra) | **Cheap quick win:** write a lint/check that fails CI when a new `impl ConfigSchema` lacks a justification comment. ~1 hour of xtask code; prevents the proliferation pattern continuing. **Recommended for a slow-day session.** |

**Lowest-cost move across this group:** 1.9.A's lint check. Pure validation infra, no production-code touch, prevents the pattern. Everything else genuinely earns "wait."

### 8.5 Item 7 — gt-design V3 stack migration

**Status:** deferred per 2026-04-26 leader direction ("ignore gt-design for now, we haven't done the migration because we've not finished building the updated tui-vfx* family, that's our goal today").

**What it is:** gt-design currently has zero V3 awareness anywhere — `crates/gtd-ssot/src/resolve/fnc_resolve_recipes.rs:55` dispatches every recipe through V2 `from_value`; no `from_value_v3`, `V3RecipeDocument`, `NormalizedRecipe`, or compiled-V3 references in the tree. Migration touches: loader dispatch, downstream payload consumers (factory, ratatui integration, theme builder), the compiled-V3 bridge, and schema migration for every gt-design vendored recipe.

**Cost:** L — multi-day packet. Not bug-fix-shaped.

**When to start:** after 1.2.A lands, after the observability bus lands, after Model B follow-ons (item 4) land. The tui-vfx family is the supplier; gt-design is the first consumer of the finished supply chain.

**Lowest-cost preparation while waiting:** pre-write the migration packet (junior-ready, like the 1.2.A packet). The audit work (where does V2 `from_value` get called, what downstream consumers touch the payload, which vendored recipes need migration) doesn't change between now and execution; capturing it as a packet means the eventual execution is mechanical. Estimated packet size: 600–800 lines, similar to the 1.2.A packet. Can be written by a sub-agent.

### 8.6 Items 1 + 8 — `focused_row_btop` bug + `ContentShell::card` producer fix

**Diagnosis (2026-04-26 evening, full transcript in conversation history):** The recipe targets `Role(Text)` cells. `ContentShell::card`'s structural render tags every cell with `Surface` → `RoleTag::Background`. Production uses the explicit role map (`semantic_buffer.to_role_map()` returns all-Background). Shader's scope matches zero cells. Trace's diagnostic preview disagrees because it uses the geometric-inference fallback (`apply_composition` not `apply_composition_with_roles`), so the preview shows fg_changes=228 while the actual surface stays plain.

**Two fix paths exist; the right one depends on item 7's order:**

- **Path A (after item 7 lands):** migrate the recipe to V3 schema with content-scope (`{"kind": "content", "value": "text"}`). Content-scope is glyph-based and role-independent. Sidesteps the producer-tagging bug entirely for this recipe. Doesn't fix the producer-contract violation for any other role-scoped recipe.
- **Path B (independent of schema version):** fix `ContentShell::card` (and the `tag_scoped_semantics` integration call site at `crates/gtd-ratatui/src/integration/fnc_tag_interaction_scoped_semantics.rs:111`) to tag inner cells with `SemanticRole::Content` (lowers to `RoleTag::Text`) and border cells `Border`. ~50 LOC plus a regression test asserting `RoleMapMaterialized.histogram` for a card render contains text-tagged inner cells.

**Recommendation:** do both. Path B is the right architectural fix (Stage-C contract: explicit roles must be at least as rich as inferred ones), regardless of whether the recipe stays V2 or moves to V3. Path A is correct cleanup once item 7 lands. They are not mutually exclusive.

**Lowest-cost move:** when item 7 starts, fold path B into the same packet — both touch gt-design's role-tagging surface.

### 8.7 Item 9 — Probe-path fidelity regression ✓ DONE

**Fixed 2026-04-26.** Root cause: `tui-vfx@ec872a4` (2026-04-20, A.2 cutover) introduced `RoleMap::all_background` as a placeholder in `run_probe`; the A.2 CLOG named it a Sub-plan C TODO that never landed. Any role-scoped scope predicate (`Role(Text)` from `region: "TextOnly"`) matched zero cells regardless of shader correctness.

**Fix applied:** replaced the all-Background placeholder in `crates/tui-vfx-probe/src/orc_run_probe.rs` with a call to `infer_roles_from_grid` (new `crates/tui-vfx-probe/src/fnc_infer_roles_from_grid.rs`). The helper assigns `RoleTag::Text` to non-whitespace glyph cells, `RoleTag::Background` to blank cells. Coordinate-scoped recipes (region: All, Row, etc.) are unaffected. V3 content/text scope is unaffected (compiles to coordinate list, not role predicate).

**Verification:** `recipes/btop_focused_row_demo.json` now produces `modified_cells: 180`; `themes/eichler/recipes/focused_row_btop.json` also produces 180. Regression test added: `tests/test_probe_fidelity_role_scoped.rs` in `tui-vfx-recipes`.

**Symptom (verified 2026-04-26 evening):** `cargo run -q -p recipe-probe -- /usr/projects/tui-vfx-recipes/recipes/btop_focused_row_demo.json --phase dwelling --sample-t 1.0 --with-causation --widget-cell 4,3` returns `modified_cells: 0` despite `shader_count: 1` and a recipe that DF-010's evidence (2026-04-14) demonstrated as actively producing per-cell color changes.

**Why it matters:** when the probe lies about whether a recipe modifies cells, the entire validator workflow loses signal. "Probe says 0" no longer means "shader broken"; it means "either shader broken OR probe regressed, indistinguishable from the CLI." The whole DF-010-style investigation pattern is degraded.

**Cost to fix:** S–M. Git-bisect from `tui-vfx-recipes` master at DF-010 close (2026-04-14) forward through `src/probe/`, `tools/recipe-probe/`, and `tools/pipeline-validator/`. The bisect itself is mechanical; the fix size depends on what regressed.

**Value:** restores the truthful-probe contract. Without this, the observability bus (parallel-session work) lands into a system where its closest comparable tool is unreliable — making bus-vs-probe parity tests less useful.

**Lowest-cost move:** run the bisect as a focused half-session. Likely candidate commits per the post-DF-010 log: `4326395 "Wire loopback merge into probe scene builders (L5 follow-on)"` is structurally suspicious as a probe-builder change. Start there.

**Sequencing:** ideally lands BEFORE the observability bus reaches its parity-test phase, so the bus has a working probe to A/B against. The parallel observability session may already have noticed this regression; coordinate.

### 8.8 Item 10 — `--runtime-params-json` silent drop on schema_v1

**Symptom:** `pipeline-validator --runtime-params-json '{"selected_row": 3}' -- <schema_v1_recipe>` accepts the flag, runs the recipe, and emits a probe report where the runtime binding shows `status: fallback_static` — the injected param was dropped without warning. Help text discloses the limitation ("currently wired for the compiled-V3 direct bridge runs"); the runtime does not.

**Cost to fix:** S. One of:
- Hard error when `--runtime-params-json` is passed with a schema_v1 recipe ("recipe must be migrated to V3 to honor runtime-params-json").
- Soft warning at probe-output emission time: a `diagnostic` entry with `code: runtime_params_dropped, severity: warning`.
- Best: thread the runtime params through the schema_v1 path too (matches the user's V3-first stance backwards: legacy paths should be honest about their limitations even while they exist).

**Value:** small but specific. Prevents the same one-hour-of-confusion this caused in the focused_row_btop investigation.

**Lowest-cost move:** add the soft-warning diagnostic. ~20 LOC in `tools/pipeline-validator/src/`. Can land as a one-shot fix any session.

### 8.9 Item 11 — Legacy `region: TextOnly` lowering to `Role(Text)`

**Architectural mismatch:** the legacy recipe field `region: TextOnly` is semantically a *content* predicate ("apply to text cells") but lowers to the `Role(Text)` *role* scope. Content and role are distinct cell properties; the lowering conflates them. The V3 canonical scope `{"kind": "content", "value": "text"}` got it right and the legacy lowering didn't follow.

**Two paths:**
- **Standalone fix:** change the legacy lowering to produce `Content(Text)` (or `And(Channel(Foreground), Content(NonEmpty))`). Audit every legacy recipe using `region: TextOnly`; verify the behavior change is "start working over more producers" not "start matching cells the author didn't want." Cost: M (audit-bound).
- **Free at V3 cutover:** when the V3 cutover ships and the legacy schema_v1 path is retired, the bug class disappears. Recipes that survived the migration use V3 content-scope from the start.

**Recommendation:** **defer to the V3 cutover.** Standalone fix is M-cost audit work for behavior the cutover deletes anyway. Track in the V3 cutover plan; no standalone packet needed.

**Lowest-cost move:** add a one-line note to `docs/design/tui-vfx-v3-upgrade-plan/00_INDEX.md` flagging this as a "free win at cutover" — so the cutover author is aware they're closing this defect.

## 7. New architectural artifacts written this session

*(Note: section number 7 was claimed late in the session after §8 was already written. File order is §1–§6, §8, §7, footer; numbering is correct for navigation.)*

- `docs/design/tui-vfx-pipeline-observability.md` — design spec for the `VfxObserver` event bus and the `vfx-inspect` tool. Motivated by the focused_row_btop investigation, where existing trace and probe tooling concealed the role-map mismatch for 30+ minutes. Includes: event taxonomy, sink contracts, cost model, plumbing requirements, inspector CLI, cultural commitments, and §11 open architectural questions for leader decision.
- `docs/design/completed/tui-vfx-2026-04-26-packet-1.2.A-bindable-generic.md` — junior-ready packet for 1.2.A with the three open Q's above, full code snippets, test plan, acceptance criteria, and rollback plan.
- `docs/design/completed/tui-vfx-2026-04-26-packet-1.2.B-pool-generalization.md` — verification + follow-on workbook (the underlying work shipped at `8cad7a2`; packet codifies the post-ship audit).
- `docs/design/completed/tui-vfx-2026-04-26-packet-1.3.A-vfx-image-source-bindable-string.md` — verification + follow-on workbook (underlying work shipped at `e64cf56`).

---

## What's known-good after this session (for orientation)

- **tui-vfx:** master at `ac2289a` ("Resolve build warnings — gate test-only helpers with #[cfg(test)]"). Build clean; zero warnings; 93 test suites OK; all doctests pass.
- **tui-vfx-recipes:** master at `92d57f4` ("Tolerate loopback_fire warnings in Madeira probe tests"). Build clean; 76 test suites OK; zero failures.
- **gt-design:** master at `384e211` ("Update command palette contract test for documented overshoot behavior"). Build clean (incl. all examples); 129 test suites OK; only 2 failures — both item 1 above (same root cause).
- **mixed-signals:** unchanged this session.

## Process notes for the next session

- The `feedback_clean_build_no_warnings.md` rule landed in memory this session: every code-touching agent prompt now includes a "warmup with ofpf-*" block AND a "build must be warning-free at commit time" verification block. Both are now standing project rules.
- The OFPF tooling sweep against gt-design / tui-vfx-recipes during F.6 confirmed neither sibling repo has any Filter/Mask/Sampler impls — the trait surface is fully self-contained inside tui-vfx-compositor. Cross-repo migrations for these three traits are no longer a concern.
- All design proposals written this session live under `docs/design/`. None of them have implementation packets yet — the next session decides which (if any) advance.

<!-- <FILE>docs/design/tui-vfx-2026-04-26-handoff-outstanding.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.4.0</VERS> -->
