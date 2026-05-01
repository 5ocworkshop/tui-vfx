# shader.borderSweep direct v3.1 signoff

Primitive: `shader.borderSweep`
Family: shader
Descriptor path: `descriptors/v3.1/packs/primitive.json`

## Runtime files

- `crates/tui-vfx-compositor-next/src/v31/load.rs`
- `crates/tui-vfx-compositor-next/src/v31/render.rs`

## Test file

- `crates/tui-vfx-compositor-next/tests/test_v31_direct_recipe.rs`

## Supported direct v3.1 inputs

- `color`: descriptor-canonical literal color.
- `speed`: descriptor-canonical literal number, mapped to `BorderSweepShader::speed`.
- `length`: descriptor-canonical literal integer, mapped to `BorderSweepShader::length`.

## Unsupported decisions

- `position` is rejected at `LoadedV31Recipe::load`. Existing compositor behavior exposes position through runtime binding lookup, and this slice must not introduce bridge/runtime-binding semantics for a literal v3.1 override.
- Runtime-sourced inputs are rejected at load time through the existing literal direct-input gate.

## Evidence

- Added focused RED tests before implementation; the targeted `border_sweep` run failed for unsupported render and missing load rejections.
- After implementation, `cargo test -p tui-vfx-compositor-next --test test_v31_direct_recipe border_sweep -- --nocapture` passed.
- Full direct recipe test and compositor-next check passed.

## OFPF / file-size decision

`render.rs` remains above the 500 LOC pressure threshold because this slice follows the existing vertical direct-render pattern and the write scope did not include extraction. Recommend a follow-up compositor-next v31 split only after the current primitive sequence stabilizes repeated helper boundaries.

Signoff status: ready for leader review.
