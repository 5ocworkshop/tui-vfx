<!-- <FILE>docs/design/tui-vfx-v3-io-contract.md</FILE> - <DESC>Phase 0 / 0A semantic lock for V3 producer/consumer I/O: author-facing vocabulary, value kinds, visibility rules, runtime integration posture, and prove-out obligations.</DESC> -->
<!-- <VERS>VERSION: 0.4.0</VERS> -->
<!-- <WCTX>Lock the first-release V3 I/O semantics before validator/runtime/family migration work fans out, and keep the contract aligned with landed as-built proofs.</WCTX> -->
<!-- <CLOG>0.4.0: record explicit sourced io.outputs for non-spatial leaves so filters, masks, shaders, and style effects can publish bound payload fields downstream. 0.3.0: record dotted first-class io.inputs for nested wrapper payloads such as spatial style-effect shader intensity. 0.2.0: add the as-built Phase 4 shared field-hint consumer proof and point at the recipe-side debug fixture/docs. 0.1.0: initial Phase 0 / 0A contract. Defines first-release author vocabulary, value kinds, sequence-vs-parallel visibility, per-frame lifetime, duplicate-producer rule, runtime-param integration posture, and stage-level debug-recipe obligations.</CLOG> -->

# tui-vfx V3 I/O contract

This document is the Phase 0 / 0A semantic lock for the V3-only producer / consumer rollout.

It answers the questions that must be settled before validator and runtime work spreads across the family surfaces.

It is not the full implementation design.
It is the **contract** the implementation must obey.

---

## 1. Scope

This contract applies to the **V3 path only**:

- V3 authoring
- normalized IR
- compiled plan
- direct V3 execution
- bridge-era compiled-plan builder only where it must stay semantically aligned during cutover

It does **not** require equivalent semantics on legacy V2 execution surfaces.

---

## 2. Author-facing vocabulary for the first release

The first release keeps the existing author-facing vocabulary:

- producer side: `emits_hint`
- consumer side: `binds`

Why:

- that is already the documented V3 posture in the schema/design docs
- it avoids needless churn while the substrate is still landing
- the real problem is that the fields are currently buried in opaque payloads, not that the words are wrong

Implementation rule:

- keep `emits_hint` / `binds` as the public JSON shape
- move them out of opaque payload-only handling into first-class V3 step-I/O structures internally
- do not rename to `inputs` / `outputs` in the first release unless a later explicit migration plan justifies it

---

## 3. First-release value kinds

The first release uses minimal **value kinds**, not effect-semantic domains.

Allowed first-release kinds:

- `scalar`
- `color`
- `vec2`
- `mask_bool`

Notes:

- `vec2` is the generic home for displacement-style values
- effect-specific meaning stays in the family contract, not in the substrate enum
- richer kinds such as `style_delta` are deferred until they earn their place through multiple real consumers

This keeps the substrate small and keeps downstream effect meaning where it belongs.

---

## 4. Visibility rules

### 4.1 Default visibility boundary

A hint is visible only within the **same pipeline** and **same layer**.

That means:

- root pipeline does not implicitly read layer-pipeline hints
- one scene layer does not implicitly read another layer's hints
- cross-layer or cross-pipeline exchange is out of scope for the first release

### 4.2 Sequence semantics

Within a `Sequence`, later siblings may read outputs produced by earlier siblings in the same sequence evaluation.

That is the main chaining path the first release supports.

### 4.3 Parallel semantics

Within a `Parallel`, branches do **not** implicitly cross-feed hints to sibling branches in the first release.

Each branch receives the same visible input snapshot from before the `Parallel` node begins.

That means:

- no branch can see outputs produced by a sibling branch during the same parallel evaluation
- any desired sharing must be expressed by restructuring into a sequence or a later explicit export/import design

Why:

- current runtime and builder behavior are not yet consistently locked here
- silent sibling cross-feeding is brittle and hard to validate
- the first release should prefer strictness over surprising coupling

### 4.4 Nested semantics

Nested trees follow the same rules recursively:

- sequence children see the accumulated visible store from earlier sequence work
- parallel children inherit a pre-parallel snapshot and do not cross-feed

---

## 5. Lifetime

Hints are **per-frame / per-evaluation ephemeral**.

They do not persist across frames.

They are recomputed during each pipeline evaluation and discarded afterward.

---

## 6. Producer conflict rule

If two visible producers emit the same hint name in the same visibility scope, that is a **validator error** in the first release.

There is no implicit "first wins" or "last wins" rule.

Explicit producer qualification is deferred unless later work proves it necessary.

---

## 7. Runtime integration posture

The rollout must not create a second parallel typed-value system.

There is already typed runtime-param machinery in the current stack.
The V3 hint/value substrate should **extend, wrap, or adapt** that machinery.

Rules:

- one canonical typed runtime value model
- one canonical resolution seam
- no family-local reinvention of value storage / lookup semantics

This is a steering-level SSOT requirement, not an implementation preference.

---

## 8. Family contract rule

The substrate only knows:

- value kind
- producer name
- consumer binding name
- visibility / lifetime rules

It does **not** know effect-specific meaning.

Effect meaning lives in the per-variant family contract matrix.

Examples:

- a sampler may emit a `vec2` named `displacement`
- a shader may consume a `scalar` named `intensity`
- a mask may emit a `mask_bool` named `visibility`

The substrate validates kind compatibility.
The family contract explains whether that binding is meaningful.

---

## 9. Debug-recipe prove-out obligations

Phase 0 / 0A are design-only stages.
They do **not** require shipping a new debug recipe by themselves.

Starting with the first implementation stage, every stage must land at least one `debug_recipes` prove-out that:

- shows I/O between **two effects or steps**
- makes the behavior visually obvious
- includes a description telling the viewer what to expect

Producer-only stages still need a bounded consumer prove-out fixture.
They are not allowed to ship as abstract producer work with no visible chain.

---

## 10. Immediate implementation consequences

The first implementation tranche should therefore do all of the following:

1. preserve `emits_hint` / `binds` as the public JSON vocabulary
2. lift them into first-class authoring / normalized / compiled structures
3. build a small value-kind enum
4. implement sequence-visible / parallel-isolated semantics
5. validate duplicate visible producers as hard errors
6. route runtime values through one shared typed substrate
7. ship one visible producer -> consumer debug fixture proving the path

---

## 11. Canonical as-built docs to keep aligned during rollout

As implementation lands, keep these hand-maintained docs aligned with the as-built system:

- `docs/design/tui-vfx-v3-io-contract.md`
- `docs/design/tui-vfx-v3-spatial-field-hint-plan.md`
- `docs/design/tui-vfx-v3-compiled-execution-plan.md`
- `docs/design/tui-vfx-v3-normalized-ir.md`

---

## 12. As-built runtime notes

### Diffusion intensity typed value support

The V3 material-light `diffusion` shader now uses the shared
`mixed_signals::SignalOrFloat` typed value substrate for `intensity` in the
runtime style model. JSON compatibility is preserved for numeric authored
values, while signal-valued recipe bindings can flow through the grouped V3
shader-family lowering seam into the executable shader. This is intentionally
source-breaking for Rust callers that construct `DiffusionShader` directly:
constructors should pass `SignalOrFloat::Static(value)` for fixed intensity.

Author-facing debug/key-parameter output formats static diffusion intensity as a
number and signal-backed intensity as `signal(<kind>)`, avoiding raw Rust debug
output in docs and inspection surfaces.

### Shared field-hint consumer chain

The recipe-side V3 direct path now has an as-built proof that one authored
`spatial_signal` hint can feed two downstream consumers in the same `Sequence`:

1. a `spatial_signal` sampler emits `wave_field`
2. a `sine_wave` sampler binds `amplitude` to `wave_field` for displacement
3. a `diffusion` shader binds `intensity` to `wave_field` for correlated
   material-light shading

Canonical proof artifacts live in the sibling recipe repo:

- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/complex/complex_field_hint_displace_shade.json`
- `/usr/projects/tui-vfx-recipes/docs/V3_FIELD_HINT_CONSUMERS.md`

This proof preserves the contract boundaries above: same pipeline, same layer,
per-frame ephemeral hints, `Sequence` feed-forward, and `Parallel` snapshot
isolation.

### Nested wrapper input paths

First-class `io.inputs` may use dotted payload paths for explicit wrapper seams.
The recipe-side direct path now proves this with a `style_effect` whose
`payload.type = "spatial"` wraps an executable shader under `payload.shader`:

```json
{
  "kind": "style_effect",
  "io": {
    "inputs": [
      { "input": "shader.intensity", "hint": "style_shade", "kind": "scalar" }
    ]
  },
  "payload": {
    "type": "spatial",
    "shader": { "type": "diffusion", "source": "right" }
  }
}
```

The binding fills `payload.shader.intensity` before the runtime spatial shader
is built. This keeps nested effect wrappers on the same producer/consumer
substrate as top-level leaves. Flat `binds` remains the preferred simple form
for direct leaf payload fields; dotted paths are for deliberate wrapper seams.

Canonical proof artifact:

- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/shaders/style_field_hint_spatial_shader.json`

### Sourced outputs for middle-of-chain leaves

First-class `io.outputs` may include a `source` path. The recipe-side direct
path reads that dot-separated path from the payload after input binding has
been applied and registers that value as the output hint. This is the generic
producer form for non-`spatial_signal` leaves that need to publish a value
downstream.

```json
{
  "kind": "filter",
  "io": {
    "inputs": [{ "input": "factor", "hint": "dim_factor", "kind": "scalar" }],
    "outputs": [
      { "hint": "shade_factor", "kind": "scalar", "source": "factor" }
    ]
  },
  "payload": { "type": "dim", "apply_to": "background" }
}
```

Canonical proof artifact:

- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/complex/complex_filter_reemits_field_hint.json`

## 13. Non-goals for the first release

Not part of the first-release contract:

- cross-layer hint exchange
- persistent multi-frame hint state
- implicit sibling cross-feeding in `Parallel`
- semantic value domains beyond the minimal set
- renaming the public author vocabulary without a separate migration plan

<!-- <FILE>docs/design/tui-vfx-v3-io-contract.md</FILE> - <DESC>Phase 0 / 0A semantic lock for V3 producer/consumer I/O</DESC> -->
<!-- <VERS>END OF VERSION: 0.4.0</VERS> -->
