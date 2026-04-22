<!-- <FILE>docs/design/tui-vfx-v3-style-model-restructure-inventory.md</FILE> - <DESC>Working inventory for the current flat style/shader model surface and its planned V3 restructuring under Decision 2. Maps the live `SpatialShaderType` catalog to the capability-catalog clusters and notes which groups are likely to become primitives, earned factories, or wrappers.</DESC> -->
<!-- <VERS>VERSION: 1.5.0</VERS> -->
<!-- <WCTX>Decision 2 adopts Pattern-as-separable-axis and an earned-factory model, but the live `tui-vfx-style` code still exports a flat `SpatialShaderType` enum. This inventory is the first execution artifact for migrating that live code surface deliberately.</WCTX> -->
<!-- <CLOG>1.5.0: record the first runtime-facing consumers of the central style-family seam so the tracker distinguishes model-only prep from actual wiring progress.
1.4.0: add the central legacy-to-V3 style-family lowering seam so runtime wiring can target one SSOT family enum instead of ad hoc per-family helpers.
1.3.0: add a dedicated V3 stripe-motion family so the last future-decision holdout also has a real grouped V3 home.
1.2.0: add a dedicated V3 cursor family surface so the future-decision bucket is narrowed to the last explicit special-case holdout.
1.1.0: add the first grouped V3 stochastic-texture family so the future-decision bucket is narrowed to the truly special-case leftovers.
1.0.0: add the first grouped V3 gradient-reveal family so the primitive bucket is fully represented by grouped V3 subfamilies.
0.9.0: add the first grouped V3 edge-distortion family so the remaining glitch/edge micro-treatment subgroup is also moving into real V3 code.
0.8.0: add the first grouped V3 motion-field family so the primitive scan/pulse/orbit subgroup is also moving into real V3 code during active family work.
0.7.0: add the first grouped V3 surface-depth family so the primitive depth/surface subgroup is also moving into real V3 code rather than waiting behind later cleanup.
0.6.0: add an explicit completed/outstanding tracker so the style-model migration state is resumable without rediscovering which grouped V3 family files already exist.
0.5.0: add the first real V3-side guidance-cue family surface so FocusedRowGradient, AffordanceWake, and WayfindingNode also move into grouped V3 code during active family work.
0.4.0: add the first real V3-side material-light family surface so the earned-factory cluster is also moving into grouped V3 code, not just tagged legacy files.
0.3.0: add the first real V3-side progress/emphasis family surface alongside the traveling-band files so active family work keeps producing grouped V3 code.
0.2.0: record the first real V3-side family files under crates/tui-vfx-style/src/models/v3/ so active family work produces parallel migration code, not just future-facing notes.
0.1.0: initial inventory. Classifies the current shader catalog into capability buckets and identifies the next migration-bearing code surface in tui-vfx proper.</CLOG> -->

# tui-vfx V3 style-model restructure inventory

This document tracks the current flat `tui-vfx-style` model surface against the
adopted V3 direction in:

- `docs/design/tui-vfx-v3-upgrade-plan/40_decisions.md`
  - **Decision 2 — Pattern as separable axis**
- `docs/design/tui-vfx-v3-capability-catalog.md`

The goal is to make the eventual `tui-vfx-style` restructure executable in
deliberate slices rather than by repeated ad hoc reclassification.

---

## 1. Current implementation surface

Today the main live surface is:

- `crates/tui-vfx-style/src/models/cls_spatial_shader_type.rs`
- re-exported through:
  - `crates/tui-vfx-style/src/models/mod.rs`

That surface is a **flat enum catalog** of named shader variants.

This is still the current implementation contract, but it is not the intended
final V3 conceptual shape.

---

## 2. Planned V3 direction

Per Decision 2:

- the internal model should move toward:
  - deeper primitive / shared substrates
  - earned named factories or presets
  - clearer separation between:
    - primitive capability
    - policy / composition

Per the capability catalog, the main buckets already identified are:

- traveling-band / sweep
- progress / emphasis
  - seeded in `crates/tui-vfx-style/src/models/v3/cls_vfx_progress_emphasis_shader.rs`
- field-rendering wrappers
- style fades
- style dwell modulation
- typography-window style effects
- cross-lane paired capabilities

---

## 3. Current shader catalog — first classification pass

### 3.1 Likely primitive / substrate-aligned

These look like true reusable substrates or very close siblings of one:

- `LinearGradient`
- `RevealWipe`

  Seeded together in `crates/tui-vfx-style/src/models/v3/cls_vfx_gradient_reveal_shader.rs`.
- `Glow`
- `AmbientOcclusion`
- `Bevel`

  Seeded together in `crates/tui-vfx-style/src/models/v3/cls_vfx_surface_depth_shader.rs`.
- `PulseWave`
- `Radar`
- `Orbit`

  Seeded together in `crates/tui-vfx-style/src/models/v3/cls_vfx_motion_field_shader.rs`.
- `GlitchLines`
- `ChromaticEdge`
- `SubCellShake`

  Seeded together in `crates/tui-vfx-style/src/models/v3/cls_vfx_edge_distortion_shader.rs`.

### 3.2 Likely earned named factories / compositions

These look like higher-level authored compositions that likely earn a stable
library-level name even if they are internally built from a deeper substrate:

- `Diffusion`
- `ConcealedLight`
- `EdgeSheen`
  - seeded in `crates/tui-vfx-style/src/models/v3/cls_vfx_material_light_shader.rs`
- `AffordanceWake`
- `WayfindingNode`
- `FocusedRowGradient`
  - seeded in `crates/tui-vfx-style/src/models/v3/cls_vfx_guidance_cue_shader.rs`

### 3.3 Traveling-band / sweep family

These likely belong to one deeper family with multiple policies or wrappers:

- `BorderSweep`
- `Reflect`
- `GlistenBand`
- `TracePropagation`
- `TracePath`

### 3.4 Category needing explicit future decision

These may be retained as direct variants for ergonomics, or folded further once
the primitive/factory split is implemented:

- `Highlighter`
- `BarberPole`
- `NeonFlicker`
- `Cursor`
- `StochasticSparkle`

  Seeded subgroup in `crates/tui-vfx-style/src/models/v3/cls_vfx_stochastic_texture_shader.rs` for `NeonFlicker` + `StochasticSparkle`.

---

## 4. Recommended next code-facing slice in `tui-vfx-style`

The next implementation-bearing slice is not “rename everything.”

It is:

1. make the current flat enum’s role explicit in rustdocs and module docs
2. classify the current variants into stable buckets
3. identify which bucket should be peeled off first into a deeper substrate or
   factory surface

The strongest candidate bucket today is:

- **traveling-band / sweep**

because it is already clearly clustered in both:

- the capability catalog
- the live variant names

and it touches real runtime behavior without requiring the whole style surface
to change at once.

That recommendation is now in motion through a real parallel V3 family surface:

- `crates/tui-vfx-style/src/models/v3/cls_vfx_traveling_band_shader.rs`
- `crates/tui-vfx-style/src/models/v3/enum_vfx_traveling_band_behavior.rs`

The migration rule for active family work is now:

- create or extend the real V3-side family files while analyzing the family
- leave the legacy flat variants operational for current playback/cutover
- defer deletion of the legacy files until the endgame V2 removal pass

Current real V3-side family files:

- `crates/tui-vfx-style/src/models/v3/cls_vfx_traveling_band_shader.rs`
- `crates/tui-vfx-style/src/models/v3/enum_vfx_traveling_band_behavior.rs`
- `crates/tui-vfx-style/src/models/v3/cls_vfx_progress_emphasis_shader.rs`
- `crates/tui-vfx-style/src/models/v3/enum_vfx_progress_emphasis_behavior.rs`
- `crates/tui-vfx-style/src/models/v3/cls_vfx_material_light_shader.rs`
- `crates/tui-vfx-style/src/models/v3/enum_vfx_material_light_behavior.rs`
- `crates/tui-vfx-style/src/models/v3/cls_vfx_guidance_cue_shader.rs`
- `crates/tui-vfx-style/src/models/v3/enum_vfx_guidance_cue_behavior.rs`
- `crates/tui-vfx-style/src/models/v3/cls_vfx_surface_depth_shader.rs`
- `crates/tui-vfx-style/src/models/v3/enum_vfx_surface_depth_behavior.rs`

---

## 4.1 Family migration tracker

Completed real V3-side family surfaces:

- [x] traveling-band / sweep
  - `cls_vfx_traveling_band_shader.rs`
  - `enum_vfx_traveling_band_behavior.rs`
- [x] progress / emphasis
  - `cls_vfx_progress_emphasis_shader.rs`
  - `enum_vfx_progress_emphasis_behavior.rs`
- [x] material-light
  - `cls_vfx_material_light_shader.rs`
  - `enum_vfx_material_light_behavior.rs`
- [x] guidance-cue
  - `cls_vfx_guidance_cue_shader.rs`
  - `enum_vfx_guidance_cue_behavior.rs`
- [x] primitive / substrate-aligned subgroup: surface-depth
  - `cls_vfx_surface_depth_shader.rs`
  - `enum_vfx_surface_depth_behavior.rs`

Outstanding style-model buckets for follow-on slices:

- [x] primitive / substrate-aligned cluster
  - completed subgroup: surface-depth (`Glow`, `AmbientOcclusion`, `Bevel`)
  - completed subgroup: motion-field (`PulseWave`, `Radar`, `Orbit`)
  - completed subgroup: edge-distortion (`GlitchLines`, `ChromaticEdge`, `SubCellShake`)
  - completed subgroup: gradient-reveal (`LinearGradient`, `RevealWipe`)
- [x] category needing explicit future decision
  - completed subgroup: stochastic-texture (`NeonFlicker`, `StochasticSparkle`)
  - completed special-case family: cursor
  - completed special-case family: stripe-motion (`BarberPole`)
  - remaining candidates: none
- [~] any additional cross-family refactors needed once runtime wiring begins
  - [x] central legacy `SpatialShaderType -> VfxSpatialShaderFamily` lowering seam
  - [~] first runtime-facing consumers
    - [x] `StyleEffect::spatial_shader_family()`
    - [x] `ShaderLayerSpec::v3_shader_family()`
    - [x] `CompositionSpec::v3_shader_families()`
    - [x] `CompositionOptions::v3_shader_families()`
  - [ ] broader runtime wiring against `models::v3` family surfaces

## 5. Current rule during the restructure

Until the full restructure lands:

- keep the current flat enum as the public implementation surface
- avoid adding new variants casually when a known family bucket already exists
- prefer documenting whether a new effect is:
  - primitive-like
  - earned factory-like
  - wrapper-like
before adding it

That keeps the style surface moving toward the V3 model without pretending the
restructure is already complete.

<!-- <FILE>docs/design/tui-vfx-v3-style-model-restructure-inventory.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.5.0</VERS> -->
