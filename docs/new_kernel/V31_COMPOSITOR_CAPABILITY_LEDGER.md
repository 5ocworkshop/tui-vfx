<!-- <FILE>docs/new_kernel/V31_COMPOSITOR_CAPABILITY_LEDGER.md</FILE> - <DESC>Compositor-first capability ledger for v3.1 recipe migration</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>Ralph compositor-first migration: inventory compositor-native primitives before changing v3.1 recipes or player boundaries.</WCTX> -->
<!-- <CLOG>0.3.0: PATCH — mark mask.radial lowerer/player-stage cleanup as completed for the second compositor-first slice.
0.2.0: mark mask.cellular descriptor/lowerer/player-stage cleanup as completed for the first compositor-first slice.
0.1.0: INIT — record compositor mask/filter/sampler/shader surfaces and first migration priorities.</CLOG> -->

# v3.1 Compositor Capability Ledger

This ledger starts migration from the renderer that owns effect semantics. Work backward from these compositor surfaces into descriptors, backend lowering, and v3.1 recipe JSON. Do not add strict-native semantics to `tui-vfx-player` to make recipes appear correct.

## Source-of-truth files

| Surface | Compositor path | Notes |
| --- | --- | --- |
| Masks | `crates/tui-vfx-compositor/src/types/cls_mask_spec.rs` | `MaskSpec` is the native mask target for v3.1 mask graph nodes. |
| Filters | `crates/tui-vfx-compositor/src/types/cls_filter_spec.rs` | `FilterSpec` is the native frame-filter target. |
| Samplers | `crates/tui-vfx-compositor/src/types/cls_sampler_spec.rs` | `SamplerSpec` is the native coordinate-sampler target. |
| Shader layers | `crates/tui-vfx-compositor/src/pipeline/cls_shader_layer_spec.rs` | `ShaderLayerSpec` wraps `SpatialShaderType` and `StyleRegion`. |
| Spatial shaders | `crates/tui-vfx-style/src/models/cls_spatial_shader_type.rs` | Current executable shader/style surface used by compositor shader layers. |
| v3.1 descriptors | `descriptors/v3.1/packs/primitive.json` | Author-facing primitive vocabulary that should map to compositor concepts. |
| Compositor lowerer | `crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs` | Boundary layer that maps sampled v3.1 graph nodes to compositor plans. |

## Mask surface

Compositor `MaskSpec` variants:

```text
mask.none
mask.wipe
mask.dissolve
mask.checkers
mask.blinds
mask.iris
mask.diamond
mask.noiseDither
mask.materialize
mask.pathReveal
mask.radial
mask.cellular
```

Descriptor aliases / derived ids:

```text
mask.materializeCorner -> should lower to existing materialize/radial-origin semantics
mask.wipeCorner        -> should lower to existing wipe direction semantics
```

First mask priorities:

| v3.1 id | Compositor target | Current action |
| --- | --- | --- |
| `mask.cellular` | `MaskSpec::Cellular { pattern, seed, cell_count }` | Done in first slice: descriptor exposes `pattern`/`seed`/`cellCount`, lowerer emits `MaskSpec::Cellular`, and backend source-stage cellular rendering was deleted. |
| `mask.radial` | `MaskSpec::Radial { origin, soft_edge }` | Done in second slice for current `center` descriptor support: lowerer emits `MaskSpec::Radial` and backend source-stage radial rendering was deleted. |
| `mask.noiseDither` | `MaskSpec::NoiseDither { seed, matrix, chunk_size }` | Verify descriptor exposes matrix/chunk size and lowerer maps without player semantics. |
| `mask.pathReveal` | `MaskSpec::PathReveal { path, soft_edge }` | Verify descriptor can express V2 path payload, not just a wipe-like direction. |
| `mask.centerWipeFadeModal` V2 recipe | likely `mask.wipe` + style fade | Missing from v3.1 hierarchy; migrate after direct mask mappings are clean. |

## Filter surface

Compositor `FilterSpec` variants:

```text
filter.dim
filter.invert
filter.tint
filter.fadeToCanvas
filter.vignette
filter.crt
filter.patternFill
filter.greyscale
filter.brailleDust
filter.charsetNoise
filter.animatedGlyphRamp
filter.matrixRain
filter.interlaceCurtain
filter.motionBlur
filter.colorBridgedShade
filter.subPixelBar
filter.subcellLight
filter.subCellShake
filter.rigidShake
filter.hoverBar
filter.underlineWipe
filter.bracketEmphasis
filter.dotIndicator
filter.edgeGrow
filter.pillButton
filter.glistenSweep
filter.kittScanner
filter.shadeScanner
filter.glyphStyle
filter.scalarFieldGlyph
filter.glyphTimeline
```

Descriptor gaps observed from inventory:

```text
filter.glyphTimeline exists in compositor but is not currently in the v3.1 primitive descriptor pack.
filter.none exists as compositor no-op but is not currently a descriptor effect id.
```

## Sampler surface

Compositor `SamplerSpec` variants:

```text
sampler.none
sampler.sineWave
sampler.ripple
sampler.shredder
sampler.faultLine
sampler.crt
sampler.crtJitter
sampler.bounce
sampler.pendulum
sampler.gravity
sampler.radialTwist
```

Descriptor gap / extra:

```text
sampler.none exists as compositor no-op but is not currently a descriptor effect id.
sampler.spatialSignal exists in descriptor vocabulary without a direct `SamplerSpec` variant.
```

## Shader/style surface

Compositor shader layers execute `ShaderLayerSpec { shader: SpatialShaderType, region: StyleRegion }`.

Current `SpatialShaderType` variants include:

```text
LinearGradient
BarberPole
Radar
Orbit
BorderSweep
Highlighter
Reflect
GlistenBand
GlitchLines
NeonFlicker
PulseWave
TerminalWater
TerminalFire
RadialSpiral
TracePropagation
TracePath
FocusedRowGradient
RevealWipe
StochasticSparkle
AmbientOcclusion
Bevel
Glow
EdgeSheen
ConcealedLight
Diffusion
FocusField
AffordanceWake
WayfindingNode
SubCellShake
ChromaticEdge
Cursor
```

Style graph ids that do not represent durable compositor shader variants may still require backend-owned adapter stages, but only after the lowerer confirms there is no equivalent compositor primitive.

## Boundary rule for this ledger

For every v3.1 graph effect:

```text
if compositor primitive exists:
  descriptor must expose the compositor concept
  lowerer must map to CompositionSpec / FilterSpec / MaskSpec / SamplerSpec / ShaderLayerSpec
  strict-native tests must not rely on player-side effect adapters
else:
  add a reusable compositor primitive or document a narrow backend adapter with promotion criteria
```

<!-- <FILE>docs/new_kernel/V31_COMPOSITOR_CAPABILITY_LEDGER.md</FILE> - <DESC>Compositor-first capability ledger for v3.1 recipe migration</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
