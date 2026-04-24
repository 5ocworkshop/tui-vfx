<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/63_edge_ingestion_runtime_adapters.md</FILE> - <DESC>V3 edge-lane plan for source ingestion, terminal runtime adapters, and pattern/path primitive gaps found during adjacent-library review.</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>Record the cellophane/whoa-inspired edge lane and shipped route/field primitive additions as one bounded V3 workstream.</WCTX> -->
<!-- <CLOG>0.3.0: update the pattern/path matrix with whoa-derived RadialTwist/RadialSpiral plus CarrierOrbit/helix and FigureEight/infinity route support; keep stateful procedural screensavers as future source lanes. 0.2.0: add explicit Cellophane adoption matrix and resize-aware runtime adapter contract while clarifying that Cellophane contributes runtime/frame infrastructure rather than additional named effect/path primitives. 0.1.0: initial lane plan with ANSI/command ingestion, standalone preview adapters, grapheme review, and path/pattern primitive matrix including swirl and attractor.</CLOG> -->

# 63 — Edge ingestion and runtime adapters

This lane covers capabilities at the edge of the V3 system. They make V3 easier
to feed, preview, and author, but they do not change the core shader/filter/mask
execution contract.

The lane exists because the adjacent-library review surfaced useful runtime and
input ideas:

- parse ANSI terminal output into a grid-backed scene source
- capture command output offline for recipe sources and probes
- provide a small standalone runner for recipe previews
- provide resize-aware preview/runtime adapter loops without moving terminal
  lifecycle ownership into the library core
- keep terminal lifecycle and diff rendering outside compositor core
- audit grapheme storage separately from visual-effect semantics
- close path/pattern gaps such as `swirl` and executable `attractor` dynamics

## Boundary rule

`tui-vfx` remains grid-first and ecosystem-agnostic. Edge adapters may read from
terminals, commands, files, or ANSI streams, but they must normalize into the
existing scene/grid contracts before shader/filter/mask execution.

Do not move terminal lifecycle ownership into the compositor. Do not make
recipe execution depend on crossterm, ratatui, or a command runner. Runtime
crates consume validated scene and pipeline data; adapters produce that data.

## Sub-lanes

| ID | Work | Owner surface | Acceptance |
|---|---|---|---|
| EIRA-01 | Plan/docs and schema vocabulary | `tui-vfx/docs/design`, `tui-vfx/docs/CAPABILITIES_REFERENCE.md` | Lane appears in the V3 plan and capabilities reference with examples and clear boundaries. |
| EIRA-02 | ANSI source ingestion | `tui-vfx-recipes` source/layer path | ANSI-styled text can become a scene layer without terminal lifecycle coupling. |
| EIRA-03 | Command-output capture | authoring/probe tooling | A command output can be captured offline and referenced by a recipe or fixture. Runtime execution does not spawn commands. |
| EIRA-04 | Standalone preview runner / diff adapter | examples/tools | A recipe can be previewed outside a host app using the canonical builder and grid output. |
| EIRA-05 | Grapheme storage review | design note + tests if adopted | Unicode cluster handling is either explicitly adopted where it improves correctness or deferred with a reason. |
| EIRA-06 | Pattern/path primitive gaps | `tui-vfx-geometry`, V3 motion lowering | Existing patterns are documented; missing reusable primitives are added with tests and rustdoc. |
| EIRA-07 | Grid-resize adapter contract | examples/tools + host integration docs | Host-owned resize loops supply a new `Rect`/grid size and preserve phase/sample state while V3 re-renders deterministically; tui-vfx remains area-in/grid-out and does not poll terminal events. |

## Cellophane review: adopt, already have, defer

Cellophane is primarily a terminal animation runtime and frame abstraction. It
does not appear to ship a catalog of named visual effects or motion paths that
competes with the V3 recipe/path system. The useful ideas are therefore runtime
and source-adapter ideas, not new shader/filter semantics.

| Cellophane capability | tui-vfx status | Decision | V3 lane |
|---|---|---|---|
| One-trait animation lifecycle (`init`, `update`, `is_done`) | V3 has recipe lifecycle plus compiled preview state, not a user-implemented trait API | Do not copy directly. Preserve recipe-first authoring; expose host adapter examples where imperative apps need a loop. | EIRA-04, EIRA-07 |
| Resize-aware loop | Core rendering is already naturally adaptive when recipes are rendered from the current `Rect`/grid size; host apps still own terminal events | Adopt the contract language, not new core machinery: host detects resize, supplies the new grid size, preserves phase/sample/runtime params, and asks V3 to render again. | EIRA-07 |
| Frame diffing renderer | Probe tooling already has frame diffs; compositor should not own terminal writes | Adopt only at preview/tool boundary if useful. Core remains deterministic grid/area rendering; terminal diff writes belong to a host/runner. | EIRA-04 |
| ANSI/VTE frame ingestion | Not a first-class V3 recipe source yet | Adopt as source ingestion: parse ANSI-styled text into a grid/scene layer offline or at authoring/tool time. | EIRA-02 |
| Command output to frame | Not a first-class recipe source yet | Adopt as offline capture only. Runtime recipe execution must not spawn commands. | EIRA-03 |
| Ratatui widget adapter | Host apps can already render tui-vfx output through their own buffers, but a small canonical example would help | Adopt as documentation/example shape, not as a core dependency. | EIRA-04 |
| Grapheme cluster storage / wide-cell discipline | tui-vfx has Unicode and color-inert-glyph handling, but should re-review cluster storage before broad ANSI ingestion | Review before adopting deeper storage changes. | EIRA-05 |
| Input forwarding to animations | V3 recipes are declarative; user input is a host concern | Defer as a recipe trigger/binding topic, not an effect primitive. Host adapters may map events to runtime params. | Future bindings/runtime-param lane |
| Named effects/path primitives | Cellophane does not appear to provide additional built-in paths/effects beyond examples authored by consumers | No direct adoption. Keep the V3 pattern/path matrix below as the source of truth. | EIRA-06 |

## Pattern/path capability matrix

| Pattern/path | Existing primitive today | Status | Notes |
|---|---:|---|---|
| Linear travel | `PathType::Linear` | Supported | Baseline carrier route. |
| Arc / curved travel | `PathType::Arc`, `PathType::Bezier` | Supported | Covers single-control spatial curves. |
| Spiral-in path | `PathType::Spiral` and `PathReveal` spiral masks | Supported | Good for reveal spirals and target-converging motion. |
| Orbit | `PathType::Orbit`, `OrbitShader` | Supported | Supports element paths and center-orbit shader dots. |
| Radar sweep | `RadarShader` / V3 `motion_field.radar` | Supported | Rotating angular field, not a moving object path. |
| Wave/ripple | `PulseWaveShader`, procedural braille flag wave, samplers | Supported | Horizontal, vertical, radial, diagonal wave fields exist. |
| Trace/path following | `TracePathShader`, `TracePropagationShader` | Supported | Good for routed signal flow, not arbitrary object motion. |
| Spring/bounce/projectile/pendulum/friction | `PathType` dynamics + V3 lowering | Supported | Physics-like dynamics already execute through the geometry substrate. |
| Swirl / vortex around a carrier route | `PathType::Swirl` backed by `mixed_signals::math::swirl_position` | Added in this lane | Preserves endpoints, unlike `Spiral`; works as a V3 dynamic over a route. The reusable math lives upstream in `mixed-signals`. |
| Attractor / gravity well | `PathType::Attractor` + V3 dynamic lowering backed by `mixed_signals::math::attract_position` | Completed in this lane | V3 had the authored dynamic shape; this lane makes it executable. The endpoint-preserving pull math lives upstream in `mixed-signals`. |
| Carrier orbit / helix | `PathType::CarrierOrbit` backed by `mixed_signals::math::carrier_orbit_position` | Added in this lane | Substrate name is `carrier_orbit`; recipes may use the `helix` alias for projected corkscrew motion. Preserves endpoints. |
| Figure-eight / infinity / lemniscate | `PathType::FigureEight` backed by `mixed_signals::math::figure_eight_position` | Added in this lane | A true 2D harmonic ∞ route, not a sideways helix. Recipe aliases: `infinity`, `infinity_symbol`, `lemniscate`. |
| Radial twist / maelstrom warp | `SamplerSpec::RadialTwist` backed by `mixed_signals::math::radial_twist_warp` | Added in this lane | Source-remapping sampler for vortex/portal/maelstrom effects; whoa demo naming stays out of the public API. |
| Radial spiral density field | `SpatialShaderType::RadialSpiral` / V3 `motion_field.radial_spiral` backed by `mixed_signals::math::radial_spiral_field` | Added in this lane | Procedural style field for spiral loaders, portals, and ambient backgrounds. |
| Cosine column wave | `SineWave` sampler plus `mixed_signals::math::column_wave_offset` substrate helper | Supported / helper added | Author as a sine wave with a phase offset where possible; shared math exists for adapters/tools that need exact column offsets. |
| Saturn-style line wave / palette cycling | `mixed_signals::math::line_wave_offset` substrate helper; existing shader/filter palette tools | Partial / future adapter | Line-wave substrate exists, but full EarthBound-style asset/palette/interlace emulation belongs in a future asset-backed procedural source lane. |
| Perlin/noise density | Existing spatial noise/stochastic shader families | Mostly supported | Use existing stochastic/noise shaders first; add a source only if recipe authors need a stateful fullscreen density surface. |
| Slime/Physarum agents | none | Defer | Stateful agent simulation belongs in a procedural source lane, not path interpolation. |
| Conway/cellular automata | none | Defer | Stateful grid evolution belongs in a procedural source lane. |
| Collapse/gravity cellular transform | `Gravity` sampler partially overlaps | Defer full behavior | Existing gravity moves content, but a cellular collapse simulation should be a future source/transform with state. |
| Particle system / flocking | none | Defer | Belongs in a later scene/procedural source lane, not the motion-path substrate. |

No further Cellophane-native named path/effect primitives were identified in the
local review. If future review of Cellophane consumers such as terminal
screensavers reveals concrete reusable motion fields, evaluate them against this
matrix and move shared math into `mixed-signals` when the primitive is useful
outside tui-vfx.

## Implementation plan

1. Land EIRA-01 as documentation first.
2. Land EIRA-06 in the geometry substrate because it has narrow blast radius:
   - add reusable route-treatment helpers to `mixed-signals` first
   - add `PathType::Swirl` that consumes the shared helper
   - add `PathType::Attractor` that consumes the shared helper
   - add interpolation coverage for endpoint safety and mid-route deviation
   - add CarrierOrbit/helix and FigureEight/infinity route support when whoa/cellophane consumer review proves they are useful
   - add RadialTwist sampler and RadialSpiral shader when source-remap/field behavior is reusable
   - wire V3 motion dynamics into executable `PathType`s
3. Split EIRA-02/EIRA-03/EIRA-04 into separate work packets. They are mostly
   independent, but all must normalize into `SemanticScene` / grid output.
4. Land EIRA-07 as a preview/tooling contract rather than core resize logic:
   - host app listens for terminal resize
   - host supplies the new target `Rect` / grid size
   - adapter preserves phase/sample/runtime params where possible
   - adapter re-renders the same V3 recipe against the new grid
   - tui-vfx core remains free of terminal polling and lifecycle ownership
   - recipes are adaptive to the extent their layout/source specs are expressed
     relative to the provided grid; fixed-size recipes remain intentionally fixed
5. Keep EIRA-05 as a review gate before broad ANSI adoption. ANSI parsing can
   preserve style without forcing a new grapheme model into the core grid.
6. Add debug recipes for each shipped slice:
   - `motion_swirl_chain.json`: route plus swirl dynamic feeding a downstream style shader
   - `motion_attractor_chain.json`: attractor dynamic feeding a downstream shader through the V3 I/O hint path
   - `ansi_source_chain.json`: ANSI layer feeding a shader/filter chain
   - `command_capture_chain.json`: captured command output feeding a shader/filter chain
   - `resize_preserve_phase_chain.json`: same V3 chain rendered at multiple
     areas while preserving phase/sample state

## Documentation requirements

Every shipped public type or schema-bearing field in this lane requires:

- rustdoc explaining intent, constraints, and example use
- generated schema/doc validation where the type participates in generation
- a hand-maintained capabilities entry describing author-facing usage
- at least one debug recipe or probe fixture that demonstrates the feature in a
  chain between two effects when it is part of the V3 I/O path

## Verification

- `cargo test -p tui-vfx-geometry`
- `cargo test -p tui-vfx-recipes` for V3 lowering/tooling slices
- `cargo xtask docs generate` or the current docs validation target when public
  schema/doc surfaces change
- probe/validator smoke tests for every added debug recipe
- adapter smoke proving a recipe can be rendered against two grid sizes without
  resetting phase/sample unless the host explicitly requests a restart

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/63_edge_ingestion_runtime_adapters.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.3.0</VERS> -->
