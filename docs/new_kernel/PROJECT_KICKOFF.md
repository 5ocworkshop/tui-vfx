<!-- <FILE>docs/new_kernel/PROJECT_KICKOFF.md</FILE> - <DESC>Clean-room kernel project kickoff and Phase A prompt</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Original kickoff guidance for bounded clean-room kernel implementation.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add OFPF metadata around captured clean-room kernel planning/status content.</CLOG> -->

Yes. Turn it into a **bounded implementation prompt** with a clear phase boundary:

```text
Phase A:
    Define the v3.1 semantic surface contract
    Build a minimal clean-room kernel beside the existing engine
    Prove the contract with tests

Not Phase A:
    Port all effects
    Replace the current compositor
    Build recipes/studio/runtime/phase graph
    Preserve legacy aliases
```

Below is a copy-pasteable prompt/spec you can give to another LLM or coding agent.

---

# Copy-paste prompt for the next LLM

```text
You are working in the tui-vfx Rust workspace.

Goal:
Build a clean-room v3.1 surface-contract spike beside the existing engine. The purpose is to define and prove the canonical render surface semantics before porting effects, recipes, or runtime workflow.

This is NOT a rewrite of the whole project. This is a bounded Phase A spike.

Context:
The existing system has:
- `tui-vfx-types::Cell`
- `tui-vfx-types::Color`
- `tui-vfx-types::OwnedGrid`
- `tui-vfx-types::RoleMap`
- `tui-vfx-types::RoleTag`
- `tui-vfx-types::SemanticScene`
- a compositor pipeline with samplers, masks, shader layers, filters, and shadow
- shader-specific `StyleRegion` scope behavior
- partial role-aware behavior, especially for shaders and shadow cells

The new v3.1 direction should treat the existing system as inspiration and an implementation oracle, not as the architecture to preserve.

Primary objective:
Create a small clean-room kernel that proves these surface-contract semantics:

1. The canonical render surface is a dense rectangular semantic surface:
   - cell grid
   - role grid
   - metadata

2. Each cell has:
   - glyph char
   - foreground RGBA color
   - background RGBA color
   - terminal modifiers
   - optional modifier alpha

3. Each cell has exactly one primary semantic role.

4. Built-in roles are:
   - background
   - text
   - title
   - caption
   - border
   - image
   - icon
   - indicator
   - highlight
   - shadow
   - decoration
   - procedural

5. Visual-only effects preserve semantic roles.

6. Copying or transforming a sampled source cell preserves the sampled source role by default.

7. Skipped cells preserve the existing destination cell and destination role.

8. Shadow-only/procedural contribution can explicitly write a role.

9. A scope that matches zero cells emits a diagnostic and skips the effect.

10. Scope evaluation must explicitly distinguish coordinate space and role space.

Hard constraints:
- Do not replace or refactor the existing compositor pipeline in this phase.
- Do not port all existing effects.
- Do not introduce legacy aliases into the new contract.
- Do not make the new clean-room kernel depend on `tui-vfx-compositor`, `tui-vfx-style`, `tui-vfx-content`, or `tui-vfx-shadow`.
- It may depend on `tui-vfx-types` and `tui-vfx-geometry`.
- Keep this phase small enough to review.
- Prefer tests over broad feature coverage.

Recommended implementation location:
Create a new experimental crate:

    crates/tui-vfx-next

or, if the workspace prefers long-term naming immediately:

    crates/tui-vfx-contract
    crates/tui-vfx-engine

For this phase, one crate is acceptable. Suggested module layout:

    crates/tui-vfx-next/src/lib.rs
    crates/tui-vfx-next/src/surface.rs
    crates/tui-vfx-next/src/scope.rs
    crates/tui-vfx-next/src/write.rs
    crates/tui-vfx-next/src/effect.rs
    crates/tui-vfx-next/src/engine.rs
    crates/tui-vfx-next/src/diagnostic.rs
    crates/tui-vfx-next/tests/surface_contract.rs

Documentation deliverable:
Create:

    docs/v3.1-surface-contract.md

The document must define:
- canonical surface
- cell contract
- role contract
- built-in roles
- custom role policy, even if custom roles are not implemented yet
- scope contract
- coordinate-space and role-space semantics
- write policy
- skip behavior
- zero-cell scope behavior
- shadow/procedural role-write behavior
- explicit non-goals for this phase

Minimal types to introduce:
- `Surface`
- `SurfaceMetadata`
- `ScopeSpec`
- `CoordinateSpace`
- `RoleSpace`
- `ScopeEvalInput`
- `RoleWritePolicy`
- `CellWritePolicy` or equivalent write-policy type
- `SurfaceDiagnostic`
- `EffectDescriptor`
- `EffectDomain`
- `CellChannel`
- one tiny visual-only effect, such as `DimEffect` or `TintEffect`
- one tiny write/procedural helper or effect that can write `RoleTag::Shadow` or another explicit role

The initial `ScopeSpec` should be deliberately small:
- `All`
- `Role(RoleTag)`
- `Rect(Rect)`
- `RowRange { start, end }`
- `ColumnRange { start, end }`

Boolean scope composition can be deferred unless it is trivial.

Default scope-space semantics:
- geometry scopes use destination-local coordinates
- role scopes use sampled-source roles

Make these defaults explicit in code and docs.

Minimum tests:
Create tests proving:

1. `copy_preserves_sampled_source_roles`
   - source cells with `text` role copied to destination retain `text`.

2. `visual_effect_preserves_roles`
   - applying a dim/tint effect changes style but leaves roles unchanged.

3. `role_scope_affects_only_matching_roles`
   - role scope `text` affects text cells but not border/background cells.

4. `skipped_cells_preserve_destination_cell_and_role`
   - cells outside scope or skipped by policy do not clear destination content or role.

5. `zero_cell_scope_emits_diagnostic`
   - a role scope that matches no cells emits a structured diagnostic and does not mutate the destination.

6. `explicit_role_write_sets_role`
   - a procedural/shadow-style write can set destination role to `shadow`.

7. `empty_transparent_cell_is_not_the_same_as_skip`
   - writing an empty transparent cell is distinguishable from skipping a cell, or the policy explicitly preserves destination when configured to skip empty writes.

8. `scope_role_space_defaults_to_sampled_source`
   - a test should make the intended default visible, even if the initial sampler implementation is only a simple identity or fake sampled coordinate.

Acceptance constraints:
- Existing crates should continue compiling.
- The new crate should have no dependency on old implementation crates.
- All new tests must pass.
- New docs must describe what is intentionally not implemented.
- No broad migration or effect-porting work should be attempted.

Suggested command checks:
    cargo test -p tui-vfx-next

If the crate is named differently, run the equivalent package test.

Optional if inexpensive:
    cargo test --workspace

Output expected from the implementation agent:
- Summary of what was added.
- List of new files.
- Tests added and run.
- Any explicit design decisions made.
- Any open questions left for the next phase.
```

---

# Phase A specification

The prompt above is the task. The actual **specification** behind it is this:

```text
Phase A exists to answer one question:

Can the project define and prove a clean semantic surface model independent of the old compositor?
```

The phase is successful when the answer is yes, even with only one or two toy effects.

It is not successful because it renders something impressive. It is successful because it proves the rules that every impressive effect will later obey.

---

# Definition of done

## Documentation done

`docs/v3.1-surface-contract.md` exists and defines:

```text
canonical surface
cell channels
empty cell
transparent cell
skipped cell
unfilled cell
role model
built-in roles
custom role policy
scope model
coordinate spaces
role spaces
write policies
zero-cell scope behavior
diagnostic behavior
phase non-goals
```

It must explicitly say that Phase A does **not** include:

```text
full recipe schema
studio manifest
phase engine
trigger engine
all effect descriptors
legacy recipe migration
full effect port
replacement compositor
```

## Code done

A new experimental crate exists, preferably:

```text
crates/tui-vfx-next
```

or the long-term split:

```text
crates/tui-vfx-contract
crates/tui-vfx-engine
```

For this phase, one crate is fine.

The new crate contains a minimal clean-room surface kernel with:

```text
Surface
ScopeSpec
scope evaluator
write policy
role propagation helper
structured diagnostics
one simple visual effect
one explicit-role-write/procedural example
```

It should depend only on foundational crates such as:

```text
tui-vfx-types
tui-vfx-geometry
```

It should not depend on:

```text
tui-vfx-compositor
tui-vfx-style
tui-vfx-content
tui-vfx-shadow
```

## Test done

The new crate has tests proving:

```text
copy preserves sampled source roles
visual effects preserve roles
role scope only affects matching roles
skipped cells preserve destination cell and role
zero-cell scope emits diagnostic
explicit role write works
empty transparent write is distinct from skip
role-space semantics are explicit
```

The minimum required command passes:

```bash
cargo test -p tui-vfx-next
```

or the equivalent package name.

## Architecture done

The new system has a clear dependency direction:

```text
types / geometry
    ↓
v3.1 surface contract
    ↓
clean-room kernel
```

The phase fails if the new contract crate starts importing the old compositor, style, content, or shadow implementation crates.

## Scope-control done

The implementation does **not** attempt to:

```text
port CRT
port typewriter
port matrix rain
port all masks
replace render_pipeline
rewrite FilterSpec
rewrite StyleRegion
build the recipe compiler
build the studio
support all legacy aliases
```

A tiny dim/tint effect is enough.

---

# Success criteria

This phase is successful when a reviewer can answer these questions confidently:

```text
What is the canonical v3.1 surface?
What is a cell?
What is a role?
What happens when an effect skips a cell?
What happens when an effect writes an empty transparent cell?
What role does a copied cell get?
What role does a visual-only effect preserve?
What role does a shadow/procedural write use?
How are scopes evaluated?
What happens when a scope matches zero cells?
Can this model be tested without the legacy compositor?
```

The phase is **not** judged by visual richness. It is judged by semantic clarity.

---

# Recommended `docs/v3.1-surface-contract.md` starter text

You can hand this directly to the LLM as the expected document shape.

```markdown
# v3.1 Surface Contract

## Status

Draft. This document defines the canonical render surface for the clean-room v3.1 kernel.

## Goals

- Define a stable semantic surface model.
- Make effect read/write behavior explicit.
- Preserve semantic roles through visual-only effects.
- Distinguish skipped, empty, transparent, and unfilled cells.
- Provide one scope model that can later apply across shaders, filters, masks, samplers, content effects, and shadows.
- Support deterministic tests and diagnostics.

## Non-goals

This phase does not define the full recipe schema, studio manifest, phase engine, trigger engine, effect registry, legacy migration path, or full compositor replacement.

## Canonical Surface

A v3.1 render surface is a dense rectangular semantic surface.

A surface contains:

- a cell grid
- a role grid
- surface metadata

The cell grid and role grid must have identical dimensions.

## Cell Contract

A cell contains:

- glyph
- foreground color
- background color
- terminal modifiers
- optional modifier alpha

## Empty Cell

A cell is empty when:

- glyph is a space
- foreground alpha is zero
- background alpha is zero

An empty cell is not automatically the same thing as a skipped write.

## Role Contract

Each cell has exactly one primary role.

Built-in roles:

- background
- text
- title
- caption
- border
- image
- icon
- indicator
- highlight
- shadow
- decoration
- procedural

Visual-only effects preserve roles.

Copied or transformed source cells preserve their sampled source role by default.

Procedural or shadow-style writes may explicitly write a role.

## Custom Roles

Strict v3.1 recipes should declare custom roles before using them. Legacy migration may infer custom roles, but strict v3.1 validation should reject undeclared custom role names.

## Scope Contract

Scopes select cells for an effect.

Initial scope kinds:

- all
- role
- rect
- rowRange
- columnRange

Future scope kinds may include:

- cell
- cells
- modulo
- and
- or
- not

## Coordinate Spaces

Geometry scopes default to destination-local coordinates.

Role scopes default to sampled-source roles.

The contract must distinguish:

- destination-local coordinates
- sampled-source coordinates
- global destination coordinates

## Write Contract

Effects do not write arbitrary semantics implicitly. Every write follows a role write policy:

- preserveDestination
- preserveSampledSource
- writeRole
- clearToBackground

## Skip Contract

A skipped cell preserves both the existing destination cell and the existing destination role.

## Zero-Cell Scope Contract

If a scope matches zero cells, the engine emits a diagnostic and skips the effect by default.

## Shadow / Procedural Contract

Shadow-only contributions write role `shadow`.

Procedural generators write a declared output role.

## Diagnostics

Diagnostics are structured. They should include:

- level
- code
- message
- optional node/effect/scope identifier
- optional hint
```

---

# Review checklist for the completed phase

Use this after the LLM returns changes.

```text
Does the new crate avoid importing old implementation crates?
Does the doc define the surface in plain terms?
Does every test correspond to a semantic rule?
Can a visual-only effect mutate style without losing roles?
Can a skipped cell preserve destination role?
Can zero-cell scopes be diagnosed?
Is role propagation explicit rather than accidental?
Is empty-cell behavior distinct from skipped-cell behavior?
Are coordinate-space and role-space semantics named?
Did the agent avoid porting a bunch of effects prematurely?
```

---

# Red flags during implementation

Stop and correct course if the LLM starts doing any of these:

```text
Refactoring existing render_pipeline before the surface tests exist.
Moving StyleRegion wholesale before defining ScopeSpec.
Porting many effects.
Adding legacy aliases to the clean contract.
Making roles optional everywhere.
Treating Grid alone as the canonical surface.
Writing only cells and forgetting role propagation.
Letting unknown custom roles silently pass strict validation.
Making zero-cell scopes silent no-ops.
Depending on tui-vfx-compositor from the clean contract crate.
```

---

# The one-sentence phase definition

```text
Phase A is done when a new clean-room surface kernel proves, with tests, that v3.1 can preserve semantic roles, evaluate scopes, apply visual writes, distinguish skipped/empty cells, and emit diagnostics without depending on the legacy compositor.
```

That is the foundation the rest of the contract system can safely build on.

<!-- <FILE>docs/new_kernel/PROJECT_KICKOFF.md</FILE> - <DESC>Clean-room kernel project kickoff and Phase A prompt</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
