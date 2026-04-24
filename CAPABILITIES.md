<!-- <FILE>CAPABILITIES.md</FILE> - <DESC>Root V3 capability coverage guide for human and AI authors</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Summarize the as-built V3 pathway so authors can find primitive inventory, I/O chaining, bindings, procedural sources, and validation tooling.</WCTX> -->
<!-- <CLOG>0.1.0: add root V3 capabilities guide pointing to generated facts, the hand-maintained reference, and current V3 authoring/tooling contracts.</CLOG> -->

# tui-vfx capabilities

Use this root guide as the V3 capability orientation for human and AI authors.
These capabilities are gt-design-first infrastructure: scene rendering, VFX
composition, and recipe-runtime surfaces built for gt-design's palette, theme,
motion, animation, and VFX needs, while staying open and extensible for other
grid-based consumers. It complements, rather than replaces, the detailed
references:

- [`docs/CAPABILITIES_REFERENCE.md`](docs/CAPABILITIES_REFERENCE.md) — hand-maintained parameter reference and V3 pathway guidance.
- [`docs/generated/CAPABILITIES.md`](docs/generated/CAPABILITIES.md) — generated primitive inventory from `cargo xtask docs generate`.
- [`docs/design/tui-vfx-v3-INDEX.md`](docs/design/tui-vfx-v3-INDEX.md) — V3 planning, schema, I/O, tooling, migration, and active work index.
- [`../tui-vfx-recipes/docs/scene/AUTHORING_GUIDE.md`](../tui-vfx-recipes/docs/scene/AUTHORING_GUIDE.md) — sibling recipe-authoring ladder.

## V3 pathway capabilities

V3 is the compositional recipe pathway. The primitive catalog still matters, but
V3 recipes combine **ingredients** into ordered scenes and pipelines:

- masks, filters, samplers, shaders, style effects, and content transforms
- scene layers with `source`, `placement`, `surface`, `visibility`, and a
  layer-local `pipeline`
- procedural sources, assets, runtime bindings, tokens, and effect I/O links
- development tools such as validators, probes, traces, and preview players

Use **ingredient** for author-facing composition guidance. Use **capability** for
library surface area, planning coverage, and engineering inventory. Use
**development tools** for CLIs and validation/debugging surfaces.

## I/O chaining and composition

As built, V3 step I/O lets effects publish and consume per-frame hints inside a
single pipeline/layer. The first-release value kinds are `scalar`, `color`,
`vec2`, and `mask_bool`.

Supported authoring forms include:

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

Current composition rules:

- Use `sequence` when a later step must consume an earlier output.
- `parallel` branches receive the same pre-parallel input snapshot; sibling
  branch cross-feed is not part of the first release.
- Hints are per-frame/per-evaluation and do not persist across frames.
- Duplicate visible producers in the same scope are validator errors.
- Root pipelines, layer pipelines, and different scene layers do not implicitly
  exchange hints.
- First-class `io.outputs[].source` can re-emit a payload field after binding,
  which enables middle-of-chain filter/mask/shader/style-effect leaves.
- First-class `io.inputs[].input` may use dotted paths for wrapper seams such as
  `shader.intensity` in a spatial style effect.

Proven as-built examples include:

- `spatial_signal -> sine_wave amplitude -> diffusion intensity`
- `filter dim.factor -> sourced output -> downstream mask/filter/shader use`
- scene-layer-local chains in `scene.layers[].pipeline`

See [`docs/design/tui-vfx-v3-io-contract.md`](docs/design/tui-vfx-v3-io-contract.md)
for the canonical I/O contract and proof fixture paths.

## Runtime bindings, tokens, and assets

Host-provided values belong at the recipe contract boundary:

- `requires_bindings` declares live host state such as hover, selection,
  progress, focus, canvas color, or procedural parameters.
- `requires_tokens` declares theme/token dependencies.
- `requires_assets` declares external assets such as dotfield or image-like
  inputs.

Binding leaves such as `{ "binding": "wave_speed", "default": 1.0 }` are
resolved once per frame by the compiled V3 scene path before procedural sources
or pipeline effects use them. Keep runtime bindings distinct from step I/O:
bindings come from the host; I/O hints are produced by effects inside the same
pipeline evaluation.

## Procedural sources and scene content

V3 scene layers can use text, card, image-like, or procedural content sources.
Procedural sources are deterministic recipe ingredients when their visible
inputs are expressed through params, bindings, tokens, and assets. Current
source guidance includes:

- keep wall-clock and mutable hidden state out of authored procedural sources
- expose variable procedural inputs as declared params/bindings/assets
- label generated content with procedural/semantic roles so downstream scopes can
  target it
- prefer visible deterministic fallbacks for missing procedural registrations
- use the sibling procedural source catalog for exact stock source IDs and fields

## Implemented vs planned/deferred

Implemented/as-built V3 coverage includes same-pipeline/layer I/O chains,
sourced outputs, dotted wrapper input paths, binding-resolved procedural/source
params, scene-layer-local pipelines, generated primitive inventory, and the
current validator/probe/trace/player tooling surfaces.

Deferred or out of first-release scope:

- cross-layer or cross-pipeline hint exchange
- persistent multi-frame I/O hint state
- implicit `parallel` sibling cross-feeding
- value kinds beyond `scalar`, `color`, `vec2`, and `mask_bool`
- a full engine-wide causation chain in generated evidence surfaces
- public ingredient promotion for one-off recipe ideas that have not passed the
  V3 promotion ladder

## Development tooling

Use the sibling recipe tooling for authoring validation and visual evidence:

```bash
# Validate recipes and inspect normalized V3/contract usage.
pipeline-validator --strict-contracts <recipe.json>

# Probe rendered behavior and output summaries.
recipe-probe <recipe.json>

# Capture lifecycle/resolution/composition/pipeline traces.
tui-vfx-trace <recipe.json>
```

For exact commands and current flags, see:

- [`docs/tooling/INDEX.md`](docs/tooling/INDEX.md)
- [`../tui-vfx-recipes/docs/V3_TOOLING_COMMAND_REFERENCE.md`](../tui-vfx-recipes/docs/V3_TOOLING_COMMAND_REFERENCE.md)
- [`docs/PIPELINE_VALIDATOR_LLM_GUIDE.md`](docs/PIPELINE_VALIDATOR_LLM_GUIDE.md)
- [`docs/PIPELINE_TRACE_LLM_GUIDE.md`](docs/PIPELINE_TRACE_LLM_GUIDE.md)
- [`docs/PIPELINE_PROBE_LLM_GUIDE.md`](docs/PIPELINE_PROBE_LLM_GUIDE.md)

<!-- <FILE>CAPABILITIES.md</FILE> - <DESC>Root V3 capability coverage guide for human and AI authors</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
