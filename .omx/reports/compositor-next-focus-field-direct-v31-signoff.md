# Compositor-Next v3.1 Focus Field Signoff

Date: 2026-05-01
Status: signed off for phase commit

## Scope

- Primitive: `shader.focusField`
- Family: shader
- Direct path: `LoadedV31Recipe::load` → `render_v31_recipe` → `SpatialShaderType::FocusField`
- Implementing worker: Zeno in `/usr/projects/tui-vfx-slice-focus-field`
- Lead integration: reviewed, conflict-resolved against the prior glisten slice, and applied to `/usr/projects/tui-vfx`

## Direct v3.1 Decisions

Supported descriptor-canonical subset:

- `color`: required literal color; mapped to compositor `FocusFieldShader.color`.
- `centerX`: required integer-valued numeric literal; mapped to compositor `center_x`.
- `centerY`: required integer-valued numeric literal; mapped to compositor `center_y`.
- `radius`: required integer-valued numeric literal; mapped to both `radius_x` and `radius_y`.
- `intensity`: optional literal number; clamped to compositor range.
- `applyTo`: optional literal enum, descriptor values only: `foreground`, `background`, `both`; absence follows descriptor/lowerer foreground behavior.
- `shape`: optional literal enum; `circle` and `ellipse` both map to compositor ellipse semantics for the first direct slice.

Unsupported descriptor-valid inputs/semantics:

- `shape = rect`: rejected at load time for this direct slice.
- Rect geometry fields: `rectX`, `rectY`, `rectWidth`, `rectHeight` rejected at load time.
- Extended ellipse geometry/falloff fields: `radiusX`, `radiusY`, `feather` rejected at load time.
- Fractional `centerX`, `centerY`, or `radius`: rejected at load time because copied compositor fields are integer cell coordinates/radii.

Runtime-sourced focus-field inputs are rejected at load time through the shared literal-input gate.

## Tests

Added in `crates/tui-vfx-compositor-next/tests/test_v31_direct_recipe.rs`:

- `load_validated_v31_focus_field_renders_directly_in_compositor_next`
- `rejects_descriptor_valid_focus_field_rect_shape_without_direct_support`
- `rejects_runtime_sourced_focus_field_inputs_at_load_time`
- `rejects_fractional_focus_field_geometry_at_load_time`

## AI De-Slop Report

Scope:

- `crates/tui-vfx-compositor-next/src/v31/load.rs`
- `crates/tui-vfx-compositor-next/src/v31/render.rs`
- `crates/tui-vfx-compositor-next/tests/test_v31_direct_recipe.rs`
- this signoff and related architecture handoff docs

Behavior lock:

```bash
cargo test -p tui-vfx-compositor-next --test test_v31_direct_recipe focus_field -- --nocapture
cargo test -p tui-vfx-compositor-next --test test_v31_direct_recipe rejects_fractional_focus_field_geometry_at_load_time -- --nocapture
```

Cleanup plan:

1. Check merge resolution preserved both `shader.glistenBand` and `shader.focusField` dispatch paths.
2. Check descriptor-canonical enum handling and no-alias policy.
3. Check copied compositor integer fields do not silently narrow descriptor-valid fractional numbers.
4. Check optional defaults match descriptor/lowerer behavior where the direct recipe omits inputs.
5. Record OFPF >500 LOC cohesion justification and keep the split as a next-phase cleanup item.

Passes completed:

1. Merge de-slop: conflict blocks removed; both parallel slices retained in load and render dispatch.
2. Duplicate removal: reused shared literal, enum, numeric, and optional numeric helpers; only added one `optional_number_input_or` helper to preserve existing glisten optional handling.
3. Naming/error handling: unsupported rect/geometry diagnostics name the direct v3.1 semantic gap; fractional geometry is rejected before render.
4. Test reinforcement: added the fractional geometry regression after lead review found possible silent narrowing.

## Architect Review

Result: pass after iteration.

Findings reviewed:

- Direct path remains pure v3.1: no backend bridge/shim additions and no legacy alias acceptance.
- Load-time validation owns acceptance; renderer can rely on canonical literal values after load.
- `shape = rect` is not partially mapped; it is rejected until a full rect semantic slice owns it.
- Fractional geometry was a design issue in the worker diff because the renderer would cast to `u16`; lead added a RED test and load-time integer-valued validation before signoff.

## Code Review

Result: pass after iteration.

Findings reviewed:

- Conflict resolution retained both glisten and focus imports, dispatch arms, validators, and helper functions.
- `focus_field_apply_to_input` defaults to foreground to match the existing lowerer and descriptor-facing behavior.
- Tests cover success, unsupported descriptor-valid rect semantics, runtime-sourced input rejection, and fractional geometry rejection.
- No unrelated staged `docs/design/post-release/transitions-demo.py` edits are included in this slice commit.

## OFPF / File-Size Review

Touched Rust files after integration:

```text
crates/tui-vfx-compositor-next/src/v31/load.rs
crates/tui-vfx-compositor-next/src/v31/render.rs
crates/tui-vfx-compositor-next/tests/test_v31_direct_recipe.rs
```

Cohesion justification:

- `render.rs` and `test_v31_direct_recipe.rs` remain above the soft 500 LOC threshold. The fourth direct slice closes the first parallel-agent merge window; splitting before this commit would make the active conflict resolution riskier.
- The next cleanup/slice boundary should split direct primitive renderers and fixture helpers before more shader slices accumulate.

## Verification Evidence

Lead ran during integration:

```bash
cargo test -p tui-vfx-compositor-next --test test_v31_direct_recipe rejects_fractional_focus_field_geometry_at_load_time -- --nocapture
# RED before load validation; GREEN after integer-valued geometry validation.

cargo test -p tui-vfx-compositor-next --test test_v31_direct_recipe focus_field -- --nocapture
# 4 passed
```

Formal review:

```text
architect review: PASS after doc-coherence iteration
code review: PASS
```

Full phase verification before commit:

```bash
cargo fmt --check
git diff --check
git diff --cached --check
cargo test -p tui-vfx-compositor-next --test test_v31_direct_recipe -- --nocapture
# 17 passed
cargo test -p tui-vfx-player-next --test player_next_v31 -- --nocapture
# 3 passed
cargo test -p tui-vfx-compositor-next --test test_old_compositor_parity
# 1 passed
cargo test -p tui-vfx-player --test test_compositor_next_primitive_tree
# 3 passed
cargo test -p tui-vfx-compositor-next
# passed
cargo check -p tui-vfx-player-next
# passed
cargo clippy -p tui-vfx-compositor-next --all-targets -- -D warnings
# passed
cargo clippy -p tui-vfx-player-next --all-targets -- -D warnings
# passed
```

## Known Risks / Deferred Work

- Rect-shaped focus fields and independent `radiusX`/`radiusY`/`feather` remain unsupported until a later direct slice deliberately owns those semantics.
- The direct renderer/test harness needs a planned split before many more primitives are added.
- Existing `docs/design/post-release/transitions-demo.py` worktree dirt is unrelated and intentionally excluded from this slice commit.
