# Compositor-Next v3.1 Highlighter + Player-Next Signoff

Date: 2026-05-01
Status: signed off for phase commit

## Scope

- Primitive: `shader.highlighter`
- Family: shader
- Player path: `tui-vfx-player-next` pure v3.1 facade
- Shared load path: `tui_vfx_compositor_next::v31::LoadedV31Recipe::load`

## Direct v3.1 Path

```text
┌──────────────────────────────────────────────────────────┐
│ Canonical v3.1 RecipeDocument                            │
│ - no legacy input format                                 │
│ - descriptor/catalog validation                          │
└──────────────────────────────┬───────────────────────────┘
                               │ load once
                               ▼
┌──────────────────────────────────────────────────────────┐
│ compositor-next::v31::LoadedV31Recipe                    │
│ - validates recipe.version == 3.1 and graph.version == 3.1│
│ - validates every authored source/effect input is literal │
│ - rejects unsupported highlighter textContrast at load    │
└──────────────────────────────┬───────────────────────────┘
                               │ shared accepted structure
                 ┌─────────────┴─────────────┐
                 ▼                           ▼
┌──────────────────────────────┐ ┌──────────────────────────────┐
│ compositor-next direct tests │ │ player-next visual test path  │
│ render_v31_recipe(...)       │ │ load_player_next_recipe(...) │
│                              │ │ render_player_next_recipe(...)│
└──────────────────────────────┘ └──────────────────────────────┘
                 │                           │
                 └─────────────┬─────────────┘
                               ▼
┌──────────────────────────────────────────────────────────┐
│ tui-vfx-compositor-next v3.1 renderer                    │
│ shader.highlighter -> copied HighlighterShader behavior  │
│ output: V31Frame with grid + applied effect evidence     │
└──────────────────────────────────────────────────────────┘
```

## Descriptor / Schema Evidence

- Descriptor path: `descriptors/v3.1/packs/primitive.json`
- Hindsight audit: `docs/arch/v31-primitive-schema-hindsight-audit.md`
- Plan update: `docs/arch/compositor-next-vertical-implementation-plan.md`

## Runtime Decisions

- `color`: descriptor color literal maps to `ColorConfig`.
- `applyTo`: supports canonical descriptor values `foreground`, `background`, and `both`.
- `mode`: supports canonical descriptor value `band`; descriptor-valid `row` and `centerOut` are rejected at load time until direct compositor semantics are implemented.
- `direction`: supports canonical descriptor values `leftToRight`, `rightToLeft`, `topToBottom`, and `bottomToTop`.
- `rowMask`: integer `>= 0` maps to a one-row compositor range for this slice.
- Source inputs: every authored source input must be literal at load time, including styling inputs that the first direct renderer currently ignores.
- `textContrast`: only `0.0` is accepted; values above `0.0` fail at load time because direct compositor-next currently maps only to `TextContrast::Preserve`.
- No separate player-next loader logic was added; player-next delegates to the compositor-next v3.1 loader.

## Generated Files

None in this slice. The direct path is hand-owned until the Primitive Workbench generated scaffold is introduced for a later slice.

## Hand-Owned Runtime Files

- `crates/tui-vfx-compositor-next/src/v31/load.rs`
- `crates/tui-vfx-compositor-next/src/v31/render.rs`
- `crates/tui-vfx-player-next/src/lib.rs`

## Fixtures and Tests

- `crates/tui-vfx-compositor-next/tests/test_v31_direct_recipe.rs`
  - `load_validated_v31_highlighter_renders_directly_in_compositor_next`
  - `rejects_unsupported_highlighter_inputs_at_load_time`
  - `rejects_descriptor_valid_highlighter_modes_without_direct_support`
  - `rejects_runtime_sourced_source_style_inputs_at_load_time`
- `crates/tui-vfx-player-next/tests/player_next_v31.rs`
  - proves player-next uses the same loader/render path for accepted v3.1 recipes
  - proves non-v3.1 and runtime-sourced inputs fail before render

## AI De-Slop Pass

Behavior lock:

```bash
cargo test -p tui-vfx-compositor-next --test test_v31_direct_recipe -- --nocapture
cargo test -p tui-vfx-player-next --test player_next_v31 -- --nocapture
```

Cleanup scope:

- `crates/tui-vfx-compositor-next/src/v31/load.rs`
- `crates/tui-vfx-player-next/Cargo.toml`
- `crates/tui-vfx-player-next/src/lib.rs`

Passes completed:

1. Duplicate removal: consolidated repeated direct-input literal error construction behind `require_declared_inputs_literal` and `literal_direct_value`.
2. Boundary cleanup: canonicalized highlighter enum support to descriptor values only and required every authored source input to be literal.
3. Naming/doc cleanup: replaced stale bridge/shim wording in player-next crate headers with compatibility-layer wording while preserving the no-legacy-support policy.
4. Test reinforcement: targeted compositor-next/player-next tests re-ran green after cleanup.

## OFPF / File-Size Review

Touched Rust files after cleanup:

```text
375 crates/tui-vfx-compositor-next/src/v31/load.rs
465 crates/tui-vfx-compositor-next/src/v31/render.rs
433 crates/tui-vfx-compositor-next/tests/test_v31_direct_recipe.rs
 33 crates/tui-vfx-player-next/src/lib.rs
161 crates/tui-vfx-player-next/tests/player_next_v31.rs
```

Decision: no file is above the hard 500 LOC split threshold. `load.rs`, `render.rs`, and the integration test are above the ~300 LOC target, but remain cohesive for this slice because splitting now would create tiny single-primitive modules before the third direct primitive proves the extraction boundary. Revisit after `shader.focusField` or after the Primitive Workbench generated-accessor boundary lands.

## Validation Commands

Targeted commands already passed before final review:

```bash
cargo test -p tui-vfx-compositor-next --test test_v31_direct_recipe -- --nocapture
cargo test -p tui-vfx-player-next --test player_next_v31 -- --nocapture
```

Final command log completed after review iteration: targeted compositor-next direct recipe tests now report 9 passed and player-next tests report 3 passed; full phase verification also passed.

## Known Risks / Deferred Work

- `textContrast` and richer `rowMask` semantics remain future descriptor/runtime decisions.
- This direct slice is still hand-owned; generated workbench accessors are deferred until enough primitive repetition proves the correct generated boundary.
- Human visual verification can now use `tui-vfx-player-next`, but no interactive player-next CLI has been added in this slice.
- No non-descriptor aliases are accepted in the highlighter direct path; legacy naming remains outside canonical v3.1.
