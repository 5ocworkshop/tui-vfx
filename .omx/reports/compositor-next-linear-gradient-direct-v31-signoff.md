# compositor-next `shader.linearGradient` direct v3.1 signoff

Date: 2026-05-01
Status: signed off for first vertical slice

## Primitive

- Primitive id: `shader.linearGradient`
- Family: shader primitive
- Descriptor path: `descriptors/v3.1/packs/primitive.json`
- Direct runtime files:
  - `crates/tui-vfx-compositor-next/src/v31/mod.rs`
  - `crates/tui-vfx-compositor-next/src/v31/load.rs`
  - `crates/tui-vfx-compositor-next/src/v31/render.rs`
- Fixtures/tests:
  - `crates/tui-vfx-compositor-next/tests/test_v31_direct_recipe.rs`

## Direct path proven

```text
RecipeDocument JSON
  │
  ▼
DescriptorCatalog + RecipeDocument validation
  │
  ▼
LoadedV31Recipe::load
  ├─ rejects recipe/graph versions other than "3.1"
  ├─ rejects runtime-sourced source inputs consumed by this slice
  └─ rejects runtime-sourced shader.linearGradient inputs consumed by this slice
  │
  ▼
render_v31_recipe(LoadedV31Recipe, V31SampleContext)
  │
  ├─ materializes the source grid from literal message/text + dimensions
  ├─ reads graph order/topology from the canonical v3.1 structure
  └─ converts shader.linearGradient literals, including canonical gradient stops
  │
  ▼
tui-vfx-compositor-next render_pipeline_with_spec
  │
  ▼
V31Frame
```

No backend bridge, shim, legacy input support, or transition-seam code was added.

## Validation commands

- `cargo fmt --check`
- `git diff --check`
- `cargo test -p tui-vfx-compositor-next --test test_v31_direct_recipe -- --nocapture`
- `cargo test -p tui-vfx-compositor-next --test test_old_compositor_parity`
- `cargo test -p tui-vfx-player --test test_compositor_next_primitive_tree`
- `cargo test -p tui-vfx-compositor-next`
- `cargo clippy -p tui-vfx-compositor-next --all-targets -- -D warnings`

## Parity evidence

- Baseline copy parity remained green via `test_old_compositor_parity`.
- Existing player primitive-tree checks remained green.
- New direct v3.1 test coverage passed with five focused tests:
  - direct load-validated render;
  - strict non-v3.1 rejection;
  - runtime-sourced effect input load rejection;
  - runtime-sourced source input load rejection;
  - canonical multi-stop gradient rendering.

## Unsupported decisions

- This first slice accepts only literal source/effect inputs that it can render directly.
- Runtime parameter, signal, graph-value, map, and sampled-field inputs are rejected at load time for this slice instead of being silently defaulted at render time.
- Broader source rendering remains out of scope; the direct slice currently consumes literal `message`/`text` source content plus optional literal dimensions.

## Commonality and OFPF decisions

- `v31/mod.rs` stays as a small entrypoint.
- Load validation and rendering are split into OFPF-sized modules:
  - `load.rs` handles acceptance gates;
  - `render.rs` handles direct compositor-next frame production.
- No new dependency was added beyond the existing workspace `tui-vfx-contract` crate required to consume canonical v3.1 recipe contracts.

## Review and cleanup gates

- AI de-slop pass: removed transient wording, renamed ambiguous applied-effect reporting, split the over-500 LOC module, and kept behavior locked with regression tests.
- Architect review: clean after iteration.
- Code review: clean after iteration.

## Known risks / next work

- The direct path is intentionally narrow: later slices must add explicit load-time support for additional input-source forms before rendering them.
- The next primitive should extend the direct path vertically without adding bridge/shim/transition-seam code.
