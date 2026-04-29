<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_H0.md</FILE> - <DESC>Architect response to Phase H0 and Phase H1 assignment</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase H1: preserve architect guidance for vocabulary and canonical recipe document schema.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture H1 assignment and recipe-evidence guidance.</CLOG> -->

Yes, add a `VOCABULARY.md`. I would make it a standing contract artifact starting in H1.

I recommend:

```text
docs/VOCABULARY.md
```

and link it from:

```text
docs/INDEX.md
docs/new_kernel/INDEX.md
docs/v3.1-architecture-overview.md
docs/v3.1-feature-contract-checklist.md
```

It should be maintained every phase the same way schemas and status memos are maintained.

# Why VOCABULARY.md matters

At this point the project has enough terms that drift will become expensive:

```text
effect
source
procedural
asset
graph
node
element
layer
scene
surface
scope
role
parameter
signal
binding
graph value
output
input
template
preset
profile
runtime
demo
player
manifest
```

The schemas tell us the machine contract. `VOCABULARY.md` should tell humans and agents what each word means, what owns it, what it must not be confused with, and what legacy words map to it.

The big value is preventing agents from accidentally reusing old names like `hint`, `binding`, `layer`, `source`, or `pipeline` in inconsistent ways.

# VOCABULARY.md requirements for H1

Ask the H1 agent to create a first draft using a sub-agent.

The sub-agent should read:

```text
schemas/v3.1/contract/*.schema.json
schemas/v3.1/next/*.schema.json
docs/v3.1-architecture-overview.md
docs/v3.1-contract-boundary.md
docs/v3.1-surface-contract.md
docs/v3.1-template-composition.md
docs/v3.1-feature-contract-checklist.md
```

Then produce a first draft:

```text
docs/VOCABULARY.md
```

Suggested structure:

```text
# v3.1 Vocabulary

## Purpose

## Canonical terms

For each term:
- Term
- Definition
- Owns / owned by
- Rust contract type, if any
- JSON/schema surface, if any
- Not the same as
- Legacy/source-authoring synonyms, if any
- Example

## Deprecated / legacy terms

## Naming rules

## Terms intentionally deferred

## Change policy
```

Example entry:

```text
## Source

A source produces an initial semantic Surface. It is not an effect.

Owned by:
    SourceDescriptor / SourceSpec

Not the same as:
    Effect — transforms or samples an existing surface.
    Asset — external data consumed by a source.
    Node — canonical graph instance that executes an effect.

Legacy/source authoring examples:
    scene.layers[].source
    config.content
```

Another:

```text
## Graph value

A graph-local typed value published by a node and consumed by later nodes.

Owned by:
    GraphValueId / GraphValueKind / GraphValueShape / NodeOutputSpec

Not the same as:
    Parameter — public recipe control.
    Signal — host-provided runtime value.
    Binding — declarative connection to a parameter target.
```

# How we will know the schemas are right

Your instinct is right: we will know we are close when migration becomes mostly a mapping exercise, not a schema redesign exercise.

More precisely, schemas are “lock-ready” when these are true:

```text
1. Representative recipes map without inventing new top-level concepts.

2. Complex recipes map by composition of existing concepts:
       source
       asset
       scene element
       graph node
       topology
       parameter
       signal
       binding
       graph value
       phase/timing, once added

3. New effects usually require only:
       descriptor
       inputs
       outputs
       adapter/implementation
   not changes to recipe/schema architecture.

4. Ambiguous legacy fields produce migration diagnostics,
   not new schema branches.

5. Canonical recipes round-trip stably:
       source authoring / migrated recipe
           -> canonical recipe
           -> validate
           -> serialize
           -> validate again
   with no semantic drift.

6. The player can execute from canonical contract data without reading legacy recipe internals.

7. The studio can build controls from manifest/contract data without reverse-engineering recipes.

8. Invalid cases fail with useful structured diagnostics.

9. The schema can represent the hard examples:
       nested sequence/parallel
       parallel snapshot merge
       node I/O
       scenes
       assets
       procedurals
       runtime-bound inputs
       event-driven dwell
       motion
       source-local pipelines
```

The key phrase I would use is:

```text
Schema lock does not mean “no future additions.”
It means “future additions are additive capabilities, not corrections to basic concepts.”
```

So the test is not “can we map 606 recipes perfectly on the first pass?” The test is:

```text
When a recipe fails to map, is the failure because:
    A. we need to port an effect/source descriptor?
    B. we need a migration rule?
    C. we found a genuinely missing contract concept?

A and B are fine.
Too many C failures means the schema is not locked yet.
```

# H1 should read real recipes, but as evidence

For H1, yes: give the implementer and any sub-agents recipe context.

But be explicit:

```text
Read these recipes as evidence only.
Do not migrate them.
Do not preserve their exact JSON shape.
Do not let legacy aliases or old field names define canonical v3.1.
Use them to test whether the canonical recipe document has homes for all major concepts.
```

# H1 recipe context list

For H1, I would provide this curated list from:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/
```

## Minimal / baseline

```text
baseline.json
```

Why: proves effect-free recipe, layout, base style, lifecycle, empty pipeline.

## Single pipeline and full graph complexity

```text
complex/complex_full_pipeline.json
complex/complex_nested_parallel_sequences.json
complex/complex_parallel_overlap_conflict_snapshot.json
complex/v3_cross_family_sequence_disjoint.json
complex/v3_scheduler_parallel_join_filter_mask.json
```

Why: masks, samplers, filters, shaders, sequence/parallel, conflict semantics, graph values, cross-family ordering.

## Node I/O / graph value examples

```text
complex/v3_io_scalar_filter.json
complex/v3_io_radial_twist_spiral_chain.json
complex/v3_io_parallel_merge_shader.json
complex/v3_io_authoring_ladder_toast_glow_chain.json
```

Why: producer/consumer chains, parallel value join, sampler-to-shader/filter-to-shader patterns.

## Scene / source / asset / procedural

```text
scene/scene_layer_full_stack.json
scene/scene_layer_visibility_binding_io.json
scene/scene_layer_io_filter_shader.json
scene/scene_authoring_ladder_flag_asset_binding.json
scene/scene_authoring_ladder_procedural_spinner_binding.json
scene/scene_braille_flag_asset_token.json
scene/scene_braille_flag_runtime_wave.json
```

Why: multiple elements, sources, procedural sources, assets, source-local pipelines, source styling, visibility, binding-driven procedural params.

## Content source/effect boundary

```text
content/content_split_flap_solari_authentic.json
```

Why: content behavior that may become source, effect, or source+effect depending on canonical modeling.

## Runtime binding / loopback evidence

```text
filters/filter_kitt_scanner_progress_binding.json
loopback/loopback_rigid_shake_severity_ramp.json
signals/wave_with_envelope_signal.json
```

Why: host parameters/signals, demo loopback, signal composition, progress meters, runtime-driven controls.

## Event-driven dwell

```text
event_driven_dwell/bool_binding_demo.json
event_driven_dwell/bool_binding_truthy_loopback.json
```

Why: dwell termination by binding/signal, latch behavior, fallback caps, phase/timing implications.

## Motion / layout / resize evidence

```text
motion_routes/motion_figure_eight_infinity.json
motion_routes/scene_layer_follow_lag.json
motion_routes/toast_shadow_edge_crossing.json
complex/resize_preserve_phase_chain.json
```

Why: motion routes, aliases to canonical route names, relative/follow motion, edge crossing, resize/host grid semantics.

## Optional source edge cases

Ask them to read these only if H1 touches source categories directly:

```text
scene/ansi_source_chain.json
scene/scene_image_source_bindable.json
complex/command_capture_chain.json
```

Why: ANSI source, image source, command/capture source.

# Suggested H1 assignment

```text
Phase H1 — Canonical Recipe Document Schema

Goal:
    Define the strict canonical v3.1 recipe document shape that packages the
    already-locked contract pieces into one compiler/runtime input.

Important:
    H1 is canonical recipe shape, not legacy/source authoring syntax.
    H1 should not implement template expansion or migration.
    H1 should not implement runtime execution beyond existing proof layers.

Inputs already available:
    Surface / Scene
    Source / Asset
    EffectDescriptor / inputs / outputs
    Value / ValueSource
    Parameter / Signal / Binding
    GraphSpec / NodeSpec / topology / graph values
    Schema/rustdoc/reference rules

Add:
    RecipeId or reuse existing id type if already present
    RecipeDocument / CanonicalRecipeDocument
    recipe metadata
    required descriptors/sources/assets
    parameters
    signals
    bindings
    assets
    sources
    graph
    scenes/elements if not already nested through graph/source shape
    lifecycle/timing placeholder only if needed
    validation entry point for cross-document consistency
    checked schema root for recipe document
```

# H1 must answer these questions

```text
1. What is the canonical root object?

2. Does a recipe contain one GraphSpec, one Scene, or both?

3. Are scene elements graph nodes, graph inputs, or a parallel contract layer beside graph?

4. Where do SourceSpec instances live?

5. Where do AssetSpec declarations live?

6. How does a scene element reference a source-produced surface?

7. How does an element-local pipeline reference GraphSpec/topology?

8. How do public parameters/signals/bindings attach to source inputs and node inputs?

9. What is explicitly omitted from canonical H1 and left to source-authoring/lowering?

10. What existing source recipe fields must lower away before canonical form?
```

# H1 non-goals

Keep these out:

```text
template expansion implementation
legacy migration implementation
runtime ParameterStore / SignalStore
binding execution
phase graph / trigger engine, unless only placeholder metadata
studio manifest
demo loopback execution
asset loading
procedural rendering
real effect ports
visual parity
```

# H1 success criteria

H1 is successful when:

```text
1. A canonical recipe document can contain:
       metadata
       parameters
       signals
       bindings
       assets
       source descriptors/specs or source refs
       effect descriptors
       graph nodes/topology
       scene/elements where needed

2. The schema is generated from Rust with strict shapes and rustdoc descriptions.

3. Validation catches:
       unknown parameter refs
       unknown signal refs
       unknown asset refs
       unknown source refs
       unknown effect refs
       node input kind mismatch
       missing required source/effect inputs
       graph order/topology errors
       graph value kind/shape mismatch

4. The curated recipes can be manually mapped conceptually without inventing
   new root concepts.

5. Any unmapped recipe concept is recorded as:
       deferred phase
       migration-only concern
       or genuine schema gap

6. No legacy aliases or interpolation syntax become canonical.
```

# H1 should produce a mapping notes doc

Ask for:

```text
docs/new_kernel/H1_RECIPE_EVIDENCE_NOTES.md
```

This should not be a full migration plan. It should be a short table:

```text
Old evidence concept              Canonical v3.1 home             Status
requires_bindings                 ParameterSpec / SignalSpec       covered
requires_assets                   AssetSpec                        covered
{{ flag_art }}                    AssetRef                         covered, canonical differs
pipeline.step.sequence            GraphStep::Sequence              covered
pipeline.step.parallel            GraphStep::Parallel              covered
io.outputs[].hint                 NodeOutputSpec / GraphValueId    covered
scene.layers[]                    SceneElement                     covered
visibility.predicate              deferred to phase/trigger        deferred
motion.enter/exit                 deferred motion/phase model      deferred
loopback.signal                   demo/player profile              deferred
```

This will help us see whether the schema is actually stabilizing.

# Add this to the H1 prompt

```text
Create docs/VOCABULARY.md.

Use a sub-agent to read existing schemas and architecture docs and generate
the first draft. The document must define canonical v3.1 terms, identify
owning Rust/schema types, list “not the same as” distinctions, and record
legacy/source-authoring synonyms where useful. Link the vocabulary document
from docs/INDEX.md and docs/new_kernel/INDEX.md.

From H1 onward, every phase that adds or changes public contract vocabulary
must update docs/VOCABULARY.md.
```

# Direct answer

Yes, create `VOCABULARY.md` now.

And yes, schema lock is proven by mapping pressure: when representative recipes mostly map into the schema without new concepts, and failures become migration rules or deferred implementation work rather than schema redesign, we know the shape is right.

For H1, have the implementer read the curated recipe set above as evidence, not as canonical syntax. That gives enough real-world pressure without drowning them in the full corpus.

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_H0.md</FILE> - <DESC>Architect response to Phase H0 and Phase H1 assignment</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
