<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/63_edge_ingestion_runtime_adapters.md</FILE> - <DESC>V3 edge-lane plan for source ingestion, terminal runtime adapters, and pattern/path primitive gaps found during adjacent-library review.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Record the cellophane-inspired edge lane and the follow-on animation-pattern/path review as one bounded V3 workstream. Keeps ingestion/runtime adapters and edge primitive additions together instead of scattering them across core V3 semantics.</WCTX> -->
<!-- <CLOG>0.1.0: initial lane plan with ANSI/command ingestion, standalone preview adapters, grapheme review, and path/pattern primitive matrix including swirl and attractor.</CLOG> -->

# 63 — Edge ingestion and runtime adapters

This lane covers capabilities at the edge of the V3 system. They make V3 easier
to feed, preview, and author, but they do not change the core shader/filter/mask
execution contract.

The lane exists because the adjacent-library review surfaced useful runtime and
input ideas:

- parse ANSI terminal output into a grid-backed scene source
- capture command output offline for recipe sources and probes
- provide a small standalone runner for recipe previews
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
| Lissajous/figure-eight | none | Defer | Add only when a recipe needs repeatable dual-axis harmonic motion. Can be expressed by composed sine signals later. |
| Particle system / flocking | none | Defer | Belongs in a later scene/procedural source lane, not the motion-path substrate. |

## Implementation plan

1. Land EIRA-01 as documentation first.
2. Land EIRA-06 in the geometry substrate because it has narrow blast radius:
   - add reusable route-treatment helpers to `mixed-signals` first
   - add `PathType::Swirl` that consumes the shared helper
   - add `PathType::Attractor` that consumes the shared helper
   - add interpolation coverage for endpoint safety and mid-route deviation
   - wire V3 motion dynamics into executable `PathType`s
3. Split EIRA-02/EIRA-03/EIRA-04 into separate work packets. They are mostly
   independent, but all must normalize into `SemanticScene` / grid output.
4. Keep EIRA-05 as a review gate before broad ANSI adoption. ANSI parsing can
   preserve style without forcing a new grapheme model into the core grid.
5. Add debug recipes for each shipped slice:
   - `motion_swirl_chain.json`: route plus swirl dynamic feeding a downstream style shader
   - `motion_attractor_chain.json`: attractor dynamic feeding a downstream shader through the V3 I/O hint path
   - `ansi_source_chain.json`: ANSI layer feeding a shader/filter chain
   - `command_capture_chain.json`: captured command output feeding a shader/filter chain

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

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/63_edge_ingestion_runtime_adapters.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
