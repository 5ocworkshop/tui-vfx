<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_D2.md</FILE> - <DESC>Architect response approving Phase D2 and assigning Phase D3</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase D3 intake: preserve architect verdict and next-phase assignment.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture D2 approval and Phase D3 recommendation.</CLOG> -->

# D2 architect verdict

**Approved.**

Phase D2 did the right thing by keeping template composition above runtime. The key lock is:

```text
Templates, mixins, presets, and profiles are authoring inputs.
Canonical strict v3.1 recipes are compiler/runtime inputs.
Runtime never sees inheritance.
```

That preserves the clean-room philosophy and prevents hidden template ancestry from leaking into execution semantics.

---

# What D2 locks

```text
+====================================================================================+
|                                  PHASE D2 LOCKS                                      |
+====================================================================================+

  TEMPLATE BOUNDARY
      [LOCK] Templates are compile-time composition inputs.
      [LOCK] Runtime receives canonical expanded v3.1 recipes only.
      [LOCK] Runtime must not branch on template ancestry.

  VOCABULARY
      [LOCK] Template defines reusable structure and slots.
      [LOCK] Mixin / trait defines additive reusable structure fragments.
      [LOCK] Preset is values-only.
      [LOCK] Profile is values-only environment/product-mode configuration.
      [LOCK] Source recipe may use templates/mixins/presets/profiles.
      [LOCK] Canonical recipe contains no template references.

  EXPANSION
      [LOCK] Expansion is deterministic.
      [LOCK] Expansion happens before strict recipe validation and compilation.
      [LOCK] Expansion report is tooling/provenance output, not runtime input.

  MERGE / OVERRIDE
      [LOCK] Maps merge by key.
      [LOCK] Arrays require explicit operations.
      [LOCK] Scalar replacement requires explicit replacement paths.
      [LOCK] Accidental collisions are errors.
      [LOCK] Explicit overrides are required.
      [LOCK] Removal is deferred until a later explicit operation is designed.

  SAFETY
      [LOCK] Sealed/final fields are part of the design vocabulary.
      [LOCK] Sealed fields protect safety and semantic identity.

  PROVENANCE
      [LOCK] Diagnostics should be able to report both source and expanded paths.
      [LOCK] Canonical recipe and expansion report remain separate artifacts.

+====================================================================================+
```

---

# Answers to D2 open questions

## 1. Should D3 be contract/engine boundary plus generalized `ScopeSpec` / write model?

**Yes.**

D3 is the right next phase. We now have enough semantic material to stop treating `tui-vfx-next` as a pile of proof types and start classifying:

```text
What is public v3.1 contract?
What is clean-room engine proof?
What is test-only scaffolding?
What will descriptors eventually consume?
```

D3 should not build descriptors yet. It should make the boundary clean enough that descriptors can be added without dragging toy engine details into the public schema.

## 2. Should the next implementation-facing design start with source authoring schemas or canonical recipe schemas?

Neither yet. First define the **contract/engine boundary**.

After D3, I would start with **canonical recipe schemas**, not source authoring schemas.

Reason:

```text
canonical recipe schema:
    compiler/runtime contract

source authoring schema:
    template/mixin/preset convenience layer
```

The canonical form must be stable before the source authoring layer expands into it.

## 3. Which sealed fields should be default in the first implementation?

Do not implement sealed fields yet. But the first likely sealed fields are:

```text
effect id
node/effect domain
surface write policy for safety-critical template nodes
role write policy for safety-critical generated content
required scope constraints
phase transition safety rules
template-owned element ids, if exposed as slots
```

Implementation should wait until recipe/template schemas exist.

## 4. Should expansion reports be mandatory build artifacts or optional diagnostics?

For first implementation:

```text
Expansion report should always be produced by the compiler API,
but writing it to disk should be optional.
```

So:

```rust
compile_source_recipe(...) -> {
    canonical_recipe,
    expansion_report,
    diagnostics
}
```

CI/tooling can choose to persist the report later.

---

# Recommended next phase

```text
+====================================================================================+
|       PHASE D3 — CONTRACT / ENGINE BOUNDARY + GENERALIZED SCOPE / WRITE MODEL        |
+====================================================================================+
```

D3 is the bridge between semantic proofs and real descriptors.

The goal is to make the clean-room crate’s public contract surface intentional.

---

# Why D3 comes before descriptors

Effect descriptors will eventually declare things like:

```text
supported scopes
cell channels read
cell channels written
role write behavior
coordinate-space behavior
surface read/write behavior
domain
diagnostics
```

But before descriptors can be correct, we need to lock the shared vocabulary they refer to.

D3 should answer:

```text
Which current types are stable contract types?
Which are engine proof types?
Which are test-only?
What is the shared ScopeSpec vocabulary?
What is the shared write-policy vocabulary?
How do surface, pipeline, scene, and future nodes talk about coordinates and roles?
```

---

# D3 target architecture

```text
+==================================================================================================+
|                         PHASE D3 — CONTRACT / ENGINE BOUNDARY                                     |
+==================================================================================================+

        +--------------------------------+
        | tui-vfx-next                   |
        | current incubator              |
        +----------------+---------------+
                         |
             logical boundary, not necessarily crate split
                         |
        +----------------+----------------+
        |                                 |
        v                                 v
+-------------------------------+   +-------------------------------+
| Contract-facing types          |   | Engine/proof implementation    |
|                               |   |                               |
| Surface                       |   | SurfaceEngine                  |
| Scene                         |   | SurfacePipeline                |
| SceneElement                  |   | PipelineStage toy enum         |
| ScopeSpec                     |   | test helpers                   |
| Write policies                |   | toy copy/dim/replace stages    |
| Sampler contract              |   | execution loops                |
| Diagnostics                   |   |                               |
| Schema roots                  |   |                               |
+-------------------------------+   +-------------------------------+
        |
        v
+-------------------------------+
| Future descriptor model        |
| uses contract vocabulary       |
| does not expose toy internals   |
+-------------------------------+

+==================================================================================================+
```

I would keep the crate as `tui-vfx-next` for D3 and introduce a **logical/module boundary** first. Physical crates can split later.

---

# What D3 should lock

```text
+====================================================================================+
|                                  PHASE D3 LOCK TARGETS                              |
+====================================================================================+

  PUBLIC CONTRACT CLASSIFICATION
      [LOCK] Which current types are public v3.1 contract DTOs.
      [LOCK] Which current types are engine/proof internals.
      [LOCK] Which current types are test-only helpers.
      [LOCK] Which schema roots remain public.

  MODULE / OWNERSHIP BOUNDARY
      [LOCK] Contract-facing module boundary exists.
      [LOCK] Engine/proof module boundary exists.
      [LOCK] Toy stages are not mistaken for effect descriptor model.

  SCOPE MODEL
      [LOCK] Initial generalized ScopeSpec vocabulary.
      [LOCK] CoordinateSpace vocabulary.
      [LOCK] RoleSpace vocabulary.
      [LOCK] How ScopeSpec applies across surface, pipeline, and scene contexts.
      [LOCK] Zero-cell scope diagnostic behavior remains stable.

  WRITE MODEL
      [LOCK] Cell write policies.
      [LOCK] Role write policies.
      [LOCK] Empty transparent write vs skip.
      [LOCK] Scene overlap write behavior reuses the same policies.
      [LOCK] Future descriptors can reference these policies.

  DIAGNOSTICS
      [LOCK] Diagnostic path conventions for surface, pipeline stage, and scene element.
      [LOCK] Decide whether stage/element ids remain path strings or get structured fields later.
      [LOCK] Diagnostic codes remain stable.

  SCHEMA / REFERENCE
      [LOCK] D0 schema-reference rule remains mandatory.
      [LOCK] Existing schema roots still generate and remain current.
      [LOCK] Internal/proof-only types are either not schema roots or clearly documented as proof-only.

+====================================================================================+
```

---

# D3 should avoid

D3 should **not** implement:

```text
effect descriptor expansion
recipe schema/compiler
source recipe authoring schema
template expansion
runtime bindings
phase graph
trigger engine
studio manifest
legacy migration
real effect ports
full layer graph
complex blend modes
```

It is a boundary-cleanup and vocabulary-locking phase.

---

# D3 likely deliverables

Docs:

```text
docs/v3.1-contract-boundary.md
docs/v3.1-surface-contract.md              # update
docs/v3.1-architecture-overview.md         # update
docs/v3.1-feature-contract-checklist.md    # update
docs/new_kernel/AGENT_BRIEFING.md          # update
docs/new_kernel/PHASE_D3_STATUS.md
docs/new_kernel/PHASE_D3_STATUS_MEMO_TO_ARCHITECT.md
```

Code, if needed:

```text
crates/tui-vfx-next/src/contract/...        # optional logical module boundary
crates/tui-vfx-next/src/engine/...          # optional logical module boundary
```

But I would not require a large file move unless it is clean and low risk.

Schemas:

```text
schemas/v3.1/next/*.schema.json
```

updated only if public contract shapes change.

---

# D3 questions the agent must answer

```text
1. Which current `tui-vfx-next` types are true public contract types?

2. Which current types exist only to prove the engine?

3. Which current types should remain schema roots?

4. Is `SurfacePipeline` a public contract type or an engine proof type?

5. Is `PipelineStage` a public contract type or a toy proof enum?

6. Should `PipelineSampler` remain contract-facing or be renamed toward a generalized sampler contract?

7. Is `ScopeSpec` already the canonical generalized scope vocabulary?

8. Does `ScopeSpec` need explicit fields for coordinateSpace and roleSpace,
   or do operation-level defaults remain sufficient for now?

9. Do scene composition and pipeline execution use the same write policy vocabulary?

10. Are diagnostic paths sufficiently stable, or do we need structured fields
    such as elementId/stageName in addition to path?

11. Which proof-only types should be hidden from future schema/reference docs?

12. Should the physical crate split happen now?
    Recommended answer: no, unless the module boundary reveals it is cheap.
```

---

# D3 tests

D3 may be mostly boundary/docs/schema work, but if code changes happen, tests should prove:

```text
contract_schema_roots_remain_current
contract_types_have_descriptions
proof_only_types_are_not_new_schema_roots_unless_intentional
scope_defaults_still_match_phase_b
write_policy_still_matches_phase_a_d1
pipeline_and_scene_reuse_same_skip_write_semantics
diagnostic_paths_remain_stable
```

Existing A/B/C/D1 tests should continue passing.

---

# Copy-paste prompt for Phase D3

```text
You are working in the tui-vfx Rust workspace.

Phases A–D2 built the clean-room v3.1 foundation:
- Phase A: semantic surface contract
- Phase B: sampled-source semantics
- Phase C: ordered pipeline/pass semantics
- Phase D0: schema/reference backfill
- Phase D1: scene / element / layer composition semantics
- Phase D2: template composition design

Your task is Phase D3: Contract / engine boundary + generalized ScopeSpec / write model.

Goal:
Classify and cleanly document which `tui-vfx-next` types are stable v3.1 contract vocabulary, which are clean-room engine proof implementation, and which are test-only scaffolding. Lock the shared scope/write/diagnostic vocabulary that future effect descriptors and canonical recipe schemas will consume.

Hard constraints:
- Do not implement effect descriptor expansion.
- Do not implement recipe schema/compiler.
- Do not implement template expansion.
- Do not add source authoring schemas.
- Do not add studio manifest, runtime bindings, phase graph, trigger engine, or legacy migration.
- Do not port real effects.
- Do not replace or refactor the legacy compositor.
- Do not add legacy aliases.
- Do not depend on `tui-vfx-compositor`, `tui-vfx-style`, `tui-vfx-content`, or `tui-vfx-shadow`.
- Use v3.1 naming consistently.
- Preserve the D0 schema/reference rule for every public contract-visible type.

Primary questions:
1. What is public contract vocabulary?
2. What is engine/proof implementation?
3. What is test-only?
4. What exact scope and write policies should future descriptors reference?

Required design decisions:
- Identify stable public contract types.
- Identify proof-only engine types.
- Identify schema roots.
- Decide whether `SurfacePipeline` and `PipelineStage` are public contract types or proof-only.
- Decide whether `ScopeSpec` is the canonical generalized scope vocabulary.
- Decide whether current coordinate/role space defaults remain operation-level or should be represented directly in scope values.
- Confirm that pipeline and scene composition reuse the same write/skip semantics.
- Confirm diagnostic path conventions for surface, pipeline stage, and scene element.

Recommended docs:
Create:
    docs/v3.1-contract-boundary.md

Update:
    docs/v3.1-surface-contract.md
    docs/v3.1-architecture-overview.md
    docs/v3.1-feature-contract-checklist.md
    docs/new_kernel/AGENT_BRIEFING.md
    docs/new_kernel/INDEX.md
    docs/INDEX.md if applicable

If code changes are small and helpful:
- Add logical module grouping for contract-facing vs engine/proof code.
- Do not do a large physical crate split unless it is trivial and clearly justified.

Schema/reference requirements:
- Existing checked schemas must stay current.
- If public contract shapes change, update generated schemas.
- Proof-only types should not become schema roots unless intentionally documented.
- Every schema-visible public field must remain described.

Suggested tests/checks:
- Existing `cargo test -p tui-vfx-next`
- Existing schema generation tests
- Add tests only if code/schema behavior changes.
- Verify forbidden dependency boundary still holds.

Verification:
Run:
    cargo fmt --package tui-vfx-next -- --check
    cargo clippy -p tui-vfx-next --all-targets -- -D warnings
    cargo test -p tui-vfx-next
    cargo test --workspace
    cargo tree -p tui-vfx-next
    grep -R -nE 'tui_vfx_(compositor|style|content|shadow)|tui-vfx-(compositor|style|content|shadow)' crates/tui-vfx-next

Deliverables:
- `docs/v3.1-contract-boundary.md`
- Updated architecture/surface/checklist/briefing docs
- Updated schemas if needed
- Optional small code organization changes
- `docs/new_kernel/PHASE_D3_STATUS.md`
- `docs/new_kernel/PHASE_D3_STATUS_MEMO_TO_ARCHITECT.md`

Definition of done:
Phase D3 is done when the project has a durable contract/engine boundary document, public contract types and proof-only engine types are clearly classified, the generalized scope/write/diagnostic vocabulary is locked for descriptor work, schema/reference artifacts remain current, and no descriptor/recipe/runtime/effect-port work has leaked into the phase.
```

---

# After D3

After D3, I would proceed to:

```text
Phase E — Effect Descriptor Model
```

D3 is the last “boundary/vocabulary” phase before descriptors.

Once D3 is done, descriptors can safely reference:

```text
Surface
Scene
ScopeSpec
CoordinateSpace
RoleSpace
CellWritePolicy
RoleWritePolicy
PipelineSampler / sampler contract
SurfaceDiagnostic
cell channels
effect domains
```

without guessing which parts are stable and which were just proof scaffolding.

---

# Updated roadmap

```text
+====================================================================================+
| CURRENT ROADMAP                                                                     |
+====================================================================================+

  [DONE]   Phase A  — Semantic surface contract
  [DONE]   Phase B  — Sampled-source semantics
  [DONE]   Phase C  — Ordered pipeline/pass semantics
  [DONE]   Phase D0 — Schema/reference backfill
  [DONE]   Phase D1 — Scene / element / layer composition semantics
  [DONE]   Phase D2 — Template composition design

  [NEXT]   Phase D3 — Contract/engine boundary + generalized ScopeSpec/write model

  [NEXT+]  Phase E  — Effect descriptor model
  [LATER]  Phase F  — Value / parameter / signal model
  [LATER]  Phase G  — Node graph
  [LATER]  Phase H  — Strict canonical recipe v3.1 schema/compiler
  [LATER]  Phase H2 — Template expansion implementation
  [LATER]  Phase I  — Phase/trigger engine
  [LATER]  Phase J  — First real effect ports

+====================================================================================+
```

Proceed with **Phase D3**.

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_D2.md</FILE> - <DESC>Architect response approving Phase D2 and assigning Phase D3</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
