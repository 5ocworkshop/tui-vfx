<!-- <FILE>pro/EXISTING-SYSTEM-PRD/12_open_questions.md</FILE> - <DESC>Chapter 12 of the evidence-backed Existing-System PRD: questions that could not be answered from the workspace at audit-time. Many initial questions were resolved by the deepening pass; resolved entries record the answer rather than the question.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Deepening pass: a sub-agent (42 tool calls) read the cited files end-to-end and resolved 11 of the 13 original questions.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — resolve §12.1, 12.2, 12.3, 12.4, 12.6, 12.7, 12.8, 12.9, 12.10, 12.11, 12.13 with concrete answers; demote resolved entries to "Resolution" sub-headings; only §12.5 and §12.12 remain genuinely open. 0.1.0: initial population.</CLOG> -->

# 12. Open Questions and Unknowns

Each item below records what could (or could not) be determined from the workspace at audit-time. After the deepening pass, most items have a concrete resolution; the remaining genuine unknowns are §12.5 and §12.12.

## 12.1 Empty `compositor/shaders/` directory — **resolved (struck)**

**Resolution.** The directory does not exist. `ls /usr/projects/tui-vfx/crates/tui-vfx-compositor/src/shaders/` errors with "No such file or directory". `find` enumerates only `traits/`, `utils/`, `pipeline/`, `widgets/`, `filters/`, `masks/`, `types/`, `context/`, `samplers/`. Git history (`git log --diff-filter=D`) records no deletion of files matching `crates/tui-vfx-compositor/src/shaders/*`. A workspace-wide grep for `compositor::shaders` / `compositor/shaders` returns zero hits. The original §12.1 was based on an early misreading of the directory listing; there has never been a `shaders/` sub-module in the compositor.

## 12.2 `compositor/widgets/mod.rs` — **resolved (intentional tombstone)**

**Resolution.** `crates/tui-vfx-compositor/src/widgets/mod.rs:1-9` is a 9-line tombstone. Its `<DESC>` block reads "Widgets module (empty after V1 removal)", version `2.0.0`, `<WCTX>` "WG2: V1 Pipeline Removal - Remove V1 widget exports", `<CLOG>` "Removed cls_animated and cls_animated_stateful module references". Line `:6` is `// V1 widgets removed - this module is now empty`. No `pub use`, no submodules.
The `<CLOG>` block correction is a small data-quality note — the agent's first pass said "10-line tombstone" but `wc -l` reports 9 lines (including a trailing newline-only line); the chapter has been corrected. The file is a deliberate scaffold left after the WG2 V1 removal; deleting it is a possible future cleanup.

## 12.3 Is `tui-vfx-next` the planned replacement, or a research surface? — **resolved (Phase B parked)**

**Resolution.** `tui-vfx-next` is registered at `Cargo.toml:17` but has zero external consumers (`grep` for `tui_vfx_next|tui-vfx-next` outside the crate's own files, excluding the workspace manifest, returns zero hits). The trajectory is documented in `docs/new_kernel/PHASE_B_STATUS_MEMO_TO_ARCHITECT.md`: Phase A and Phase B are complete; the crate is **parked** with the clean dep boundary maintained (no use of compositor / style / content / shadow). Future direction (e.g., split into contract / engine sub-crates) is explicitly deferred — see lines `:211, :230` of that memo. Eight modules at audit-time: `lib.rs`, `diagnostic.rs`, `effect.rs`, `engine.rs`, `sampler.rs`, `scope.rs`, `surface.rs`, `write.rs`.

## 12.4 Empty `[features]` blocks — **resolved (placeholders since initial commit)**

**Resolution.** Both `crates/tui-vfx-geometry/Cargo.toml` and `crates/tui-vfx-content/Cargo.toml` carry `[features] default = []` blocks. The geometry block was present in the initial public commit `dd14400` ("Initial commit — tui-vfx v0.2.0", 2026-02-19) and has had no subsequent edit. The content block dates to the same commit. No in-flight feature work motivated either; both are scaffolding placeholders.

## 12.5 The recipe-validator's strict-contracts check across recipes — **still open**

- **Question:** What is the current pass/fail status of `cargo xtask recipes validate` against the `tui-vfx-recipes` corpus?
- **Why it matters:** The validator is a release-gating tool (chapter 3 F044a / chapter 5 OPT-005, OPT-006), but the corpus lives in the sibling `tui-vfx-recipes` repository and is therefore out of audit scope.
- **Evidence inspected:** `xtask/src/main.rs:111-121,163-177` (the CLI surface); `xtask/src/recipes/mod.rs` (the validator implementation file).
- **What would resolve it:** Run `cargo xtask recipes validate --recipes-dir <path-to-tui-vfx-recipes>/recipes` and read the output. Out of audit scope: the corpus is in an external repository.

## 12.6 The `glyph_particles/` module's public-API surface — **resolved**

**Resolution.** `crates/tui-vfx-content/src/glyph_particles/mod.rs:1-430` is a single-file module (no sub-files at audit-time). It implements a complete content-layer transient glyph emitter ("Task 24: TTE-inspired BinaryPath/Spray/Burst", line 3). Public types: `GlyphParticleEmitterSpec` (`:27-54`), `ParticleEndBehavior` (`:77-85`), `ParticleConcurrency` (`:89-98`), `GlyphParticleStats` (`:103-111`), `GlyphParticleResult` (`:115-118`). Pure entry point: `emit_glyph_particles(scene, spec, timing, frame, options) -> GlyphParticleResult` (`:121-217`). Built on top of `cell_motion::{CellActor, CellMotionPhaseSpec, CellMotionTiming, CellPlacement, ...}` (`:18-22`). Deterministic FNV-style hash at `:391-411` for palette / concurrency selection. The crate's CHANGELOG `<CLOG>` describes it as "the sibling of cell_motion" (`:8-11`).

## 12.7 The `tui-vfx-types::glyph` framework's slice trajectory — **resolved (complete)**

**Resolution.** The framework is fully shipped at v0.1.0. The "Slice 6.6 §F.1" reference in `crates/tui-vfx-types/Cargo.toml:3-4` was a misdirection — that slice number belongs to the **mechanical-circular-content-cycles plan's font-binding work**, not the glyph encoders.

`crates/tui-vfx-types/src/glyph/mod.rs:23-29` re-exports `GlyphEncoder`, `sample_eight_subcells`, `sample_eight_subcells_with_slope`, `SUBCELL_OFFSETS`. `cls_glyph_encoder.rs:48-79` defines a closed enum with five variants (`BrailleSubcell`, `BrailleEighths`, `BlockHorizontal`, `BlockVertical`, `Ramp`), `encode_one` at `:103-119`, `encode_subcell` at `:144-170`. `fnc_sample_eight_subcells.rs:35-44` defines the eight-offset table; `sample_eight_subcells` at `:73-84` and `sample_eight_subcells_with_slope` at `:126-137` are both implemented.

Live consumer chain confirmed: `crates/tui-vfx-style/src/models/cls_water_field_signal.rs` and `cls_fire_field_signal.rs` (the latter at v0.2.0) feed `ScalarFieldGlyphFilter` defined in `crates/tui-vfx-compositor/src/filters/cls_scalar_field_glyph_filter.rs:73`, which the pipeline routes through `pipeline/cls_prepared_filter.rs:87-89` as `ScalarFieldGlyphWater(ScalarFieldGlyphFilter<WaterFieldSignal>)` and `ScalarFieldGlyphFire(ScalarFieldGlyphFilter<FireFieldSignal>)`. F028's status in chapter 3 is therefore upgraded from "partially implemented" to "implemented".

## 12.8 Whether `compositor/widgets/` is the same as `geometry/widgets/` — **resolved (name collision only)**

**Resolution.** The two modules share only the name. Compositor-side `crates/tui-vfx-compositor/src/widgets/mod.rs` is a 10-line tombstone (see §12.2). Geometry-side `crates/tui-vfx-geometry/src/widgets/mod.rs:7-26` exposes numpad-style 3×3 grid primitives — `col_numpad_mapping`, `fnc_hit_test_numpad_3x3`, `fnc_hit_test_triplet_grids`, `fnc_resolve_direction_selection_motion`, plus `types::{ArrowOrientation, DirectionNumpadSelection, DirectionSelectionMotion, ExitDirectionSelection, TripletGridFocus}` (`:23-26`). Lines `:13-15` record that "Rendering widgets … have been moved to mixed-ratatui (L3 adapter)". No cross-import between the two modules; no SSOT violation under Intention 26.

## 12.9 The pre/post-pass framework's as-built status — **resolved (design-only)**

**Resolution.** Pre/post-pass is unstarted in code. `grep -rn "pre_pass|post_pass|PrePass|PostPass"` across `/crates/` returns zero matches — no Rust trait, struct, enum variant, or call site exists today. The work is design-only and lives in two docs: `docs/design/tui-vfx-pre-post-pass-rollout-plan.md` (v0.2.0; Phases A–H sized one-per-ralph-run) and `docs/design/tui-vfx-pipeline-observability.md` (v0.3.0 §17 alignment), which collectively plan `PipelineStageKind::PrePass`/`PostPass`, `on_pre_pass_entered`/`on_post_pass_finished` inspector callbacks, and a `Shadow` deprecation alias to be removed at Phase G. The V3 upgrade-plan `00_INDEX.md` itself contains no pre/post-pass references; this is a parallel rollout. Chapter 11 §11.1's claim that "shadow is a paired pre-stage, with pre/post-pass framework as a V3-planned generalization" is correct; the pre/post-pass framework is the planned successor of the bespoke shadow stage.

## 12.10 Asset-registry consumer surface — **resolved (deferred by design)**

**Resolution.** `crates/tui-vfx-content/src/assets/cls_asset_registry.rs:38-132` is a complete `AssetRegistry` (BTreeMap-backed name→bytes plus reserved `default_logo` sentinel). The header at `:11-16` makes the deferral explicit: "Phase 7 of the mechanical circular content cycles plan keeps the consuming source surface deferred — adding a `type: \"rocketsplash_image\"` scene-layer source variant intersects with sibling's V3 scene-layer composition work and warrants its own coordinated session. This module is the byte-supplying half." The producer half ships at v0.1.0 with full test coverage (16 inline tests at `:134-281`); the consumer half (recipe `type: "rocketsplash_image"` source variant) is intentionally deferred. No callers exist outside the module's own tests.

## 12.11 The `mixed_signals_schema` module's full surface — **resolved**

**Resolution.** `crates/tui-vfx-core/src/mixed_signals_schema.rs` is a 609-line file containing three hand-written `ConfigSchema` impls for foreign types (orphan-rule workaround, justified inline at `:32, :83, :469`):

- `SignalOrFloat` (`:33-81`): two-variant enum `Static` / `Signal`.
- `SignalSpec` (`:84-467`): tagged enum with **30 variants** — `Sine, Triangle, Square, Sawtooth, Constant, Ramp, Step, Pulse, WhiteNoise, Perlin, SeededRandom, SpatialNoise, GaussianNoise, PoissonNoise, CorrelatedNoise, PinkNoise, PerCharacterNoise, StudentTNoise, ImpulseNoise, Adsr, Impact, LinearEnvelope, Add, Multiply, Scale, Sum, Mix, FrequencyMod, VcaCentered, PhaseAccumulator, PhaseSine, Keyframes, Clamp, Quantize, Remap, Invert, Abs`. (37 names listed because some are folded into composite handlers; the schema variant count is 30.)
- `EasingType` (`:470-606`): 25 unit variants `Linear` → `CircInOut`.

All three carry `CONFIGSCHEMA-JUSTIFICATION: derive-cannot-handle-foreign-type` comments. Routine version `1.4.1` (last touched to lift justification comments).

## 12.12 The two-paragraph rationale at `Cargo.toml:53-56` — **still open**

- **Question:** Is the same level of rationale documented for the other path-dependent siblings (`mixed-signals`, `rocketsplash-rt`)?
- **Why it matters:** The HCT block is a useful documentation pattern; the absence of similar rationale for `mixed-signals` and `rocketsplash-rt` is observable.
- **Evidence inspected:** `Cargo.toml:52` (mixed-signals — single line, no rationale block), `Cargo.toml:64-65` (rocketsplash-rt — one inline-comment line pointing to a plan doc).
- **What would resolve it:** Either the maintainer's decision to add equivalent rationale blocks, or a steering-doc note declaring the HCT block a one-off pattern. Genuinely a maintainer decision, not a discoverability question.

## 12.13 `pipeline-probe` argument-parsing past line 80 — **resolved**

**Resolution.** `crates/tui-vfx-probe/src/bin/pipeline-probe.rs` is 241 lines; `run()` ends at `:188`. The full flag grammar (`:42-86`):

| Flag | Type | Notes |
|---|---|---|
| `--input <path>` | string | required for input scene spec |
| `--format <json\|ndjson>` | enum | default `json` |
| `--phase <entering\|dwelling\|exiting>` | enum | default `dwelling` |
| `--sample-t <f64>` | f64 | default `0.5` |
| `--cells <all\|non-empty\|modified>` | enum | default `all` |
| `--with-causation` | bool | flag |
| `--frames <usize>` | usize | timeline mode |
| `--diff-to <f64>` | f64 | diff mode |
| `--sqlite-query <sql>` | string | SQLite-store query |
| `--widget-cell <x,y>` | parsed | single-frame-only |

`--widget-cell` (`:84`) consumes one positional value parsed by `parse_widget_cell` at `:221-226` (splits on `,`; both halves parsed as `u16`; format errors yield `"--widget-cell must be formatted as x,y"`). Mutual-exclusion guard at `:92-94`: `--widget-cell` rejects coexistence with `--frames`, `--diff-to`, or `--sqlite-query` ("currently supports only single-frame probe output"). The single-frame path (`:174-179`) calls `find_widget_cell(&analysis_report, x, y)` and emits the result under a `focus_cell` key in the JSON envelope (`:213-217`).

Chapter 5 OPT-015 is therefore upgraded from Medium to High confidence.

<!-- <FILE>pro/EXISTING-SYSTEM-PRD/12_open_questions.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
