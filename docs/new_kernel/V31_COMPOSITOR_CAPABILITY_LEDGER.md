<!-- <FILE>docs/new_kernel/V31_COMPOSITOR_CAPABILITY_LEDGER.md</FILE> - <DESC>Compositor-first capability ledger for v3.1 recipe migration</DESC> -->
<!-- <VERS>VERSION: 1.2.0</VERS> -->
<!-- <WCTX>Ralph compositor-first migration: inventory compositor-native primitives before changing v3.1 recipes or player boundaries.</WCTX> -->
<!-- <CLOG>1.2.0: mark filter.edgeGrow as compositor-owned and record style-stage deletion.
1.1.0: mark sampler.faultLine as fully compositor-owned and record source-stage deletion.
1.0.1: PATCH — record that legacy player pathReveal no longer aliases structured paths to direction wipes.
1.0.0: mark sampler.crt and sampler.crtJitter as compositor-owned SamplerSpec lowerings and record source-stage deletion.
0.9.0: PATCH — mark mask.pathReveal schema/lowerer cleanup as completed using structured RevealPathType payloads.
0.8.0: mark mask.wipe and mask.wipeCorner lowering as compositor-owned where exact WipeDirection variants exist.
0.7.0: mark mask.blinds lowerer/player-stage cleanup as completed for the sixth compositor-first slice.
0.6.0: mark mask.iris lowerer/player-stage cleanup as completed for the fifth compositor-first slice.
0.5.0: mark mask.dissolve lowerer/player-stage cleanup as completed for the fourth compositor-first slice.
0.4.0: mark mask.diamond lowerer/player-stage cleanup as completed for the third compositor-first slice.
0.3.0: mark mask.radial lowerer/player-stage cleanup as completed for the second compositor-first slice.
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
| `mask.diamond` | `MaskSpec::Diamond { soft_edge }` | Done in third slice: lowerer emits `MaskSpec::Diamond` and backend source-stage diamond rendering was deleted. |
| `mask.dissolve` | `MaskSpec::Dissolve { seed, chunk_size }` | Done in fourth slice: lowerer emits `MaskSpec::Dissolve` and backend source-stage dissolve rendering was deleted. |
| `mask.iris` | `MaskSpec::Iris { shape, soft_edge }` | Done in fifth slice: lowerer emits `MaskSpec::Iris` and backend source-stage iris/shape rendering was deleted. |
| `mask.blinds` | `MaskSpec::Blinds { orientation, count }` | Done in sixth slice: lowerer emits `MaskSpec::Blinds` and backend source-stage blinds rendering was deleted. |
| `mask.wipe` | `MaskSpec::Wipe { reveal, hide, direction, soft_edge }` | Done for direct wipe directions: lowerer emits `MaskSpec::Wipe`. |
| `mask.wipeCorner` | `MaskSpec::Wipe { reveal: Corner*, soft_edge }` | Done for corner directions backed by compositor `WipeDirection` variants. |
| `mask.pathReveal` | `MaskSpec::PathReveal { path, soft_edge }` | Done after descriptor/recipe schema cleanup: v3.1 now authors structured `RevealPathType` payloads and the final backend `WipeMask` source stage was deleted. |
| `mask.noiseDither` | `MaskSpec::NoiseDither { seed, matrix, chunk_size }` | Verify descriptor exposes matrix/chunk size and lowerer maps without player semantics. |
| `mask.centerWipeFadeModal` V2 recipe | likely `mask.wipe` + style fade | Missing from v3.1 hierarchy; migrate after direct mask mappings are clean. |

Legacy player boundary note: `mask.pathReveal` must not be aliased to the old `mask.wipe`
player adapter. If a non-compositor render path samples a structured path-reveal recipe before
being routed through compositor-native rendering, it should report an unsupported adapter rather
than silently defaulting to `direction = leftToRight`.

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

Filter migration status:

| v3.1 id | Compositor target | Current action |
| --- | --- | --- |
| `filter.edgeGrow` | `FilterSpec::EdgeGrow { rest_eighths, peak_eighths, edge, fill_color, bg_color, progress, margin_width }` | Done: lowerer emits `FilterSpec::EdgeGrow` for static and binding recipes; backend `NativeStyleStage::EdgeGrow` and style mutation helper were deleted. |

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

Sampler migration status:

| v3.1 id | Compositor target | Current action |
| --- | --- | --- |
| `sampler.sineWave` | `SamplerSpec::SineWave` | Already compositor-native. |
| `sampler.ripple` | `SamplerSpec::Ripple` | Already compositor-native. |
| `sampler.crt` | `SamplerSpec::Crt { scanline_strength, jitter, curvature }` | Done: lowerer emits `SamplerSpec::Crt`; backend `NativeContentStage::CrtSampler` and source-row implementation were deleted. |
| `sampler.crtJitter` | `SamplerSpec::CrtJitter { intensity, speed_hz, decay_ms }` | Done: lowerer maps authored `amplitude`/`frequency`/`decayMs` to compositor fields; backend `NativeContentStage::CrtJitterSampler` and source-row implementation were deleted. |
| `sampler.faultLine` | `SamplerSpec::FaultLine { seed, intensity, split_bias, offset }` | Done: lowerer emits `SamplerSpec::FaultLine` for both dynamic and fixed-offset recipes; backend `NativeContentStage::FaultLineSampler` and source-row implementation were deleted. |
| `sampler.shredder` | `SamplerSpec::Shredder` | Already compositor-native. |
| `sampler.radialTwist` | `SamplerSpec::RadialTwist` | Already compositor-native. |

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
