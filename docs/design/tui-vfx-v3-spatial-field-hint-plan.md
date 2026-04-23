<!-- <FILE>docs/design/tui-vfx-v3-spatial-field-hint-plan.md</FILE> - <DESC>Design plan for spatial signals, typed field hints, and first-class chained visual fields in V3</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Record the post-bridge architectural plan for moving from replay-seam cleanup into the deeper semantic work needed for Madeira-flag-class V3 recipes and future complex chained animations.</WCTX> -->
<!-- <CLOG>0.1.0: initial design note defining the recommended staged path: mixed-signals spatial-coordinate leaves, typed per-step field hints, layer-model threading, and field-driven shader/filter consumers.</CLOG> -->

# tui-vfx V3 spatial field + hint plan

This document defines the recommended **post-bridge** execution direction for V3.

The compiled V3 path has already crossed the important structural milestone of
having no remaining compiled-path replay callsites in the recipes-side source
surface. That means the main remaining work is no longer “remove one more hidden
bridge.”

It is now mostly about **semantics**:

- how one step can produce data for the next step
- how spatially varying authored signals are expressed
- how that data is threaded consistently through layers, preview, and runtime
- how downstream shader/filter consumers can reuse upstream work instead of
  recomputing or duplicating it

The immediate forcing function is the `madeira_flag` first-class V3 recipe, but
this plan is intentionally broader than a single showcase.

---

## 1. Design goal

The goal is not only to make `madeira_flag` look better.

The goal is to add a **foundational substrate** that supports:

- cloth / banner / flag motion fields
- wave-correlated lighting
- mask-correlated shading
- field-driven displacement
- downstream consumers that reuse upstream computed data
- richer chained animation systems in future V3 recipes

In short:

> use mixed-signals for **field generation**, and use V3 typed hints for
> **field propagation**.

---

## 2. Recommended path

Use **architecture C** implemented via **strategy D**.

### Architecture target (C)

Build a generic field/hint system:

- step outputs become typed per-frame hints
- downstream steps bind to typed upstream outputs
- shader/filter/sampler consumers read those outputs without recomputing them

### Delivery strategy (D)

Implement that architecture incrementally:

1. add the minimal missing mixed-signals spatial primitives
2. thread those through the layer/runtime model consistently
3. add typed per-step field hints
4. add one or two downstream field consumers
5. use `madeira_flag` as the first serious proof case
6. leave unrelated complex systems procedural until the foundation is stable

This is the strongest path to a reusable foundation without freezing progress
behind a giant all-at-once runtime rewrite.

---

## 3. Current capability survey

The current `tui-vfx` stack already provides a large amount of reusable power.

### 3.1 Existing strengths

#### Spatial shaders
The runtime already has many shading families that operate over the rendered
surface, including:

- gradients / bands / highlights
- focus fields
- concealed light / diffusion / edge sheen
- traveling bands
- stochastic textures
- chromatic/glitch shaders
- border / waypoint / route-oriented cues

This means **dynamic shading is already a strong foundation**. We do not need to
invent a new shading framework from scratch.

#### Filters
The filter layer already supports:

- color-domain postprocess work
- glyph-domain postprocess work
- texture / indicator / sweep effects
- several runtime-bound parameters

#### Samplers
The sampler layer already supports:

- coordinate displacement / resampling
- wave / ripple / fault / CRT / pendulum / gravity classes

#### Scene + layering
The recipes-side V3 path now supports:

- typed scene layers
- layer-local pipelines
- direct preview of compiled V3 scenes
- direct preview-path composition without recipes-side compositor replay

#### Runtime seams
The current codebase already has:

- compiled V3 plans
- ordered compiled-step execution slices
- role/channel/scope-aware handling
- validator checks for duplicate/missing hint names

So the problem is **not** a lack of family breadth.
The missing piece is the execution substrate between those families.

### 3.2 What is still missing

#### Missing in `mixed-signals`
The main missing signal primitives are spatial-coordinate leaves:

- `sample_norm_x`
- `sample_norm_y`
- `sample_cell_x`
- `sample_cell_y`

The current `SignalContext` already carries:

- `width`
- `height`
- phase / phase_t / loop_t / absolute_t
- `char_index`

but it does **not** currently carry per-cell coordinates.

#### Missing in V3 runtime
The main missing execution pieces are:

- typed per-step field outputs / hints
- typed downstream bindings to those outputs
- field-aware consumers that operate on those hints rather than recomputing
  upstream intent

---

## 4. The key architectural idea

The core idea is:

```text
mixed-signals
  -> generates a spatially varying field
V3 step output
  -> stores that field as a typed hint
later V3 steps
  -> consume the same field for displacement/shading/masking/etc.
```

This avoids two bad outcomes:

1. adding lots of bespoke one-off primitives only for a single showcase
2. duplicating the same authored signal math in multiple downstream steps

A good field/hint model lets a recipe say:

- Step A produces a displacement field
- Step B samples content using that displacement field
- Step C shades based on that same field

That is exactly the kind of chaining we want for the flag and for future complex
animations.

---

## 5. Proposed minimal mixed-signals additions

### 5.1 `SignalContext` additions

Add per-sample spatial coordinates:

- `cell_x: Option<u16>`
- `cell_y: Option<u16>`

And convenience builders:

- `with_cell_position(x, y)`
- optionally `with_sample_position(x, y)` if preferred naming is broader than cells

### 5.2 `SignalSpec` additions

Add spatial-coordinate leaves:

- `SampleNormX`
- `SampleNormY`
- `SampleCellX`
- `SampleCellY`

### 5.3 Semantics

Suggested meaning:

- `SampleNormX`:
  - `0.0` at left edge
  - `1.0` at right edge
  - `0.0` when width is degenerate
- `SampleNormY`:
  - `0.0` at top edge
  - `1.0` at bottom edge
- `SampleCellX` / `SampleCellY`:
  - raw integer cell coordinates as `f32`

These are intentionally small additions with high leverage.

---

## 6. Layer-model threading requirements

The new signals should not be special-cased in one seam.
They should be threaded consistently through the places that already build
`SignalContext`.

### 6.1 Procedural layers

`ProceduralCtx.signal_ctx` should carry:

- dimensions
- phase / phase_t / loop_t / absolute_t
- and, when a procedural source evaluates per-cell signals, the source should be
  able to derive a cell-local context via `with_cell_position`

### 6.2 Native replay helper

The native replay helper already evaluates many `SignalOrFloat` parameters.
Where evaluation is per-cell, it should be able to provide:

- dimensions
- current destination/local cell coordinates

This ensures spatial leaves behave consistently in:

- filters
- samplers
- other future parameterized replay tranches

### 6.3 Preview/runtime seams

Any preview/runtime path that already constructs `SignalContext` should keep
feeding the same timing fields, and should not invent alternate semantics for
spatial-coordinate leaves.

The key rule is:

> one meaning for spatial context, everywhere.

---

## 7. Proposed typed hint model

Hints should become typed, not “just names in JSON”.

### 7.1 Candidate first hint shapes

Start with a small set:

- `ScalarFieldHint`
- `Vec2FieldHint`
- maybe later `MaskFieldHint`
- maybe later `ColorFieldHint`

For the flag, the likely first real need is:

- `ScalarFieldHint` for a wave/displacement scalar
- or `Vec2FieldHint` if we want true displacement vectors from the start

### 7.2 Lifetime and visibility

Recommended defaults:

- per-frame / ephemeral
- visible only within the same pipeline / same layer unless explicitly exported
- duplicate producers in visible scope are validator errors unless explicitly
  qualified

Those defaults already align with the earlier reviewer lean in the open
questions doc.

---

## 8. First downstream consumers worth adding

### 8.1 Displacement-field consumer

A step should be able to consume a typed displacement field and use it to sample
source content.

This is better than encoding the displacement directly into a one-off sampler if
we want multiple downstream consumers to reuse the same field.

### 8.2 Field-correlated shading consumer

A shader/filter should be able to shade based on the same upstream field.

This is the important “dynamic shading is powerful” point:

- the field does not only move the content
- it can also light the content
- it can also gate effects
- it can also influence masks or postprocess work later

That gives one authored field multiple visual consumers.

---

## 9. Madeira flag application

The eventual richer `madeira_flag` should look like this conceptually:

1. **Generate field**
   - compound spatial-temporal sine graph
   - damped by normalized X
2. **Consume field for displacement**
   - sample the flag image using that field
3. **Consume same field for shading**
   - brighten peaks / darken troughs
4. **Keep other layers independent**
   - fireworks can remain procedural initially
   - text card can remain ordinary scene-layer content

This is why the field/hint substrate is worth building.
The flag is not the only use case.
It is just the clearest first one.

---

## 10. Immediate phased plan

### Phase 1 — mixed-signals spatial leaves

- extend `SignalContext`
- add `SampleNormX/SampleNormY/SampleCellX/SampleCellY`
- add tests

### Phase 2 — recipes-side threading

- pass spatial context through the existing layer/runtime seams
- update per-cell evaluation sites to use cell-local contexts where appropriate
- add focused tests in recipes-side proving signal evaluation sees coordinates

### Phase 3 — typed field hints

- add typed field/hint storage in the V3 runtime
- make validator/runtime agreement explicit
- support producer/consumer execution ordering within a bounded vertical slice

### Phase 4 — first real consumer pair

- one field producer
- one displacement consumer
- one shading consumer

### Phase 5 — restore richer `madeira_flag`

- move the flag from approximation subset back toward its intended semantics
- keep fireworks procedural until/if they justify a more general particle-field substrate

---

## 11. Recommendation summary

If the goal is the most extensible and foundational path, the recommendation is:

- **Architecture:** generic field/hint chaining
- **Delivery:** staged implementation beginning with mixed-signals spatial leaves
- **Design principle:** fewer bespoke primitives, stronger reusable execution substrate

In one sentence:

> add the minimal spatial leaves to `mixed-signals`, thread them consistently
> through the layer/runtime model, then build typed field/hint chaining so one
> step’s computed field can drive the next step’s displacement and shading.

That is the most foundational path for future complex V3 animations.

<!-- <FILE>docs/design/tui-vfx-v3-spatial-field-hint-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
