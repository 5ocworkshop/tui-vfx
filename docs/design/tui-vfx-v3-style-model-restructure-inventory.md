<!-- <FILE>docs/design/tui-vfx-v3-style-model-restructure-inventory.md</FILE> - <DESC>Working inventory for the current flat style/shader model surface and its planned V3 restructuring under Decision 2. Maps the live `SpatialShaderType` catalog to the capability-catalog clusters and notes which groups are likely to become primitives, earned factories, or wrappers.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Decision 2 adopts Pattern-as-separable-axis and an earned-factory model, but the live `tui-vfx-style` code still exports a flat `SpatialShaderType` enum. This inventory is the first execution artifact for migrating that live code surface deliberately.</WCTX> -->
<!-- <CLOG>0.2.0: record the first real V3-side family files under crates/tui-vfx-style/src/models/v3/ so active family work produces parallel migration code, not just future-facing notes.
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
- `Glow`
- `AmbientOcclusion`
- `Bevel`
- `PulseWave`
- `Radar`
- `Orbit`
- `GlitchLines`
- `ChromaticEdge`
- `SubCellShake`

### 3.2 Likely earned named factories / compositions

These look like higher-level authored compositions that likely earn a stable
library-level name even if they are internally built from a deeper substrate:

- `Diffusion`
- `ConcealedLight`
- `EdgeSheen`
- `AffordanceWake`
- `WayfindingNode`
- `FocusedRowGradient`

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

---

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
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
