<!-- <FILE>docs/new_kernel/AGENT_BRIEFING.md</FILE> - <DESC>Reusable briefing for clean-room kernel agents and phase workers</DESC> -->
<!-- <VERS>VERSION: 0.16.0</VERS> -->
<!-- <WCTX>New kernel Phase H1: add canonical recipe document and vocabulary guidance.</WCTX> -->
<!-- <CLOG>0.16.0: MINOR — add Phase H1 canonical recipe document, vocabulary, and recipe-evidence guidance.
0.15.0: MINOR — add Phase H0 source/asset contract rules and contextual recipe-example read list.
0.14.0: MINOR — replace role-suppressed gpt-5.5 dispatch guidance with role-routed OMX profile guidance.
0.13.0: MINOR — add Phase G4 node I/O, graph value bus, and spatial field guidance.
0.12.0: MINOR — add Phase G3 topology, parallel snapshot, and channel-aware merge guidance.
0.11.0: MINOR — add Phase G2 canonical graph execution proof guidance.
0.10.0: MINOR — add Phase G1 canonical graph container guidance.
0.9.0: MINOR — add Phase F2 declarative value source and parameter binding guidance.
0.8.0: MINOR — add Phase F1 typed input/value contract guidance and F2 deferral.
0.7.0: MINOR — add EffectDescriptor capability model, schema root, and Phase F input deferral.
0.6.0: MINOR — add tui-vfx-contract ownership, schema paths, and E0 phase guidance.
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
    - `docs/VOCABULARY.md`
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

Checked stable contract schema roots live under `schemas/v3.1/contract/`: surface, scope, write, diagnostic, scene, element, outcome, effect-descriptor, value, effect-input, value-source, parameter, signal, binding, graph, graph-step, node, graph value/output, source, source descriptor, source input/output, asset, asset requirement, and asset ref. Proof-pipeline roots remain under `schemas/v3.1/next/`: sampler and pipeline.

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
- Keep E1 descriptors capability-only; do not implement broad input/value/schema/runtime systems before the architect approves that phase.

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
- `DimEffect` and `ExplicitRoleWriteEffect` are proof artifacts; the old tiny proof `EffectDescriptor` was retired when E1 added the durable contract descriptor.
- D3 must not implement descriptors, recipe schema/compiler, source authoring schemas, template expansion, runtime bindings, phase graph, trigger engine, studio manifest, legacy migration, real effect ports, full layer graph, or complex blending.

Key docs:

- `docs/new_kernel/ARCH-RESP-TO-PHASE_D2.md`
- `docs/v3.1-contract-boundary.md`
- `docs/v3.1-surface-contract.md`
- `docs/new_kernel/PHASE_D3_STATUS.md`
- `docs/new_kernel/PHASE_D3_STATUS_MEMO_TO_ARCHITECT.md`
- `docs/new_kernel/ARCH-RESP-TO-PHASE_D3.md`


### Phase E0 — physical contract split

Locked.

Target lock:

- `crates/tui-vfx-contract` exists and owns stable v3.1 DTOs.
- `crates/tui-vfx-next` depends on `tui-vfx-contract` and keeps proof execution.
- `tui-vfx-contract` has no dependency on `tui-vfx-next` or legacy compositor/style/content/shadow crates.
- Stable schemas generate from `tui-vfx-contract` under `schemas/v3.1/contract/`.
- Proof-pipeline schemas remain under `schemas/v3.1/next/`.
- `PipelineStage` remains proof-only and is not promoted to descriptor model.
- E0 implements no effect descriptors, recipes, runtime, studio, migration, or real effects.

Key docs:

- `docs/new_kernel/ARCH-RESP-TO-PHASE_D3.md`
- `docs/v3.1-contract-boundary.md`
- `docs/new_kernel/PHASE_E0_STATUS.md`
- `docs/new_kernel/ARCH-RESP-TO-PHASE_E0.md`


### Phase E1 — minimal effect descriptor model

Completed phase.

Target lock:

- `EffectDescriptor` lives in `tui-vfx-contract` as a durable schema-backed contract root.
- Descriptor DTOs declare identity, `EffectDomain`, `CellAccess`, `ScopeSupport`, `WriteSupport`, and `EffectLifecycle`.
- `EffectDomain` initial vocabulary is `contentGenerator`, `contentTransform`, `cellShader`, `frameFilter`, `coordinateSampler`, `mask`, `shadow`, `postProcess`, and `diagnosticTooling`.
- Descriptor validation accepts supported scope/write/channel requests and rejects unsupported ones with `DescriptorValidationError`.
- `schemas/v3.1/contract/effect-descriptor.schema.json` is checked and rustdoc-described.
- `PipelineStage`, `SurfacePipeline`, `PipelineSampler`, `SurfaceEngine`, and proof effects remain proof-only in `tui-vfx-next`.
- Phase F1 now owns the first typed input contract. Do not backfill `ValueSource`, parameters, signals, bindings, recipe nodes, registries, runtime graphs, studio controls, migration, or real effect ports into E1-era descriptor capability work.

Key docs:

- `docs/new_kernel/ARCH-RESP-TO-PHASE_E0.md`
- `docs/v3.1-contract-boundary.md`
- `docs/v3.1-surface-contract.md`
- `docs/new_kernel/PHASE_E1_STATUS.md` once created.


### Phase F1 — typed Value / EffectInputSpec model

Completed phase.

Target lock:

- `ValueKind` is a closed schema-backed vocabulary: `null`, `boolean`, `integer`, `number`, `string`, `text`, `color`, `duration`, `enum`, `role`, `scope`, and `rect`.
- `Value` is a tagged typed literal, not raw JSON.
- `ValueSpec` declares the expected kind, optional typed default, optional numeric range, enum allowed values, and documentation-only `unit` / `semantic` strings.
- `EffectInputSpec` declares documentation-only `displayName` / `description`, `value`, `bindable`, and `runtimeMutability`.
- `EffectDescriptor.inputs` is a descriptor-local map keyed by `EffectInputId`.
- Descriptor validation validates input ids, default kind compatibility, numeric ranges, enum allowed values, and non-finite numeric values.
- Checked schemas include `value.schema.json`, `effect-input.schema.json`, and the updated `effect-descriptor.schema.json`.

Hard deferrals for F2 and later:

- Do not add `ValueSource`, `ParameterSpec`, `SignalSpec`, `BindingSpec`, refs, expression languages, presets, runtime override precedence, recipe schema/compiler, template expansion, studio controls, phase graph, trigger engine, migration, real effect ports, or legacy aliases.

Key docs:

- `docs/new_kernel/ARCH-RESP-TO-PHASE_E1.md`
- `docs/v3.1-contract-boundary.md`
- `docs/v3.1-surface-contract.md`
- `docs/new_kernel/PHASE_F1_STATUS.md` once created.


### Phase F2 — declarative ValueSource / ParameterSpec / SignalSpec / BindingSpec

Completed phase.

Target lock:

- `ValueSource` represents literal, parameter, signal, and simple numeric map sources.
- `ParameterId` / `ParameterSpec` define public recipe controls separately from effect inputs.
- `SignalId` / `SignalSpec` define host/runtime-provided values separately from parameters.
- `BindingSpec` is declarative and parameter-target only; direct node/effect-input bindings remain deferred beyond G1.
- `BindingMode` is replace-only for F2.
- Validation resolves parameter/signal references, checks source/target kind compatibility, validates fallbacks, and rejects non-numeric map sources.
- Checked schemas include `value-source.schema.json`, `parameter.schema.json`, `signal.schema.json`, and `binding.schema.json`.

Hard deferrals after F2:

- Do not add runtime `ParameterStore` / `SignalStore`, live override execution, preset/profile persistence, direct node/effect-input bindings, recipe compiler/schema, studio controls/manifest, expression language, phase graph, trigger engine, migration, real effect ports, or legacy aliases.

Key docs:

- `docs/new_kernel/ARCH-RESP-TO-PHASE_F1.md`
- `docs/v3.1-contract-boundary.md`
- `docs/v3.1-surface-contract.md`
- `docs/new_kernel/PHASE_F2_STATUS.md` once created.

### Phase G1 — canonical node graph container

Completed phase.

Target lock:

- `GraphId` and `NodeId` are stable, schema-constrained identifiers.
- `NodeSpec` references an `EffectId`, supplies descriptor-local inputs as `BTreeMap<EffectInputId, ValueSource>`, and may request existing `ScopeSpec`, `CellWritePolicy`, and `RoleWritePolicy` vocabulary.
- `GraphSpec` contains parameters, signals, F2 parameter-target bindings, declared effect descriptors, nodes, and deterministic node order.
- Graph validation proves effect ids exist, node inputs are declared by descriptors, `ValueSource` kinds match `EffectInputSpec.value.kind`, parameter/signal refs are known, map sources remain numeric-only, requested scope/write policies are supported by descriptors, bindings validate through F2 rules, and order references exactly known nodes without duplicates.
- Checked schemas include `graph.schema.json` and `node.schema.json`.

Hard deferrals after G1:

- Do not add runtime graph execution, runtime `ParameterStore` / `SignalStore`, live override execution, direct node/effect-input bindings, source recipe authoring schema, recipe compiler implementation, template expansion, studio controls/manifest, expression language, phase graph, trigger engine, migration, real effect ports, or legacy aliases.

Key docs:

- `docs/new_kernel/ARCH-RESP-TO-PHASE_F2.md`
- `docs/v3.1-contract-boundary.md`
- `docs/v3.1-surface-contract.md`
- `docs/new_kernel/PHASE_G1_STATUS.md` once created.

### Phase G2 — canonical graph execution proof

Completed phase.

Target lock:

- `tui-vfx-next` consumes `GraphSpec` from `tui-vfx-contract` and runs `GraphSpec::validate()` before execution.
- `GraphExecutor` executes nodes in `GraphSpec.order` over semantic `Surface` values using proof-only adapters.
- `GraphExecutionContext` is a one-shot value snapshot with parameter and signal values; it is not a runtime store.
- `ValueSource` resolution covers literals, parameter snapshot/default values, signal snapshot/fallback/default values, and numeric maps.
- Later nodes see earlier node cell and role writes.
- Node scope and write-policy semantics reuse the existing surface engine.
- F2 `BindingSpec` entries remain validation-only in G2 and are not applied.

Hard deferrals after G2:

- Do not add source recipe authoring schema, canonical recipe compiler, runtime `ParameterStore` / `SignalStore`, live override precedence, direct node/effect-input binding targets, phase graph, trigger engine, studio controls/manifest, migration, real effect ports, or legacy aliases.

Key docs:

- `docs/new_kernel/ARCH-RESP-TO-PHASE_G1.md`
- `docs/v3.1-architecture-overview.md`
- `docs/v3.1-contract-boundary.md`
- `docs/new_kernel/PHASE_G2_STATUS.md` once created.

### Phase G3 — topology / parallel snapshot / channel-aware merge semantics

Completed phase.

Locked:

- `GraphStep` is the stable topology DTO for node, sequence, and parallel graph execution.
- `GraphSpec.topology` is optional; when absent, `GraphSpec.order` remains the linear fallback.
- Topology validation rejects unknown node references, duplicate node references, and topologies that do not cover declared nodes.
- Sequence children execute in order and later children see earlier writes.
- Parallel children all read the same pre-parallel surface snapshot and do not see sibling branch writes before join.
- Parallel branches produce proof deltas that record written cell channels.
- Channel-aware merge composes different-channel writes and resolves same-channel conflicts by explicit `ParallelMergePolicy`.

### Phase G4 — node I/O / graph-local value bus

Completed phase.

Target lock:

- `EffectDescriptor.outputs` declares descriptor-local effect outputs keyed by `EffectOutputId`.
- `NodeSpec.outputs` publishes graph-local values keyed by `GraphValueId`.
- `NodeOutputSource` supports `effectOutput` and input re-emission.
- `ValueSource::GraphValue` is allowed for node inputs and rejected in binding/parameter/signal contexts.
- `GraphValueKind` and `GraphValueShape` type outputs and distinguish frame-wide `frameValue` from per-cell `cellField`.
- Spatial scalar fields such as normalized-x must remain cell fields and must not be collapsed to one global number.
- Sequence execution updates the value bus after each node; later sequence nodes can consume prior outputs, and one output can fan out to multiple consumers.
- Parallel branches receive the same value-bus snapshot, cannot see sibling outputs before join, and merge branch outputs after join.
- `GraphValueMergePolicy` makes same-output parallel conflicts deterministic via child-order LWW or explicit error.
- Proof execution uses toy `proof.*` adapters only; do not port real effects.

Hard deferrals after G4:

- Do not add source recipe schema/compiler, runtime `ParameterStore` / `SignalStore`, F2 binding execution, live override precedence, direct node/effect-input binding targets, phase graph, trigger/dwell/visibility engines, loopback/demo signal execution, asset/procedural source system, studio controls/manifest, migration, real effect ports, or legacy aliases.

Key docs:

- `docs/new_kernel/ARCH-RESP-TO-PHASE_G3.md`
- `docs/v3.1-architecture-overview.md`
- `docs/v3.1-contract-boundary.md`
- `docs/new_kernel/PHASE_G4_STATUS.md` once created.

### Phase H0 — source / asset / procedural source contract

Completed phase.

Target lock:

- `SourceDescriptor` describes a surface producer, not an effect over an existing surface.
- `SourceSpec` instantiates a source descriptor with typed inputs and structural asset refs.
- `SourceInputSpec` reuses `ValueSpec`, `Value`, `ValueKind`, and `ValueSource`; do not fork the value model.
- `AssetSpec`, `AssetRequirement`, and `AssetRef` make assets explicit and structural. Canonical asset refs are ids, not string interpolation tokens such as `{{ flag_art }}`.
- `SourceOutputSpec` declares produced-surface size behavior and role behavior (`Explicit`, `DefaultRole`, or `Generated`).
- Source kinds account for text, card, procedural, image, ANSI, command-capture, asset-backed, scene-layer, and custom needs without adopting legacy recipe field names.
- Validation rejects unknown sources, unknown source inputs, kind mismatches, missing required source inputs/assets, unknown asset refs, wrong asset kind/format, graph values outside graph context, and interpolated asset locators.
- Source-local pipelines are future integration points after a source-produced surface exists; H0 does not add recipe syntax for them.

Context examples requested by the owner:

- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/scene/scene_authoring_ladder_flag_asset_binding.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/scene/scene_braille_flag_asset_token.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/scene/scene_braille_flag_runtime_wave.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/scene/scene_authoring_ladder_procedural_spinner_binding.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/scene/scene_layer_full_stack.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/scene/scene_layer_io_filter_shader.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/scene/scene_layer_visibility_binding_io.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/content/content_split_flap_solari_authentic.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/baseline.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/scene/ansi_source_chain.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/scene/scene_image_source_bindable.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/complex/command_capture_chain.json`

These recipes are not gospel. Use them to check source/asset needs only; do not adopt current recipe field names as canonical v3.1.

Hard deferrals during H0:

- Do not add canonical recipe document schema, source lowering/compiler, source-local pipeline syntax, real asset loading/resolution, real procedural rendering, runtime stores, phase/trigger/dwell engines, studio manifests, migration, loopback/demo execution, or real source/effect ports.

Key docs:

- `docs/new_kernel/ARCH-RESP-TO-PHASE_G4.md`
- `docs/v3.1-architecture-overview.md`
- `docs/v3.1-contract-boundary.md`
- `docs/new_kernel/PHASE_H0_STATUS.md` once created.

### Phase H1 — canonical recipe document schema

Current phase.

Target lock:

- `RecipeDocument` is the strict canonical post-authoring/lowering root. It packages metadata, assets, source descriptors, source instances, graph, and scenes.
- `RecipeMetadata` carries human-facing title/description/authors/tags only; it does not become a studio manifest or demo profile.
- `SourceInstanceId` distinguishes recipe-local source instances from stable `SourceId` descriptors.
- `RecipeScene` declares dimensions and source-backed `RecipeSceneElement` values. H1 scene elements reference source-produced surfaces by source instance id rather than embedding legacy layer/source fields.
- `RecipeElementPipeline` is a future integration seam: it references the canonical graph/topology for element-local pipelines but does not execute runtime source-local pipeline syntax.
- `docs/VOCABULARY.md` is now a standing contract artifact. Every phase that adds or changes public vocabulary must update it.
- `docs/new_kernel/H1_RECIPE_EVIDENCE_NOTES.md` records mapping pressure from curated recipes as evidence only. Do not let old field names define canonical v3.1.

Non-canonical legacy evidence examples:

- `config.pipeline.step`
- `io.outputs[].hint`
- `requires_assets`
- `{{ flag_art }}` interpolation tokens
- `scene.layers[]` as a root concept

Hard deferrals during H1:

- Do not implement template expansion, legacy migration/lowering, runtime `ParameterStore` / `SignalStore`, F2 binding execution, direct node/effect-input binding targets, phase/trigger/dwell engines, studio manifests, demo/player execution, real asset loading, real procedural rendering, real effect/source ports, or visual parity.

Key docs:

- `docs/new_kernel/ARCH-RESP-TO-PHASE_H0.md`
- `docs/VOCABULARY.md`
- `docs/new_kernel/H1_RECIPE_EVIDENCE_NOTES.md`
- `docs/v3.1-architecture-overview.md`
- `docs/v3.1-contract-boundary.md`
- `docs/new_kernel/PHASE_H1_STATUS.md` once created.

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

When dispatching subagents, choose the role that matches the task shape (for
example `executor`, `test-engineer`, `architect`, `writer`, or `verifier`) and
rely on current OMX profiles for the correct gpt-5.5 lane. Do not leave roles
unset merely to preserve a model choice; instead, keep exact write scopes and
concrete tests to prevent architectural freelancing.

## Reporting expectations

Final reports should be concise and evidence-dense:

- What changed.
- Files changed.
- Tests/commands run and pass/fail results.
- OFPF/deslop notes.
- Remaining risks/open questions.

Do not claim completion without fresh verification evidence and architect/reviewer approval when Ralph is active.

<!-- <FILE>docs/new_kernel/AGENT_BRIEFING.md</FILE> - <DESC>Reusable briefing for clean-room kernel agents and phase workers</DESC> -->
<!-- <VERS>END OF VERSION: 0.16.0</VERS> -->
