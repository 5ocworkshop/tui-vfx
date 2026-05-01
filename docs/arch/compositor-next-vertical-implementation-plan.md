<!-- <FILE>docs/arch/compositor-next-vertical-implementation-plan.md</FILE> - <DESC>Detailed implementation plan for copied compositor-next and vertical primitive-by-primitive migration</DESC> -->
<!-- <VERS>VERSION: 0.7.0</VERS> -->
<!-- <WCTX>v3.1 north-star execution plan: validate recipes at load time, then pass canonical v3.1 structures through to copied compositor-next via vertical slices.</WCTX> -->
<!-- <CLOG>0.7.0: MINOR — clarify the pure v3.1 target: load-time recipe validation, then direct canonical v3.1 flow to compositor-next with no bridge/shim investment.
0.6.0: MINOR — elevate validation/tooling maturity to an early gate before broad primitive work.
0.5.0: MINOR — account for presentation/update cadence and absolute-time procedural sources in the schema audit.
0.4.0: MINOR — add IndexedField source descriptor as first from-scratch workflow test.
0.3.0: MINOR — add bounded descriptor/schema hindsight audit before workbench generation.
0.2.3: PATCH — set 300 LOC target and require split or strong justification above 500 LOC.
0.2.2: PATCH — clarify OFPF line counts are soft guidelines and closely coupled code may stay together.
0.2.1: PATCH — record ofpf-loc file-size command and current compositor/style output.
0.2.0: MINOR — switch from broad revert policy to additive work-forward isolation and archive-later recipe policy.
0.1.1: PATCH — add explicit current-state numbered next steps and concise block diagram.
0.1.0: INIT — define vertical-slice implementation plan, gates, block diagrams, and compositor-next rollout sequence.</CLOG> -->

# Compositor-Next Vertical Implementation Plan

## Status

Draft implementation plan.

This plan assumes the direction documented in:

- [`v31-schema-boundary-north-star.md`](v31-schema-boundary-north-star.md)
- [`primitive-workbench-schema-driven-workflow.md`](primitive-workbench-schema-driven-workflow.md)

## Executive Summary

Build a new schema-driven compositor runtime by **copying the hardened existing compositor code**, not rewriting it. The new crate, referred to here as `tui-vfx-compositor-next`, should preserve current behavior first, then progressively align primitive boundaries with v3.1 descriptor/schema contracts.

The target data path is **validate on recipe load, then pass canonical v3.1 structures through directly**. Once a `RecipeDocument` is structurally valid and normalized into the canonical v3.1 in-memory model, runtime should pass that model plus explicit sample context to compositor-next. New work must not add bridge, shim, or legacy-input support layers; if the current player path cannot stay clean, create a stripped `player-next` path instead of carrying the old path forward.

The work must proceed in **vertical primitive slices**, not horizontal layers. Pick one primitive, drive it all the way from descriptor/schema through generated scaffolding, compositor-next runtime behavior, player → compositor-next integration, fixtures, migration mapping, and parity validation, then sign it off before moving to the next primitive.

This avoids the failure mode where broad horizontal work appears complete at one layer but integration issues are discovered much later.

## Non-Negotiable Principles

1. **Copy, do not rewrite.** Existing compositor behavior is hardened and valuable. `compositor-next` starts as a copied implementation with behavior parity tests.
2. **Schema-driven boundaries.** Primitive inputs and public contracts should be descriptor/schema-owned or generated from descriptor/schema-owned data.
3. **Validate once, then pass through directly.** Recipe validation belongs at load time. After a recipe is accepted as canonical v3.1, compositor-next should consume that loaded structure plus sample context directly. Do not add bridge/shim code to keep legacy or compositor-shaped inputs alive in the v3.1 path.
4. **Vertical slices only.** Work one primitive end-to-end through every layer. Do not complete an entire horizontal layer for all primitives before integration.
5. **Every primitive is signed off individually.** Do not prove one primitive and assume all others will behave the same.
6. **No silent fallback.** Unsupported semantics must produce explicit diagnostics.
7. **OFPF alignment is a real design constraint with judgment.** Around 300 LOC is the normal target because smaller files improve agent focus and reviewability. Files above 500 LOC should be split unless there is a strong written cohesion justification.
8. **Commonality extraction is part of the work.** If the same primitive-internal pattern appears in 3 or more places, extract it or open an explicit extraction ticket.
9. **Recipe migration waits for runtime confidence.** Broad recipe migration resumes only after the relevant primitive has passed its compositor-next vertical gate.

## Numbered Next Steps From Current State

1. **Stop expanding current migration edits and fence the current state.**
   - Do not broadly revert main, because multiple contributors may have useful exploratory work in place.
   - Treat the current `v3.1/debug_recipes/` tree as reference/exploratory evidence.
   - Work forward in additive compositor-next/workbench paths.
   - Only mutate existing migrated recipes when a specific vertical primitive slice owns that recipe.

2. **Write and approve the compositor-next architecture plan.**
   - Target crate name: working default `tui-vfx-compositor-next`.
   - State explicitly: copy the existing compositor; do not rewrite from scratch.
   - Define boundary direction: compositor-next consumes v3.1-derived primitive contracts.

3. **Inventory current compositor file sizes.**
   - Identify multi-thousand-line files and other unusually large files.
   - Treat ~300 LOC as the healthy target.
   - Treat >500 LOC as requiring split or strong written justification.
   - Classify each as leave-as-is-with-justification, split mechanically, or refactor around shared helpers.
   - Preserve closely coupled code only when the cohesion justification is stronger than the focus/reviewability benefit of smaller files.
   - Do not mix cleanup with behavior changes.

4. **Copy the existing compositor crate.**
   - Preserve old `tui-vfx-compositor`.
   - Add new copied crate.
   - First commit should be a mostly mechanical copy/rename.

5. **Establish baseline parity tests.**
   - Existing compositor tests should pass in copied crate.
   - Add smoke tests proving compositor-next behavior matches current compositor before schema-boundary changes.

6. **Bring v3.1 validation tooling to first-class status before broad primitive work.**
   - Treat tooling as part of the product, not an afterthought after primitives are implemented.
   - Baseline the current v3.1 tools against V2-era capabilities: structural validation, frame/timeline sampling, frame diff, per-cell capture, fixture QC, field coverage, direct runtime support gaps, and oracle comparison.
   - Keep `render-frame`, `render-timeline`, `render-frame-diff`, `capture-cells`, `fixture-qc`, `primitive-field-coverage`, direct runtime-support reports, and migration reports in the early execution path.
   - Add or fix tooling before scaling a primitive family when validation cannot localize failures at least to recipe/frame/cell/primitive-field level.

7. **Run a bounded descriptor/schema hindsight audit.**
   - Identify common primitive fields and duplicated semantic concepts before generating scaffolding around them.
   - Classify commonality without reopening unbounded schema redesign.
   - Feed accepted common concepts into Primitive Workbench generation.
   - Include timing/cadence concepts explicitly: presentation target frame rate, recipe/source/effect update cadence, fixed-step versus continuous sampling, and absolute elapsed time requirements such as the Madeira flag procedural source.

8. **Design one representative co-located primitive tree.**
   - Start with one shader primitive, likely `shader.highlighter` or `shader.focusField`.
   - Include descriptor, generated assets, compositor-next runtime module, fixtures, tests, docs, and migration mapping.

9. **Build the Primitive Workbench MVP.**
   - Read descriptor/schema.
   - Emit typed inputs, accessors, validation manifest, fixture skeleton, control metadata, and migration skeleton.
   - Do not generate visual behavior.

10. **Wire one primitive end-to-end.**
   - Use existing compositor implementation.
   - Replace ad hoc hand mapping with generated v3.1-derived input surfaces consumed directly by compositor-next.
   - Prove strict compositor-next output still matches existing behavior.

11. **Design timing/cadence semantics before broad primitive generation.**
    - Keep presentation frame rate separate from recipe semantics.
    - Treat player/runtime `fps` as a runtime playback control unless the schema audit accepts an optional presentation hint.
    - Add or formalize a reusable update-clock/update-rate concept only when a primitive/source genuinely needs deterministic fixed-step sampling.
    - Preserve absolute elapsed time as a first-class runtime sample input for continuous procedural sources such as Madeira flag wave/fireworks generation.
    - Decide whether cadence can be inherited from recipe to scene element to source/effect, and document the fallback behavior.

12. **Extract common primitive utilities.**
   - Apply the 3+ repetition rule.
   - Share helpers for colors, gradients, bindable progress, apply-to routing, directions, falloff, seeded noise, subcell encoding, diagnostics, and migration normalization when repetition is proven.

13. **Repeat by primitive family, but still primitive-by-primitive.**
    - Shaders first.
    - Then filters, masks, samplers, and style effects.
    - Each primitive gets generated scaffolding, runtime wiring, parity validation, and signoff before the next primitive.

14. **Update player → compositor-next integration without bridges.**
    - Recipe loading validates and normalizes canonical v3.1 once.
    - Player/runtime passes the loaded canonical v3.1 structure plus explicit sample context directly to compositor-next-owned v3.1 entrypoints.
    - Do not add new bridge, shim, or legacy-input support code to the old player/runtime path.
    - If the existing player/runtime architecture forces translation work, copy the player into a stripped `player-next` path and remove non-v3.1 compatibility code there.
    - Current compositor backend remains available separately until compositor-next is proven; it is not part of the new v3.1 pathway.

15. **Only then resume recipe migration in owned slices.**
    - Migration becomes: V2 source recipe → v3.1 recipe → compositor-next native primitive.
    - Validation compares old oracle against compositor-next output.
    - Existing `v3.1/debug_recipes/` remains visible as reference until owner-approved archival/reseed.

## Concise Path Diagram

```text
CURRENT STATE
┌──────────────────────────────────────────────┐
│ Existing hardened tui-vfx-compositor         │
│                                              │
│ filters / masks / samplers / shaders         │
│ existing tests                               │
│ existing behavior                            │
└───────────────────────┬──────────────────────┘
                        │ copy, do not rewrite
                        ▼
┌──────────────────────────────────────────────┐
│ New compositor-next crate                    │
│                                              │
│ mechanically copied implementation           │
│ behavior parity tests against old compositor │
│ OFPF/file-size inventory                     │
└───────────────────────┬──────────────────────┘
                        │
                        │ schema-boundary alignment
                        ▼
┌──────────────────────────────────────────────┐
│ Co-located primitive source trees            │
│                                              │
│ primitives/shader/highlighter/               │
│   descriptor.v31.json                        │
│   migration.v2.json                          │
│   generated/                                 │
│   runtime/                                   │
│   fixtures/                                  │
│   tests/                                     │
│   docs/                                      │
└───────────────────────┬──────────────────────┘
                        │ generated by
                        ▼
┌──────────────────────────────────────────────┐
│ Primitive Workbench                          │
│                                              │
│ reads v3.1 descriptors/schema                │
│ emits typed inputs                           │
│ emits accessors                              │
│ emits validation manifests                   │
│ emits fixture skeletons                      │
│ emits migration skeletons                    │
└───────────────────────┬──────────────────────┘
                        │ load-time validation,
                        │ then v3.1 pass-through
                        ▼
┌──────────────────────────────────────────────┐
│ compositor-next v3.1 primitive runtime       │
│                                              │
│ existing behavior reused                     │
│ v3.1-derived input contracts                 │
│ shared primitive utilities                   │
│ OFPF-aligned file/module layout              │
└───────────────────────┬──────────────────────┘
                        │
                        │ validation gates
                        ▼
┌──────────────────────────────────────────────┐
│ Evidence + parity layer                      │
│                                              │
│ current compositor parity                    │
│ v2 oracle parity                             │
│ fixture QC                                   │
│ primitive field coverage                     │
│ strict compositor-next evidence               │
└───────────────────────┬──────────────────────┘
                        │
                        ▼
TARGET STATE
┌──────────────────────────────────────────────┐
│ v3.1 schema-driven compositor-next           │
│                                              │
│ copied/hardened behavior preserved           │
│ primitive contracts generated from schema    │
│ recipes migrate through reusable tooling     │
│ old compositor remains stable fallback       │
└──────────────────────────────────────────────┘
```

## Why Vertical Slices

Horizontal work means doing one layer across many primitives, for example:

```text
all descriptors → all generated structs → all runtime mappings → all fixtures → all validation
```

This looks efficient but defers integration risk. Problems with value-source resolution, runtime defaults, compositor behavior, player → compositor-next boundaries, or parity tooling may not appear until many primitives are partially converted.

The required strategy is vertical:

```text
one primitive → descriptor → generated code → runtime → backend → fixtures → migration → parity → signoff
then next primitive
```

Vertical work finds boundary issues immediately and forces the tooling to prove itself against real compositor behavior before it scales.

## Target System Block Diagram

```text
CURRENT HARDENED RUNTIME
┌──────────────────────────────────────────────┐
│ Existing tui-vfx-compositor                  │
│                                              │
│ filters / masks / samplers / shaders         │
│ pipeline + types                             │
│ current tests and hardened behavior          │
└───────────────────────┬──────────────────────┘
                        │
                        │ mechanical copy, rename, baseline tests
                        ▼
┌──────────────────────────────────────────────┐
│ tui-vfx-compositor-next                      │
│                                              │
│ starts as copied compositor behavior         │
│ old compositor remains in place              │
│ behavior parity proven before schema changes │
└───────────────────────┬──────────────────────┘
                        │
                        │ vertical primitive slice
                        ▼
┌──────────────────────────────────────────────┐
│ Co-located primitive source tree             │
│                                              │
│ primitives/<family>/<primitive>/             │
│   descriptor.v31.json                        │
│   migration.v2.json                          │
│   generated/                                 │
│   runtime/                                   │
│   fixtures/                                  │
│   tests/                                     │
│   docs/                                      │
└───────────────────────┬──────────────────────┘
                        │
                        │ generated by Primitive Workbench
                        ▼
┌──────────────────────────────────────────────┐
│ Schema-derived primitive boundary            │
│                                              │
│ typed inputs                                 │
│ accessors                                    │
│ diagnostics                                  │
│ validation manifests                         │
│ fixture skeletons                            │
│ migration skeletons                          │
└───────────────────────┬──────────────────────┘
                        │
                        │ human fills compositor semantics
                        ▼
┌──────────────────────────────────────────────┐
│ compositor-next runtime primitive            │
│                                              │
│ copied behavior preserved                    │
│ v3.1-derived inputs                          │
│ shared primitive utilities                   │
│ OFPF-aligned module structure where useful   │
└───────────────────────┬──────────────────────┘
                        │
                        │ validation gate
                        ▼
┌──────────────────────────────────────────────┐
│ Evidence and signoff                         │
│                                              │
│ old compositor parity                        │
│ strict compositor-next evidence      │
│ fixture QC                                   │
│ primitive field coverage                     │
│ V2 oracle parity where applicable            │
│ commonality extraction review                │
└───────────────────────┬──────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────┐
│ Signed-off v3.1 runtime primitive            │
│                                              │
│ available to canonical recipes               │
│ usable by migration tooling                  │
│ visible to studio/control catalog            │
│ ready for next primitive slice               │
└──────────────────────────────────────────────┘
```

## Vertical Slice Gate Diagram

```text
Select primitive
      │
      ▼
Descriptor/schema contract complete?
      │ no
      ├── define or repair descriptor
      ▼ yes
Generate scaffold from descriptor
      │
      ▼
Runtime behavior connected in compositor-next?
      │ no
      ├── reuse copied compositor behavior
      ├── fill compositor semantic body
      └── extract common helpers when 3+ repetition appears
      ▼ yes
Player → compositor-next path can execute strict v3.1?
      │ no
      ├── add direct v3.1 boundary integration
      └── require explicit unsupported diagnostics
      ▼ yes
Fixtures and migration mapping complete?
      │ no
      ├── generate minimal fixture
      ├── add V2 parity fixture when source exists
      └── record mapping decisions
      ▼ yes
Validation green?
      │ no
      ├── fix descriptor / generated code / runtime / fixture / mapping
      └── repeat this same primitive slice
      ▼ yes
Primitive signoff
      │
      ▼
Move to next primitive
```

## Detailed Numbered Implementation Plan

### Phase 0 — Current-State Fencing and Planning Baseline

1. Do not broadly revert main. Current recipe and code edits may include useful contributor work.
2. Stop expanding broad experimental migration edits until compositor-next has an approved vertical-slice path.
3. Preserve and commit the architecture/planning docs that describe the new direction.
4. Confirm both relevant repositories have an intentionally documented starting state:
   - `/usr/projects/tui-vfx`
   - `/usr/projects/tui-vfx-recipes`
5. Classify current `v3.1/debug_recipes/` content as reference/exploratory unless a vertical primitive slice explicitly owns a recipe.
6. Record the baseline state in a short `.omx` note or architecture handoff report.
7. Do not resume broad directory recipe migration until the compositor-next plan has an approved first vertical slice.

Acceptance criteria:

- No destructive cleanup is performed merely to make the tree look clean.
- New architecture docs are indexed under `docs/arch/` and `docs/INDEX.md`.
- Current recipe artifacts remain available for human inspection.
- New compositor-next/workbench work has additive paths and does not depend on exploratory recipe state being perfect.
- Dirty state is understood and separated from compositor-next-owned work.

### Phase 1 — Name and Create the Compositor-Next Work Packet

1. Choose the crate name. Working default: `tui-vfx-compositor-next`.
2. Define crate purpose in `Cargo.toml` and crate-level docs:
   - copied compositor behavior;
   - v3.1 schema-driven primitive boundaries;
   - no behavior rewrite during initial copy.
3. Decide whether `compositor-next` depends on `tui-vfx-compositor` temporarily or starts as a full code copy.
   - Recommended: full code copy first, so behavior can be preserved independently while the original remains stable.
4. Add a work packet or plan entry naming the first primitive vertical slice.

Acceptance criteria:

- Crate name and purpose are documented.
- The copy-first policy is explicit.
- The first primitive candidate is named but not yet modified.

### Phase 2 — Mechanical Copy and Baseline Parity

1. Copy `crates/tui-vfx-compositor` to `crates/tui-vfx-compositor-next`.
2. Rename crate metadata and Rust module references mechanically.
3. Add the new crate to the workspace.
4. Run formatting and tests for the copied crate.
5. Add a parity harness that can compare selected outputs between old compositor and compositor-next.
6. Make the first commit primarily mechanical.

Acceptance criteria:

- `cargo test -p tui-vfx-compositor-next` passes.
- Existing old compositor tests still pass.
- Parity smoke proves copied behavior matches old compositor for representative filters/masks/samplers/shaders.
- Diff is reviewable as copy/rename, not behavior rewrite.

### Phase 3 — OFPF and File-Size Inventory

1. Generate file-size inventory for copied compositor-next and adjacent style primitive files. Use `ofpf-loc`; it produces the relevant file-size list in seconds.
2. Treat ~300 LOC as the normal target for focused agent work and review.
3. Treat files above 500 LOC as requiring a split unless a strong written cohesion justification explains why the code should stay together.
4. Being slightly over the target is acceptable; the concern is sustained size/coupling that hurts focus, review, and safe edits.
5. Identify files whose size or coupling is likely to impede safe primitive work.
6. Classify each large file:
   - `leave-for-now` — large but not in first slice;
   - `mechanical-split` — split modules without behavior change;
   - `extract-common-helper` — repeated logic should become shared utility;
   - `defer` — too risky until tests improve.
7. For the first primitive slice only, perform mechanical splits before semantic changes when needed and when the split improves readability, ownership, or safety.
8. Leave >500 LOC code together only with strong written justification, such as tightly coupled invariants that would become harder to reason about if split.

Current observed large-file examples from the existing codebase, generated with:

```bash
ofpf-loc --root . --filter crates/tui-vfx-compositor/src --ext rs --output-format text 300
ofpf-loc --root . --filter crates/tui-vfx-style/src --ext rs --output-format text 300
```

Current output:

```text
== compositor >300 ==
2893	crates/tui-vfx-compositor/src/types/cls_filter_spec.rs
2202	crates/tui-vfx-compositor/src/pipeline/cls_prepared_filter.rs
1233	crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs
989	crates/tui-vfx-compositor/src/filters/cls_glyph_timeline.rs
832	crates/tui-vfx-compositor/src/filters/cls_kitt_scanner.rs
799	crates/tui-vfx-compositor/src/types/cls_mask_spec.rs
769	crates/tui-vfx-compositor/src/types/cls_sampler_spec.rs
534	crates/tui-vfx-compositor/src/filters/cls_animated_glyph_ramp.rs
484	crates/tui-vfx-compositor/src/pipeline/orc_pipeline_observability.rs
471	crates/tui-vfx-compositor/src/filters/cls_glyph_style.rs
449	crates/tui-vfx-compositor/src/pipeline/cls_prepared_sampler.rs
443	crates/tui-vfx-compositor/src/filters/cls_pattern_fill.rs
414	crates/tui-vfx-compositor/src/filters/cls_matrix_rain.rs
405	crates/tui-vfx-compositor/src/filters/test_cls_scalar_field_glyph_filter.rs
395	crates/tui-vfx-compositor/src/filters/cls_braille_dust.rs
394	crates/tui-vfx-compositor/src/filters/cls_motion_blur.rs
390	crates/tui-vfx-compositor/src/filters/cls_sub_pixel_bar.rs
388	crates/tui-vfx-compositor/src/filters/cls_glisten_sweep.rs
383	crates/tui-vfx-compositor/src/filters/cls_rigid_shake.rs
368	crates/tui-vfx-compositor/src/pipeline/cls_composition_options.rs
359	crates/tui-vfx-compositor/src/masks/cls_wipe.rs
358	crates/tui-vfx-compositor/src/filters/cls_sub_cell_shake.rs
346	crates/tui-vfx-compositor/src/filters/cls_underline_wipe.rs
324	crates/tui-vfx-compositor/src/filters/cls_vignette.rs
314	crates/tui-vfx-compositor/src/filters/cls_hover_bar.rs
306	crates/tui-vfx-compositor/src/filters/cls_charset_noise.rs

== style >300 ==
1060	crates/tui-vfx-style/src/models/cls_terminal_water_shader.rs
1033	crates/tui-vfx-style/src/models/cls_spatial_shader_type.rs
1023	crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs
921	crates/tui-vfx-style/src/models/cls_terminal_fire_shader.rs
868	crates/tui-vfx-style/src/models/cls_style_effect.rs
742	crates/tui-vfx-style/src/models/cls_highlighter_shader.rs
660	crates/tui-vfx-style/src/models/cls_fade_spec.rs
500	crates/tui-vfx-style/src/models/cls_glisten_band_shader.rs
487	crates/tui-vfx-style/src/traits/cls_shader_context.rs
454	crates/tui-vfx-style/src/schedules/fnc_poisson_burst_schedule.rs
451	crates/tui-vfx-style/src/models/cls_trace_path_shader.rs
417	crates/tui-vfx-style/src/models/cls_linear_gradient_shader.rs
413	crates/tui-vfx-style/src/models/cls_sub_cell_shake_shader.rs
388	crates/tui-vfx-style/src/models/cls_bevel_shader.rs
367	crates/tui-vfx-style/src/models/cls_stochastic_sparkle_shader.rs
364	crates/tui-vfx-style/src/models/cls_focus_field_shader.rs
362	crates/tui-vfx-style/src/models/cls_diffusion_shader.rs
355	crates/tui-vfx-style/src/models/cls_pulse_wave_shader.rs
335	crates/tui-vfx-style/src/models/cls_glitch_lines_shader.rs
329	crates/tui-vfx-style/src/models/cls_ambient_occlusion_shader.rs
324	crates/tui-vfx-style/src/models/cls_color_ramp.rs
322	crates/tui-vfx-style/src/models/cls_trace_common.rs
320	crates/tui-vfx-style/src/models/v3/test_try_lower_v3_spatial_shader_family.rs
```

Initial triage:

| File | Approx LOC | Initial classification |
| --- | ---: | --- |
| `crates/tui-vfx-compositor/src/types/cls_filter_spec.rs` | 2893 | likely split by primitive family / generated DTO boundary |
| `crates/tui-vfx-compositor/src/pipeline/cls_prepared_filter.rs` | 2202 | likely split by filter preparation family |
| `crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs` | 1233 | inspect before modifying; pipeline risk |
| `crates/tui-vfx-style/src/models/cls_terminal_water_shader.rs` | 1060 | candidate for shader-local decomposition |
| `crates/tui-vfx-style/src/models/cls_spatial_shader_type.rs` | 1033 | likely enum/family decomposition candidate |
| `crates/tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs` | 1023 | candidate for generated/registry approach |

Acceptance criteria:

- File-size inventory exists.
- No behavior-changing refactor is mixed with mechanical copy.
- First primitive slice has a scoped cleanup plan if it touches large files.

### Phase 3.5 — Descriptor/Schema Hindsight Audit

Before generating scaffolding from the current descriptor pack, perform a bounded hindsight audit. The goal is to identify common primitive concepts that should be shared, normalized, or classified for tooling before the workbench bakes current duplication into generated code.

This is not an unbounded v3.1 redesign. It is a commonality and schema-hardening pass informed by the benefit of hindsight.

Audit targets:

1. Primitive descriptor inputs across families.
2. Repeated semantic fields such as `color`, `foreground`, `background`, `applyTo`, `progress`, `direction`, `axis`, `radius`, `strength`, `speed`, `seed`, `density`, `frequency`, `falloff`, `feather`, `width`, and `height`.
3. Same semantics expressed with different names.
4. Same names carrying different semantics.
5. Legacy aliases that should remain migration-only instead of becoming canonical v3.1 vocabulary.
6. Descriptor fragments that the workbench can treat as common even if recipe JSON remains flattened for readability.

Classification values:

- `same-name-same-semantics`
- `different-name-same-semantics`
- `same-name-different-semantics`
- `family-specific-semantic`
- `legacy-alias-only`
- `candidate-common-contract`
- `keep-distinct`
- `owner-decision-needed`

Candidate common contracts:

- progress/bindable control;
- color and color-channel routing;
- foreground/background/apply-to routing;
- direction/axis/edge/side/corner geometry;
- seeded randomness;
- temporal speed/frequency;
- presentation cadence, semantic update cadence, and absolute sample time;
- radius/falloff/feather;
- glyph/charset selection;
- density/threshold/intensity controls.

Timing/cadence is a required audit topic. Do not collapse it into `fps`: presentation cadence, semantic update cadence, and sample time are different contracts. The Madeira flag fixtures are the reference case because `source.procedural` flag/fireworks motion advances from absolute elapsed time, while authored loopback ramps also use elapsed time to honor their duration.

Deliverables:

- `docs/arch/v31-primitive-schema-hindsight-audit.md` for human-readable findings and decisions.
- Optional generated report: `.omx/reports/v31-primitive-field-commonality-report.json`.
- Optional summary report: `.omx/reports/v31-primitive-field-commonality-report.md`.
- Workbench input metadata for accepted common concepts.

Acceptance criteria:

- Common fields are classified before broad scaffold generation.
- Accepted common concepts are documented.
- Rejected collapses include rationale.
- Owner-decision items are explicit and do not block unrelated vertical primitive slices.
- The audit does not introduce compatibility aliases solely to make legacy JSON validate.

### Phase 3.75 — Validation Tooling Maturity Gate

Before broad primitive implementation, make validation tooling strong enough to prevent blind migration. The v2 tools set the usability bar: a contributor should be able to sample frames, inspect per-cell evidence, diff samples, and localize a failure without manually reading renderer internals first.

Current v3.1 tooling already includes these important surfaces:

- `render-frame` / `render-ir` for single-sample player evidence;
- `render-timeline` with schema `v3.1.player.frameTimeline.1`;
- `render-frame-diff` with schema `v3.1.player.frameDiff.1`;
- `capture-cells` SQLite output with schema `v3.1.player.cellCapture.sqlite.1`, including dense cells, frame timing, diagnostics, provenance, layers, and graph values;
- `fixture-qc` for combined validation, render, field-coverage, direct runtime-support, timeline, and diff smoke evidence;
- `primitive-field-coverage`, direct runtime-support gap, `migration-gap`, `migration-mapping-batch`, `schema-readiness`, and `implementation-readiness` reports.

The early tooling gate must decide whether these are sufficient for the first vertical primitive. If not, tool gaps are first-class blockers for scaling, not optional polish.

Minimum acceptance criteria:

1. One command sequence can validate a primitive fixture structurally, render it, sample a timeline, diff two samples, and capture dense per-cell evidence.
2. Per-frame evidence includes sample timing (`phaseT`, `loopT`, absolute/sample milliseconds where applicable), dimensions, render hash, non-empty cell count, rows, sparse cells, diagnostics, and style-known/substrate provenance.
3. Dense capture can answer row/column/glyph/style/role questions for every sampled frame and preserve scene/source/layer provenance when available.
4. Reports distinguish structural validity, player rendering, direct compositor-next execution, runtime support, field coverage, and parity/oracle status.
5. Known limitations are documented before primitive scaling. In particular, `capture-cells --sample-ms` intentionally fixes all frames at one elapsed sample; omit `--sample-ms` when sweeping frames over `--duration-ms`.
6. If v2-era tooling has a capability that v3.1 still lacks and that capability is needed to sign off the first primitive, implement the v3.1 equivalent before continuing the primitive family.

### Phase 4 — Primitive Workbench MVP Design

1. Define the co-located primitive source-tree layout.
2. Define `primitive.toml` or equivalent metadata file.
3. Define generated file ownership rules:
   - generated files are overwritten by tooling;
   - hand-owned runtime files are never overwritten;
   - generated files include headers identifying source descriptor and generator version.
4. Define workbench MVP commands:
   - `scaffold <primitive-id>`
   - `validate <primitive-id>`
   - `migrate-v2 <primitive-id> --source <path>`
   - `report <primitive-id>`
5. Define generated outputs for first MVP:
   - typed input struct;
   - accessor/extraction helper;
   - unsupported-field guard;
   - fixture skeleton;
   - validation manifest;
   - migration mapping skeleton;
   - docs/control metadata stub.

Acceptance criteria:

- Primitive source-tree layout is documented.
- Generated vs hand-owned paths are documented.
- Workbench MVP can be implemented for one primitive without committing to every primitive family.

### Phase 5 — Select the First Representative Primitive

Recommended first candidate: `shader.highlighter`.

Why:

- It is rich enough to exercise real schema/runtime boundaries.
- It includes color, numeric fields, enums, direction/mode/apply-to semantics, and known unsupported cases.
- It is not as broad as terminal fire/water.
- Current code already has descriptor entries and compositor/style implementation surfaces.

Alternative candidates:

- `shader.linearGradient` — safer but may not expose enough complexity.
- `shader.focusField` — rich geometry/falloff case, good second primitive.
- `shader.glistenBand` — good for direction/blend/band-width decisions.

Acceptance criteria:

- First primitive is chosen.
- Existing descriptor, recipes, runtime files, lowerer paths, and tests are inventoried.
- A first-slice checklist is created before edits.

### Phase 6 — First Primitive Vertical Slice

For the selected primitive, perform the full end-to-end slice.

1. Create co-located primitive tree:

```text
primitives/shader/highlighter/
  descriptor.v31.json
  migration.v2.json
  primitive.toml
  generated/
  runtime/
  fixtures/
  tests/
  docs/
```

2. Copy or reference the existing descriptor data into the primitive tree.
3. Generate typed input/accessor scaffold.
4. Connect scaffold to compositor-next runtime behavior.
5. Add strict unsupported-field diagnostics for semantics compositor-next does not support.
6. Add minimal fixture.
7. Add V2 parity fixture if a source recipe exists.
8. Add validation manifest.
9. Add player → compositor-next direct v3.1 route for this primitive only.
10. Run full vertical validation.
11. Perform commonality extraction review.
12. Sign off the primitive.

Acceptance criteria:

- Structural validation passes.
- Field coverage reports no unhandled authored fields.
- Strict compositor-next execution succeeds without fallback.
- Old compositor vs compositor-next parity holds for the copied behavior path.
- V2 oracle parity holds where applicable, or blocker is explicit.
- Unsupported fields fail loudly.
- No unrelated primitive behavior changes.

### Phase 7 — Primitive Signoff Artifact

Each primitive gets a signoff artifact under the primitive tree or `.omx/reports/`.

Required fields:

- primitive id;
- family;
- descriptor path;
- generated files;
- hand-owned runtime files;
- fixtures;
- validation commands;
- parity evidence;
- unsupported decisions;
- commonality extraction decisions;
- OFPF/file-size decisions;
- known risks;
- signoff status.

Acceptance criteria:

- The primitive has a durable audit trail.
- Future agents can understand what is generated, what is hand-owned, and what is intentionally unsupported.

### Phase 7.5 — First From-Scratch Primitive Test: IndexedField

After several existing primitives have proven the copy-and-align workflow, use a new primitive as the first from-scratch test of the Primitive Workbench and compositor-next process.

Test primitive:

- Spec: [`../design/post-release/indexed-palette-cycling-spec.md`](../design/post-release/indexed-palette-cycling-spec.md)
- Working descriptor id: `source.indexedField`
- Classification: **source descriptor**, not effect descriptor
- Schema impact: **zero**; required fields already exist in v3.1 schema

Why it is a source:

`IndexedField` produces cells from pattern, palette, and rotation. It is a shader-family source: it generates a colored field rather than transforming existing cells. That classification matters because source descriptors can already consume assets through existing `SourceDescriptor` / `SourceSpec` asset slots.

Do not model this as an effect unless the schema gains a separate first-class `AssetRef` input/value path for effect descriptors. That is a separate future design concern and should not block `IndexedField`.

Practical mapping against existing v3.1 schema:

| Need | How it lands | Schema work |
| --- | --- | --- |
| Indexed-palette shader/source | New `SourceDescriptor` registered at runtime | none |
| Pattern selector, eight named patterns | Descriptor-internal `Value::Enum` input | none |
| Inline shader-local palette | Existing `Value::Gradient` plus `Value::Integer` entry count; descriptor samples N-entry palette | none |
| Cross-recipe shared palette | `AssetKind::Custom { name: "palette" }` plus stable format string `tui-vfx.palette.v1`; consumed through source asset slot | none |
| Render mode: full/half/quarter cell | Descriptor-internal `Value::Enum` input | none |
| Rotation speed, direction, binding | Existing `Value::Number` and binding mechanism | none |
| Glyph-set selection for quarter-cell mode | Descriptor enum or `AssetKind::Custom { name: "glyphSet" }` | none |

IndexedField vertical-slice goals:

1. Create co-located primitive tree under the chosen primitive root.
2. Add `source.indexedField` descriptor.
3. Generate source input accessors and fixture skeleton through Primitive Workbench.
4. Implement runtime source generator in compositor-next/player source path using the existing schema seams.
5. Add inline gradient-palette fixture.
6. Add shared palette asset fixture using `AssetKind::Custom { name: "palette" }` and format `tui-vfx.palette.v1`.
7. Add render-mode fixtures for full, half, and quarter cell modes.
8. Add pattern fixtures for the eight named patterns from the spec.
9. Add validation reports proving no schema changes were required.
10. Add signoff artifact marking this as the first successful from-scratch primitive workflow test.

Acceptance criteria:

- `source.indexedField` validates through existing v3.1 source/asset schema.
- Runtime can render inline and shared-palette variants.
- Rotation speed/direction/binding are exercised.
- Full/half/quarter render modes are exercised.
- No effect-descriptor `AssetRef` workaround is introduced.
- The workflow demonstrates that a new primitive can start from descriptor/schema, generate scaffolding, implement runtime behavior, and ship without contract changes when existing seams are sufficient.

### Phase 8 — Iterate Primitive by Primitive

Repeat the full vertical slice for the next primitive.

Recommended order:

1. `shader.highlighter`
2. `shader.linearGradient`
3. `shader.focusField`
4. `shader.glistenBand`
5. `shader.borderSweep`
6. `shader.revealWipe`
7. Remaining shader primitives by complexity and migration demand
8. Filters
9. Masks
10. Samplers
11. Style effects
12. Complex/composition primitives last

Rules:

- Do not batch-generate all primitives and fill behavior later.
- Do not accept a primitive from smoke evidence alone.
- Do not migrate a directory of recipes that depends on unsigned primitives.
- If the second or third primitive reveals workbench design flaws, fix the workbench before proceeding.

Acceptance criteria:

- Each primitive is signed off independently.
- Workbench abstractions improve as repetition appears.
- Shared helpers emerge only from proven repetition.

### Recipe Corpus Policy

The current `recipes/v3.1/debug_recipes/` tree should remain available during compositor-next development. It is useful as a human-readable reference and contributor evidence corpus even when individual recipes are not yet signed off by the new vertical process.

Do not archive or reseed this tree until compositor-next and Primitive Workbench have proven the replacement workflow and the owner explicitly approves the transition.

Recommended classifications for current recipes:

- `reference-current` — useful current artifact, not necessarily signed off by compositor-next.
- `exploratory` — contributor or experiment output retained for inspection.
- `validated-v31` — proven by the new vertical validation process.
- `needs-regeneration` — should be regenerated once its primitive is signed off.
- `blocked-by-primitive` — recipe waits for a primitive vertical slice.
- `superseded-by-workbench` — replaced by generated/validated output but retained in archive.

Archive policy:

```text
Current tree remains visible until replacement workflow is proven.
When confident, archive the existing tree as historical/reference evidence.
Then start a clean generated/validated v3.1 debug recipe tree.
Do this only with owner approval.
```

### Phase 9 — Resume Recipe Migration on Signed-Off Primitives

Once a primitive is signed off, recipe migration for recipes depending on that primitive can resume.

Workflow:

```text
V2 source recipe
  → migration mapping for signed primitive
  → canonical v3.1 recipe
  → compositor-next backend output
  → V2 oracle comparison
  → recipe signoff
```

Acceptance criteria:

- Recipe migration uses signed primitives only.
- V2 parity evidence is deterministic.
- Missing primitive support blocks the recipe honestly instead of being worked around.

### Phase 10 — Clean Cutover Strategy

1. Keep the old compositor available while compositor-next matures, but do not route v3.1 work through old bridge/shim code.
2. Add a clean runtime selector or `player-next` entrypoint only if it routes canonical v3.1 recipes directly into compositor-next-owned v3.1 boundaries.
3. Validate at recipe load, then pass canonical v3.1 structures and explicit sample context into compositor-next.
4. If old player/runtime code requires translation to progress, strip a copied `player-next` path instead of expanding that translation layer.
5. Use CI to run old compositor tests and compositor-next tests side by side until cutover.
6. Promote compositor-next only after a sufficient signed primitive set and recipe corpus slice pass parity.
7. Retire old paths only after explicit owner approval.

Acceptance criteria:

- No forced migration to compositor-next before evidence exists.
- Current working behavior remains available.
- Cutover is explicit and reversible.
- The v3.1 path validates at recipe load and passes canonical v3.1 through directly; bridge/shim code is not added to make legacy or compositor-shaped inputs work.

## Validation Gates

### Per-Primitive Required Gates

Each primitive must pass:

1. Descriptor/schema validation.
2. Generated code compile check.
3. Unit tests for input/accessor behavior.
4. Unit tests for runtime primitive behavior where applicable.
5. Old compositor vs compositor-next parity smoke.
6. Player → compositor-next strict v3.1 execution.
7. Fixture QC.
8. Timeline, frame-diff, and dense cell-capture evidence when visual behavior changes over time or needs localization.
9. Primitive field coverage.
10. Unsupported-field diagnostics.
11. V2 oracle parity where source evidence exists.
12. Commonality extraction review.
13. OFPF/file-size review for touched files.

### Per-Family Optional Gates

For each primitive family, add aggregate checks after several primitives are signed off:

- family registry completeness;
- family descriptor consistency;
- shared helper coverage;
- generated docs/control catalog consistency;
- cross-primitive conflict tests.

## Commonality Extraction Checklist

Before signing off a primitive, check whether it repeats patterns already seen in 3 or more places.

Common candidates:

- color parsing and blending;
- apply-to foreground/background routing;
- gradient sampling;
- bindable progress resolution;
- phase and time normalization;
- side/edge/axis/direction enum normalization;
- falloff/radius/feather distance math;
- seeded noise helpers;
- glyph ramp helpers;
- partial-block/subcell encoding;
- shader blend policies;
- unsupported-field diagnostics;
- migration rename tables.

Decision outcomes:

- `extracted-now` — safe small extraction performed in this slice;
- `ticketed` — extraction needed but too broad for this primitive slice;
- `not-repeated` — no 3+ repetition yet;
- `intentionally-local` — repeated shape looks similar but semantics differ.

## OFPF Alignment Policy

Because compositor-next starts as a copy, do not mix large cleanup with the initial copy commit. File-size guidance is a real design constraint: ~300 LOC is the target, and >500 LOC should split unless strong cohesion justification is documented. Some cohesive implementations may still be clearer and safer when kept together, but that should be an explicit exception rather than the default.

Use this sequence:

1. Copy and prove parity.
2. Inventory large files.
3. For the active primitive only, split files mechanically if needed and when the resulting boundaries are natural.
4. Prefer files near the 300 LOC target for focused agent work.
5. Split files above 500 LOC unless a strong written cohesion justification is recorded.
6. A small overage beyond the target is acceptable when cohesion is better preserved.
7. Run tests after any mechanical split.
8. Then perform semantic/schema-boundary changes.
9. Run full primitive validation.

This keeps cleanup reviewable, prevents behavior drift from being hidden inside file movement, and avoids both artificial fragmentation and multi-thousand-line focus traps.

## Risks and Mitigations

| Risk | Mitigation |
| --- | --- |
| Copied crate diverges from old compositor before parity is proven | Mechanical copy commit first; old-vs-next parity smoke before changes |
| Workbench over-generates behavior | Generator owns boilerplate only; humans own semantics |
| Vertical slices become too slow | Speed comes after first several slices; do not optimize before boundary issues are known |
| OFPF cleanup changes behavior | Mechanical split first, then tests, then semantic edits |
| Primitive mappings silently drop fields | Generated unsupported-field guards and field coverage are mandatory |
| One signed primitive creates false confidence | Every primitive must go end-to-end and receive its own signoff |
| Recipe migration gets ahead of runtime | Recipes depending on unsigned primitives remain blocked |
| Shared helper extraction overgeneralizes | 3+ rule plus semantic review; ticket rather than extract when uncertain |

## Initial Implementation Milestones

### Milestone A — Plan and Baseline

- Architecture docs committed.
- Current exploratory recipe/code state documented rather than broadly reverted.
- Additive compositor-next/workbench paths identified.
- `compositor-next` crate name accepted.
- Descriptor/schema hindsight audit scoped.
- First primitive selected.

### Milestone B — Copied Runtime

- `tui-vfx-compositor-next` exists.
- Copy/rename compiles.
- Baseline copied tests pass.
- Old-vs-next parity smoke exists.

### Milestone C — Schema Audit and Workbench MVP

- Descriptor/schema hindsight audit completed for first primitive family.
- Accepted common concepts are available to the workbench.
- Primitive source-tree layout exists.
- Workbench scaffolds first primitive.
- Generated ownership rules are documented.

### Milestone D — First Signed Primitive

- First shader primitive is end-to-end in compositor-next.
- Strict backend execution works.
- Fixture QC and field coverage pass.
- V2 parity is proven or exact blocker is recorded.
- Commonality/OFPF review complete.

### Milestone E — Repeatable Pattern

- Second and third primitives complete.
- Workbench design adjusted based on real issues.
- At least one common helper extraction is performed or explicitly rejected with rationale.

### Milestone F — From-Scratch Primitive Workflow Test

- `source.indexedField` is implemented from `docs/design/post-release/indexed-palette-cycling-spec.md`.
- It lands as a source descriptor with zero schema changes.
- Inline palette, shared palette asset, render mode, rotation, and pattern variants are validated.
- Signoff confirms the Primitive Workbench can support a new primitive from scratch.

### Milestone G — Resume Targeted Recipe Migration

- Recipes using signed primitives migrate through compositor-next.
- Directory migration resumes only where primitive support is signed off.

## Definition of Done for Compositor-Next MVP

The MVP is done when:

- compositor-next exists as a copied, tested crate;
- current exploratory recipe corpus remains available or is archived only with owner approval;
- one representative shader primitive is fully vertical-slice signed off;
- one from-scratch source descriptor primitive (`source.indexedField`) is proven without schema changes;
- Primitive Workbench can regenerate that primitive's boilerplate using accepted common schema concepts;
- player/runtime can execute that load-validated canonical v3.1 primitive through compositor-next strict mode;
- old compositor behavior parity is proven for the copied path;
- V2 oracle parity is proven for at least one migrated recipe using the primitive;
- commonality extraction and OFPF review, including any >500 LOC cohesion justifications, are part of the signoff artifact;
- docs and indexes are updated.

## Summary

```text
Work forward additively without broad reverts.
Copy the hardened compositor.
Prove behavior parity.
Audit descriptor/schema commonality before baking it into generators.
Pick one existing primitive.
Drive it vertically through schema, generated code, runtime, backend, fixtures, migration, validation, and signoff.
Extract common patterns when they recur.
Repeat primitive by primitive.
Then prove the workflow on one new from-scratch primitive: `source.indexedField`.
Only resume broad recipe migration on signed-off primitives.
```

<!-- <FILE>docs/arch/compositor-next-vertical-implementation-plan.md</FILE> - <DESC>Detailed implementation plan for copied compositor-next and vertical primitive-by-primitive migration</DESC> -->
<!-- <VERS>END OF VERSION: 0.4.0</VERS> -->
