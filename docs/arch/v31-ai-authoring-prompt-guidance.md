<!-- <FILE>docs/arch/v31-ai-authoring-prompt-guidance.md</FILE> - <DESC>Prompt guidance for AI-assisted v3.1 recipe authoring using engine-neutral animation/compositing vocabulary</DESC> -->
<!-- <VERS>VERSION: 0.3.1</VERS> -->
<!-- <WCTX>v3.1 schema audit: provide safe AI authoring anchors without importing app/design-system assumptions or legacy execution layers.</WCTX> -->
<!-- <CLOG>0.3.1: PATCH — clarify app-state wording so existing engine descriptor names can use focus/hover metaphors without importing UI policy.</CLOG> -->

# AI Authoring Prompt Guidance for v3.1 Recipes

## Purpose

This document stores reusable prompt language for AI-assisted tui-vfx recipe
authoring. It is meant for recipe-generation agents, documentation agents, and
future authoring tools that need a compact mental model for the v3.1 schema.

The goal is to help AI use familiar animation and compositing vocabulary without
smuggling in UI-framework, app-specific, design-system, CSS/DOM, Material, or
legacy tui-vfx assumptions.

## Recommended mental model

Do not say “this is basically After Effects” or “this is Material motion.” Those
anchors are too strong and can cause the model to import wrong assumptions.

Use a hybrid mental model instead:

```text
A grid-native After Effects / Flash / Director / game-animation-clip style
compositor: layers/surfaces + timelines + tracks + masks/mattes +
shaders/filters/samplers, expressed as strict JSON and executed directly by
tui-vfx.
```

Important clarifier:

```text
Use that comparison only to guide vocabulary. tui-vfx remains a platform-agnostic
grid animation and compositing engine, not an app framework and not a design
system.
```

## Compact five-home prompt block

Use this short block when an AI or human needs the working model without the full
architecture rationale:

```text
tui-vfx has five homes:

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

Decision tree:

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

Do not ask only “is this an effect or a transition?” Ask whether the behavior is
a bounded state change, ongoing processing, generated content, static attached
appearance, or a runtime value.

Do not overfit corpus edge cases. Promote a concept only when the compositor needs
that concept to execute common recipes directly. Otherwise keep it in descriptors,
structured payloads, examples, or future work.


## Full prompt block

```text
You are authoring for tui-vfx, a platform-agnostic grid animation and
compositing engine.

Closest mental model:
- Think of a compact After Effects / Flash / Director-style timeline system
  combined with game-engine animation clips.
- The engine works on grid surfaces, cells, glyphs, colors, roles, masks,
  samplers, filters, shaders, and transitions.
- It is not a UI component library, not Material Design, not CSS/DOM, not
  Ratatui-specific, and not app-specific.

Core concepts:
- scene: arranges source-produced surfaces on a grid.
- source: produces an initial surface, such as text, card, image, ANSI,
  asset-backed, or procedural content.
- lifecycle: recipe progression through enter, dwell, and exit.
- transition: a state/surface-change interval inside or across lifecycle phases.
- track: one executable visual concern inside a transition, such as visibility.*,
  opacity.*, motion.*, relation.*, style.*, or content.*.
- effect node: a phase-scoped shader/filter/sampler/mask/content transform,
  especially for persistent dwell behavior.
- signal/value source: runtime or preview value used to drive parameters.

Classification rule:
- Use a transition only for a state or surface change, such as enter, exit,
  reveal, hide, crossfade, push, iris, wipe, dissolve, typewriter-in, or
  split-flap-in.
- Use graph/effect nodes or procedural sources for ongoing dwell behavior, such
  as matrix rain, scanners, pulse waves, radial twist, progress bars, vignette
  breathing, or looping shaders.
- Use sources and scenes for structural content and placement.
- Use surface/shadow/style objects for static appearance.

Important constraints:
- Author shorthand may be canonicalized into canonical v3.1, but there is no
  runtime bridge and no legacy DTO.
- The compositor executes canonical v3.1 structures directly.
- Preserve author intent when expanding presets, but execute
  tracks/effects/sources natively.
- Avoid app-level semantics such as modal, primary action, navigation, Material,
  focused, hovered, selected, or design-system tokens unless they are opaque
  metadata supplied by the consumer or part of an existing engine descriptor id.
  Engine primitive names may use focus/hover words as visual-effect names, but
  the engine must not infer UI state policy from them.
- Prefer precise engine vocabulary: subject, scope, timing, easing, transition,
  track, source, scene, surface, signal, shader, filter, sampler, mask,
  visibility, opacity, relation, motion, content.
- Avoid ambiguous field names where possible. Prefer domain-specific names such
  as transitionProgress, phaseProgress, fillProgress, revealProgress, scanProgress,
  activation, channelTarget, focal, anchor, sweepProgress, sourceDescriptor,
  sourceInstance, predicateSource, and styleFade over generic target, source,
  progress, amount, mode, or type.
```

## Short operational reminder

```text
When generating tui-vfx recipes:
1. First decide whether the behavior is structural scene/source setup, a
   transition, a persistent dwell effect, or a signal-driven parameter.
2. Use transitions for enter/exit/reveal/hide/from-to changes.
3. Use graph/effect nodes for ongoing shaders, filters, samplers, masks, content
   transforms, and procedural animation.
4. Use signals/value sources for runtime-bound or preview-loopback values.
5. Keep output canonical v3.1 and platform-agnostic.
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


## Field-name guardrails

The prompt should steer AI toward precise field names but must not invent fields
that the current schema does not support. When the exact schema field is known,
use the schema field. When proposing future fields, keep names domain-specific:

| Prefer | Avoid when ambiguous |
| --- | --- |
| `transitionProgress`, `phaseProgress`, `revealProgress`, `scanProgress`, `fillProgress` | generic `progress` |
| `channelTarget` | `applyTo`, `affect` |
| `focal`, `anchor`, `emitter` | generic `origin` |
| `subject` for from/to/both/shared | generic `target` |
| `sourceDescriptor`, `sourceInstance`, `predicateSource`, `sampleSource`, `fieldSource`, `from`, `to` | generic `source` |
| `revealDirection`, `travelDirection`, `scanDirection` | generic `direction` |
| `blendMode`, `renderMode`, `glyphMode` | generic `mode` |
| `bandWidth`, `stripeWidth`, `tileWidth` | generic `width` when multiple widths exist |

## Relationship to official transition docs

This prompt guidance is downstream of the official transition model:

- `docs/arch/v31-native-transition-model.md`
- `docs/arch/v31-schema-boundary-north-star.md`
- `docs/arch/CLOCKS_AND_TIMING.md`
- `docs/VOCABULARY.md`

If this document conflicts with the schema or those architecture documents, the
schema and official architecture docs win. Update this prompt guidance rather
than treating it as a parallel schema authority.

<!-- <FILE>docs/arch/v31-ai-authoring-prompt-guidance.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.3.1</VERS> -->
