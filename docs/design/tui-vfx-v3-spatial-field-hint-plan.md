<!-- <FILE>docs/design/tui-vfx-v3-spatial-field-hint-plan.md</FILE> - <DESC>Design plan for spatial signals, typed field hints, and first-class chained visual fields in V3</DESC> -->
<!-- <VERS>VERSION: 0.10.0</VERS> -->
<!-- <WCTX>Keep the spatial field/hint plan aligned with the as-built V3 timing model and landed shared-consumer proofs, so the next tranche starts from the remaining showcase/runtime gaps instead of repeating complete field-hint work.</WCTX> -->
<!-- <CLOG>0.10.0: record the scene-layer-local proof where spatial_signal, filter sourced output, and shader consumer run inside one scene layer pipeline. 0.9.0: record the mask consumer proof where a sourced output drives checkers.cell_size. 0.8.0: record the sourced-output proof where a filter consumes a field, re-emits its bound payload field, and drives a downstream shader. 0.7.0: record the nested style-effect shader consumer proof where dotted io.inputs bind an upstream field hint into payload.shader.intensity. 0.6.0: record the first Phase 5 Madeira asset-contract slice: braille flag artwork now resolves through requires_assets and has a scene debug fixture proving the reusable token path. 0.5.0: record the Phase 4 shared field-hint proof where one spatial_signal drives both displacement and field-correlated shading through a sequenced recipe-side debug fixture. 0.4.0: record that the first spatial-coordinate leaves and the basic cell-position threading work are already landed across mixed-signals and the current runtime seams, and shift the next active tranche toward typed field hints and real producer/consumer runtime support. 0.3.0: clarify the as-built timing model so docs distinguish normalized phase/loop progress from monotonic elapsed time and state that cadence-driven motion consumes elapsed time. 0.2.0: add the two-basis spatial model (cell basis vs surface/frame basis), explain why optical falloff consumers like vignette should not redefine the existing sample_radius leaf, and propose companion surface-space leaves as the next foundational mixed-signals extension. 0.1.0: initial design note defining the recommended staged path: mixed-signals spatial-coordinate leaves, typed per-step field hints, layer-model threading, and field-driven shader/filter consumers.</CLOG> -->

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

The canonical external reference implementation for this work is:

- `/usr/projects/madeira-flag`
- especially `/usr/projects/madeira-flag/examples/demo.rs`

That demo should be treated as the ground-truth experiential target for the
first full field/hint showcase pass.

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

Use **strategy D as the near-term execution goal**, explicitly aiming to
graduate into **full architecture C** once the substrate is proven.

### Architecture target (C)

Build a generic field/hint system:

- step outputs become typed per-frame hints
- downstream steps bind to typed upstream outputs
- shader/filter/sampler consumers read those outputs without recomputing them

### Near-term execution strategy (D)

Implement that architecture incrementally:

1. add the minimal missing mixed-signals spatial primitives
2. thread those through the layer/runtime model consistently
3. add typed per-step field hints
4. add one or two downstream field consumers
5. use `madeira_flag` as the first serious proof case
6. leave unrelated complex systems procedural until the foundation is stable

This is the strongest path to a reusable foundation without freezing progress
behind a giant all-at-once runtime rewrite.

### What “fully vetted” means before moving from D to full C

The system should not be considered ready for the full-C rollout until it has
proven all of these inside the bounded D implementation lane:

- mixed-signals spatial-coordinate leaves are implemented and stable
- layer/runtime seams all carry consistent spatial signal context
- one typed field/hint producer works end to end
- at least one displacement consumer and one shading consumer bind to the same
  upstream field
- a first-class consumer (`madeira_flag`) uses that field/hint path in a way
  that is meaningfully closer to its intended semantics than the current
  approximation rewrite

Once those are true, the work should continue into **full C**, meaning:

- field/hint chaining is treated as a general V3 execution substrate
- more downstream consumers are migrated to read typed hints
- one-off approximations are retired where the new substrate can replace them

So the real sequencing is:

```text
near-term D (bounded validation lane)
  -> vet the substrate
    -> continue into full C
```

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

The first missing signal primitives were spatial-coordinate leaves:

- `sample_norm_x`
- `sample_norm_y`
- `sample_cell_x`
- `sample_cell_y`

The current `SignalContext` already carries:

- `width`
- `height`
- `phase`
- normalized progress values (`phase_t`, `loop_t`)
- monotonic elapsed time (`absolute_t`)
- `char_index`

The distinction is load-bearing:

- `phase_t` and `loop_t` are normalized progress values and may reset when a
  phase or loop restarts
- `absolute_t` is monotonic elapsed time from playback start and does not reset
  at loop boundaries

but it did **not** initially carry per-cell coordinates.

That first tranche is now landed.

The next missing conceptual piece is more subtle:

- a second spatial basis for **continuous frame / surface geometry**

The current cell-space leaves are correct for:

- radar
- ripple / pulse-wave
- many authored per-cell field graphs

But adoption work against vignette/optical-falloff consumers showed that some
effects want a different basis:

- not “where is this sampled cell on the discrete lattice?”
- but “where is this point on the continuous surface/frame geometry?”

That distinction should be treated as foundational rather than as one
effect-local quirk.

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

### 5.4 A new discovery: there are two valid spatial bases

The current audit/adoption pass found that we should explicitly model **two**
spatial bases in `mixed-signals`:

#### A. Cell basis

This is the basis the current leaves use.

Good for:

- `sample_norm_x`
- `sample_norm_y`
- `sample_cell_x`
- `sample_cell_y`
- `sample_centered_x`
- `sample_centered_y`
- `sample_radius`
- `sample_angle`

Semantics:

- tied to the discrete sampled cell lattice
- normalized against `0 .. width-1` / `0 .. height-1`
- ideal for authored field graphs, displacement fields, sweeps, and radar-like
  or ripple-like motion

#### B. Surface / frame basis

This is the next foundational extension we should add.

Recommended companion leaves:

- `sample_surface_centered_x`
- `sample_surface_centered_y`
- `sample_surface_radius`

Possible later companion:

- `sample_surface_angle`

Semantics:

- tied to the continuous frame/surface geometry
- center derived from the frame rather than from the edge-inclusive sampled
  cell lattice
- ideal for optical falloff, diffusion-style lighting, spotlight illumination,
  and vignette-like effects

### 5.5 Why this should be additive, not a semantic redefinition

We should **not** redefine the existing `sample_radius` leaf.

Why:

- it already has coherent cell-space meaning
- it is already useful for real downstream uses
- changing it would silently break effects that are correctly using the
  existing basis

The better design is:

- keep the current cell-basis leaves as they are
- add explicit surface/frame-basis companion leaves
- let downstream effects choose the correct basis intentionally

That gives us a cleaner and more maintainable substrate than trying to force
one “radius” primitive to satisfy two different geometric models.

### 5.6 Why these first spatial leaves were added

These first leaves are not “features for feature's sake.”
They were added because they unlock a broad class of reusable authored effects
with a very small amount of new substrate:

- `sample_norm_x` / `sample_norm_y`
  - useful for edge-to-edge waves
  - useful for progressions across a surface
  - useful for directional reveals and sweeps
  - useful for “strong on one side, weak on the other” motion fields
- `sample_cell_x` / `sample_cell_y`
  - useful for deterministic grid-aware effects
  - useful for game-like tile/cell logic
  - useful for procedural noise layouts that need stable integer coordinates

In practical terms, these make it possible to author:

- cloth / banner / flag waves
- lighting falloff tied to position
- procedural sweeps and beats across a HUD or UI shell
- simple game-field effects without custom closures

They were chosen because they are:

- foundational
- broadly reusable
- small enough not to bloat `mixed-signals`
- directly useful in both application TUI work and lightweight game/story work

### 5.7 Additional spatial leaves now chosen for the 0.2.3 substrate

To support the current post-bridge field/hint work and the near-term showcase
goals, the next spatial leaves are now part of the intended substrate:

- `sample_centered_x`
- `sample_centered_y`
- `sample_radius`
- `sample_angle`

Those would primarily help with:

- radial shockwaves
- spotlight / cone-like lighting
- orbit / spiral / directional field logic
- centered composition beats for cinematic or game-style motion

These are still considered foundational rather than “feature bloat” because:

- `sample_centered_*` supports symmetry-based motion and centered composition
  beats
- `sample_radius` supports shockwaves, radial reveals, and spotlight falloff
- `sample_angle` supports radar sweeps, rotational fields, and animated light
  paths

The current recommendation is therefore:

- land **all three tiers** of spatial leaves in `mixed-signals`
- then use those leaves to simplify bespoke downstream spatial math where that
  math is clearly reusable

And after that:

- add the explicit **surface/frame-basis** companions for optical/illumination
  consumers
- then continue the downstream audit with both bases available

### 5.8 Spotlight / cone feature candidate

There is one higher-level candidate worth keeping explicitly in view:

- a **spotlight / cone field primitive**

Why it matters:

- it is an impressive downstream demo candidate
- it serves application TUIs as well as lightweight game/story work
- it benefits from animated origin + angle changes
- it is a natural consumer-facing way to express “swept light path” behavior

The intended downstream use case is:

- a GT-Design surface that approximates a moving spotlight on the screen
- the spotlight can be placed on-screen or off-screen
- rotating the spotlight changes the direction/path of the emitted cone
- downstream shading / masking / emphasis stages respond to that moving cone

This should be treated as a **design target** in the current plan.

The recommended sequencing is:

1. land the lower-level spatial leaves (all three tiers)
2. vet typed field/hint chaining through D
3. continue through full C
4. use the resulting substrate to implement a spotlight/cone showcase in a
   downstream application
5. only then decide whether a dedicated spotlight/cone primitive has earned a
   first-class place in `mixed-signals`

Why this is still the right order:

- the lower-level leaves may already make spotlight/cone authoring ergonomic
  enough
- if they do not, the spotlight/cone primitive becomes a justified
  higher-level addition rather than speculative surface area

So the spotlight/cone remains an explicit **showcase target** and a likely
future justification point, but not yet an automatic primitive commitment.

---

## 5.9 Proposed schema changes

The schema changes should be additive, tree-shaped, and ergonomic.

### A. Signal graph leaves

The author-facing `SignalGraphNode` surface should grow the spatial-coordinate
leaves already drafted conceptually:

- `sample_norm_x`
- `sample_norm_y`
- `sample_cell_x`
- `sample_cell_y`
- `sample_centered_x`
- `sample_centered_y`
- `sample_radius`
- `sample_angle`

And the next additive family should be the companion surface-space leaves:

- `sample_surface_centered_x`
- `sample_surface_centered_y`
- `sample_surface_radius`

These should remain **leaves**, not wrappers, so they compose naturally inside
existing `add` / `multiply` / `mix` graphs.

### B. Step-output hint fields

The current draft already points in the right direction:

- producer-side: `emits_hint`
- consumer-side: `binds`

That should remain the ergonomic surface.

Recommended near-term D shape:

- producer declares `emits_hint: <name>`
- consumer declares `binds: { <input_name>: <hint_name> }`
- validator enforces same-pipeline visibility and duplicate-producer errors

### C. Typed runtime interpretation behind the same schema

The _schema_ does not need to expose runtime storage classes like
`ScalarFieldHint` or `Vec2FieldHint` directly yet.

Instead:

- keep the author-facing fields small and ergonomic
- let the runtime/type system infer or assign the underlying typed hint class

That preserves a clean tree while still enabling a strong internal
implementation.

### D. Consumer payload ergonomics

Consumers should follow the existing leaf pattern:

```text
step
  -> kind
  -> scope
  -> payload
```

not a special “graph stage” wrapper.

So a future displacement-aware shader should still look like a normal shader
leaf whose payload includes:

- `type`
- `binds`
- family-specific payload fields

This keeps the tree consistent with the rest of V3.

### E. Showcase recipe posture

First-class recipes like `madeira_flag` should:

- use the currently supported runtime subset in executable payloads until the
  richer field/hint substrate lands
- preserve the richer intended semantics in `authoring_notes`
- be upgraded back to the fuller chained semantics once the D substrate is
  vetted and the work proceeds into full C

---

## 6. Layer-model threading requirements

The new signals should not be special-cased in one seam.
They should be threaded consistently through the places that already build
`SignalContext`.

### 6.1 Procedural layers

`ProceduralCtx.signal_ctx` should carry:

- dimensions
- `phase`
- normalized progress values (`phase_t`, `loop_t`)
- monotonic elapsed time (`absolute_t`)
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

That includes timing semantics. Preview and runtime paths should expose the
same two timing views:

- normalized phase/loop progress for lifecycle-aware interpolation and
  phase-gated execution
- monotonic elapsed time for cadence-driven motion and signal consumers

Cadence-driven families such as scanner sweeps or BPM-driven oscillation should
consume elapsed time (`absolute_t`), not reset-on-loop normalized progress.
Using loop progress for cadence hides discontinuities until the loop boundary
and then reintroduces them as visible timing jumps.

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

**Status: landed.**

- `SignalContext` now carries `cell_x` / `cell_y`
- the first coordinate leaves are present in `mixed-signals`
- focused tests already prove those leaves read the per-sample context correctly

### Phase 2 — recipes-side threading

**Status: materially landed for the current direct path.**

- cell-position threading already flows through the current runtime/evaluation seams
- multiple compositor/style/recipes consumers already call `with_cell_position(...)`
- focused tests now prove that signal evaluation can see per-cell coordinates on the current path

So the next missing work is no longer “add basic spatial leaves” or “thread
cell coordinates once.” The next missing work is to make that substrate useful
for step-to-step reuse.

### Phase 3 — typed field hints

**Status: first vertical slice landed.**

- add typed field/hint storage in the V3 runtime
- make validator/runtime agreement explicit
- support producer/consumer execution ordering within a bounded vertical slice

The first bounded runtime slice now exists on the direct compiled V3 path:

- compiled leaf steps retain `emits_hint` / `binds` metadata
- a pure `spatial_signal` producer can emit a named hint
- a later consumer can bind that hint into a real runtime field
- the current proof slice is a sequenced producer → `dim` filter consumer on
  the direct path

This is intentionally narrow. It proves that step-to-step hint wiring is now
operational for one bounded pair without claiming that the whole typed field
architecture is complete.

### Phase 4 — first real consumer pair

**Status: generic shared field-hint pair landed; showcase-specific richer
consumers remain open.**

- one field producer ✅
- one real downstream consumer ✅
- displacement consumer ✅ (`sine_wave.amplitude` bound to a shared field)
- field-correlated shading consumer ✅ (`diffusion.intensity` bound to the same field)
- nested wrapper consumer ✅ (`style_effect` spatial shader binding `shader.intensity` through dotted `io.inputs`)
- middle-of-chain non-spatial producer ✅ (`filter` re-emitting a bound payload field through `io.outputs[].source`)
- mask consumer ✅ (`checkers.cell_size` bound from a sourced filter output)
- scene-layer-local I/O chain ✅ (same sourced-output substrate inside one `scene.layers[].pipeline`)
- showcase/braille-dotfield consumers remain follow-up work

As-built proof artifacts:

- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/complex/complex_field_hint_displace_shade.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/shaders/style_field_hint_spatial_shader.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/complex/complex_filter_reemits_field_hint.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/complex/complex_filter_to_mask_sourced_output.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/scene/scene_layer_io_filter_shader.json`
- `/usr/projects/tui-vfx-recipes/docs/V3_FIELD_HINT_CONSUMERS.md`

The canonical chain is `Sequence[spatial_signal producer, sine_wave sampler,
diffusion shader]`: the sampler consumes the shared field for displacement, and
the shader consumes the same field for correlated shading after the sampler has
modified the output.

A second proof exercises a wrapper seam: `Sequence[spatial_signal producer,
style_effect spatial shader]` binds `style_shade` into `payload.shader.intensity`
using `io.inputs[].input = "shader.intensity"`. The recipe lives under shader
debug fixtures because the spatial style-effect wrapper lowers into the shader
runtime stage, but it preserves style/effect wrappers on the same I/O contract.

A third proof exercises middle-of-chain publishing: `Sequence[spatial_signal
producer, dim filter, diffusion shader]` binds `dim_factor` into the filter's
`payload.factor`, publishes that field as `shade_factor` via
`io.outputs[].source = "factor"`, then binds `shade_factor` into shader
`intensity`. This removes the earlier sampler-only producer limitation for
explicitly sourced non-spatial leaves.

A fourth proof covers masks: `Sequence[dim filter, checkers mask]` publishes
`payload.factor` as `checker_size`, then binds that value into
`checkers.cell_size`. This keeps mask consumers on the same sourced-output /
first-class-input path without introducing a mask-specific binding mechanism.

A fifth proof covers scene-layer-local execution: inside one
`scene.layers[].pipeline`, a `spatial_signal` publishes `layer_field`, a dim
filter consumes it into `payload.factor` and publishes `layer_shade`, and a
diffusion shader consumes `layer_shade` into `intensity`. The visibility remains
same-layer; this is not a cross-layer exchange contract.

### Phase 5 — restore richer `madeira_flag`

**Status: first asset-agnostic Madeira slice landed; richer showcase parity still
in progress.**

- move the flag from approximation subset back toward its intended semantics ✅
  first step: contract-backed braille-dotfield artwork
- keep fireworks procedural until/if they justify a more general particle-field substrate
- keep building toward richer Madeira parity on top of the reusable
  `braille_flag_field` path instead of re-embedding artwork in Rust

As-built proof artifacts:

- `/usr/projects/tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/scene/scene_braille_flag_asset_token.json`
- `/usr/projects/tui-vfx-recipes/docs/scene/PROCEDURAL_SOURCES.md`

### Phase 6 — showcase parity and demonstration

After the generic Phase 4 proof, the project should explicitly re-create the
`/usr/projects/madeira-flag` demo as a first-class V3 recipe showcase using the
new capabilities:

- spatial signal field generation
- typed hint/field propagation
- displacement from upstream fields
- field-correlated shading
- scene-layer composition

The point is not only compatibility.
It is also to create a **showcase recipe** that demonstrates why the new
substrate matters and gives the project a strong “look what V3 can do now”
reference artifact.

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
<!-- <VERS>END OF VERSION: 0.10.0</VERS> -->
