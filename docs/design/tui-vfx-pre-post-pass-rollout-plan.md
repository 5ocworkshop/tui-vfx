<!-- <FILE>docs/design/tui-vfx-pre-post-pass-rollout-plan.md</FILE> - <DESC>Phased rollout plan for the pre/post-pass slot architecture decided in tui-vfx-effect-composition-model.md §11. Each phase is sized for one ralph run; phases end at clean build + green tests with a discussable artifact.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>2026-04-27: convert §12 deliverables into iteration-sized phases. Foundation traits, compositor port with parallel-running fallback, debug tooling, V3 schema + validator, authoring + autogen, V2→V3 lowering + corpus migration, cutover gate, first post-pass primitive.</WCTX> -->
<!-- <CLOG>0.1.0: initial draft — eight phases (A–H) with stories, acceptance, dependencies, risk; explicit non-goals; deferred-work pointer to archived Unit A handoff for Units B/C.</CLOG> -->

# tui-vfx pre/post-pass slot architecture — phased rollout plan

> **Companion to:** `docs/design/tui-vfx-effect-composition-model.md` §11 (decision) and §12 (deliverables list).
>
> **Sizing:** each phase below is sized for one ralph run. A phase ends at clean workspace build, green tests, and a discussable artifact. Phases A–H are the closed set; nothing else ships in this rollout.
>
> **Pacing:** the user controls cadence between phases. The plan does not assume continuous execution.

## Phase ordering and dependencies

```
                          ┌──────────────────────┐
                          │  Phase A — traits    │
                          └──────────┬───────────┘
                                     │
        ┌──────────────────┬─────────┼──────────────┬───────────────┐
        ▼                  ▼         ▼              ▼               ▼
  ┌──────────┐      ┌──────────┐  ┌─────────┐  ┌──────────┐   ┌──────────┐
  │ Phase B  │      │ Phase C  │  │ Phase D │  │ Phase E  │   │ Phase H  │
  │ port +   │      │ debug    │  │ schema +│  │ docs +   │   │ first    │
  │ shadow   │      │ tooling  │  │validator│  │ autogen  │   │ PostPass │
  └────┬─────┘      └──────────┘  └─────────┘  └────┬─────┘   └──────────┘
       │                                            │
       │                                            ▼
       │                                      ┌──────────┐
       │                                      │ Phase F  │
       │                                      │ lowering+│
       │                                      │  corpus  │
       │                                      └────┬─────┘
       │                                           │
       └─────────────┬─────────────────────────────┘
                     ▼
               ┌──────────┐
               │ Phase G  │
               │ cutover  │
               │  gate    │
               └──────────┘
```

A is foundation. B/C/D/E run in parallel after A. F depends on E. G is the cutover and depends on B + F. H is parallel-safe after A.

## Stage map

| Phase | Title | Maps to §12 | Risk |
|---|---|---|---|
| A | Foundation traits | 12.1 + part of 12.8 | low |
| B | Compositor pre-pass driver + Shadow port (parallel-running) | 12.2 (port) + part of 12.10 (gate fixture) | moderate |
| C | Debug tooling extension | 12.6 | low |
| D | V3 schema + validator slot enforcement | 12.3 + 12.4 + 12.5 | moderate |
| E | Authoring docs + autogen | 12.7 + 12.8 (autogen output) | low |
| F | V2→V3 lowering + corpus migration | 12.9 | moderate |
| G | Cutover (delete legacy fork after rendering-equivalence gate) | 12.2 (delete) + 12.10 (gate) + 12.11 | high |
| H | First post-pass primitive (proves the framework) | 12.1 (instance) + 12.7 + 12.8 | low |

---

## Phase A — Foundation traits

**Goal:** define `PrePass`, `PostPass`, `BlendMode`, `CanvasExtent` with full rustdoc. No consumers yet.

**Stories:**
- **US-A.1** — Define `BlendMode` enum (lift `ShadowCompositeMode` variants `GlyphOverlay`, `GradeUnderlying`, `BlendUnderlying`; extend with `Additive`, `Screen`). Home: `tui-vfx-types` or `tui-vfx-shadow`.
- **US-A.2** — Define `CanvasExtent` shape: `Element` for same-rect passes, `Extruded { extra_w, extra_h, offset_x, offset_y }` for shadow-style passes.
- **US-A.3** — Define `PrePass` trait. Methods document blend-mode contract, canvas-extent semantic, generation vs. transform distinction.
- **US-A.4** — Define `PostPass` trait. Methods document dest-aware blend contract, transform shape.
- **US-A.5** — Add traits and types to `docs/templates/capabilities.toml` so autogen picks them up. Run `cargo xtask docs generate`; verify reference output lists them.

**Acceptance:**
- `cargo build --workspace` clean.
- `cargo test --workspace` results identical to pre-phase baseline.
- `cargo xtask docs check` green.
- New traits appear in regenerated capability manifest.

**Risk:** low — no behavior change, no consumers.

---

## Phase B — Compositor pre-pass driver + Shadow port (parallel-running)

**Goal:** wire a `pre_passes → element → post_passes` driver into `render_pipeline`. Port `Shadow` onto `PrePass`. Keep the legacy `render_pipeline_with_shadow` fork in-tree as fallback; new driver runs only when `CompositionOptions.pre_passes` is non-empty.

**Stories:**
- **US-B.1** — Add `pre_passes: Vec<Box<dyn PrePass>>` and `post_passes: Vec<Box<dyn PostPass>>` to `CompositionOptions`. Default empty.
- **US-B.2** — Implement Shadow as a `PrePass` impl in `tui-vfx-shadow`. Reuses `render_shadow` and the existing blend helpers.
- **US-B.3** — Add the ordered driver in `render_pipeline`: walk pre-passes (extended-canvas buffers, blend modes), run element pipeline (unchanged), walk post-passes (dest-aware transforms). Legacy `render_pipeline_with_shadow` stays — pre-pass path is opt-in via `pre_passes` field.
- **US-B.4** — Rendering-equivalence test fixture across the existing shadow recipes. Same `CompositionOptions` modeled both ways (legacy `shadow:` vs. new `pre_passes:[Shadow]`); fingerprints must match cell-for-cell.
- **US-B.5** — `bench_full_trace_60fps` regression check with one pre-pass active.

**Acceptance:**
- Identical output across all current shadow recipes between legacy fork and pre-pass driver (cell-for-cell fingerprint).
- Legacy `render_pipeline_with_shadow` untouched.
- 60fps bench stays within criterion noise band.

**Risk:** moderate — shadow output equivalence is fiddly. The fixture is the gate.

---

## Phase C — Debug tooling extension

**Goal:** `PipelineStageKind` variants for `PrePass` / `PostPass`. Inspector callbacks. Probe events.

**Stories:**
- **US-C.1** — `PipelineStageKind` gains `PrePass` and `PostPass` variants. Existing `Shadow` variant stays as alias during the migration window (Phase G removes it).
- **US-C.2** — `CompositorInspector` callbacks: `on_pre_pass_entered`, `on_pre_pass_finished`, `on_post_pass_entered`, `on_post_pass_finished`. Default empty bodies.
- **US-C.3** — `InspectionSinkBridge` overrides the four new callbacks; emits matching `TraceEvent` variants via existing `self.emit()`.
- **US-C.4** — New driver in Phase B emits the per-pass entered/finished pairs. step_id discipline mirrors existing per-stage emit.
- **US-C.5** — Legacy `render_pipeline_with_shadow` shadow stage emit folds into `PrePass { kind: "shadow" }` shape (deprecation alias).
- **US-C.6** — `AssertingInspector` convenience constructors: `forbid_post_pass_class` for parity with existing `forbid_zero_cell_scope_matches`.
- **US-C.7** — Round-trip tests for the four new callbacks. Probe-fidelity test for pre/post-pass variants.

**Acceptance:**
- Existing observability tests still green.
- New tests confirm pre/post-pass entered/finished pairs emit with stable step_id ordering.
- Legacy shadow stage events still recorded under the alias.

**Risk:** low — same shape as the Unit A per-stage emit work.

---

## Phase D — V3 schema + validator slot enforcement

**Goal:** `pre_passes` / `post_passes` top-level fields in V3 draft. Slot-applicability registry. Validator slot rules. Contract-discovery surface extension.

**Stories:**
- **US-D.1** — `docs/design/tui-vfx-v3-schema-draft.json` gains `pre_passes: []` and `post_passes: []` top-level fields. Worked-example pre-pass (shadow) and post-pass (vignette stub) entries with annotations.
- **US-D.2** — Per-field annotation comments documenting the six-slot taxonomy (pipeline-execution order: pre_pass top → post_pass bottom).
- **US-D.3** — `applicable_slots` registry in `tui-vfx-recipes`. Single-slot primitives inherit from trait shape; multi-slot primitives declare explicitly.
- **US-D.4** — Validator rule: reject element-stage primitives placed in `pre_passes` / `post_passes` arrays.
- **US-D.5** — Validator rule: reject pre/post-pass primitives placed in element-stage arrays.
- **US-D.6** — Validator rule: honour `applicable_slots` for multi-slot primitives.
- **US-D.7** — Contract-discovery surface: report which slots each recipe occupies and which primitive families.
- **US-D.8** — Strict-contracts: reject primitives with no registered slot affinity (no silent default).
- **US-D.9** — Per-test fixture set covering each rejection class.

**Acceptance:**
- Draft schema validates with new fields.
- Validator rejects every misplacement category with a clear error.
- Contract-discovery API reports slot occupancy.
- Strict-contracts test fixture green.

**Risk:** moderate — touches `tui-vfx-recipes` deserialization and validator surface; both are downstream consumer touchpoints.

---

## Phase E — Authoring docs + autogen

**Goal:** "Passes" section in V3 authoring guide. Per-primitive Slot lines. capabilities.toml entries. Autogen run.

**Stories:**
- **US-E.1** — New top-level "Passes" section in V3 authoring guide. Sibling of "Effects", "Scopes", "Signals". Explains six-slot model, when to reach for pre vs. post, mask-as-writeback-gate semantic.
- **US-E.2** — Per-primitive guide entries gain `Slot:` line. Single-slot primitives quote one; multi-slot primitives quote all and explain per-slot semantic.
- **US-E.3** — Document the family-named distinct primitive convention (`CellTint` vs. `FrameTint`).
- **US-E.4** — Migration note: V2 `shadow:` field becomes V3 `pre_passes: [{ kind: "shadow", ... }]`.
- **US-E.5** — `docs/templates/capabilities.toml` gains entries for new traits, the six-slot taxonomy as a vocabulary entry, per-primitive slot-applicability metadata.
- **US-E.6** — Run `cargo xtask docs generate`. Verify capability manifest, schema reference, authoring reference all reflect §11.
- **US-E.7** — `cargo xtask docs check` (freshness gate) green.

**Acceptance:**
- Authoring guide renders with Passes section.
- Capability manifest includes pre/post-pass entries.
- Freshness check green.

**Risk:** low — additive to the existing authoring/autogen pipeline.

---

## Phase F — V2→V3 lowering + corpus migration

**Goal:** lowering rule for shadow. Mechanized migration across 92 affected recipes.

**Stories:**
- **US-F.1** — `docs/design/tui-vfx-v3-upgrade-plan/57_v2_to_v3_lowering_rules.md` gains rule: V2 top-level `shadow:` → V3 `pre_passes: [{ kind: "shadow", ... }]`.
- **US-F.2** — Migration script (xtask) that mechanizes the lowering. Idempotent; run repeatedly safe.
- **US-F.3** — Run migration across the 92 shadow-using recipes in `tui-vfx-recipes`.
- **US-F.4** — Probe-fidelity tests: migrated recipes produce identical output to pre-migration baseline (cell-for-cell fingerprint).
- **US-F.5** — Migration log entry recording per-recipe counts and any manual overrides.

**Acceptance:**
- 92 recipes migrated; mechanization log records each.
- Probe fingerprints match pre-migration baseline.
- Recipe-side `cargo test` green.

**Risk:** moderate — 92 files; mechanization required to avoid drift; equivalence proof is the gate.

---

## Phase G — Cutover (delete legacy fork after rendering-equivalence gate)

**Goal:** rendering-equivalence gate runs across the full migrated corpus from Phase F. On green, delete `render_pipeline_with_shadow` and the dispatch.

**Stories:**
- **US-G.1** — Run the rendering-equivalence test fixture from Phase B across the migrated corpus from Phase F.
- **US-G.2** — Compare fingerprints against pre-cutover baseline. Fail loudly on any mismatch.
- **US-G.3** — Once gate is green, remove `render_pipeline_with_shadow` and the `if options.shadow.is_some()` dispatch in `render_pipeline`. Remove `options.shadow` field if no consumer remains.
- **US-G.4** — Remove `PipelineStageKind::Shadow` alias variant.
- **US-G.5** — `bench_full_trace_60fps` post-cutover regression check.
- **US-G.6** — Cross-repo audit (`tui-vfx-recipes`, `gt-design`, `mixed-signals`): confirm no consumer remains pinned to legacy shadow surface.
- **US-G.7** — Final cutover commit. Optional Intention if a durable principle emerged from the work.

**Acceptance:**
- Every migrated recipe's rendering-equivalence fingerprint matches.
- 60fps bench within criterion noise band.
- Legacy fork code physically removed from tree.
- Cross-repo audit clean.

**Risk:** high — hard to reverse. Gate must be airtight before deletion.

---

## Phase H — First post-pass primitive (parallel-safe after A)

**Goal:** ship one `PostPass` impl to prove the framework. Vignette is the recommended candidate — well-defined output, terminal-friendly, single recipe parameter.

**Stories:**
- **US-H.1** — Implement Vignette as `PostPass`. Home: `tui-vfx-style` or new `tui-vfx-passes` crate (decide based on §12.1 placement).
- **US-H.2** — Worked-example recipe in `tui-vfx-recipes` exercising Vignette.
- **US-H.3** — Tests: rendering correctness, blend-mode contract, canvas-extent honoring.
- **US-H.4** — Rustdoc on `Vignette` and its public methods.
- **US-H.5** — `capabilities.toml` entry; autogen run.

**Acceptance:**
- Vignette produces a recognized vignette over the worked-example recipe.
- Autogen catalog lists Vignette under `PostPass`.
- Tests green.

**Risk:** low — self-contained; no other consumers.

---

## What this plan does NOT include

- **Other shadow-shaped primitives** (Glow, Outline, Reflection, BackdropBlur, MotionTrail, etc.). They follow the framework laid in Phases A–G but are separate ralph loops, sized one primitive per loop.
- **Plugin loader.** Out of scope per §8.
- **Cross-stage reordering inside the element pipeline.** Out of scope per §8.
- **Pipeline observability Unit B (gt-design wiring) and Unit C (tui-vfx-trace explain subcommands).** Tracked at `.omc/archive/2026-04-27-unit-a-pipeline-observability/` — separate work loops.
- **Recipe-signal facade Phases 1–3.** Tracked in `steering/work-packets/64-66` — separate work loops.

## Open questions for the user before starting

1. **Trait home.** Should `PrePass` / `PostPass` live in `tui-vfx-types`, `tui-vfx-compositor`, or a new `tui-vfx-passes` crate? Argues for `tui-vfx-types`: pure surface definition, no impl. Argues for `tui-vfx-compositor`: traits and the driver live together. Argues for new crate: future plugin surface (rejected; §8). Recommend `tui-vfx-types` for the trait shape and `tui-vfx-compositor` for the driver — same split as today's per-cell traits.
2. **First post-pass primitive.** Vignette is the recommended pick. Alternative: Scanline (CRT-style horizontal lines). Either proves the framework; pick the one with the most concrete near-term consumer demand.
3. **Phase G dispatch removal.** After deleting `render_pipeline_with_shadow`, does `CompositionOptions.shadow` stay (deprecated, lowered into `pre_passes` at construction) or get removed? Recommend remove; consumers move to `pre_passes` form. Lowering rule in Phase F covers the recipe side; the in-code field rename is a one-time consumer audit.

<!-- <FILE>docs/design/tui-vfx-pre-post-pass-rollout-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
