# D0 architect verdict

**Approved.**

Phase D0 did exactly what we needed before the model grows: it turned the clean-room kernel from “tested semantics” into “tested semantics with generated contract artifacts.” That is a big quality lock.

The new standing rule is now:

```text
No undocumented public contract type.
No schema-visible field without rustdoc.
No public wire shape without Serde + JsonSchema + checked generated schema output.
```

That rule should be included in every future phase prompt.

---

# What D0 locks

```text
+====================================================================================+
|                                  PHASE D0 LOCKS                                      |
+====================================================================================+

  SCHEMA SOURCE OF TRUTH
      [LOCK] Rust contract types own the schema shape.
      [LOCK] Serde owns the JSON wire representation.
      [LOCK] Schemars generates JSON Schema.
      [LOCK] Rustdoc comments feed human-facing schema/reference text.

  GENERATED ARTIFACTS
      [LOCK] Checked schemas live under schemas/v3.1/next/.
      [LOCK] Schema generation is deterministic.
      [LOCK] Stale checked schemas are caught by tests.
      [LOCK] Schema-visible object shapes are strict.

  DOCUMENTATION HYGIENE
      [LOCK] Public contract roots require rustdoc descriptions.
      [LOCK] Non-kind payload properties require descriptions.
      [LOCK] Architecture overview and philosophy docs now exist.

  DEPENDENCY BOUNDARY
      [LOCK] tui-vfx-next remains independent of compositor/style/content/shadow crates.
      [LOCK] Foundation Schemars support was added narrowly, not as a global ConfigSchema replacement.

+====================================================================================+
```

---

# Answers to D0’s open questions

## 1. Should the next phase generalize `ScopeSpec` / write policies or begin descriptors?

I would **not begin descriptors yet**.

The next semantic gap is **multiple elements / scene composition**. We already discussed that real recipes will often contain many elements at once: cards, text blocks, buttons, shadows, overlays, cursors, particles, backdrops, and notifications.

Effect descriptors need to know what they can target:

```text
surface
element
layer
scene
role within element
role across scene
```

So scene/element/layer semantics should be locked before descriptors.

Recommended next phase:

```text
Phase D1 — Scene / Element / Layer Composition Semantics
```

## 2. Should schema export remain test-driven or become `xtask schema --check` now?

Keep the schema export **test-driven for D1** unless adding `xtask` is tiny.

The current tests already prove determinism and fixture freshness. I would add an `xtask schema --check` once we have either:

```text
scene schemas
effect descriptor schemas
recipe schemas
```

or once schema files become too numerous for test-only management.

## 3. Which proof-only types should become public contract types?

For now:

```text
True public contract types:
    Surface
    ScopeSpec
    CellWrite
    RoleWritePolicy
    PipelineSampler
    SurfacePipeline
    SurfaceDiagnostic

Still proof-only:
    tiny EffectDescriptor DTO
    toy Dim / ReplaceGlyph / ExplicitRoleWrite stages
```

Do not expand `EffectDescriptor` until scene/element semantics are known.

## 4. Should the crate split happen now?

Not yet.

Keep:

```text
tui-vfx-next
```

as the incubator through D1.

After D1, decide whether to split:

```text
tui-vfx-contract
tui-vfx-engine
```

The split becomes more useful once we can separate:

```text
contract DTOs:
    surface, scope, write, sampler, pipeline, scene

engine proof:
    execution helpers, toy stages, test pipeline runners
```

---

# Recommended next phase

```text
+====================================================================================+
|                         PHASE D1 — SCENE / ELEMENT / LAYER SEMANTICS                 |
+====================================================================================+
```

## Why D1 is next

Phases A–C proved a single-surface execution model:

```text
surface
    -> sampled-source semantics
    -> ordered stages
```

Real v3.1 authoring needs a scene model:

```text
scene
    -> elements
    -> placements
    -> z/layer ordering
    -> overlap rules
    -> final composed surface
```

Without D1, descriptors and recipes will have to guess whether an effect targets:

```text
one surface
one element
one layer
the whole scene
roles inside one element
roles across all elements
```

That ambiguity is too important to leave until recipe work.

---

# D1 high-level block diagram

```text
+==================================================================================================+
|                              PHASE D1 — SCENE COMPOSITION                                         |
+==================================================================================================+

        +------------------------------+
        | Scene                        |
        |                              |
        |  width / height              |
        |  elements[]                  |
        |  metadata                    |
        +---------------+--------------+
                        |
                        v
        +------------------------------+
        | Ordered Elements             |
        |                              |
        |  sort by zIndex              |
        |  tie-break by declaration    |
        |  each has local surface      |
        +---------------+--------------+
                        |
                        v
        +------------------------------+
        | Element                      |
        |                              |
        |  elementId                   |
        |  optional layerId            |
        |  placement in scene coords   |
        |  local surface               |
        |  clip policy                 |
        |  write policy                |
        +---------------+--------------+
                        |
                        v
        +------------------------------+
        | Compose Element Into Current |
        |                              |
        |  local coord -> scene coord  |
        |  skip preserves current      |
        |  write updates cell + role   |
        +---------------+--------------+
                        |
                        v
        +------------------------------+
        | Final Composed Surface       |
        |                              |
        |  cells                       |
        |  roles                       |
        |  diagnostics                 |
        +------------------------------+

+==================================================================================================+
```

---

# What D1 should lock

```text
+====================================================================================+
|                                  PHASE D1 LOCK TARGETS                              |
+====================================================================================+

  SCENE
      [LOCK] A scene has a final destination size.
      [LOCK] A scene contains ordered elements.
      [LOCK] Scene composition produces one final Surface.

  ELEMENT
      [LOCK] Element identity is distinct from RoleTag.
      [LOCK] Element has a local Surface.
      [LOCK] Element has placement in scene coordinates.
      [LOCK] Element-local coordinates are distinct from scene-global coordinates.

  LAYER / ORDER
      [LOCK] zIndex determines composition order.
      [LOCK] Declaration order is deterministic tie-break.
      [LOCK] Optional LayerId may group elements, but Phase D1 does not need a full layer graph.

  OVERLAP
      [LOCK] Later/higher elements can overwrite earlier/lower elements.
      [LOCK] Skipped cells preserve the current composed surface.
      [LOCK] Transparent empty writes remain writes unless skipped by policy.

  ROLES
      [LOCK] Element identity is not role identity.
      [LOCK] Written cells carry roles according to existing role write policy.
      [LOCK] A top element writing text can overwrite a lower element's role.
      [LOCK] A skipped top element preserves the lower element's cell and role.

  DIAGNOSTICS
      [LOCK] Diagnostics identify scene element.
      [LOCK] Diagnostic order follows deterministic composition order.

  SCHEMA / REFERENCE
      [LOCK] New public scene/element types follow D0 schema-reference rules.

+====================================================================================+
```

---

# D1 should avoid

D1 should **not** build:

```text
effect descriptors
recipe schema
studio manifest
runtime bindings
phase graph
trigger engine
legacy migration
real effect ports
template inheritance implementation
full layer graph
complex blending engine
```

Keep it a semantic proof.

---

# Minimal D1 type shape

The agent does not have to use exactly these names, but this is the intended scale.

```rust
/// Scene composed from one or more placed semantic elements.
pub struct Scene {
    /// Width of the final composed scene in cells.
    pub width: u16,

    /// Height of the final composed scene in cells.
    pub height: u16,

    /// Elements composed into the scene.
    pub elements: Vec<SceneElement>,

    /// Optional scene-level metadata.
    pub metadata: SceneMetadata,
}
```

```rust
/// One placed semantic surface inside a scene.
pub struct SceneElement {
    /// Stable element identity used by diagnostics and future recipe references.
    pub id: ElementId,

    /// Optional layer identity for grouping and future layer tooling.
    pub layer: Option<LayerId>,

    /// Z order. Higher values compose later and appear above lower values.
    pub z_index: i32,

    /// Placement of the element's local surface in scene coordinates.
    pub placement: ElementPlacement,

    /// Element-local semantic surface.
    pub surface: Surface,

    /// Policy for cells outside the final scene bounds.
    pub clip_policy: ClipPolicy,

    /// Policy for empty transparent element cells.
    pub cell_write_policy: CellWritePolicy,
}
```

```rust
/// Stable element identifier.
pub struct ElementId(String);
```

```rust
/// Placement of an element-local surface into scene coordinates.
pub struct ElementPlacement {
    /// Scene x coordinate where local x=0 is placed.
    pub x: i32,

    /// Scene y coordinate where local y=0 is placed.
    pub y: i32,
}
```

```rust
/// How composition handles element cells outside the scene bounds.
pub enum ClipPolicy {
    /// Ignore out-of-bounds element cells and preserve the current scene surface.
    Clip,

    /// Emit a diagnostic when any element cell lands outside the scene.
    Warn,

    /// Treat out-of-bounds placement as invalid.
    Error,
}
```

For D1, `Clip` is enough to implement. `Warn` / `Error` can be contract placeholders only if they are documented and schema-ready.

---

# D1 required tests

D1 should add tests like these:

```text
scene_composes_multiple_elements
    Two non-overlapping elements appear in final surface.

element_identity_is_distinct_from_role
    Element id is "titleCard" while cells inside may have RoleTag::Text.

higher_z_element_overwrites_lower_cell_and_role
    Lower element writes glyph A / role Background.
    Higher element writes glyph B / role Text.
    Final output has glyph B / role Text.

z_tie_breaks_by_declaration_order
    Two elements same zIndex overlap.
    Later declaration wins deterministically.

skipped_top_element_preserves_lower_output
    Lower element writes cell + role.
    Higher element has empty transparent cell with SkipTransparentEmpty.
    Final output preserves lower cell + role.

transparent_empty_top_write_can_clear_when_policy_writes
    Higher element writes empty transparent cell with WriteCell.
    Final output reflects empty transparent write, proving write != skip.

element_placement_uses_scene_coordinates
    Element local (0,0) lands at placement x/y.

out_of_bounds_element_cells_are_clipped
    Partially offscreen element composes visible cells only.
    Out-of-bounds cells do not panic and do not mutate.

scene_diagnostics_include_element_identity
    Diagnostic path or structured field identifies the element id.

scene_schema_generation_is_current
    New scene/element schemas generate deterministically and include descriptions.
```

---

# Schema files to add in D1

Add checked schema roots if corresponding public root types are added:

```text
schemas/v3.1/next/scene.schema.json
schemas/v3.1/next/element.schema.json
```

Possibly:

```text
schemas/v3.1/next/layer.schema.json
```

only if `LayerSpec` becomes a real root. If D1 only uses `LayerId` as an optional field, do not invent a layer root yet.

---

# Copy-paste prompt for Phase D1

```text
You are working in the tui-vfx Rust workspace.

Phases A, B, C, and D0 built and schema-backed the clean-room crate:

    crates/tui-vfx-next

Current locks:
- semantic surface contract
- sampled-source semantics
- ordered pipeline/pass semantics
- rustdoc-backed Serde/Schemars schema generation

Your task is Phase D1: scene / element / layer composition semantics.

Goal:
Prove that the clean-room kernel can compose multiple semantic elements into one final semantic surface while preserving the existing surface, role, write, skip, and diagnostic rules.

Primary question:
Can v3.1 represent multiple elements at once with deterministic placement, ordering, overlap, role propagation, and diagnostics?

Hard constraints:
- Do not replace or refactor the legacy compositor.
- Do not port real effects such as CRT, typewriter, matrix rain, or shadow.
- Do not add effect descriptor expansion.
- Do not add recipe compiler, studio manifest, runtime bindings, phase graph, trigger engine, or legacy migration.
- Do not add legacy aliases.
- Do not depend on `tui-vfx-compositor`, `tui-vfx-style`, `tui-vfx-content`, or `tui-vfx-shadow`.
- Keep the phase semantic, small, and test-focused.
- Use v3.1 naming consistently.
- Follow the D0 schema-reference rule for every public contract-visible type.

D0 schema-reference rule:
Every public v3.1 contract-visible type added or modified in this phase must:
- derive or intentionally implement Serialize, Deserialize, and JsonSchema where JSON-facing;
- use strict Serde shape where public;
- include rustdoc comments on public types, fields, and variants;
- update generated schema fixtures or schema-generation tests;
- avoid schema-visible fields without descriptions.

Recommended implementation:
Add minimal scene composition types to `tui-vfx-next`, such as:

- Scene
- SceneElement
- ElementId
- ElementPlacement
- ClipPolicy
- SceneOutcome or equivalent

The scene should compose ordered elements into a final `Surface`.

Recommended semantics:
- A scene has width and height.
- Each element owns a local `Surface`.
- Each element has placement in scene coordinates.
- Each element has zIndex.
- Elements compose in ascending zIndex.
- Elements with equal zIndex compose in declaration order.
- Later-composed written cells overwrite earlier-composed cells.
- Skipped cells preserve the current composed surface.
- Empty transparent writes are still writes unless the element's cell write policy says to skip them.
- Element identity is distinct from RoleTag.
- Diagnostics identify the element that caused them.

Optional:
- Add optional LayerId to SceneElement if it is easy and does not grow the phase.
- Do not build a full layer graph.

Required docs:
Update:
- docs/v3.1-architecture-overview.md
- docs/v3.1-surface-contract.md
- docs/v3.1-feature-contract-checklist.md
- docs/new_kernel/AGENT_BRIEFING.md if standing process rules need update

Add or update schema roots:
- schemas/v3.1/next/scene.schema.json
- schemas/v3.1/next/element.schema.json if useful as a root

Required tests:
- scene_composes_multiple_elements
- element_identity_is_distinct_from_role
- higher_z_element_overwrites_lower_cell_and_role
- z_tie_breaks_by_declaration_order
- skipped_top_element_preserves_lower_output
- transparent_empty_top_write_can_clear_when_policy_writes
- element_placement_uses_scene_coordinates
- out_of_bounds_element_cells_are_clipped
- scene_diagnostics_include_element_identity
- scene_schema_generation_is_current

Verification:
Run:
    cargo fmt --package tui-vfx-next -- --check
    cargo clippy -p tui-vfx-next --all-targets -- -D warnings
    cargo test -p tui-vfx-next
    cargo test --workspace
    cargo tree -p tui-vfx-next
    grep -R -nE 'tui_vfx_(compositor|style|content|shadow)|tui-vfx-(compositor|style|content|shadow)' crates/tui-vfx-next

Deliverables:
- Code changes in tui-vfx-next
- Updated generated schemas
- Updated docs
- Status memo summarizing:
  - what was added
  - what was deliberately not added
  - tests and verification results
  - open questions

Definition of done:
Phase D1 is done when the clean-room kernel can compose multiple placed semantic elements into one final surface, overlap behavior is deterministic, skipped top elements preserve lower/current output, element identity is distinct from role identity, diagnostics are element-aware, generated schemas are current and described, and the crate remains independent of legacy compositor/style/content/shadow crates.
```

---

# Where template inheritance fits

Do **not** implement template inheritance in D1.

But keep it visible in the roadmap. I would add:

```text
Phase D2 — Template Composition Design
```

as a **design document phase**, not an implementation phase.

Reason:

```text
Template inheritance belongs above the surface/scene engine.
It should expand into canonical strict v3.1 recipes before validation and runtime.
```

D1 gives templates something concrete to talk about:

```text
templates can define scenes
templates can define elements
templates can define layer/default policies
templates can expose slots
```

But the engine should not know whether an element came from a template.

---

# Updated roadmap

```text
+====================================================================================+
| CURRENT ROADMAP POSITION                                                            |
+====================================================================================+

  [DONE]   Phase A  — Semantic surface contract
  [DONE]   Phase B  — Sampled-source semantics
  [DONE]   Phase C  — Ordered pipeline/pass semantics
  [DONE]   Phase D0 — Schema/reference backfill

  [NEXT]   Phase D1 — Scene / element / layer composition semantics

  [NEXT+]  Phase D2 — Template composition design document
             compile-time expansion rules, no runtime implementation yet

  [LATER]  Phase E  — Effect descriptor model
  [LATER]  Phase F  — Value / parameter / signal model
  [LATER]  Phase G  — Node graph
  [LATER]  Phase H  — Strict recipe v3.1 schema/compiler
  [LATER]  Phase H2 — Template expansion implementation
  [LATER]  Phase I  — Phase/trigger engine
  [LATER]  Phase J  — First real effect ports

+====================================================================================+
```

---

# Final recommendation

Proceed with **Phase D1: Scene / Element / Layer Composition Semantics**.

That is the right next lock because it resolves the multi-element question before descriptors and recipes hard-code assumptions about what an effect targets.
