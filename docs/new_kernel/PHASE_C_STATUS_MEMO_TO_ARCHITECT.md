<!-- <FILE>docs/new_kernel/PHASE_C_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase C status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase C: summarize ordered pipeline/pass semantics proof for architect review.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add Phase C architect memo in the same style as Phase A/B status memos.</CLOG> -->

# Phase C Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-28
Repo: `/usr/projects/tui-vfx`

## Executive summary

Phase C has reached the proof point recommended in `ARCH-RESP-TO-PHASE_B.md`:

> Can the clean-room kernel execute multiple ordered stages while preserving the same surface, sampling, scope, write, skip, and diagnostic semantics proven in Phases A and B?

Current answer: **yes, for the bounded Phase C contract spike.**

The clean-room crate, `tui-vfx-next`, now includes a tiny ordered pipeline model. Each stage reads the current surface, writes a cloned next surface, and then next becomes current. Later stages therefore observe earlier stage cell writes and role writes. Skipped writes preserve the current stage destination, not the original input. Diagnostics are emitted in deterministic stage order and carry stage identity.

The implementation deliberately remains a semantic proof. It does not add recipes, descriptor registry/model expansion, studio/runtime bindings, a phase graph, trigger semantics, legacy migration, or real effect ports.

## Current implementation state

### Existing clean-room crate extended

Crate:

```text
crates/tui-vfx-next
```

Phase C-relevant files:

```text
crates/tui-vfx-next/src/cls_surface_pipeline.rs
crates/tui-vfx-next/src/cls_pipeline_stage.rs
crates/tui-vfx-next/src/cls_pipeline_outcome.rs
crates/tui-vfx-next/src/cls_pipeline_sampler.rs
crates/tui-vfx-next/src/fnc_annotate_stage_diagnostic.rs
crates/tui-vfx-next/src/fnc_annotate_stage_diagnostics.rs
crates/tui-vfx-next/src/fnc_rewrite_glyph_cell.rs
crates/tui-vfx-next/src/lib.rs
crates/tui-vfx-next/tests/test_surface_contract.rs
```

Workspace wiring remains:

```text
Cargo.toml
Cargo.lock
```

### Added ordered pipeline semantics

New pipeline surface:

```text
SurfacePipeline
PipelineStage
PipelineOutcome
PipelineSampler
```

Tiny supported stage semantics:

```text
PipelineStage::Copy
PipelineStage::Dim
PipelineStage::ExplicitRoleWrite
PipelineStage::ReplaceGlyph
```

The pipeline rule is:

```text
current surface
    -> stage reads current
    -> stage writes cloned next surface
    -> next becomes current
    -> following stage reads prior stage output
```

That rule is now part of `docs/v3.1-surface-contract.md`.

### Added reusable agent/process docs

New durable briefing/index files:

```text
docs/new_kernel/AGENT_BRIEFING.md
docs/new_kernel/INDEX.md
```

`AGENT_BRIEFING.md` captures cross-phase institutional knowledge for future agents:

- mandatory read order;
- steering and OFPF tool expectations;
- OFPF file prefixes and LOC limits;
- metadata header/footer rules;
- recyclebin protocol from `../global_prompts/standards/90_recycle_bin.md`;
- clean-room dependency and scope boundaries;
- Phase A/B/C history;
- verification gates;
- subagent packet requirements.

It explicitly marks `steering/ORCHESTRATION.md` as leader-only guidance and says not to put it in subagent must-read lists. Leaders should distill relevant orchestration rules into concrete task packets instead.

`INDEX.md` indexes the new-kernel directory and includes the two parent-level v3.1 docs:

```text
docs/v3.1-surface-contract.md
docs/v3.1-feature-contract-checklist.md
```

## Goal-by-goal status against the Phase C recommendation

| Phase C goal / constraint | Current status |
|---|---|
| Add minimal ordered pipeline abstraction | **Done.** `SurfacePipeline` owns ordered `PipelineStage` values and returns `PipelineOutcome`. |
| Define stage read/write rule | **Done.** Each stage reads current, writes cloned next, and then next becomes current. |
| Later stages see earlier cell writes | **Done.** `pipeline_later_stage_reads_earlier_stage_cells` requires Stage 2 to rewrite a glyph materialized by Stage 1. |
| Later stages see earlier role writes | **Done.** `pipeline_later_stage_reads_earlier_stage_roles` requires Stage 2 to match a role written by Stage 1. |
| Stage order is deterministic and semantic | **Done.** `pipeline_stage_order_is_semantic` proves reversing two stages changes output. |
| Visual-only stages preserve prior roles | **Done.** `visual_stage_preserves_prior_stage_roles` proves dimming preserves roles materialized by an earlier stage. |
| Skips preserve current surface, not original input | **Done.** `stage_skip_preserves_current_surface` proves a skipped later-stage coordinate preserves Stage 1 output. |
| Zero-cell diagnostics include stage identity | **Done.** Diagnostics get `pipeline.stage[index].name` paths and stage-name message prefixes. |
| Diagnostics are deterministic and ordered | **Done.** `pipeline_diagnostics_are_deterministic` checks stage-order diagnostic paths. |
| Phase B sampled-source role semantics hold inside a pipeline | **Done.** `pipeline_keeps_phase_b_sampled_role_semantics` now requires Stage 2 to sample a Stage 1-materialized role/cell from the current surface. |
| Do not start recipe/descriptors/studio/runtime/phase graph | **Respected.** No recipe compiler, descriptor registry/model expansion, studio manifest, runtime store, trigger engine, or phase graph was added. |
| Do not port real effects | **Respected.** Only tiny proof stages exist; no CRT/typewriter/matrix/shadow/etc. porting was attempted. |
| Do not replace old compositor | **Respected.** Legacy compositor/style/content/shadow implementation files were not modified. |
| Preserve clean-room dependency boundary | **Respected.** `tui-vfx-next` still has no dependency on `tui-vfx-compositor`, `tui-vfx-style`, `tui-vfx-content`, or `tui-vfx-shadow`. |
| Preserve OFPF standards | **Done.** Clean-room source files were refactored to OFPF-prefixed names and kept below hard LOC limits. |

## Required tests now present

The required Phase C tests from the architecture response are present in `crates/tui-vfx-next/tests/test_surface_contract.rs`:

```text
pipeline_later_stage_reads_earlier_stage_cells
pipeline_later_stage_reads_earlier_stage_roles
pipeline_stage_order_is_semantic
visual_stage_preserves_prior_stage_roles
stage_skip_preserves_current_surface
stage_zero_cell_scope_diagnostic_names_stage
pipeline_diagnostics_are_deterministic
pipeline_keeps_phase_b_sampled_role_semantics
```

The Phase A and Phase B tests remain present in the same test file. The package currently has 24 integration tests for the clean-room surface/sampling/pipeline contract.

## Dependency and architecture status

Current dependency direction remains:

```text
tui-vfx-types
    ↓
tui-vfx-next
```

`cargo tree -p tui-vfx-next` shows no dependency on:

```text
tui-vfx-compositor
tui-vfx-style
tui-vfx-content
tui-vfx-shadow
```

No new third-party dependency was added for Phase C.

## OFPF / file-structure status

Before Phase C implementation continued, the existing clean-room crate was refactored to follow OFPF naming and file-size guidance.

Current convention:

```text
crates/tui-vfx-next/src/lib.rs       # crate-root exception
crates/tui-vfx-next/src/cls_*.rs     # cohesive type/enum/class files
crates/tui-vfx-next/src/fnc_*.rs     # helper/function files
crates/tui-vfx-next/src/tr_*.rs      # trait files
crates/tui-vfx-next/tests/test_*.rs  # tests
```

Largest relevant source files after Phase C/deslop:

```text
cls_pipeline_stage.rs              144 LOC
cls_surface.rs                     117 LOC
cls_surface_engine.rs              112 LOC
fnc_apply_from_source_with_sampler.rs 102 LOC
```

These remain below the hard limits from `../global_prompts/standards/40_ofpf_standards.md`.

## Verification evidence

Post-implementation, post-review-fix, and post-deslop verification passed:

```text
cargo fmt --package tui-vfx-next -- --check
cargo clippy -p tui-vfx-next --all-targets -- -D warnings
cargo test -p tui-vfx-next
cargo test --workspace
cargo tree -p tui-vfx-next
```

Package test result:

```text
24 passed; 0 failed
Doc-tests tui_vfx_next: 0 passed; 0 failed
```

Dependency guardrail check:

```text
grep -R -nE 'tui_vfx_(compositor|style|content|shadow)|tui-vfx-(compositor|style|content|shadow)' crates/tui-vfx-next
```

Result: no matches.

OFPF graph status after reload:

```text
ofpf-load
ofpf-status
```

Result: graph loaded and `is_stale: false`.

An independent architect verifier initially returned `REJECTED` for two issues:

1. the Phase C descriptor non-goal wording was contradictory because the tiny Phase A `EffectDescriptor` DTO still exists;
2. `pipeline_keeps_phase_b_sampled_role_semantics` could be stronger.

Both were fixed:

- Docs now say Phase C does not add descriptor registry/model expansion; the tiny Phase A `EffectDescriptor` proof DTO remains unchanged.
- The sampled-role pipeline test now requires Stage 2 to read a Stage 1-materialized role/cell from the current surface.

A second independent architect verifier returned:

```text
APPROVED
```

No blocking findings remained.

## Notable implementation choices

1. **Enum-based pipeline instead of trait object framework.**
   `PipelineStage` is a tiny enum. This avoids prematurely designing a full descriptor/runtime stage system before the contract is locked.

2. **Current/next buffer rule.**
   Each stage writes into a clone of current. That makes skip semantics crisp: a skipped cell preserves the current stage destination, which already contains prior stage output.

3. **Stage order is observable by design.**
   The tests intentionally use stage combinations where reversing order changes the final surface.

4. **Pipeline sampled-source role means stage-read-surface role.**
   Phase C clarifies that `sampled source` is relative to a stage's read surface. Once Stage 1 has materialized roles, Stage 2 samples from those current roles.

5. **Diagnostics are stage-aware without adding a runtime graph.**
   Diagnostics are annotated with `pipeline.stage[index].name`. This gives deterministic provenance without introducing recipe nodes, phase graphs, or trigger semantics.

6. **Glyph rewrite is a test helper, not a real effect.**
   `ReplaceGlyph` exists only to prove that a later stage reads a prior cell write. It is not a production effect port.

7. **No physical crate split.**
   The crate remains `tui-vfx-next`. Phase C kept the split logical and OFPF-file-based rather than introducing `tui-vfx-contract` / `tui-vfx-engine` crates.

## Scope-control status

The implementation did **not** attempt to:

```text
build full recipes
expand descriptor registry/model
build studio manifest
build runtime bindings
build phase engine
build trigger engine
port CRT
port typewriter
port matrix rain
port shadow rendering
replace old compositor
migrate legacy filters
support legacy aliases
split tui-vfx-next into multiple crates
```

That matches the Phase C boundary recommended in the Phase B architecture response.

## Deslop / cleanup performed after verification

After architect approval, a bounded deslop pass was run on Phase C-owned files only.

Changes made during deslop:

- extracted the multi-diagnostic stage annotation helper into `fnc_annotate_stage_diagnostics.rs` so `cls_surface_pipeline.rs` stays focused on current/next execution;
- updated stale Phase A/B wording in `docs/v3.1-surface-contract.md` where Phase C now also applies;
- preserved behavior and API shape.

All verification gates were rerun after that cleanup.

## Open questions / recommended next decisions

1. **Next semantic proof vs descriptor phase.**
   Phase C locks ordered stages. The next decision is whether to begin descriptor/schema work, or first prove another engine semantic such as named surfaces/layers, mask composition, or multi-source reads.

2. **Stage-name validation.**
   Stage names are caller-provided strings and are embedded in diagnostic paths. A later recipe/runtime phase should decide whether these need strict path-segment validation.

3. **Pipeline API shape before generalization.**
   The enum model is intentionally toy-sized. Before real descriptors or recipe nodes depend on it, decide whether stages become trait objects, descriptor-backed nodes, or a compiled runtime graph representation.

4. **Diagnostic schema.**
   Stage identity currently reuses `SurfaceDiagnostic.path`. That is enough for Phase C. Later runtime work may need structured stage/node fields rather than path strings.

5. **Descriptor maturity.**
   The tiny `EffectDescriptor` DTO remains a Phase A proof artifact. It has not become the real descriptor model from `DRAFT_CONTRACTS.md`.

6. **Crate topology.**
   `tui-vfx-next` remains the incubator. A physical contract/engine split may become useful after one more phase or when descriptor work starts.

7. **Legacy integration boundary.**
   No old compositor migration path has been validated yet. Continue deferring migration until the contract model is stable enough to prevent legacy assumptions leaking into the clean-room kernel.

## Bottom line

Phase C is complete as an ordered pipeline/pass semantics proof. The clean-room kernel now demonstrates that the semantic surface model holds across multiple ordered stages: later stages see earlier cell and role writes, skips preserve current output, diagnostics are stage-aware and ordered, and Phase B sampled-source semantics still apply when the sampled source is the stage read surface rather than the original input.

This closes the main execution-model ambiguity called out in the Phase B architecture response. The next useful step is to choose the next bounded contract layer: either begin descriptor/schema work now that “stage” has semantics, or lock one more engine semantic such as layers/named surfaces before descriptors consume it.
