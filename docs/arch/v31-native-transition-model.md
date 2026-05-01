<!-- <FILE>docs/arch/v31-native-transition-model.md</FILE> - <DESC>Architecture rationale and canonical shape for native v3.1 transition specs and tracks</DESC> -->
<!-- <VERS>VERSION: 0.4.1</VERS> -->
<!-- <WCTX>v3.1 schema audit: make transitions first-class without adding legacy execution layers.</WCTX> -->
<!-- <CLOG>0.4.1: PATCH — clarify schema-approved canonicalization only and shared-kernel guidance for overlapping transition tracks and primitives.</CLOG> -->

# Native v3.1 Transition Model

## Purpose

This document explains why v3.1 has a native transition model and how authors,
loaders, players, and compositors should think about it.

The core decision is:

> A transition is a state/surface-change interval with executable canonical
> tracks. It is not a legacy effect chain and not a runtime translation layer.

This is part of the pure v3.1 end-to-end direction. Author shorthand may be
canonicalized at load time, but only when that shorthand is schema-approved
v3.1 authoring syntax. Canonicalization must not accept legacy field aliases,
compositor-shaped DTOs, or compatibility inputs. The loaded recipe remains v3.1
all the way through and the compositor executes v3.1 transition tracks directly.

## Why transitions are first-class

Common visual changes have names that authors already understand:

- iris reveal;
- wipe left-to-right;
- crossfade;
- push;
- dissolve;
- slide up and fade in;
- typewriter reveal;
- split-flap cascade;
- blinds reveal.

Forcing authors or AI systems to assemble those ideas from low-level masks,
samplers, shaders, blend operations, and timing nodes every time creates several
problems:

1. **Authorability.** Authors think in visible operations, not implementation
   fragments.
2. **Validatability.** A typed transition can validate required fields at load
   time.
3. **Discoverability.** The schema can enumerate available transition and track
   kinds.
4. **Optimization.** The compositor can use the intent and track kind to choose
   direct grid-native execution paths.
5. **Documentation.** Each transition/track family can have examples and visual
   signatures.
6. **AI authoring.** Named track kinds and closed field vocabularies reduce the
   search space for generated recipes.
7. **Theme and capability substitution.** Variants and reduced-motion policies
   can replace a transition without rewriting unrelated scene/source structure.

The goal is not to hide the lower-level effect system. Effects, masks, shaders,
filters, samplers, and content transforms remain direct primitives and escape
hatches. The transition model exists so common state-change intervals are not
accidental chains of unrelated primitives.

## Mental model

```text
transition = state/surface-change interval
track      = one executable visual concern inside that interval
effect     = reusable single-surface/per-cell operation
source     = produces a surface
scene      = arranges surfaces
graph node = persistent or phase-scoped effect work
```

A transition answers: what visibly changes between states, over what timing, and
for which subjects?

A source answers: what surface exists?

A scene answers: where do surfaces go?

A graph node answers: what ongoing or phase-scoped effect work applies to a
surface?

## Five-home decision model

Use the recipe corpus to identify boundaries, then keep the engine schema small.
Most recipe concerns belong in one of five homes:

```text
1. source
   Produces a surface.

2. scene
   Places surfaces on the grid.

3. transition
   Handles bounded enter / exit / reveal / hide / from-to state changes.

4. graph/effect node
   Handles ongoing or phase-scoped visual processing.

5. signal/value source
   Provides runtime or preview-driven values.
```

Simple authoring decision tree:

```text
Is it producing content?
  → source

Is it placing content on the grid?
  → scene element

Is it a bounded change from one visual state to another?
  → transition

Does it run continuously or during dwell?
  → graph/effect node

Is it a runtime/preview value?
  → signal/value source
```

Examples:

```text
iris reveal          → transition
wipe                 → transition
split flap in        → transition
typewriter in        → transition
fade in/out          → transition
push/crossfade       → transition

matrix rain          → graph/effect node or procedural source
KITT scanner         → graph/effect node
pill progress        → graph/effect node
vignette breathing   → graph/effect node
radial twist         → graph/effect node
border sweep         → graph/effect node

braille flag source  → source
asset-backed art     → asset + source
shadow               → surface attachment
row modulo styling   → graph/effect node with scope
```

The practical boundary is:

```text
V3.1 core should make common animation/compositing intent explicit.
It should not make every visual trick a top-level schema concept.
```


## Canonical contract shape

Schematic canonical v3.1 shape:

```json
{
  "id": "enter",
  "kind": "transition",
  "activePhases": ["enter"],
  "subjects": {
    "from": { "kind": "empty" },
    "to": { "kind": "scene", "id": "main" }
  },
  "timing": {
    "duration": { "kind": "milliseconds", "value": 300 },
    "easing": { "kind": "named", "value": "outCubic" }
  },
  "interruption": "reverseFromCurrent",
  "reducedMotion": {
    "policy": "substitute",
    "transition": "enterReduced"
  },
  "variants": [],
  "tracks": [
    {
      "kind": "motion.slide",
      "subject": "to",
      "travelDirection": "up"
    },
    {
      "kind": "opacity.fade",
      "subject": "to",
      "from": { "kind": "literal", "value": { "kind": "number", "value": 0 } },
      "to": { "kind": "literal", "value": { "kind": "number", "value": 1 } }
    }
  ]
}
```

Important fields:

- `id` names the transition in `RecipeDocument.transitions`.
- `intent` may preserve author shorthand such as a preset. It is metadata, not
  the executable form.
- `subjects` names from/to/shared participants.
- `timing` supplies default duration/easing/stagger for tracks.
- `activePhases` records lifecycle phase participation.
- `tracks` is the executable canonical form.
- `interruption` is required and says how superseded transitions behave.
- `reducedMotion` is required and says how accessibility fallback works.
- `variants` carries engine-neutral conditional replacements.

## Track families

Current native track families:

```text
visibility.*  coverage, reveal, aperture, dissolve, stipple, braille, blinds
opacity.*     one-subject alpha/opacity changes
motion.*      one-subject placement changes, including path motion
relation.*    coordinated from/to surface relationships
content.*     content-reveal or content-mutation intervals
style.*       transient style sweeps during a transition interval
```

Initial concrete track kinds include:

```text
visibility.wipe
visibility.iris
visibility.dissolve
visibility.stippled
visibility.braille
visibility.blinds

opacity.fade

motion.slide
motion.path

relation.crossfade
relation.push
relation.morph

content.typewriter
content.splitFlap

style.glistenBand
```

The list is intentionally small enough to document and validate, but broad
enough to cover current recipe evidence.

Conservative promotion rule:

```text
Does the compositor need this concept to execute common recipes directly?
If yes, add or keep the native concept.
If no, keep it in descriptors, structured payloads, docs, or future work.
```

Current priority stance:

| Concept | Priority | Why |
| --- | --- | --- |
| `TransitionSpec` / `TransitionTrack` | keep | Core state-change primitive and executable shape. |
| `activePhases` | keep | Aligns transitions with lifecycle without replacing lifecycle. |
| `interruption` | keep | Runtime behavior that is hard to retrofit. |
| `reducedMotion` / variants | keep | Accessibility and capability behavior that is hard to retrofit. |
| `visibility.*`, `opacity.fade`, `motion.*`, `relation.*`, `content.*` | keep | Common bounded transition concerns proven by the corpus. |
| Ambiguous field renames | do now | Highest clarity gain; especially protect `progress`. |
| Full signal algebra expansion | defer | Useful, but it is a separate mini-language beyond the seed typed expression support. |
| Full shadow system expansion | defer | Keep the typed shadow attachment narrow until more shadow recipes stabilize. |
| Phase-variant inputs | defer | Ergonomic, but not essential to the transition decision. |
| Motifs/shared rhythm | defer | Useful, but can be represented through signals first. |
| Capability variants outside transitions | defer | Good future reuse; not needed to settle transition shape. |
| Design-system semantics | never in engine core | Belongs to consumers such as gt-design. |


## Lifecycle relationship

Lifecycle remains public and meaningful:

```text
enter -> dwell -> exit
```

Transitions do not replace lifecycle. They participate in lifecycle phases via
`activePhases`.

Examples:

- split-flap text reveal: transition with `activePhases: ["enter"]` and a
  `content.splitFlap` track;
- matrix rain: graph/source behavior with `activePhases: ["dwell"]`, not a
  transition by default;
- fade out: transition with `activePhases: ["exit"]` and an `opacity.fade` track;
- CRT jitter on crash entry: phase-scoped graph node unless it is explicitly
  modeled as a transition track.

Triggers request lifecycle actions. Transition tracks express visible behavior
inside the selected interval.

## Reduced motion and interruption

Every transition must declare `interruption` and `reducedMotion`.

Reduced-motion substitution must be terminal and acyclic. This is valid:

```text
enter -> enterReduced -> none
```

This is invalid:

```text
enter -> enterReduced -> enter
```

A reduced transition can use a short fade, instant snap, or another non-motion
policy. It must not require an infinite replacement chain.

Interruption is separate from reduced motion. Interruption handles supersession
of an in-flight transition, for example `reverseFromCurrent`, `snapToEnd`,
`snapToEndThenStartNext`, or `preserveCurrentFrame`.

## Author shorthand and canonicalization

Authoring surfaces may allow shorthand:

```json
{
  "kind": "transition",
  "preset": "iris",
  "durationMs": 300
}
```

The loader may canonicalize that to:

```json
{
  "kind": "transition",
  "intent": {
    "kind": "preset",
    "preset": "iris"
  },
  "timing": {
    "duration": { "kind": "milliseconds", "value": 300 }
  },
  "tracks": [
    { "kind": "visibility.iris", "subject": "to", "shape": "circle" }
  ]
}
```

That is canonicalization, not runtime translation. The compositor receives and
executes canonical v3.1 structures. Canonicalization may only expand
schema-approved v3.1 authoring shorthand; it must not accept legacy field
aliases or compositor-shaped DTOs.


## Shared kernels with primitive descriptors

Some transition tracks share visual semantics with graph/effect primitives, for
example `visibility.wipe` with `mask.wipe`, `visibility.iris` with `mask.iris`,
`content.typewriter` with `content.typewriter`, or a transient glisten track with
`shader.glistenBand` behavior.

When a transition track and a primitive descriptor share visual semantics,
prefer a shared runtime helper/kernel where practical. The transition track owns
interval, subject, timing, interruption, and reduced-motion semantics. The
primitive descriptor owns graph/effect-node semantics, dwell usage, and direct
primitive inputs. Keep those contracts distinct even when the math is shared.

## Recipe-oracle mapping rules

Recent recipe review produced these migration rules:

| Old/current recipe concern | Native v3.1 home |
| --- | --- |
| `requires_assets` | `assets` plus structural source asset refs |
| runtime binding requirements | graph `signals` or `parameters` |
| layout dimensions | `RecipeScene.width` / `RecipeScene.height` |
| layer placement | `RecipeScene.elements[].placement` |
| source generator parameters | source inputs, preferably typed/bindable |
| base style, title, border, message | source or element surface concerns |
| enter duration/easing | transition `timing` with `activePhases: ["enter"]` |
| exit duration/easing | transition `timing` with `activePhases: ["exit"]` |
| motion path | `motion.path` transition track |
| grid snapping/rounding | `motion.path.sampling` |
| enter mask/wipe/iris/blinds | `visibility.*` transition track |
| style enter sweep | `style.*` transition track or phase-scoped graph node |
| typewriter/split-flap reveal | `content.*` transition track |
| fade in/out | `opacity.fade` transition track |
| crossfade/push/morph | `relation.*` transition track |
| canvas-aware color fade | `style.colorFade`, not `opacity.fade` |
| shadow geometry | `SceneElementSurface.shadow` / `ShadowSpec`, not transition |
| progress indicators/scanners | dwell graph/effect nodes with precise fields such as `fillProgress`, `scanProgress`, or `activation` |
| signal waveforms/mixes | `SignalExpressionSpec`, preview loopback expression, or authored signal expression value source |
| continuous procedural animation | source or dwell graph node, not transition by default |
| dwell filter/shader/sampler | graph node with `activePhases: ["dwell"]` |

Rule of thumb:

```text
Transitions handle the moments of change.
Sources and graph nodes handle ongoing generated visuals.
Scenes arrange surfaces.
Lifecycle controls progression.
Triggers request lifecycle actions.
```


## Second recipe-oracle findings

A later recipe review reinforced that the transition model should not absorb all
visual behavior. The new schema pressure is mostly outside transitions:

- static shadows belong on `SceneElementSurface.shadow` as typed `ShadowSpec`
  data, including paint expansion when support surfaces extend outside the source
  bounds;
- oscillator, ramp, and mixed demo values belong in typed `SignalExpressionSpec`
  or time-derived value sources, not opaque structured payloads;
- progress indicators and scanners are persistent dwell graph/effect nodes, not
  transitions; their inputs should use precise names such as `fillProgress`,
  `scanProgress`, `sweepProgress`, or `activation`;
- canvas-aware color fades are `style.colorFade` tracks using `StyleColorSource`,
  not opacity fades;
- modulo row/column styling belongs to scope algebra (`moduloRows`,
  `moduloColumns`), not transition vocabulary;
- radial twist and similar coordinate remaps are dwell samplers unless explicitly
  wrapped in a state-change interval;
- corner wipes and materialize masks are legitimate visibility transition tracks,
  but should use structured geometry/pattern fields rather than long opaque
  direction strings.

The second pass also clarified a restraint: the narrow `SignalExpressionSpec` and
`ShadowSpec` contract types are seed surfaces for validated recipes, not a mandate
to grow a full signal algebra or shadow subsystem immediately. Expand those only
when canonical examples prove the need.


## Completed v3.1 naming audit

The v3.1 contract schemas and primitive descriptor pack now reject ambiguous
wire/input names such as `type`, `target`, `source`, `progress`, `amount`,
`mode`, `direction`, `motion`, `speed`, `color`, `applyTo`, and `affect`, except
for deliberately narrow contexts such as gradient stop `color`.

Representative canonical names after the audit:

```text
BindingSpec.target/source/mode        → bindingTarget / valueSource / bindingMode
SourceSpec.source                     → sourceDescriptor
RecipeSceneElement.source             → sourceInstance
RecipeSceneElement.motion             → placementMotion
SceneElementVisibility.source         → predicateSource
TriggerCondition.source               → predicateSource
NodeOutputSpec.source                 → outputSource
ClockSpec.mode                        → clockMode
TransitionTrack.progress              → transitionProgress
TransitionVisibilityGeometry.mode     → cornerArcMode
ShadowSpec.color                      → shadowColor
StyleColorSource.color                → explicitColor
```

Descriptor inputs follow the same rule: use domain-specific names such as
`channelTarget`, `fillProgress`, `scanProgress`, `revealProgress`, `sweepRate`,
`glyphsPerSecond`, `targetColor`, `diffusionOrigin`, and `renderMode` rather than
generic one-word fields.

This is enforced by `v31_schema_and_descriptor_field_names_are_domain_specific`
in `crates/tui-vfx-contract/tests/test_schema_generation.rs`.

## Boundaries and prohibitions

Native transitions must not introduce:

- `CompositionSpec`, `ShaderLayerSpec`, or any other legacy-shaped execution DTO;
- bridge, shim, adapter, or translation layers;
- generic `type` fields in canonical transition JSON;
- app-specific semantics such as route names, Material component concepts,
  gt-design token names, or design-system policy;
- transition-shaped wrappers around all effects by default.

A generic effect track may be considered later, but it must remain canonical v3.1
and must not turn transitions into an arbitrary scheduler for legacy effect
chains.

## Open follow-ups

- Build 10–15 canonical examples before adding more schema concepts.
- Decide whether broader variant families are needed beyond transition variants:
  source variants, scene variants, and recipe-level capability variants.
- Decide how far source parameters should move from structured opaque payloads to
  typed bindable `ParamTree<ValueSource>` shapes.
- Continue recipe-oracle review with more examples before hardening every field
  name.
- Add visual reference examples for each transition track family.

<!-- <FILE>docs/arch/v31-native-transition-model.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.4.1</VERS> -->
