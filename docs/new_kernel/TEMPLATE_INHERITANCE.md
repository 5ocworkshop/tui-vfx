Yes, but with an important distinction:

```text
Multiple elements at once:
    The current surface/stage model can support it conceptually,
    but we have not yet locked the scene/element/layer contract.

Template inheritance:
    This belongs above the surface engine, in the recipe/document compiler layer.
    We should design it as compile-time template composition, not runtime inheritance.
```

So the approach is compatible with both, but we should explicitly add them to the roadmap and decision checklist.

---

# 1. Multiple elements at once

Right now, Phases A–C prove this:

```text
one semantic surface
    +
sampled-source semantics
    +
ordered stages
```

That is enough to prove cell/role/scope/write behavior. But a real scene often has:

```text
modal + backdrop
card + title + body + button
HUD + cursor + notification
multiple animated widgets
foreground content + shadow + particles
```

A single flat `Surface` can represent the final result, but it cannot by itself describe **element identity**, **placement**, **z-order**, **local coordinates**, or **overlap rules**.

So we need a scene layer above `Surface`.

## Recommended model

```text
+====================================================================================+
|                                  v3.1 SCENE MODEL                                   |
+====================================================================================+

        +---------------------------+
        | Scene                     |
        |                           |
        |  elements[]               |
        |  layers[]                 |
        |  global metadata          |
        +-------------+-------------+
                      |
                      v
        +---------------------------+
        | Element                   |
        |                           |
        |  id                       |
        |  local surface            |
        |  placement / rect         |
        |  z / layer                |
        |  clip policy              |
        |  blend/write policy       |
        |  diagnostics identity     |
        +-------------+-------------+
                      |
                      v
        +---------------------------+
        | Semantic Surface          |
        |                           |
        |  cells                    |
        |  roles                    |
        |  metadata                 |
        +-------------+-------------+
                      |
                      v
        +---------------------------+
        | Final Composed Surface    |
        +---------------------------+

+====================================================================================+
```

The rule should be:

```text
Surface answers: what is at each cell?
Element answers: what object/local surface produced those cells?
Scene answers: how multiple elements compose into one final surface?
```

## Key decision

Do **not** overload `RoleTag` to mean element identity.

These are different:

```text
Role:
    text, border, shadow, background, icon

Element:
    loginButton, titleCard, modalBackdrop, notificationToast
```

A cell can have role `Text` and belong to element `titleCard`. Role is semantic class; element is instance identity.

## What we need to lock for multi-element scenes

```text
ElementId:
    stable element identity

LayerId:
    optional grouping / z-order identity

Placement:
    where the element local surface lands in scene coordinates

Local coordinates:
    element-local coordinate space

Global coordinates:
    final scene coordinate space

Overlap:
    what happens when two elements write the same cell

Role propagation:
    whose role wins on overlap

Skip behavior:
    skipped write preserves what? prior element output or original background?

Diagnostics:
    paths include element id + stage id
```

## Questions for future agents

When adding multi-element support, answer:

```text
1. Is this feature operating on one surface or a scene of elements?

2. Does it need element identity, or is role targeting enough?

3. Does it use element-local, scene-global, or sampled-source coordinates?

4. Can elements overlap?

5. If elements overlap, is order determined by:
   - array order
   - z-index
   - layer order
   - explicit composition graph

6. When a top element skips a cell, does the lower element remain visible?

7. When a top element writes an empty transparent cell, does it clear lower content or blend through?

8. Does the top element’s role replace the lower role?

9. Can an effect target:
   - whole scene
   - one element
   - one layer
   - role within one element
   - role across all elements

10. Do diagnostics identify element id, layer id, stage id, and scope?
```

This should be a dedicated phase before real recipe/effect descriptor work consumes it.

---

# 2. Template inheritance

Template inheritance is a document/compiler-layer concern, not a surface-kernel concern.

The surface engine should not know whether a recipe came from:

```text
a hand-written recipe
a template
three mixins
a preset
a generated pack
```

The compiler should resolve that before runtime.

## Strong recommendation

Avoid runtime inheritance.

Use **compile-time template composition**.

```text
template + overrides + mixins
        ↓
expanded canonical v3.1 recipe
        ↓
strict validation
        ↓
compiled runtime graph
```

The runtime should only see the canonical result.

```text
+====================================================================================+
|                              TEMPLATE COMPOSITION MODEL                              |
+====================================================================================+

        +---------------------------+
        | Base Template             |
        |                           |
        |  parameters               |
        |  elements                 |
        |  nodes/stages             |
        |  slots                    |
        |  defaults                 |
        +-------------+-------------+
                      |
                      v
        +---------------------------+
        | Recipe                    |
        |                           |
        |  extends / uses template  |
        |  fills slots              |
        |  overrides defaults       |
        |  adds elements/nodes      |
        +-------------+-------------+
                      |
                      v
        +---------------------------+
        | Template Expansion        |
        |                           |
        |  deterministic merge      |
        |  validate conflicts       |
        |  namespace ids            |
        |  produce canonical recipe |
        +-------------+-------------+
                      |
                      v
        +---------------------------+
        | Strict v3.1 Recipe        |
        |                           |
        |  no inheritance remaining |
        |  no aliases               |
        |  concrete graph           |
        +-------------+-------------+
                      |
                      v
        +---------------------------+
        | Runtime Graph             |
        +---------------------------+

+====================================================================================+
```

## Why composition over inheritance?

Classic inheritance tends to create hidden behavior:

```text
Why did this effect appear?
Which template added this parameter?
Which parent wins?
What happens if two parents define the same node?
Can a child accidentally override a required safety policy?
```

For v3.1, we want:

```text
deterministic expansion
explicit override rules
clear diagnostics
canonical compiled output
no runtime ambiguity
```

So I would call them:

```text
templates
traits
mixins
fragments
profiles
```

but define them with explicit merge semantics, not open-ended inheritance.

---

# Template questions future agents must answer

Before adding template behavior, answer:

```text
1. Is this actually a template, or just a preset?

2. Does it define structure?
   - elements
   - stages
   - nodes
   - phases
   - scopes

3. Or does it only define values?
   - parameter defaults
   - colors
   - timing
   - effect strengths

4. Does the child recipe override, extend, or fill slots?

5. Are arrays merged, appended, replaced, or forbidden to override?

6. Are maps merged by key?

7. Are node IDs namespaced?

8. Can a child remove a node from a template?

9. Can a child override an effect type?

10. Can a child override a scope?

11. Can a child override write policy or role policy?

12. Can templates depend on other templates?

13. Is multiple inheritance allowed?

14. If multiple templates define the same parameter/node/element, who wins?

15. Are some fields sealed/final?

16. How are diagnostics reported?
    - original template path
    - expanded recipe path
    - both?

17. What version range does a recipe require for a template?

18. Does the final canonical recipe contain any template references?
```

My recommended defaults:

```text
Preset:
    values only

Template:
    structure + defaults + slots

Mixin / trait:
    additive reusable fragment

Canonical compiled recipe:
    no inheritance remaining

Conflicts:
    error unless an explicit override is declared

Multiple inheritance:
    avoid at first; allow ordered mixins later only with deterministic merge rules

Child removal:
    disallow initially unless there is a clear `remove` operation with diagnostics

Sealed fields:
    support eventually for safety-critical template policies
```

---

# How these fit into the roadmap

I would update the roadmap like this:

```text
[DONE]   Phase A  — Semantic surface contract
[DONE]   Phase B  — Sampled-source semantics
[DONE]   Phase C  — Ordered pipeline/pass semantics

[NEXT]   Phase D0 — Schema/reference backfill
          rustdoc + schemars + architecture overview

[NEXT]   Phase D1 — Scene / element / layer semantics
          multiple elements, placement, z-order, overlap, element identity

[LATER]  Phase D2 — Template composition design document
          compile-time template expansion rules, not implementation-heavy yet

[LATER]  Phase E  — Effect descriptors
[LATER]  Phase F  — Values / parameters / signals
[LATER]  Phase G  — Node graph
[LATER]  Phase H  — Strict recipe v3.1 schema/compiler
[LATER]  Phase H2 — Template expansion implementation
[LATER]  Phase I  — Phase/trigger engine
[LATER]  Phase J  — First real effect ports
```

I would put **multi-element scenes before effect descriptors**, because descriptors need to know whether effects target:

```text
surface
element
layer
scene
role within element
role across scene
```

I would put **template implementation after recipe schema/compiler**, because templates expand into canonical recipes. But I would write the template design document earlier so we do not paint ourselves into a corner.

---

# What to add to the checklist now

Add this section to `docs/v3.1-feature-contract-checklist.md`.

```text
## Multi-element scene questions

If a feature involves more than one visual object, answer:

- Does it need element identity?
- Does it need layer identity?
- Does it operate in element-local or scene-global coordinates?
- Can it target one element, a group, a layer, or the whole scene?
- What happens on overlap?
- Which cell wins?
- Which role wins?
- Does transparent write blend, clear, or skip?
- Does skip preserve lower element output?
- How are diagnostics attributed to element/layer/stage?
```

And this:

```text
## Template composition questions

If a feature reuses recipe structure, answer:

- Is this a preset, template, mixin, or profile?
- Does it define structure or only values?
- Does it use slots?
- What may children override?
- What is sealed?
- How are conflicts resolved?
- Are node/element/parameter IDs namespaced?
- Is expansion deterministic?
- Does final canonical v3.1 contain no template references?
```

---

# Direct answer

```text
1. Multiple elements at once:
   The approach supports this direction, but we have not locked it yet.
   We need a Scene / Element / Layer contract phase before real descriptors and recipes.

2. Template inheritance:
   The approach supports this as compile-time template composition.
   It should not exist in the runtime surface engine.
   It should expand into strict canonical v3.1 recipes before validation/compilation.
```

So the adjustment I would make is:

```text
Add Phase D1 for scenes/elements/layers.
Add Phase D2 for template composition design.
Implement template expansion later in the recipe compiler phase.
```

That keeps our philosophy intact:

```text
Surface semantics first.
Scene composition second.
Contract descriptors third.
Recipe/template authoring after the execution model is clear.
```
