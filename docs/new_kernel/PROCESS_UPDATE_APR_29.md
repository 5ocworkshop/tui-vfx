## Where we are relative to the original roadmap

The original plan was framed as a sequence of **contract locks**. We have not followed it linearly, because K2 pushed tooling, fixture mapping, and player evidence ahead of some unresolved schema decisions. That was productive: we now have visibility into the whole debug corpus instead of guessing. But the core roadmap still applies.

The current state is best described as:

```text
Surface / boundary / basic descriptor / player evidence: substantially achieved
Corpus mapping / QC / migration control surfaces: far ahead of the original plan
Schema-decision readiness: achieved for the active v3.1 debug corpus
Main blockers: descriptor/adapters, graph-player integration, scene-local runtime evidence, source/content migration, backend evidence, and GUI holdback signoff
```

The last explicit readiness number was:

```text
603 legacy debug records
249 schema-ready records after the first descriptor/adapter tranche
354 offender rows with --include-offenders
estimated schema readiness: 41.3%
canDeclareSchemaReady=true
```

The readiness number is now an implementation burn-down metric, not a schema-decision blocker. It means the remaining work is descriptor, adapter, fixture, backend, GUI-review, and evidence delivery.

## Roadmap status by phase

| Original phase                               |                            Current status | Assessment                                                                                                                                                                              |
| -------------------------------------------- | ----------------------------------------: | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **A — Semantic surface contract**            |                           Mostly achieved | Styled-cell evidence exists, roles/cells are represented, and player output is no longer just opaque strings. Still not compositor parity.                                              |
| **B — Sampled-source semantics**             |                        Partially achieved | Basic sampler/mask/filter evidence exists, but sampled-surface value sources and source-derived runtime values remain unresolved.                                                       |
| **C — Contract / engine boundary**           |                                  Achieved | `tui-vfx-contract`, `tui-vfx-player`, `tui-vfx-player-cli`, and `tui-vfx-player-ui` establish the clean-room boundary. Legacy remains oracle/evidence, not authority.                   |
| **D — Scope + write model**                  |                        Partially achieved | `all`, `role`, `rect`, `channel`, basic style scopes exist; modulo, outer/content, predicate/ref, runtime-bound cell coordinates, and element-local targeting are not fully locked.     |
| **D1 — Scene / element / layer semantics**   |            Newly elevated, active blocker | This was implicit in the original map but now must be explicit. Multi-element scene semantics are essential before full schema readiness.                                               |
| **D2 — Template composition**                |    Required, deferred above runtime layer | Templates must be supported, but as compile-time composition before strict validation/runtime. Design now; implementation later in compiler/template phase.                             |
| **E — Effect descriptor model**              |                Active, partially achieved | Descriptor pack exists and has expanded. Primitive adapter/field reports exist. Descriptor backlog remains, but the first descriptor/adapter tranche has started burning it down with canonical fixtures and green field-coverage gates.     |
| **F — Value + parameter contract**           |                  Major unresolved blocker | Binding execution, parameter overrides, signal loopbacks, sampled value sources, runtime defaults, and enum/numeric binding behavior are the biggest schema-readiness blocker.          |
| **G — Node graph + mini pipeline**           |                        Partially achieved | Sequence/parallel trees and I/O hints exist in evidence. But scene-local pipelines, branch isolation/merge, sourced outputs, and overlap semantics still need contract-level decisions. |
| **H — Strict recipe v3.1 schema + compiler** |                        Partially achieved | Strict canonical fixtures validate and render. But we cannot call the schema complete while source/content, runtime dynamism, and scene semantics are unsettled.                        |
| **I — Phase engine + trigger contract**      |                        Partially achieved | Bool dwell trigger is supported. Integer/text triggers, lifecycle timing, loopbacks, easing/motion, and trigger-vs-binding-vs-signal distinctions remain open.                          |
| **J — First real ports**                     | Ahead of original plan, but evidence-only | Many primitive adapters now render through player evidence, and unsupported count is down for the canonical corpus. This is useful but still not visual parity.                         |
| **K — Studio manifest + tooling**            |        Tooling far ahead; studio deferred | CLI reports, timeline/diff, fixture-QC, mapping, field coverage, adapter gaps, and GUI shell exist. Dynamic studio controls depend on F/H/K locks.                                      |
| **L — Legacy bridge + migration**            |                     Active and controlled | The full 603-record legacy debug corpus is mapped. Migration is no longer blind. We have explicit offender classes and holdback categories.                                             |
| **M — CI / release gates**                   |                        Partially achieved | Many report gates exist. Auto-generated schema/API/docs infrastructure still needs to become a formal release gate.                                                                     |

## What we have achieved

The project has moved from “can we validate a few v3.1 fixtures?” to a real migration control system.

We now have:

```text
contract-native validation
player render evidence
styled-cell visual-frame evidence
timeline and frame diff
primitive adapter gap reports
primitive field coverage reports
fixture-QC
inventory reports
migration-gap reports
corpus-wide migration mapping
schema-readiness ledger
offender ledger
Ratatui UI shell over player evidence
```

The most important achievement is not any one command. It is that we can now say, mechanically:

```text
This recipe is canonical and passing.
This recipe needs descriptor expansion.
This recipe needs source semantics.
This recipe needs runtime value/binding semantics.
This recipe needs scene-local pipeline semantics.
This recipe is backend-renderer holdback.
This recipe is oracle-only.
This recipe is duplicate/variant.
```

That lets us stop treating the 603 legacy debug recipes as a foggy backlog.

## What this now allows us to do

First, we can migrate **low-friction fixtures safely** without pretending everything is solved. A recipe can move when its descriptor exists, its fields are handled, the player adapter is honest, and fixture-QC stays green.

Second, we can **hold back problematic items explicitly**. Shadows, subcell shapes, oracle-only command capture, GUI-human-review conflict fixtures, and backend-renderer candidates do not need to block schema readiness forever if they are formally signed off as non-schema holdbacks.

Third, we can burn down blockers by **cluster**, not one file at a time. The offender ledger makes it possible to assign large lanes like runtime dynamism, source/content, scene-local pipelines, field coverage, and descriptor expansion.

Fourth, we can begin preparing for the studio. The current player reports already expose many ingredients a studio manifest will need: descriptor inputs, fields, bindings, phases, runtime controls, render hashes, diagnostics, and styled cells. But the actual auto-generated control surface should wait until value-source and binding semantics are locked.

## The main gap between the original plan and today

The original plan assumed we would finish core semantic locks before broad migration. In practice, we built tooling that revealed where the schema is still weak.

That is a good outcome, but it means we are in a hybrid state:

```text
Some later-phase tools exist.
Some earlier-phase locks are still incomplete.
```

The most dangerous mistake now would be to confuse “we have reports” or “the player can render evidence” with “the schema is locked.”

The second most dangerous mistake would be to keep re-reporting the same blockers instead of making the architectural decisions needed to clear them.

## The critical remaining locks

### 1. Runtime dynamism lock

This was the largest schema-decision blocker and is now an implementation/evidence backlog.

We need one coherent model for:

```text
requires_bindings
parameter defaults
runtime overrides
loopback values
signal generators
signal fallback
binding value types
binding resolution order
field-local value sources
sampled-surface value sources
enum bindings
numeric bindings
color bindings
```

The key decision should be:

```text
Bindings are host-facing named runtime inputs.
Loopbacks are preview/offline fallback producers.
Signals are time/sample-driven value producers.
ValueSource is the typed AST that lets descriptor inputs consume literal, binding, signal, parameter, or mapped values.
```

Without this, bindable rates, event-driven dwell, signal demos, pill-button progress, border-sweep position, focus-field center, sampled-surface dim factors, and runtime-bound scope coordinates all remain ambiguous.

### 2. Scene / element / layer lock

This must become a first-class part of schema finalization.

We need to lock:

```text
Element identity
Layer identity
z-order
placement
local coordinates
scene-global coordinates
clip / overflow
visibility predicates
element-local pipelines
layer-local pipelines
overlap rules
role propagation on overlap
transparent write vs skip over lower layers
diagnostic paths with element/layer identity
```

This is not optional. Multi-element scenes are already present in the debug corpus, and future studio workflows depend on element identity.

Important distinction:

```text
Role is semantic class.
Element is instance identity.
Do not overload RoleTag to mean element id.
```

### 3. Source/content identity lock

`source.text` is now the easy baseline, but the corpus needs more structure.

We need to decide:

```text
source.text
source.ansi
source.image
source.procedural
source.card
contentTransform effects
contentGenerator effects
glyph emitters
offline command-capture artifacts
```

The likely split is:

```text
Sources produce initial surfaces.
Content transforms modify or generate text/glyph content before pipeline effects.
Command capture is offline authoring/oracle evidence, not runtime execution.
```

That lets typewriter, marquee, split-flap, odometer, scramble, morph, wrap-indicator, glyph particles, ANSI layers, image layers, and procedural sources stop competing for the same schema slot.

### 4. Scope vocabulary lock

The former “unknown style” records revealed real scope needs:

```text
modulo
outer / inner
non-empty content
predicate/ref
runtime-bound cell coordinates
role within element
role across scene
channel within element
```

Some are low-risk:

```text
modulo rows/columns
outer band
non-empty content
```

Some are high-risk:

```text
predicate/ref registry
runtime-bound scope coordinates
```

The right move is to accept the simple geometric/content scopes and hold predicate/ref behind a registry decision.

### 5. Field coverage lock

The exact blockers are now known:

```text
gradient
applyTo
position
```

Likely decisions:

```text
gradient:
  accept as canonical structured gradient input, not as an opaque legacy blob.

applyTo:
  normalize as channel/application target vocabulary shared by filters/shaders/style effects.

position:
  treat as a typed ValueSource input for runtime-controlled progress/position, not a special ad-hoc binding field.
```

These should be closed soon because they block only a small number of rows but have high leverage.

### 6. Descriptor expansion lock

There are many descriptor-pack blockers, but descriptor expansion should happen after the value/source/scene locks stop moving.

Examples ready for structured tranches:

```text
mask.pathReveal
mask.materialize
mask.noiseDither
sampler.shredder
sampler.faultLine
sampler.radialTwist
filter.crt
filter.matrixRain
filter.patternFill
filter.pillButton
shader.revealWipe
shader.highlighter
shader.focusField
shader.glistenBand
shader.wayfindingNode
```

But each descriptor must declare real inputs, ranges, mutability, read/write channels, scope support, and handled fields. No green-by-label.

### 7. Holdback signoff lock

Some records should not block schema readiness if explicitly held back:

```text
backendRenderer: shadows, subcell shapes
guiHumanReview: overlap/conflict visual adjudication fixtures
oracleOnly: command-capture and loopback demo-only artifacts
duplicateOrVariant: known duplicate mask variants
```

We need formal owner signoff language:

```text
These are not schema blockers for v3.1 lock.
They are tracked as backend, GUI, oracle, or duplicate holdbacks.
They remain outside the migration-complete count until their later phase.
```

## Templates and where they fit

Templates are required, but they are not a runtime surface-engine feature.

They belong here:

```text
authoring/template layer
  -> deterministic template expansion
  -> strict canonical v3.1 recipe
  -> validation
  -> compiled runtime graph
```

The final runtime should not know whether a recipe came from a template.

So relative to the original roadmap:

```text
Template design belongs before final recipe/compiler lock.
Template implementation belongs after strict canonical recipe shape is stable.
Template references must not remain in the final canonical recipe.
```

The immediate action is not to implement templates now. The immediate action is to ensure the schema/compiler architecture reserves a clean place for:

```text
templates
presets
mixins/fragments
slots
explicit overrides
sealed fields
deterministic expansion diagnostics
```

## Studio controls and where they fit

The reach goal is still valid:

```text
Load a recipe.
Auto-generate sliders/input boxes from the recipe/descriptor/manifest.
Adjust values live.
See updated output.
```

But this is Phase K, and it depends heavily on the unresolved Phase F value contract.

The studio manifest can only be trustworthy after we know:

```text
Which inputs are parameters?
Which inputs are runtime bindings?
Which inputs are signals?
Which inputs are editable controls?
Which values are enum/number/color/text/bool?
What are their ranges/defaults?
What phase/runtime mutability do they support?
What node/input does each control affect?
```

Once the runtime dynamism lock lands, a minimal studio manifest becomes realistic.

## What “completion of the plan” means now

For this debug-recipes schema-readiness phase, completion should mean:

```text
Every debug_recipes record is either:
  canonical v3.1 and fixture-QC passing,
  or explicitly assigned to a non-schema holdback bucket,
  or queued behind an accepted descriptor/source/runtime/scene decision with no vague ownerAudit/unknown.
```

For full v3.1 schema readiness, completion should mean:

```text
No unresolved schemaDecision/sourceDecision/sceneDecision/valueSourceDecision/fieldCoverageDecision rows remain.
All remaining descriptor-only gaps are either accepted expansion work or signed holdbacks.
Schema/API/docs generation is wired into CI.
Templates are designed as compile-time composition.
Scene/element/layer semantics are part of the contract.
Studio manifest generation has a stable descriptor/value foundation.
```

## Critical next steps

The highest-leverage sequence is:

1. **Lock runtime dynamism.** Decide bindings, parameters, signals, loopbacks, value sources, field hints, and resolution order. This unblocks many separate-looking families at once.

2. **Lock scene/element/layer semantics.** Multi-element scenes must become part of the schema finalization path, not a future afterthought.

3. **Lock source/content identity.** Separate sources from content transforms and define ANSI/image/procedural/offline artifact policy.

4. **Maintain field-coverage closure.** `gradient`, `applyTo`, and `position` are accepted; keep future descriptor additions honest by proving every authored field in descriptor/player coverage.

5. **Accept low-risk scope vocabulary.** Modulo, outer/inner, and non-empty/content scopes should be settled; predicate/ref should be deferred unless a registry policy is approved.

6. **Sign off holdbacks.** Backend renderer, oracle-only, GUI human-review, duplicate/variant, and command-capture categories should stop counting as schema uncertainty once accepted.

7. **Run descriptor expansion in tranches.** After the above locks, add filters/masks/samplers/shaders/styles in clusters, with field coverage and fixture-QC as gates.

8. **Add schema/API doc generation as a release gate.** Rustdoc + schemars output should become an artifact, not just an aspiration.

9. **Then move toward studio manifest/control generation.** This becomes realistic once descriptor inputs and runtime mutability are trustworthy.

## Bottom line

We are no longer at the “rough roadmap” stage. We have built a working migration observatory and evidence pipeline.

But the plan is not complete because the most important locks remaining are semantic, not mechanical:

```text
runtime dynamism
scene composition
source/content identity
scope vocabulary
template expansion boundary
studio manifest/control foundation
```

The next phase should stop re-listing these as blockers and make the architectural decisions needed to retire them.
