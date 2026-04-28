<!-- <FILE>docs/new_kernel/AGENT_BRIEFING.md</FILE> - <DESC>Reusable briefing for clean-room kernel agents and phase workers</DESC> -->
<!-- <VERS>VERSION: 0.6.0</VERS> -->
<!-- <WCTX>New kernel Phase E0: add physical contract split guidance.</WCTX> -->
<!-- <CLOG>0.6.0: MINOR — add tui-vfx-contract ownership, schema paths, and E0 phase guidance.
0.5.0: MINOR — add Phase D3 contract/engine boundary guidance and mark D2 complete.
0.4.0: MINOR — add Phase D2 template composition design history and current-phase guidance.
0.3.1: PATCH — include public SceneOutcome schema root in D1 guidance.
0.3.0: MINOR — add Phase D1 scene composition history, schema roots, and verification gate.
0.2.0: MINOR — add Phase D0 schema/reference readiness, architecture overview, and updated phase history.
0.1.2: PATCH — add 90_recycle_bin.md to read order and summarize move-instead-of-delete protocol.</CLOG> -->

# New Kernel Agent Briefing

This briefing is the reusable starting point for agents working on the clean-room `tui-vfx-contract` / `tui-vfx-next` kernel stack. It should evolve as phases complete. Phase-specific PRDs and test specs add local detail; this file carries durable institutional knowledge.

## Mandatory read order

Read these before planning or editing:

1. `docs/new_kernel/AGENT_BRIEFING.md` — this file.
2. `steering/INTENTIONS.md` — durable project rules and architectural intent.
3. `steering/OFPF-TOOLS.md` — OFPF/librarian tool usage rules.
4. `../global_prompts/standards/00_tooling_preamble.md` — OFPF tools first.
5. `../global_prompts/standards/40_ofpf_standards.md` — file naming and size limits.
6. `../global_prompts/standards/50_tdd_protocol.md` — red/green/refactor discipline.
7. `../global_prompts/standards/60_file_centric_execution.md` — group work by file.
8. `../global_prompts/standards/65_subagent_orchestration.md` — bounded task packet/report shape.
9. `../global_prompts/standards/70_metadata_headers.md` — required file metadata.
10. `../global_prompts/standards/85_edge_cases.md` — extraction and edge-case judgment.
11. `../global_prompts/standards/90_recycle_bin.md` — move-instead-of-delete protocol for removed files.
12. Phase-specific docs, for example:
    - `docs/new_kernel/DRAFT_CONTRACTS.md`
    - `docs/new_kernel/PROJECT_KICKOFF.md`
    - `docs/new_kernel/ARCH-RESP-TO-PHASE_A.md`
    - `docs/new_kernel/ARCH-RESP-TO-PHASE_B.md`
    - `docs/new_kernel/ARCH-RESP-TO-PHASE_C.md`
    - `docs/v3.1-architecture-overview.md`
    - `docs/v3.1-template-composition.md`
    - `docs/v3.1-contract-boundary.md`
    - `.omx/plans/prd-new-kernel-phase-*.md`
    - `.omx/plans/test-spec-new-kernel-phase-*.md`

In final reports, agents must explicitly confirm which governing docs they read.

Leader-only note: `steering/ORCHESTRATION.md` is guidance for the leader/orchestrator. Do not put it in subagent must-read lists; distill its relevant packet rules into each subagent prompt instead.

## Project intent that matters most here

- `tui-vfx` is grid-first and ecosystem-agnostic. The semantic surface is not ratatui-specific.
- Clean version boundaries are allowed to break confusing legacy names. Do not preserve legacy aliases in the clean-room kernel.
- Documentation is part of the engineering contract. Public behavior changes require docs/rustdoc updates.
- Build the contract before porting effects. Every semantic fact needs one owner, one test, and one generated reference path.
- Work is test-first and audit-gated. A phase is not complete until verification and review pass.
- Fix root causes. Do not add local `#[allow]` escapes or leave known landmines.
- Use OFPF tools for repo questions. Do not guess file contents, symbols, or dependencies.

## OFPF tooling rules

Use `ofpf-*` tools before ad hoc reading/searching:

- Health/orientation: `ofpf-status`, `ofpf-overview`, `ofpf-orientation --root /usr/projects/tui-vfx`.
- File lookup: `ofpf-search-files <pattern>` for indexed code files.
- Content search: `ofpf-content <pattern>`; add `--glob` for docs/scripts/config.
- Focused reads: `ofpf-focus <path>`, `ofpf-read <path> --from N --to M`, `ofpf-extract <path> <symbol>`, `ofpf-around <path> <query>`.
- Dependencies/impact: `ofpf-context <path>`, `ofpf-inspect <path>`, `ofpf-blast <path>`, `ofpf-trace <from> <to>`.
- Tests/metrics: `ofpf-tests <path>`, `ofpf-loc <threshold>`, `ofpf-dag`, `ofpf-cycles`.

Treat zero results as valid data. Try 2-3 query variants before concluding something is absent. If a tool errors, correct syntax/check daemon/retry before falling back.

## OFPF file naming and size rules

Clean-room source files must follow OFPF naming unless there is a strong Rust module-root reason not to.

| Prefix | Purpose | Soft / hard LOC |
|---|---|---|
| `fnc_` | Single public function/helper | 75 / 120 |
| `col_` | Pure leaf helpers | 50 / 100 |
| `orc_` | Coordinates `fnc_` calls | 150 / 250 |
| `cls_` | Single cohesive class/type/enum | 150 / 200 |
| `ui_` | Presentation layer | 150 / 200 |
| `tr_` | Trait files in this Rust repo convention | keep small and cohesive |
| `test_` | Test files | no strict limit |

Current clean-room convention:

- `crates/tui-vfx-contract/src/lib.rs` and `crates/tui-vfx-next/src/lib.rs` are crate-root exceptions.
- New source files under `crates/tui-vfx-contract/src` and `crates/tui-vfx-next/src` should be prefixed (`cls_`, `fnc_`, `tr_`, etc.).
- Keep one logical unit per file. Split before exceeding hard LOC limits.
- Do not add broad aggregation files to dodge naming rules.
- Test files under `crates/tui-vfx-contract/tests` and `crates/tui-vfx-next/tests` should use `test_*.rs`.

## Metadata rules

Every changed source, test, and markdown file should carry metadata when practical.

Header shape:

```text
<comment> <FILE>path/to/file.ext</FILE> - <DESC>Short purpose</DESC>
<comment> <VERS>VERSION: x.y.z</VERS>
<comment> <WCTX>Work context</WCTX>
<comment> <CLOG>Change summary</CLOG>
```

Footer shape for source/test files:

```text
<comment> <FILE>path/to/file.ext</FILE> - <DESC>Short purpose</DESC>
<comment> <VERS>END OF VERSION: x.y.z</VERS>
```

Use Rust `//` comments and markdown `<!-- -->` comments. Update `<CLOG>` for the latest file change; git history is the long-term changelog.


## Schema/reference readiness rules

Phase D0 makes this a standing rule for all future clean-room phases:

- Serde owns JSON wire shape.
- Schemars generates JSON Schema from Rust types.
- Rustdoc comments become schema/reference descriptions.
- Generated schemas/reference docs are public contract artifacts.
- Public v3.1 contract-visible types must derive or intentionally implement `Serialize`, `Deserialize`, and `JsonSchema` where JSON-facing.
- Closed JSON-facing structs should use `deny_unknown_fields`; JSON-facing fields should be camelCase unless an existing foundation type intentionally preserves another shape.
- Enums need an explicit tagging strategy.
- Every public contract-visible type, field, variant, and non-obvious policy/default needs rustdoc.
- Intentionally internal public helpers must be marked or explained in status docs.

Checked stable contract schema roots live under `schemas/v3.1/contract/`: surface, scope, write, diagnostic, scene, element, and outcome. Proof-pipeline roots remain under `schemas/v3.1/next/`: sampler and pipeline.

## Recyclebin protocol

When a file must be removed during cleanup or refactor, do not delete it directly. Follow `../global_prompts/standards/90_recycle_bin.md`:

1. Check callers/callees and document impact.
2. Move the file to `recyclebin/` while mirroring the project path.
3. Update imports/references so active code no longer uses the moved file.
4. Move related tests to the matching `recyclebin/tests/` path when applicable.
5. Ensure `recyclebin/` is ignored.
6. Empty the recycle bin only with explicit user approval.

This may not be needed in every phase, but all agents must know the protocol before refactor work.

## Clean-room kernel boundaries

`crates/tui-vfx-contract` owns stable v3.1 DTOs and schema roots. `crates/tui-vfx-next` is a proof engine beside the existing legacy engine and depends on `tui-vfx-contract`.

Allowed:

- Depend on `tui-vfx-types`.
- Depend on `tui-vfx-geometry` only if actually needed.
- Add tiny test-only/proof effects when needed to prove semantics.
- Update `docs/v3.1-surface-contract.md` and phase status memos.

Forbidden unless the owner explicitly changes phase scope:

- Do not replace/refactor legacy compositor code.
- Do not port real CRT/typewriter/matrix/shadow/etc. effects.
- Do not add recipe compiler, studio manifest, phase graph, trigger engine, runtime binding system, or legacy aliases.
- Do not depend on `tui-vfx-next`, `tui-vfx-compositor`, `tui-vfx-style`, `tui-vfx-content`, or `tui-vfx-shadow` from `tui-vfx-contract`.
- Do not depend on `tui-vfx-compositor`, `tui-vfx-style`, `tui-vfx-content`, or `tui-vfx-shadow` from `tui-vfx-next`.
- Do not implement broad descriptor/schema/runtime systems before the architect approves that phase.

## Phase history summary

### Phase A — semantic surface contract

Locked:

- Surface = dense cell grid + semantic role channel + metadata.
- Role is a surface-position semantic channel, not a field on `Cell`.
- Visual-only effects preserve destination roles.
- Copy/transform writes sampled-source roles by default.
- Skipped cells preserve destination cell and role.
- Zero-cell scopes emit structured diagnostics.
- Procedural/shadow-like writers can explicitly set roles.
- Empty transparent writes are distinct from skips.

Key docs:

- `docs/new_kernel/PROJECT_KICKOFF.md`
- `docs/new_kernel/PHASE_A_STATUS_MEMO_TO_ARCHITECT.md`
- `docs/new_kernel/ARCH-RESP-TO-PHASE_A.md`

### Phase B — sampled-source semantics

Locked:

- Destination coordinate may map to a different sampled source coordinate.
- `ShiftSampler { dx, dy }`: destination `(x, y)` samples source `(x + dx, y + dy)`.
- Out-of-bounds samples return `None` and skip writes.
- Role scopes default to sampled-source roles.
- Geometry scopes default to destination-local coordinates.
- `RoleSpace::Destination` is explicit and behaves differently.
- Zero-cell diagnostics are sampler-aware.

Key docs:

- `docs/new_kernel/PHASE_B_STATUS.md`
- `docs/new_kernel/PHASE_B_STATUS_MEMO_TO_ARCHITECT.md`
- `docs/new_kernel/ARCH-RESP-TO-PHASE_B.md`

### Phase C — ordered linear pipeline semantics

Locked:

- Each stage reads the current surface and writes the next surface.
- Later stages see earlier stage cell and role writes.
- Stage order is deterministic and semantic.
- Skips preserve the current stage destination, not the original input.
- Diagnostics include stage identity and remain ordered.
- In a pipeline, sampled-source role means the role at the sampled coordinate of the stage read surface.

Key docs:

- `.omx/plans/prd-new-kernel-phase-c.md`
- `.omx/plans/test-spec-new-kernel-phase-c.md`
- `docs/new_kernel/PHASE_C_STATUS.md`
- `docs/new_kernel/PHASE_C_STATUS_MEMO_TO_ARCHITECT.md`
- `docs/new_kernel/ARCH-RESP-TO-PHASE_C.md`

### Phase D0 — schema/reference backfill

Locked:

- Public contract-visible Phase A/B/C types are schema-reference ready.
- Rustdoc-backed Schemars output proves type, field, and variant descriptions are available to tools.
- Checked schemas lived in `schemas/v3.1/next/` through D3; E0 moves stable roots to `schemas/v3.1/contract/`.
- The architecture overview records the contract-first philosophy and progressive phase stack.
- Runtime behavior remained unchanged.
- Descriptor expansion, recipes, studio, runtime bindings, real effects, and legacy migration stayed out of scope.

Key docs:

- `.omx/plans/prd-new-kernel-phase-d0.md`
- `.omx/plans/test-spec-new-kernel-phase-d0.md`
- `docs/v3.1-architecture-overview.md`
- `docs/new_kernel/PHASE_D0_STATUS.md`
- `docs/new_kernel/ARCH-RESP-TO-PHASE_D0.md`

### Phase D1 — scene / element / layer composition semantics

Locked:

- `Scene` composes multiple placed `SceneElement` values into one final `Surface`.
- `ElementId` is instance identity and remains distinct from `RoleTag`.
- Optional `LayerId` is lightweight grouping only; no full layer graph.
- Element placement uses signed scene coordinates so partially offscreen elements can clip.
- Composition order is ascending `z_index`; declaration order breaks ties deterministically.
- Higher/later written cells overwrite lower/current cells and roles according to write policy.
- `SkipTransparentEmpty` on a top element preserves lower/current cell and role.
- `WriteCell` writes transparent empty cells and can clear lower/current content.
- Scene diagnostics identify element id with paths such as `scene.element[index].id`.
- Scene, element, and outcome schema roots are checked under `schemas/v3.1/contract/` after E0.

Still out of scope:

- Effect descriptor expansion.
- Recipe schema/compiler.
- Studio manifest.
- Runtime bindings.
- Phase graph or trigger engine.
- Legacy migration.
- Real effect ports.
- Template inheritance implementation.
- Full layer graph or complex blending.

Key docs:

- `docs/new_kernel/ARCH-RESP-TO-PHASE_D0.md`
- `docs/v3.1-architecture-overview.md`
- `docs/v3.1-surface-contract.md`
- `docs/new_kernel/PHASE_D1_STATUS.md`
- `docs/new_kernel/PHASE_D1_STATUS_MEMO_TO_ARCHITECT.md`
- `docs/new_kernel/ARCH-RESP-TO-PHASE_D1.md`


### Phase D2 — template composition design

Locked:

- Template composition is compile-time authoring behavior, not runtime inheritance.
- Runtime receives only canonical expanded strict v3.1 recipes.
- Templates define reusable structure and slots.
- Mixins/traits are additive reusable fragments with deterministic order.
- Presets and profiles are values-only.
- Expansion namespaces ids deterministically.
- Slot contracts define accepted kind, cardinality, id namespace, and diagnostics.
- Conflicts require explicit override syntax or fail.
- Sealed/final fields protect safety and semantic identity.
- Diagnostics report both source-template path and expanded canonical path.
- Canonical v3.1 recipes contain no template refs, inheritance pointers, presets, profiles, mixin references, or legacy aliases.

Still out of scope:

- Template expansion implementation.
- Recipe schema/compiler implementation.
- Runtime inheritance.
- Effect descriptor expansion.
- Studio manifest.
- Runtime bindings.
- Phase graph or trigger engine.
- Legacy migration.
- Real effect ports.

Key docs:

- `docs/new_kernel/ARCH-RESP-TO-PHASE_D1.md`
- `docs/v3.1-template-composition.md`
- `docs/new_kernel/PHASE_D2_STATUS.md`
- `docs/new_kernel/PHASE_D2_STATUS_MEMO_TO_ARCHITECT.md`
- `docs/new_kernel/ARCH-RESP-TO-PHASE_D2.md`

### Phase D3 — contract / engine boundary

Locked.

Target lock:

- Classify stable v3.1 public contract vocabulary.
- Classify clean-room engine/proof implementation.
- Classify test-only scaffolding.
- Identify checked schema roots and proof roots.
- Keep `tui-vfx-next` as one crate; use logical grouping before physical split.
- Treat `ScopeSpec` as the current canonical generalized scope vocabulary.
- Keep `CoordinateSpace` and `RoleSpace` as operation-level context for now.
- Confirm pipeline and scene composition reuse `CellWritePolicy` and `RoleWritePolicy`.
- Keep diagnostic path strings for now; defer structured identity fields until descriptor/recipe schemas exist.

D3 guardrails:

- `SurfacePipeline` is a checked proof-pipeline root, not the final runtime graph.
- `PipelineStage` is a toy proof enum, not the future effect descriptor model.
- `DimEffect`, `ExplicitRoleWriteEffect`, and the tiny `EffectDescriptor` are proof artifacts.
- D3 must not implement descriptors, recipe schema/compiler, source authoring schemas, template expansion, runtime bindings, phase graph, trigger engine, studio manifest, legacy migration, real effect ports, full layer graph, or complex blending.

Key docs:

- `docs/new_kernel/ARCH-RESP-TO-PHASE_D2.md`
- `docs/v3.1-contract-boundary.md`
- `docs/v3.1-surface-contract.md`
- `docs/new_kernel/PHASE_D3_STATUS.md`
- `docs/new_kernel/PHASE_D3_STATUS_MEMO_TO_ARCHITECT.md`
- `docs/new_kernel/ARCH-RESP-TO-PHASE_D3.md`


### Phase E0 — physical contract split

Current phase.

Target lock:

- `crates/tui-vfx-contract` exists and owns stable v3.1 DTOs.
- `crates/tui-vfx-next` depends on `tui-vfx-contract` and keeps proof execution.
- `tui-vfx-contract` has no dependency on `tui-vfx-next` or legacy compositor/style/content/shadow crates.
- Stable schemas generate from `tui-vfx-contract` under `schemas/v3.1/contract/`.
- Proof-pipeline schemas remain under `schemas/v3.1/next/`.
- `PipelineStage` remains proof-only and is not promoted to descriptor model.
- No effect descriptors, recipes, runtime, studio, migration, or real effects are implemented.

Key docs:

- `docs/new_kernel/ARCH-RESP-TO-PHASE_D3.md`
- `docs/v3.1-contract-boundary.md`
- `docs/new_kernel/PHASE_E0_STATUS.md` once created.

## Verification gates

Before reporting completion for code changes, run and read output:

```bash
cargo fmt --package tui-vfx-contract -- --check
cargo fmt --package tui-vfx-next -- --check
cargo clippy -p tui-vfx-contract --all-targets -- -D warnings
cargo clippy -p tui-vfx-next --all-targets -- -D warnings
cargo test -p tui-vfx-contract
cargo test -p tui-vfx-next
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-contract --test test_schema_generation -- checked_in_contract_schemas_are_current
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-next --test test_schema_generation -- checked_in_proof_schemas_are_current
cargo test -p tui-vfx-contract --test test_schema_generation
cargo test -p tui-vfx-next --test test_schema_generation
cargo tree -p tui-vfx-contract
cargo tree -p tui-vfx-next
grep -R -nE 'tui_vfx_(compositor|style|content|shadow)|tui-vfx-(compositor|style|content|shadow)' crates/tui-vfx-contract crates/tui-vfx-next
```

For phase completion, also run:

```bash
cargo test --workspace
```

The forbidden dependency grep should produce no matches. `cargo tree -p tui-vfx-contract
cargo tree -p tui-vfx-next` should show no dependency on compositor/style/content/shadow crates.

## Subagent packet requirements

Every subagent packet must include:

- The schema/reference requirement for any public contract-visible type it adds or modifies.

- One-sentence task and why it matters.
- Exact read order, starting with this briefing and steering docs.
- Exact write scope and explicit out-of-scope items.
- OFPF naming, LOC, metadata, and TDD requirements.
- Expected verification commands.
- Report format requiring changed files, commands run, evidence, and risks.
- A statement that the subagent is not alone in the codebase and must not revert others' edits.

For the owner-requested unroled `gpt-5.5` lanes, do not set a role in the spawn request. Use exact write scopes and concrete tests to prevent architectural freelancing.

## Reporting expectations

Final reports should be concise and evidence-dense:

- What changed.
- Files changed.
- Tests/commands run and pass/fail results.
- OFPF/deslop notes.
- Remaining risks/open questions.

Do not claim completion without fresh verification evidence and architect/reviewer approval when Ralph is active.

<!-- <FILE>docs/new_kernel/AGENT_BRIEFING.md</FILE> - <DESC>Reusable briefing for clean-room kernel agents and phase workers</DESC> -->
<!-- <VERS>END OF VERSION: 0.6.0</VERS> -->
