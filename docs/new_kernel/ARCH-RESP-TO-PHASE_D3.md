<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_D3.md</FILE> - <DESC>Architect response approving Phase D3 and assigning Phase E0</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase E0 intake: preserve architect recommendation for physical contract split.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture Phase D3 approval and Phase E0 physical split assignment.</CLOG> -->

I would treat **D3 as the earliest safe point**, and I would split **at the start of Phase E**, before the effect descriptor model becomes real.

D3 created the logical boundary:

```text
tui_vfx_next::contract
tui_vfx_next::proof
tui_vfx_next::schema_roots
```

That means the project has done the thinking required to split safely. The next phase, **Effect Descriptor Model**, will add durable public schema roots. That is exactly when we should stop putting stable DTOs and proof-engine scaffolding in the same crate.

# Recommendation

Do a small **Phase E0 — Physical Contract Split** before Phase E proper.

```text
+====================================================================================+
|                         PHASE E0 — PHYSICAL CONTRACT SPLIT                           |
+====================================================================================+

  BEFORE

      crates/tui-vfx-next
          contract vocabulary
          proof engine
          toy stages
          schema roots
          tests

  AFTER

      crates/tui-vfx-contract
          stable v3.1 contract DTOs
          Serde/Schemars schemas
          rustdoc-backed reference types
          surface / scene / scope / write / diagnostic vocabulary

      crates/tui-vfx-next
          clean-room proof engine
          toy stages
          semantic tests
          depends on tui-vfx-contract

+====================================================================================+
```

Then Phase E adds `EffectDescriptor` to `tui-vfx-contract`, not to the incubator crate.

---

# Why split now, not earlier?

Before D3, the split would have been premature because we had not classified:

```text
public contract vocabulary
proof implementation
test scaffolding
schema roots
```

Splitting before that would have turned uncertainty into crate boundaries.

D3 answered those questions. Now the split is mechanical rather than speculative.

---

# Why split before descriptors?

Effect descriptors will become one of the most important public v3.1 contracts. They will be consumed later by:

```text
recipe compiler
runtime graph compiler
studio manifest generation
schema/reference docs
legacy migration tools
demo player
possibly external apps
```

If we add descriptors to `tui-vfx-next`, then later split them out, we create avoidable churn:

```text
schema paths move
imports change
docs change
tests change
downstream crates change
```

Better to put the first durable descriptor work directly into the contract crate.

---

# Split trigger rule

Use this rule going forward:

```text
Split the crate when a type is both:
    1. stable enough to be referenced by future phases, and
    2. useful without the proof engine.
```

The current surface/scene/scope/write/diagnostic vocabulary now meets that bar.

---

# What goes into `tui-vfx-contract`

Move or re-export the stable contract-facing types:

```text
Surface
SurfaceMetadata
Scene
SceneElement
SceneOutcome
ElementId
LayerId
ElementPlacement
ClipPolicy

ScopeSpec
CoordinateSpace
RoleSpace
ScopeEvalInput, if it is contract-facing

CellWrite / CellWritePolicy
RoleWritePolicy

PipelineSampler or renamed sampler contract type
SurfaceDiagnostic
SurfaceDiagnosticCode

schema root helpers, if present
```

Also include the schema/reference machinery:

```text
Serialize
Deserialize
JsonSchema
rustdoc comments
checked generated schema tests/helpers
```

The contract crate should depend on only foundational crates:

```text
tui-vfx-types
serde
schemars
```

Maybe later:

```text
semver
```

but only once descriptors require version ranges.

It should **not** depend on:

```text
tui-vfx-next
tui-vfx-compositor
tui-vfx-style
tui-vfx-content
tui-vfx-shadow
```

---

# What stays in `tui-vfx-next`

Keep proof/execution material in the incubator:

```text
SurfaceEngine
SurfacePipeline
PipelineStage toy enum
PipelineOutcome if considered proof-only
DimEffect toy proof
ReplaceGlyph proof helper
ExplicitRoleWrite proof helper
apply/copy helper functions
semantic proof tests
```

After the split:

```text
tui-vfx-next
    depends on tui-vfx-contract
```

That proves the contract crate is sufficient to drive an engine.

---

# What to do with `SurfacePipeline` and `PipelineStage`

Based on D3:

```text
SurfacePipeline:
    checked proof-pipeline root, not final runtime graph

PipelineStage:
    toy proof enum, not descriptor model
```

So I would **not** move `PipelineStage` into `tui-vfx-contract`.

For `SurfacePipeline`, there are two options:

```text
Option A:
    leave it in tui-vfx-next as proof-only

Option B:
    move only the minimal pipeline DTO if it is intentionally a checked schema root
```

My preference:

```text
Leave SurfacePipeline and PipelineStage in tui-vfx-next for now.
```

Then Phase E descriptor work can define the real public “effect node / stage descriptor” vocabulary without being constrained by the toy proof pipeline.

If the existing checked `pipeline.schema.json` is explicitly labeled as proof-pipeline artifact, keep it under a proof schema path or retire it before public v3.1 release.

---

# Schema path recommendation

Right now schemas live under:

```text
schemas/v3.1/next/
```

At split time, I would move stable contract schemas to:

```text
schemas/v3.1/contract/
```

For example:

```text
schemas/v3.1/contract/surface.schema.json
schemas/v3.1/contract/scope.schema.json
schemas/v3.1/contract/write.schema.json
schemas/v3.1/contract/scene.schema.json
schemas/v3.1/contract/element.schema.json
schemas/v3.1/contract/diagnostic.schema.json
```

Keep proof-only schemas, if any, separate:

```text
schemas/v3.1/proof/
```

or leave them in `next` with clear docs. But before public release, I would avoid shipping proof schemas as if they were stable runtime contracts.

---

# Phase E0 definition of done

```text
+====================================================================================+
|                         PHASE E0 — DEFINITION OF DONE                                |
+====================================================================================+

  CRATES
      [DONE] crates/tui-vfx-contract exists.
      [DONE] crates/tui-vfx-next depends on tui-vfx-contract.
      [DONE] tui-vfx-contract does not depend on tui-vfx-next.
      [DONE] tui-vfx-contract has no compositor/style/content/shadow dependency.

  TYPES
      [DONE] Stable contract DTOs moved or cleanly re-exported from tui-vfx-contract.
      [DONE] Proof-only toy types remain in tui-vfx-next.
      [DONE] PipelineStage is not promoted to descriptor model.

  SCHEMAS
      [DONE] Checked schema roots generate from tui-vfx-contract.
      [DONE] Schema fixtures are current.
      [DONE] Schema-reference rustdoc requirements still pass.
      [DONE] Proof-only schemas are clearly labeled or not exported.

  TESTS
      [DONE] Existing A/B/C/D1 semantic tests still pass.
      [DONE] Schema generation tests still pass.
      [DONE] Workspace tests pass.
      [DONE] Dependency guardrails pass.

  DOCS
      [DONE] docs/v3.1-contract-boundary.md updated.
      [DONE] architecture overview updated.
      [DONE] agent briefing updated.
      [DONE] status memo records the split.

+====================================================================================+
```

---

# When not to split

Do **not** split if the next phase is only another docs-only design phase.

But if the next phase is implementation-facing and adds durable public contracts, especially descriptors, split first.

So the practical rule is:

```text
Docs-only D2-style phase:
    no split required

Next Rust/schema phase adding durable public DTOs:
    split first
```

Phase E is exactly that kind of phase.

---

# Suggested next sequence

```text
[DONE]   D3 — logical contract/proof boundary

[NEXT]   E0 — physical split:
          tui-vfx-contract + tui-vfx-next

[AFTER]  E1 — minimal effect descriptor model:
          EffectDescriptor
          EffectDomain
          CellChannel
          InputSpec
          ScopeSupport
          WriteSupport
          Lifecycle stub

[AFTER]  E2 — descriptor validation against existing contract vocabulary
```

This keeps Phase E from becoming too large.

---

# Copy-paste Phase E0 prompt

```text
You are working in the tui-vfx Rust workspace.

Phases A–D3 built the clean-room v3.1 foundation in `tui-vfx-next`.
D3 created a logical boundary between contract vocabulary, proof implementation, schema roots, and test-only scaffolding.

Your task is Phase E0: Physical Contract Split.

Goal:
Create a dedicated `tui-vfx-contract` crate for stable v3.1 public contract DTOs before Phase E adds the effect descriptor model.

Hard constraints:
- Do not implement effect descriptors yet.
- Do not implement recipe schema/compiler.
- Do not implement template expansion.
- Do not add runtime bindings, phase graph, trigger engine, studio manifest, or legacy migration.
- Do not port real effects.
- Do not replace or refactor the legacy compositor.
- Do not add legacy aliases.
- Preserve v3.1 naming.
- Preserve D0 schema/reference rules.
- Keep proof-only toy engine types out of the contract crate.

Create:
    crates/tui-vfx-contract

Move or re-home stable contract-facing types into `tui-vfx-contract`, including:
- Surface and metadata
- Scene / SceneElement / SceneOutcome
- ElementId / LayerId / ElementPlacement / ClipPolicy
- ScopeSpec / CoordinateSpace / RoleSpace
- write policies
- sampler contract type if it is stable contract vocabulary
- SurfaceDiagnostic / SurfaceDiagnosticCode

Keep in `tui-vfx-next`:
- SurfaceEngine
- proof pipeline execution
- PipelineStage toy enum
- toy proof stages/effects
- low-level proof helpers
- semantic proof tests

Dependency direction:
    tui-vfx-types
        ↓
    tui-vfx-contract
        ↓
    tui-vfx-next

`tui-vfx-contract` must not depend on:
- tui-vfx-next
- tui-vfx-compositor
- tui-vfx-style
- tui-vfx-content
- tui-vfx-shadow

Schema requirements:
- Existing stable contract schemas should generate from `tui-vfx-contract`.
- Move checked stable schemas to `schemas/v3.1/contract/` if practical.
- If any proof-only schemas remain, clearly document them as proof artifacts.
- All schema fixtures must be current.
- Rustdoc-backed schema description tests must still pass.

Docs to update:
- docs/v3.1-contract-boundary.md
- docs/v3.1-architecture-overview.md
- docs/v3.1-surface-contract.md if imports/ownership are mentioned
- docs/v3.1-feature-contract-checklist.md
- docs/new_kernel/AGENT_BRIEFING.md
- docs/new_kernel/INDEX.md
- docs/INDEX.md if applicable

Verification:
Run:
    cargo fmt --package tui-vfx-contract -- --check
    cargo fmt --package tui-vfx-next -- --check
    cargo clippy -p tui-vfx-contract --all-targets -- -D warnings
    cargo clippy -p tui-vfx-next --all-targets -- -D warnings
    cargo test -p tui-vfx-contract
    cargo test -p tui-vfx-next
    cargo test --workspace
    cargo tree -p tui-vfx-contract
    cargo tree -p tui-vfx-next
    grep -R -nE 'tui_vfx_(compositor|style|content|shadow)|tui-vfx-(compositor|style|content|shadow)' crates/tui-vfx-contract crates/tui-vfx-next

Deliverables:
- new `tui-vfx-contract` crate
- updated `tui-vfx-next` imports
- updated schemas
- updated docs
- `docs/new_kernel/PHASE_E0_STATUS.md`
- `docs/new_kernel/PHASE_E0_STATUS_MEMO_TO_ARCHITECT.md`

Definition of done:
Phase E0 is done when stable contract vocabulary physically lives in `tui-vfx-contract`, proof execution remains in `tui-vfx-next`, schema generation still works, all A/B/C/D1 semantic tests still pass, and no descriptor/recipe/runtime/effect-port work has leaked into the split.
```

---

# Direct answer

Split the crate **now, as Phase E0**, immediately before starting the effect descriptor model.

The reason is simple:

```text
D3 made the boundary logical.
Phase E will make descriptors durable.
Physical split belongs between those two moments.
```

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_D3.md</FILE> - <DESC>Architect response approving Phase D3 and assigning Phase E0</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
