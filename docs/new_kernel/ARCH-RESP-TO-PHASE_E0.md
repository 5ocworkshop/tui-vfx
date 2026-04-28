<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_E0.md</FILE> - <DESC>Architect response approving Phase E0 and assigning Phase E1</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase E1 intake: preserve architect recommendation for minimal effect descriptor model.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture Phase E0 approval and Phase E1 descriptor assignment.</CLOG> -->

# E0 architect verdict

**Approved.**

This split happened at the right moment. D3 created the logical boundary; E0 made it physical before effect descriptors become durable public schema. That keeps descriptor work from landing in the incubator crate and having to migrate later.

```text
+====================================================================================+
|                                  PHASE E0 LOCKS                                      |
+====================================================================================+

  CRATE OWNERSHIP
      [LOCK] Stable v3.1 contract DTOs live in tui-vfx-contract.
      [LOCK] Proof execution remains in tui-vfx-next.
      [LOCK] tui-vfx-next proves the contract crate instead of owning the contract.
      [LOCK] Dependency direction is:
                 tui-vfx-types
                     ↓
                 tui-vfx-contract
                     ↓
                 tui-vfx-next

  SCHEMA OWNERSHIP
      [LOCK] Stable schemas live under schemas/v3.1/contract/.
      [LOCK] Proof-pipeline schemas remain under schemas/v3.1/next/.
      [LOCK] Proof artifacts are not presented as final runtime graph contracts.

  BOUNDARY
      [LOCK] PipelineStage remains proof-only.
      [LOCK] SurfaceEngine / SurfacePipeline remain proof execution machinery.
      [LOCK] Stable surface / scene / scope / write / diagnostic vocabulary is now reusable by future descriptor and recipe phases.

+====================================================================================+
```

---

# Answers to E0 open questions

## 1. Should E1 introduce only descriptor identity/domain/channel/scope/write support, or also input/value specs?

**E1 should introduce only the minimal descriptor capability model:**

```text
identity
domain
lifecycle stub
cell channel reads/writes
scope support
write-policy support
diagnostic/schema support
```

Do **not** add the full input/value model yet.

Reason: the input/value/source system is a larger design involving:

```text
ValueKind
Value
ValueSource
parameters
signals
bindings
defaults
ranges
runtime mutability
studio controls
```

That should be Phase F, not part of E1.

E1 can include a clear placeholder in the docs:

```text
Effect input contracts are intentionally deferred to Phase F.
```

But I would avoid adding an incomplete `InputSpec` that we know will churn.

## 2. Should `EffectDomain` be expanded in E1 or stay minimal?

Use the full initial domain vocabulary now, but keep semantics light.

Recommended initial enum:

```text
contentGenerator
contentTransform
cellShader
frameFilter
coordinateSampler
mask
shadow
postProcess
diagnosticTooling
```

Not every domain needs implementation in E1. The point is to lock the language descriptors will use.

## 3. Should descriptor schema roots live under `schemas/v3.1/contract/` or a subdirectory?

Start simple:

```text
schemas/v3.1/contract/effect-descriptor.schema.json
```

Do not create a descriptor subdirectory yet. If descriptor schemas multiply later, we can reorganize.

## 4. Should proof `PipelineSampler` be replaced by a contract-owned generalized sampler declaration in E1?

No.

Keep proof `PipelineSampler` where it is. E1 should only let descriptors declare:

```text
domain = coordinateSampler
```

and possibly cell/surface behavior like:

```text
reads no cell channels directly
writes no cell channels directly
affects sampled-source coordinate
```

A real sampler declaration DSL can come later when descriptor inputs and node graphs exist.

---

# Recommended next phase

```text
+====================================================================================+
|                         PHASE E1 — MINIMAL EFFECT DESCRIPTOR MODEL                   |
+====================================================================================+
```

E1’s purpose is to define what an effect **is allowed to do**, not how all inputs are configured.

The core question:

```text
Can a descriptor declare an effect’s domain, surface/cell access, scope support,
write-policy support, and lifecycle category using the stable contract vocabulary?
```

---

# E1 target model

```text
+==================================================================================================+
|                          PHASE E1 — EFFECT DESCRIPTOR MODEL                                       |
+==================================================================================================+

        +-------------------------------+
        | EffectDescriptor              |
        +---------------+---------------+
                        |
      +-----------------+------------------+------------------+
      |                                    |                  |
      v                                    v                  v
+-------------------+          +---------------------+   +-------------------+
| Identity           |          | Capability           |   | Lifecycle          |
|                    |          |                      |   |                    |
| id                 |          | domain               |   | completion         |
| version            |          | cell reads           |   | resettable         |
| display name       |          | cell writes          |   | seekable           |
| category           |          | scope support        |   | deterministic      |
+-------------------+          | write policy support |   +-------------------+
                               +----------------------+

  DEFERRED TO PHASE F
  ────────────────────────────────────────────────────────────────────────────────────────────────
      typed effect inputs
      value kinds
      defaults
      ranges
      runtime bindings
      parameters
      signals
      studio controls

+==================================================================================================+
```

---

# What E1 should lock

```text
+====================================================================================+
|                                  PHASE E1 LOCK TARGETS                              |
+====================================================================================+

  DESCRIPTOR IDENTITY
      [LOCK] EffectDescriptor exists in tui-vfx-contract.
      [LOCK] EffectId exists or is clearly represented.
      [LOCK] Effect version is represented.
      [LOCK] Display/category metadata exists but does not drive semantics.

  DOMAIN
      [LOCK] EffectDomain vocabulary exists.
      [LOCK] Domain communicates the effect’s broad execution role.

  CELL ACCESS
      [LOCK] CellChannel vocabulary exists:
             glyph, foreground, background, modifiers, modifierAlpha, role.
      [LOCK] Descriptor declares which channels an effect may read.
      [LOCK] Descriptor declares which channels an effect may write.

  SCOPE SUPPORT
      [LOCK] Descriptor declares supported ScopeSpec kinds.
      [LOCK] Descriptor declares supported CoordinateSpace and RoleSpace behavior if needed.
      [LOCK] Descriptor declares zero-cell scope policy or uses the default.

  WRITE SUPPORT
      [LOCK] Descriptor declares supported cell write policies.
      [LOCK] Descriptor declares supported role write policies.

  LIFECYCLE STUB
      [LOCK] Minimal completion vocabulary exists:
             never, instant, timeBound, eventual, external.
      [LOCK] resettable / seekable / deterministicWithSeed are represented if cheap.
      [LOCK] Full events are deferred unless trivial.

  VALIDATION
      [LOCK] Contract can validate that a requested scope/write policy is supported.
      [LOCK] Unsupported scope/write requests produce structured diagnostics or validation errors.

  SCHEMA
      [LOCK] effect-descriptor schema root exists.
      [LOCK] Rustdoc-backed schema descriptions exist.
      [LOCK] Checked schema fixture is current.

+====================================================================================+
```

---

# What E1 should avoid

```text
Do not add:
    full input specs
    ValueKind / ValueSource
    parameter model
    signal model
    recipe nodes
    node graph
    phase engine
    trigger engine
    studio manifest
    legacy migration
    real effect ports
    descriptor registry
```

A tiny static descriptor catalog for proof tests is okay, but not a full registry.

---

# Minimal E1 types

Suggested contract types:

```rust
EffectDescriptor
EffectId
EffectDomain
EffectLifecycle
EffectCompletion
CellChannel
CellAccess
ScopeSupport
ScopeKind
WriteSupport
DescriptorDiagnostic or reuse SurfaceDiagnostic if appropriate
```

I would not overload `SurfaceDiagnostic` too much. If descriptor validation starts producing its own errors, create descriptor-specific diagnostic codes or a generic `ContractDiagnostic`.

For E1, a simple validation result is fine.

---

# E1 example descriptors

These can be test descriptors, not real effect ports.

## Visual-only dim descriptor

```text
id:
    terminal.dim

domain:
    frameFilter

reads:
    foreground
    background

writes:
    foreground
    background

scope support:
    all
    role
    rect
    rowRange
    columnRange

role write support:
    preserveDestination

cell write support:
    writeCell
    skipTransparentEmpty

completion:
    instant or never, depending on stage interpretation
```

## Explicit role writer descriptor

```text
id:
    terminal.explicitRoleWrite

domain:
    contentGenerator or postProcess proof

reads:
    none or glyph/background depending on proof behavior

writes:
    glyph
    foreground
    background
    role

role write support:
    setExplicit

completion:
    instant
```

## Shift sampler descriptor

```text
id:
    terminal.shiftSampler

domain:
    coordinateSampler

reads:
    none directly

writes:
    none directly

sampling:
    changes sampled source coordinate

scope support:
    maybe all only for E1

completion:
    instant
```

Again, these are descriptor proofs, not real ports.

---

# Required E1 tests

```text
effect_descriptor_schema_is_current
    Generated effect descriptor schema matches checked fixture.

effect_descriptor_has_rustdoc_descriptions
    Schema has descriptions for key fields.

dim_descriptor_declares_visual_only_access
    Reads/writes fg/bg, does not write role.

role_writer_descriptor_declares_role_write
    Allows SetExplicit role write.

sampler_descriptor_declares_coordinate_sampler_domain
    Domain is coordinateSampler and does not claim cell writes.

descriptor_rejects_unsupported_scope_kind
    Request a scope kind not in ScopeSupport; validation fails.

descriptor_accepts_supported_role_scope
    Role scope accepted when descriptor supports Role.

descriptor_rejects_unsupported_role_write_policy
    Example: visual-only dim descriptor rejects SetExplicit role.

descriptor_rejects_writing_channel_not_declared
    Validation catches an attempted write outside declared cell channels.

descriptor_does_not_import_proof_pipeline_stage
    Compile/test-level check or docs assertion that PipelineStage is proof-only.
```

If the agent keeps validation smaller, at least test:

```text
supported scope accepted
unsupported scope rejected
supported role write accepted
unsupported role write rejected
schema current
```

---

# Copy-paste Phase E1 prompt

```text
You are working in the tui-vfx Rust workspace.

Phases A–E0 built the v3.1 clean-room foundation:
- A: semantic surface contract
- B: sampled-source semantics
- C: ordered pipeline/pass semantics
- D0: schema/reference backfill
- D1: scene / element / layer composition semantics
- D2: template composition design
- D3: contract/engine boundary
- E0: physical split into tui-vfx-contract and tui-vfx-next

Your task is Phase E1: Minimal Effect Descriptor Model.

Goal:
Add the first durable effect descriptor contract to `tui-vfx-contract`, using the stable D3 vocabulary. This phase defines what an effect is allowed to read, write, target, and support. It does not define the full input/value/parameter model.

Primary question:
Can an effect descriptor declare domain, cell channel access, scope support, write-policy support, and lifecycle category in a schema-backed Rust-owned contract?

Hard constraints:
- Add descriptor DTOs to `tui-vfx-contract`, not `tui-vfx-next`.
- Do not implement full effect inputs or ValueSource.
- Do not implement recipe schema/compiler.
- Do not implement template expansion.
- Do not add runtime bindings, phase graph, trigger engine, studio manifest, or legacy migration.
- Do not port real effects.
- Do not replace or refactor the legacy compositor.
- Do not add legacy aliases.
- Do not make proof `PipelineStage` the descriptor model.
- Preserve v3.1 naming.
- Preserve D0 schema/reference rules.

Required descriptor concepts:
- EffectDescriptor
- EffectId or equivalent stable id newtype
- EffectDomain
- CellChannel
- CellAccess
- ScopeSupport
- WriteSupport
- EffectLifecycle
- EffectCompletion

Recommended EffectDomain values:
- contentGenerator
- contentTransform
- cellShader
- frameFilter
- coordinateSampler
- mask
- shadow
- postProcess
- diagnosticTooling

Recommended CellChannel values:
- glyph
- foreground
- background
- modifiers
- modifierAlpha
- role

Inputs:
Do not build the full input/value model in E1.
Document that typed effect inputs, defaults, ranges, runtime mutability, parameters, signals, and bindings are deferred to Phase F.

Validation:
Add small descriptor validation helpers if useful:
- supported scope kind accepted
- unsupported scope kind rejected
- supported role write policy accepted
- unsupported role write policy rejected
- attempted write outside declared cell channels rejected

Schema:
Add checked schema root:
    schemas/v3.1/contract/effect-descriptor.schema.json

All public descriptor DTOs must:
- derive or intentionally implement Serialize, Deserialize, JsonSchema
- use strict Serde shape
- include rustdoc comments on public types, fields, and variants
- pass schema description tests

Docs to update:
- docs/v3.1-contract-boundary.md
- docs/v3.1-architecture-overview.md
- docs/v3.1-feature-contract-checklist.md
- docs/new_kernel/AGENT_BRIEFING.md
- docs/new_kernel/INDEX.md
- docs/INDEX.md if applicable

Tests:
Add tests covering:
- descriptor schema current
- descriptor schema has rustdoc descriptions
- visual-only descriptor does not write role
- explicit role writer descriptor supports SetExplicit role write
- coordinate sampler descriptor uses coordinateSampler domain
- supported scope accepted
- unsupported scope rejected
- supported write policy accepted
- unsupported write policy rejected

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
- Effect descriptor DTOs in tui-vfx-contract
- Checked effect descriptor schema
- Descriptor validation tests
- Updated docs
- docs/new_kernel/PHASE_E1_STATUS.md
- docs/new_kernel/PHASE_E1_STATUS_MEMO_TO_ARCHITECT.md

Definition of done:
Phase E1 is done when `tui-vfx-contract` contains a schema-backed minimal EffectDescriptor model that declares effect identity, domain, cell access, scope support, write support, and lifecycle category; descriptor validation can accept/reject supported scope/write behavior; checked schemas are current and described; and no input/value/recipe/runtime/effect-port work has leaked into the phase.
```

---

# Roadmap update

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
  [DONE]   Phase D3 — Contract/engine boundary + generalized scope/write vocabulary
  [DONE]   Phase E0 — Physical contract split

  [NEXT]   Phase E1 — Minimal effect descriptor model

  [LATER]  Phase F  — Value / input / parameter / signal model
  [LATER]  Phase G  — Node graph
  [LATER]  Phase H  — Strict canonical recipe v3.1 schema/compiler
  [LATER]  Phase H2 — Template expansion implementation
  [LATER]  Phase I  — Phase/trigger engine
  [LATER]  Phase J  — First real effect ports

+====================================================================================+
```

Proceed with **Phase E1**.

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_E0.md</FILE> - <DESC>Architect response approving Phase E0 and assigning Phase E1</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
