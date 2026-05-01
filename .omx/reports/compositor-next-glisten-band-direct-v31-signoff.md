# Compositor-Next v3.1 Glisten Band Signoff

Date: 2026-05-01
Status: signed off for phase commit

## Scope

- Primitive: `shader.glistenBand`
- Family: shader
- Direct path: `LoadedV31Recipe::load` → `render_v31_recipe` → `SpatialShaderType::GlistenBand`
- Implementing worker: Tesla in `/usr/projects/tui-vfx-slice-glisten-band`
- Lead integration: reviewed and applied to `/usr/projects/tui-vfx`

## Direct v3.1 Decisions

Supported descriptor-canonical subset:

- `color`: required literal color; mapped to both compositor `head` and `tail` colors.
- `bandWidth`: required literal number; mapped to compositor band width.
- `direction`: optional literal enum, descriptor values only: `leftToRight`, `rightToLeft`.
- `blendStrength`: optional literal number, clamped to compositor range.
- `angleDeg`: optional literal number.
- `speed`: optional literal number, clamped to copied compositor range.

Unsupported descriptor-valid inputs:

- `head`: rejected at load time.
- `tail`: rejected at load time.
- fractional `bandWidth`: rejected at load time because compositor-native band width is integer cell width.

Reason: the descriptor currently models `head`/`tail` as numeric band-position fields, while copied compositor `GlistenBandShader` models `head`/`tail` as colors. Direct v3.1 should not reinterpret those mismatched semantics silently.

Runtime-sourced glisten inputs are rejected at load time through the shared literal-input gate.

## Tests

Added in `crates/tui-vfx-compositor-next/tests/test_v31_direct_recipe.rs`:

- `load_validated_v31_glisten_band_renders_directly_in_compositor_next`
- `rejects_descriptor_valid_glisten_band_tail_input_without_direct_support`
- `rejects_runtime_sourced_glisten_band_inputs_at_load_time`
- `rejects_fractional_glisten_band_width_without_direct_support`

## AI De-Slop Report

Scope:

- `crates/tui-vfx-compositor-next/src/v31/load.rs`
- `crates/tui-vfx-compositor-next/src/v31/render.rs`
- `crates/tui-vfx-compositor-next/tests/test_v31_direct_recipe.rs`
- this signoff and related architecture handoff docs

Behavior lock:

```bash
cargo test -p tui-vfx-compositor-next --test test_v31_direct_recipe glisten_band -- --nocapture
cargo test -p tui-vfx-compositor-next --test test_v31_direct_recipe -- --nocapture
```

Cleanup plan:

1. Check descriptor-canonical enum handling and no-alias policy.
2. Check load-time unsupported decisions for descriptor-valid semantic gaps.
3. Check optional input handling does not mask runtime-sourced values.
4. Check numeric descriptor-valid values that would be silently narrowed by copied compositor types.
5. Record OFPF >500 LOC cohesion justification instead of splitting mid-phase.

Passes completed:

1. Dead code deletion: none found in slice-owned diff.
2. Duplicate removal: existing direct helpers are reused; no new generic helper extracted because this is still the first glisten-only mapping.
3. Naming/error handling: unsupported diagnostics are explicit for numeric `head`/`tail` semantic mismatch and fractional `bandWidth` narrowing.
4. Test reinforcement: targeted glisten tests plus the full direct recipe suite are green.

## OFPF / File-Size Review

Touched Rust files after integration:

```text
426 crates/tui-vfx-compositor-next/src/v31/load.rs
509 crates/tui-vfx-compositor-next/src/v31/render.rs
574 crates/tui-vfx-compositor-next/tests/test_v31_direct_recipe.rs
```

Cohesion justification:

- `render.rs` crossed 500 LOC with the third direct primitive. Splitting during this phase would create a moving conflict boundary while `shader.focusField` is already implemented in a parallel worktree against the same file. Keep the file cohesive for this commit, then split by primitive/direct-render module after the parallel focusField slice is integrated or rebased.
- `test_v31_direct_recipe.rs` crossed 500 LOC because it intentionally keeps direct v3.1 recipe fixtures in one integration harness while the first few primitives establish common fixture shape. Split fixtures/tests after the next integrated slice proves the shared helper boundary.

## Verification Evidence

Lead reran these after applying Tesla's diff to `master`:

```bash
cargo fmt --check
cargo test -p tui-vfx-compositor-next --test test_v31_direct_recipe glisten_band -- --nocapture
# 4 passed
cargo test -p tui-vfx-compositor-next --test test_v31_direct_recipe -- --nocapture
# 13 passed
cargo clippy -p tui-vfx-compositor-next --all-targets -- -D warnings
# passed
git diff --check
# passed
```

Full phase verification is run before commit and recorded in the final commit summary.

## Known Risks / Deferred Work

- Numeric `head`/`tail` remain unsupported until the descriptor/runtime semantic mismatch is resolved deliberately.
- `render.rs` and the direct recipe integration test now require a planned split after the focusField conflict window closes.
- No player-next-specific test was added for glisten; player-next delegates to the shared loader/render path covered by existing player-next tests.
