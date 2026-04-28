<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_C.md</FILE> - <DESC>Architect response to Phase C status memo</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Captured clean-room kernel Phase C architect approval and Phase D0 schema/reference recommendation.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add OFPF metadata around captured clean-room kernel planning/status content.</CLOG> -->

Yes. I would bake this in **now** and backfill Phases A–C before descriptor/schema work begins.

For v3.1 public contract types, I would use:

```text
Serde
    owns JSON wire shape

Schemars
    generates JSON Schema from Rust types

Rustdoc comments
    become human-facing schema descriptions/reference text

Generated schema/reference docs
    become the public contract artifact
```

Schemars is a good fit here because its `JsonSchema` derive is designed to generate schemas from Rust types, its `schema_for!` macro generates schema documents, and its generated schema is intended to match the type’s `serde_json` representation. It also generally respects Serde attributes such as renaming and unknown-field behavior, which is exactly what we want for strict v3.1 contract types. ([Docs.rs][1]) Serde remains the right serialization layer because it defines the Rust `Serialize` / `Deserialize` model and supports derived implementations for Rust structs/enums. ([Serde][2]) Schemars can also use Rust doc comments as schema descriptions, and a first Markdown heading can become a schema title, so disciplined rustdoc gives us generated schema reference material with much less duplicate writing. ([Graham’s Cool Site][3])

---

# Immediate architect decision

```text
+====================================================================================+
|                         v3.1 DOCUMENTATION / SCHEMA DECISION                         |
+====================================================================================+
|                                                                                      |
|  For all public v3.1 contract-visible Rust types:                                    |
|                                                                                      |
|      derive Serialize                                                               |
|      derive Deserialize                                                             |
|      derive JsonSchema                                                              |
|      use strict Serde shape                                                         |
|      write rustdoc comments for every public type, variant, and field               |
|      generate JSON Schema from Rust                                                 |
|      generate schema reference docs from generated schema + rustdoc text            |
|                                                                                      |
|  Rust types are the schema source of truth.                                          |
|  Hand-written schema/reference docs are explanatory, not authoritative.              |
|                                                                                      |
+====================================================================================+
```

I would **not** try to replace the project’s existing `ConfigSchema` system globally right now. For v3.1 clean-room contract work, use Serde + Schemars. Existing legacy/internal documentation systems can remain until there is a reason to migrate them.

---

# Philosophy statement

I would add this near the top of `docs/v3.1-surface-contract.md`, `docs/new_kernel/AGENT_BRIEFING.md`, and future schema docs:

```text
v3.1 is a contract-first rendering model.

The goal is not to preserve the old implementation shape. The goal is to make every public semantic rule explicit, testable, documented, and discoverable by tools.

A render effect should not hide what it reads, writes, targets, emits, or mutates. Recipes, studio controls, validation, diagnostics, and runtime behavior should all flow from the same Rust-owned contract model.

The legacy engine is an oracle and inventory. It is not the source of truth for v3.1 semantics.
```

Shorter version for prompts:

```text
Build the contract before porting effects. Every semantic fact needs one owner, one test, and one generated reference path.
```

---

# High-level progressive block diagram

This is the diagram I would maintain and extend after each phase.

```text
+==================================================================================================+
|                                 TUI-VFX v3.1 CLEAN-ROOM KERNEL                                    |
|                           Progressive contract stack, extended by phase                           |
+==================================================================================================+

  PHASE A — SURFACE
  ────────────────────────────────────────────────────────────────────────────────────────────────

        +-------------------------------+
        | Semantic Surface               |
        |                               |
        |  cells: dense rectangular grid |
        |  roles: one role per cell      |
        |  metadata                      |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Surface Writes                 |
        |                               |
        |  write cell                    |
        |  copy sampled-source role      |
        |  preserve destination role     |
        |  set explicit role             |
        |  skip transparent empty        |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Diagnostics                    |
        |                               |
        |  zero-cell scope               |
        |  size mismatch                 |
        +-------------------------------+


  PHASE B — SAMPLED SOURCE
  ────────────────────────────────────────────────────────────────────────────────────────────────

        +-------------------------------+
        | Destination coordinate         |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | CoordinateSampler              |
        |                               |
        |  identity                      |
        |  shift                         |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Sampled source coordinate      |
        | sampled source cell            |
        | sampled source role            |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Scope evaluation               |
        |                               |
        |  geometry: destination-local   |
        |  role: sampled-source          |
        +-------------------------------+


  PHASE C — ORDERED PIPELINE
  ────────────────────────────────────────────────────────────────────────────────────────────────

        +-------------------------------+
        | Current Surface                |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Stage N reads current          |
        | Stage N writes cloned next     |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Next becomes current           |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Later stages see prior writes  |
        | cells + roles + diagnostics    |
        +-------------------------------+


  PHASE D — SCHEMA / REFERENCE HYGIENE
  ────────────────────────────────────────────────────────────────────────────────────────────────

        +-------------------------------+
        | Rust contract types            |
        |                               |
        |  serde shape                   |
        |  rustdoc comments              |
        |  JsonSchema derive             |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Generated JSON Schemas         |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Generated Schema Reference     |
        |                               |
        |  type docs                     |
        |  field docs                    |
        |  examples                      |
        |  validation notes              |
        +-------------------------------+


  FUTURE — EFFECT DESCRIPTORS / VALUES / RECIPES
  ────────────────────────────────────────────────────────────────────────────────────────────────

        +-------------------------------+
        | Effect Descriptor              |
        | inputs, reads, writes, scope   |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | ValueSource / Parameters       |
        | literals, params, signals      |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Recipe v3.1 Compiler           |
        | strict schema, diagnostics     |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Runtime / Studio / Demos       |
        +-------------------------------+
```

I would keep this in a durable doc and extend it, not redraw from scratch every phase.

Suggested file:

```text
docs/v3.1-architecture-overview.md
```

---

# New requirement: schema-reference readiness

Add this as a standing requirement for every future phase.

```text
+====================================================================================+
|                         SCHEMA REFERENCE READINESS REQUIREMENT                       |
+====================================================================================+

  Every public v3.1 contract-visible type must be schema-reference ready.

  A type is schema-reference ready when:

      1. It derives or intentionally implements:
             Serialize
             Deserialize
             JsonSchema

      2. It has strict Serde shape:
             camelCase fields where JSON-facing
             deny_unknown_fields for closed objects
             explicit enum tagging strategy

      3. It has rustdoc comments on:
             the type
             every public field
             every enum variant
             every non-obvious policy/default

      4. Its docs answer:
             What is this?
             Who owns it?
             What are the defaults?
             What does it preserve?
             What does it mutate?
             What diagnostics can it cause?
             What phase locked it?

      5. It appears in generated schema/reference output or is explicitly marked internal.

+====================================================================================+
```

---

# What gets rustdoc comments?

For public contract-visible types:

```rust
/// # Surface
/// Dense rectangular semantic render surface used by the v3.1 clean-room kernel.
///
/// A surface owns visual cells and one semantic role per cell. The cell grid and
/// role grid must have identical dimensions.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Surface {
    /// Visual cell grid for the surface.
    pub cells: OwnedGrid,

    /// One semantic role per surface position, row-major.
    pub roles: Vec<RoleTag>,

    /// Optional producer and tracing metadata.
    pub metadata: SurfaceMetadata,
}
```

For enum variants:

```rust
/// Policy for writing the semantic role channel of a destination cell.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum RoleWritePolicy {
    /// Leave the destination role unchanged.
    PreserveDestination,

    /// Copy the role from the sampled source coordinate.
    CopySampledSource,

    /// Write the provided explicit role, such as `shadow` or `procedural`.
    SetExplicit {
        /// Role to write into the destination role channel.
        role: RoleTag,
    },
}
```

Doc comments should be **short but semantic**. Avoid long essays in field docs; put extended explanations in contract docs.

---

# Rustdoc comment style guide

Use this for v3.1 contract-visible types.

```text
Type doc:
    One-line purpose.
    One short paragraph explaining contract semantics.
    Mention owner/phase if useful.

Field doc:
    One sentence explaining meaning.
    Mention default if it is not obvious.
    Mention coordinate/role space if relevant.
    Mention whether it is semantic or visual.

Enum doc:
    Explain what choice is being made.
    Variants explain behavior, not implementation.

Avoid:
    "The x value."
    "Used by the engine."
    "TODO."
    Legacy names unless in migration docs.
```

Good:

```rust
/// Destination-local x coordinate evaluated before sampling.
pub dest_x: u16;
```

Bad:

```rust
/// X.
pub x: u16;
```

Good:

```rust
/// Role observed at the sampled source coordinate.
pub sampled_source_role: Option<RoleTag>;
```

Bad:

```rust
/// Role.
pub role: Option<RoleTag>;
```

---

# Backfill requirements for Phases A–C

I would create a small **Phase D0: Schema/Reference Backfill** before the next semantic phase.

It should not change behavior. It should only make the existing clean-room kernel schema-reference ready.

```text
+====================================================================================+
|                       PHASE D0 — SCHEMA / REFERENCE BACKFILL                         |
+====================================================================================+

  Goal:
      Make Phase A/B/C public contract types ready for generated schema/reference docs.

  Must update:
      docs/v3.1-surface-contract.md
      docs/v3.1-architecture-overview.md
      docs/new_kernel/AGENT_BRIEFING.md
      crates/tui-vfx-next contract-visible Rust types

  Must add:
      serde derives where missing
      schemars dependency
      JsonSchema derives where appropriate
      rustdoc comments where missing
      schema export command or test
      generated schema fixtures or checked generated output

  Must not add:
      effect descriptor expansion
      recipe schema
      studio manifest
      real effect ports
      legacy migration
```

---

# D0 definition of done

```text
+====================================================================================+
|                             PHASE D0 DEFINITION OF DONE                              |
+====================================================================================+

  Documentation
      [DONE] `docs/v3.1-architecture-overview.md` exists.
      [DONE] Philosophy statement exists.
      [DONE] Progressive block diagram exists.
      [DONE] Schema-reference requirement added to agent briefing/checklist.

  Rust types
      [DONE] Public contract-visible types have rustdoc comments.
      [DONE] Public contract-visible types derive Serialize/Deserialize where appropriate.
      [DONE] Public contract-visible types derive JsonSchema where appropriate.
      [DONE] Closed JSON-facing structs use deny_unknown_fields.
      [DONE] JSON-facing fields use camelCase unless deliberately internal.

  Schema generation
      [DONE] There is a repeatable command/test to generate schemas.
      [DONE] Generated schemas include doc descriptions.
      [DONE] Schema output is deterministic.
      [DONE] CI/check command can detect stale generated schemas.

  Guardrails
      [DONE] No dependency on legacy compositor/style/content/shadow crates.
      [DONE] Phase A/B/C tests still pass.
      [DONE] Workspace tests still pass.

+====================================================================================+
```

---

# Recommended schema output layout

I would start small:

```text
schemas/v3.1/next/
  surface.schema.json
  scope.schema.json
  write.schema.json
  sampler.schema.json
  pipeline.schema.json
  diagnostic.schema.json
```

Or if you want to avoid committing generated output until the contract crate split:

```text
target/generated-schemas/v3.1/next/
```

But I would prefer checking in generated schemas once stable enough, with a stale-schema check.

Later, when `tui-vfx-contract` exists:

```text
schemas/v3.1/
  surface.schema.json
  scope.schema.json
  write.schema.json
  effect-descriptor.schema.json
  recipe.schema.json
  preset.schema.json
  runtime-bindings.schema.json
  studio-manifest.schema.json
```

---

# Schema generation command

Eventually, use an `xtask` command:

```bash
cargo xtask schema --package tui-vfx-next --check
```

For the immediate backfill, a test is enough:

```text
crates/tui-vfx-next/tests/test_schema_generation.rs
```

It can verify that schema generation works and that key descriptions are present.

Then later formalize it into `xtask`.

---

# Agent requirement patch

Add this to future phase prompts:

```text
Schema/reference requirement:
All public v3.1 contract-visible types added or modified in this phase must be schema-reference ready.

That means:
- derive or intentionally implement Serialize, Deserialize, and JsonSchema where JSON-facing;
- use strict Serde attributes for public JSON shapes;
- include rustdoc comments on public types, fields, and variants;
- ensure generated JSON Schema includes meaningful descriptions;
- update generated schema/reference artifacts or schema-generation tests;
- do not add public contract fields without documentation.

If a type is intentionally internal and should not appear in generated schema/reference docs, mark that explicitly in code comments and explain why in the status memo.
```

---

# Where this fits in the roadmap

Updated roadmap:

```text
+====================================================================================+
| CURRENT ROADMAP                                                                     |
+====================================================================================+

  [DONE]   Phase A  — Semantic surface contract
  [DONE]   Phase B  — Sampled-source semantics
  [DONE]   Phase C  — Ordered pipeline semantics

  [NEXT]   Phase D0 — Schema/reference backfill
            - rustdoc comments
            - schemars derives
            - generated schema proof
            - high-level architecture doc
            - philosophy statement

  [NEXT+]  Phase D  — Contract/engine boundary and generalized ScopeSpec/write model

  [LATER]  Phase E  — Effect descriptors
  [LATER]  Phase F  — Value/parameter/source model
  [LATER]  Phase G  — Node graph
  [LATER]  Phase H  — Strict recipe v3.1 schema/compiler
  [LATER]  Phase I  — Phase/trigger engine
  [LATER]  Phase J  — First real effect ports

+====================================================================================+
```

I would do D0 immediately because it is much cheaper to enforce now than to retrofit after descriptors, values, recipes, and manifests exist.

---

# Copy-paste prompt for D0

```text
You are working in the tui-vfx Rust workspace.

Phases A, B, and C built the clean-room `tui-vfx-next` kernel and proved:
- semantic surface contract
- sampled-source semantics
- ordered pipeline/pass semantics

Your task is Phase D0: schema/reference backfill and documentation framing.

Goal:
Make the existing v3.1 clean-room contract types schema-reference ready before descriptor/schema work starts.

Hard constraints:
- Do not change runtime behavior.
- Do not port real effects.
- Do not build recipes, studio manifest, runtime bindings, phase graph, trigger engine, or legacy migration.
- Do not replace or refactor the legacy compositor.
- Do not add dependency on `tui-vfx-compositor`, `tui-vfx-style`, `tui-vfx-content`, or `tui-vfx-shadow`.
- Use v3.1 naming. Do not call this v2.

Implementation requirements:

1. Add Schemars to `tui-vfx-next` for public contract schema generation.

2. For public contract-visible types in `tui-vfx-next`, add or verify:
   - `serde::Serialize`
   - `serde::Deserialize`
   - `schemars::JsonSchema`
   - strict Serde attributes where JSON-facing:
       `#[serde(rename_all = "camelCase", deny_unknown_fields)]`
     or explicit enum tagging strategy.

3. Add rustdoc comments to every public contract-visible:
   - struct
   - enum
   - enum variant
   - public field

4. Do not over-document implementation internals. Mark internal-only types clearly if they should not be part of generated schema/reference output.

5. Add a schema generation proof.

Acceptable first version:
   - integration test that calls `schemars::schema_for!` for the main contract roots
   - verifies schema generation succeeds
   - verifies expected descriptions/titles are present

Preferred if small:
   - add a tiny schema export helper that writes JSON schemas into `schemas/v3.1/next/`

6. Add or update documentation:
   - `docs/v3.1-architecture-overview.md`
   - `docs/v3.1-surface-contract.md`
   - `docs/v3.1-feature-contract-checklist.md`
   - `docs/new_kernel/AGENT_BRIEFING.md`

The architecture overview must include:
   - brief philosophy statement
   - progressive block diagram showing Phases A/B/C/D0 and future descriptor/recipe layers
   - explanation that Rust contract types + rustdoc + generated schema are the reference path

7. Backfill status docs or add a Phase D0 status memo:
   - what was documented
   - what schemas generate
   - what types are intentionally internal
   - verification commands/results

Suggested verification:
   cargo fmt --package tui-vfx-next -- --check
   cargo clippy -p tui-vfx-next --all-targets -- -D warnings
   cargo test -p tui-vfx-next
   cargo test --workspace
   cargo tree -p tui-vfx-next
   grep -R -nE 'tui_vfx_(compositor|style|content|shadow)|tui-vfx-(compositor|style|content|shadow)' crates/tui-vfx-next

Definition of done:
Phase D0 is done when the existing Phase A/B/C clean-room contract types have rustdoc-backed schema generation proof, the high-level architecture/philosophy docs exist, and all existing behavior/tests remain unchanged.
```

---

# My recommendation

Yes: use Schemars for v3.1 public contract schemas.

Do D0 now.

Then proceed to the next semantic/descriptor phase with this rule already enforced:

```text
No undocumented public contract type.
No schema-visible field without rustdoc.
No recipe/studio-facing concept without generated schema.
```

That will save a lot of pain later.

[1]: https://docs.rs/schemars/latest/schemars/derive.JsonSchema.html "JsonSchema in schemars - Rust"
[2]: https://serde.rs/ "Overview · Serde"
[3]: https://graham.cool/schemars/examples/6-doc_comments/ "Doc Comments | Schemars"

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_C.md</FILE> - <DESC>Architect response to Phase C status memo</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
