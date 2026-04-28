# D1 architect verdict

**Approved.**

Phase D1 locked the multi-element scene question at the right level of abstraction. We now have:

```text
Surface
    cells + roles + metadata

Pipeline
    ordered stages over current/next surfaces

Scene
    ordered placed elements composed into final surface
```

That is enough foundation for descriptors and recipes to avoid guessing what “multiple things on screen” means.

---

# What D1 locks

```text
+====================================================================================+
|                                  PHASE D1 LOCKS                                      |
+====================================================================================+

  SCENE
      [LOCK] A Scene composes multiple SceneElements into one final Surface.
      [LOCK] Scene has final width/height.
      [LOCK] Scene composition is deterministic.

  ELEMENT
      [LOCK] SceneElement owns a local Surface.
      [LOCK] ElementId is instance identity.
      [LOCK] Element identity is distinct from RoleTag.
      [LOCK] ElementPlacement is signed and maps local coords into scene coords.

  ORDERING
      [LOCK] Elements compose by ascending zIndex.
      [LOCK] Declaration order is the deterministic tie-break for equal zIndex.
      [LOCK] Higher/later written cells overwrite lower/current cell and role.

  WRITE / SKIP
      [LOCK] SkipTransparentEmpty preserves lower/current output.
      [LOCK] WriteCell can write empty transparent cells and clear lower/current output.
      [LOCK] RoleWritePolicy applies during scene composition.

  LAYER
      [LOCK] LayerId currently exists as optional grouping metadata only.
      [LOCK] D1 does not define a full layer graph.

  DIAGNOSTICS
      [LOCK] Scene diagnostics are element-aware.
      [LOCK] Diagnostic paths use stable field paths, not instance-id path segments.

  SCHEMA
      [LOCK] Scene, SceneElement, and SceneOutcome are schema roots.
      [LOCK] D0 schema-reference rule applies to new scene contract types.

+====================================================================================+
```

---

# Answers to D1 open questions

## 1. Should next phase be D2 template composition design?

**Yes.** Proceed with **Phase D2 — Template Composition Design**, but keep it as a **design-document phase**, not an implementation phase.

Reason: we now have the scene/element vocabulary templates need to talk about. We can design template composition rules without adding recipe compiler code yet.

## 2. Should a future scene phase add `ClipPolicy::Error`?

Not now. Keep hard placement validation for the recipe/compiler layer.

Current runtime/engine proof can keep clipping semantics simple. Later, strict recipe validation can decide whether a particular recipe/template declares:

```text
clip
warn
error
```

Do not harden this before we know the recipe validation model.

## 3. Should layer semantics remain optional IDs?

Yes. Keep `LayerId` as optional grouping metadata for now.

Do not build a full layer graph until descriptors or recipes need one of these:

```text
target a layer
apply effects to a layer group
compose layers independently
preserve layer provenance
render layer diagnostics
```

D1 was correct to avoid inventing that early.

---

# Recommended next phase

```text
+====================================================================================+
|                         PHASE D2 — TEMPLATE COMPOSITION DESIGN                       |
+====================================================================================+
```

This should be a **docs-only or mostly-docs** phase.

The purpose is to answer:

```text
How do templates, traits, mixins, presets, and recipes compose into one strict
canonical v3.1 recipe before runtime?
```

The runtime engine should not know whether a scene came from a template. Templates should be resolved before validation/compilation.

---

# D2 philosophy

Use **compile-time template composition**, not runtime inheritance.

```text
template / mixin / preset / recipe
        ↓
deterministic expansion
        ↓
strict canonical v3.1 recipe
        ↓
validation
        ↓
compiled runtime graph
```

Avoid classic inheritance ambiguity:

```text
Which parent wins?
Who added this node?
Can a child override safety policy?
Can two templates define the same element id?
Can removing a parent silently change behavior?
```

Prefer explicit composition:

```text
templates define structure and slots
mixins add reusable fragments
presets override values only
recipes fill slots and declare overrides
compiler produces canonical recipe with no template references remaining
```

---

# D2 block diagram

```text
+==================================================================================================+
|                          PHASE D2 — TEMPLATE COMPOSITION DESIGN                                   |
+==================================================================================================+

        +-------------------------------+
        | Template                      |
        |                               |
        |  parameters                   |
        |  scene/elements               |
        |  nodes/stages                 |
        |  slots                        |
        |  defaults                     |
        |  sealed fields                |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Mixin / Trait                 |
        |                               |
        |  additive reusable fragment   |
        |  no hidden parent hierarchy   |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Recipe                        |
        |                               |
        |  uses templates/mixins        |
        |  fills slots                  |
        |  declares overrides           |
        |  adds scene elements          |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Template Expansion            |
        |                               |
        |  deterministic merge          |
        |  namespace ids                |
        |  validate conflicts           |
        |  report provenance            |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Canonical v3.1 Recipe         |
        |                               |
        |  no inheritance remains       |
        |  no aliases                   |
        |  strict schema                |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Recipe Compiler / Runtime     |
        |                               |
        |  future phase                 |
        +-------------------------------+

+==================================================================================================+
```

---

# What D2 should lock

```text
+====================================================================================+
|                                  PHASE D2 LOCK TARGETS                              |
+====================================================================================+

  CONCEPTS
      [LOCK] Difference between template, mixin, preset, profile, and recipe.
      [LOCK] Templates define structure.
      [LOCK] Presets define values only.
      [LOCK] Runtime receives canonical expanded recipes only.

  EXPANSION
      [LOCK] Template expansion is deterministic.
      [LOCK] Expansion happens before strict recipe validation.
      [LOCK] Final canonical v3.1 has no template/inheritance references.

  OVERRIDES
      [LOCK] Explicit override syntax is required for conflicts.
      [LOCK] Accidental key collisions are errors.
      [LOCK] Some fields may be sealed/final.
      [LOCK] Removal is disallowed initially unless explicitly designed.

  IDS
      [LOCK] Parameter/node/element ids need deterministic namespacing rules.
      [LOCK] Slot-filled content must not create accidental id collisions.

  DIAGNOSTICS
      [LOCK] Diagnostics should report both template source and expanded recipe path.
      [LOCK] Merge conflicts have structured diagnostic codes.

  SCOPE CONTROL
      [LOCK] D2 does not implement recipe compiler.
      [LOCK] D2 does not implement runtime inheritance.
      [LOCK] D2 does not port effects.

+====================================================================================+
```

---

# D2 design questions

The D2 document should answer these directly.

```text
1. What is a template?

2. What is a mixin / trait?

3. What is a preset?

4. What is a profile?

5. What is a recipe?

6. Which of these can define:
   - scene elements
   - stages/nodes
   - parameters
   - signals
   - scopes
   - write policies
   - phases
   - defaults

7. Can templates extend templates?

8. Is multiple inheritance allowed?
   Recommended: no for base templates; ordered mixins may be allowed later.

9. How are maps merged?

10. How are arrays merged?
    Recommended: replace by default, append only with explicit operation.

11. How are ids namespaced?

12. How are slot names resolved?

13. Can a child recipe remove inherited structure?
    Recommended: no initially.

14. Can a child override:
    - effect id
    - scope
    - write policy
    - role policy
    - parameter default
    - phase transition

15. Which fields can be sealed?

16. How are conflicts diagnosed?

17. How is provenance preserved in diagnostics?

18. What does the canonical expanded recipe look like?

19. Does the runtime ever see templates?
    Recommended: no.

20. How does this affect generated schema/reference docs?
```

---

# D2 recommended files

Add:

```text
docs/v3.1-template-composition.md
docs/new_kernel/PHASE_D2_STATUS.md
docs/new_kernel/PHASE_D2_STATUS_MEMO_TO_ARCHITECT.md
```

Update:

```text
docs/v3.1-architecture-overview.md
docs/v3.1-feature-contract-checklist.md
docs/new_kernel/AGENT_BRIEFING.md
docs/new_kernel/INDEX.md
docs/INDEX.md
```

No Rust code is required unless the agent adds small examples/tests for documentation consistency. Prefer no new public contract types in D2.

---

# Copy-paste Phase D2 prompt

```text
You are working in the tui-vfx Rust workspace.

Phases A–D1 built and schema-backed the clean-room `tui-vfx-next` kernel:
- Phase A: semantic surface contract
- Phase B: sampled-source semantics
- Phase C: ordered pipeline/pass semantics
- Phase D0: schema/reference backfill
- Phase D1: scene / element / layer composition semantics

Your task is Phase D2: Template Composition Design.

Goal:
Design how v3.1 templates, mixins/traits, presets, profiles, and recipes compose into one strict canonical v3.1 recipe before validation, compilation, and runtime.

This is primarily a design-document phase.

Primary question:
How can recipes reuse common structure without runtime inheritance or hidden legacy behavior?

Hard constraints:
- Do not implement the recipe compiler.
- Do not implement template expansion.
- Do not add runtime inheritance.
- Do not add effect descriptor expansion.
- Do not add studio manifest, runtime bindings, phase graph, trigger engine, or legacy migration.
- Do not port real effects.
- Do not replace or refactor the legacy compositor.
- Do not add legacy aliases.
- Use v3.1 naming consistently.
- Preserve the D0 schema/reference rule for any public contract-visible types if you add any Rust types. Prefer not to add Rust types in this phase.

Required design position:
- Templates are compile-time composition inputs.
- Runtime sees only canonical expanded strict v3.1 recipes.
- Presets are values-only.
- Templates may define structure and slots.
- Mixins/traits are additive reusable fragments.
- Expansion must be deterministic.
- Conflicts must produce structured diagnostics.
- Strict v3.1 recipes contain no legacy aliases.

Required document:
Create:

    docs/v3.1-template-composition.md

It must define:
- philosophy and non-goals
- template vs mixin/trait vs preset vs profile vs recipe
- expansion pipeline
- merge rules
- override rules
- sealed/final fields
- id namespacing
- slot filling
- conflict diagnostics
- provenance model
- canonical expanded recipe output
- what runtime does and does not see
- open questions for implementation phase

Recommended diagrams:
Include at least one block diagram showing:

    template + mixins + recipe + preset
        -> expansion
        -> canonical v3.1 recipe
        -> validation/compiler/runtime

Also update:
- docs/v3.1-architecture-overview.md
- docs/v3.1-feature-contract-checklist.md
- docs/new_kernel/AGENT_BRIEFING.md
- docs/new_kernel/INDEX.md
- docs/INDEX.md if applicable

Status memo:
Add:

    docs/new_kernel/PHASE_D2_STATUS.md
    docs/new_kernel/PHASE_D2_STATUS_MEMO_TO_ARCHITECT.md

The memo should summarize:
- design decisions
- what was deliberately not implemented
- open questions
- recommended next phase

Verification:
Since this is a docs-focused phase, run appropriate checks available in the workspace.
At minimum:
- ensure no forbidden clean-room dependencies were added
- ensure no unintended Rust changes require schema fixture updates
- run `cargo test -p tui-vfx-next` if Rust files changed
- run `cargo test --workspace` if feasible
- run formatting checks on changed docs/source if applicable

Definition of done:
Phase D2 is done when the project has a durable template composition design that explains how reusable authoring structure expands deterministically into canonical strict v3.1 recipes, without implementing the recipe compiler or introducing runtime inheritance.
```

---

# After D2

After D2, I would choose between:

```text
Phase D3 — Contract/engine boundary + generalized ScopeSpec/write model
```

or:

```text
Phase E — Effect descriptor model
```

My current leaning:

```text
D2 next.
Then D3.
Then E.
```

Reason: descriptors will be cleaner if we first separate contract DTOs from engine proof code and make `ScopeSpec` / write policy the stable contract vocabulary.

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

  [NEXT]   Phase D2 — Template composition design document

  [NEXT+]  Phase D3 — Contract/engine boundary + generalized ScopeSpec/write model

  [LATER]  Phase E  — Effect descriptor model
  [LATER]  Phase F  — Value / parameter / signal model
  [LATER]  Phase G  — Node graph
  [LATER]  Phase H  — Strict recipe v3.1 schema/compiler
  [LATER]  Phase H2 — Template expansion implementation
  [LATER]  Phase I  — Phase/trigger engine
  [LATER]  Phase J  — First real effect ports

+====================================================================================+
```

Proceed with **Phase D2** as a design phase.
