Yes. The refined direction should be:

```text
K0 is the established contract-native CLI player.
The next phase is not a replacement for K0.
The next phase is a basic visual GUI/TUI shell layered on top of the K0 player capability.
```

Below is the formal version you can send.

---

# Architect Response — Phase K1 Assignment

Date: 2026-04-29
Implementation repo: `/usr/projects/tui-vfx`
Recipe repo: `/usr/projects/tui-vfx-recipes`
Previous phase assumed complete: **K0 — Contract-Native v3.1 CLI Player**
Next phase: **K1 — Basic Visual Player UI on Top of K0**

## Executive decision

K0 should be treated as the established contract-native player path. The next phase should **not** replace, fork, or weaken the K0 CLI player.

The correct next step is:

```text
Phase K1 — Basic Visual Player UI on Top of K0
```

K1 should build a small visual player interface that reuses the K0 loading, descriptor-pack resolution, validation, lifecycle sampling, rendering, snapshot, and reporting logic.

The GUI/TUI is an additional interactive surface. The CLI remains the authoritative regression and automation surface.

## Key architectural rule

Do not create a second player engine.

K1 should use the same underlying K0 player code path that the CLI already uses:

```text
RecipeDocument JSON
+ DescriptorPack / DescriptorCatalog
+ validate_with_catalog()
+ K0 player state / timing / runtime inputs
+ K0 render snapshot
→ visual presentation
```

The new UI may add browsing, keybindings, frame ticking, pause/resume, and visual display, but it must not invent separate lifecycle, trigger, descriptor, source, graph, or rendering semantics.

## Why this matters

The CLI player is the stable contract test harness. It is scriptable, CI-friendly, and easy for agents to inspect. The visual player is for human confirmation and interactive debugging.

Going forward, the validation ladder should be:

```text
1. Contract validation
2. K0 CLI player render/snapshot validation
3. K1 visual player human inspection
4. Later probe/frame-diff tooling
```

K1 adds step 3. It does not replace steps 1 or 2.

## Reference code

Use the existing demo as reference for interaction design only:

```text
/usr/projects/tui-vfx-recipes/examples/demo.rs
```

That demo already proves useful UI patterns:

```text
recipe browser
preview pane
pause/resume
motion-disabled mode
restart/reload
phase cycling
sample_t scrubbing
event-driven dwell trigger key
render hash / non-empty cell display
FPS and frame timing display
basic help modal
```

However, K1 must not depend on the old `tui-vfx-recipes` runtime path as its core execution model.

The old demo uses transitional APIs such as:

```text
preview_from_recipe_path_with_cutover_fallback
DirectV3PreviewState
load_runtime_recipe_with_cutover_fallback
render_v3_frame_to_buffer
```

Those are useful references, not the new canonical player contract.

## K1 objective

Create a basic visual player UI in `/usr/projects/tui-vfx` that can interactively preview canonical v3.1 recipes using the already-established K0 CLI/player engine.

Minimum target corpus:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

Descriptor pack:

```text
/usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json
```

The UI should be able to load and display the same recipes that the K0 CLI player can render.

## Required implementation behavior

K1 should support:

```text
load one canonical v3.1 recipe path
load descriptor pack path
validate recipe before rendering
render current K0 snapshot into a terminal visual surface
tick elapsed time when unpaused
pause / resume
restart / reset
motion-disabled stable sample
phase cycle
sample_t scrub
show recipe path
show phase
show sample_t
show loop_t when present
show render hash
show non-empty cell count
show unsupported primitive diagnostics if K0 reports any
show validation/render errors clearly
```

If implementing a browser is small, include it. If it risks scope creep, start with:

```text
cargo run -p tui-vfx-player-ui -- \
  --descriptor-pack /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json
```

A browser can follow once the single-recipe player is stable.

## CLI preservation requirement

K1 must keep the K0 CLI player working exactly as before.

Before and after K1 changes, the implementer should run the K0 CLI regression commands and confirm no behavioral drift.

Expected command shape, adjusted to the actual K0 binary name if different:

```text
cargo run -p tui-vfx-player -- render-recipe \
  --descriptor-pack /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json \
  --elapsed-ms 1000 \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json
```

And recursive smoke:

```text
cargo run -p tui-vfx-player -- render-recipes \
  --descriptor-pack /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

K1 should add UI tests without weakening these CLI tests.

## Preferred structure

If K0 currently has shared library code plus CLI code, add a UI binary that calls the shared library.

Preferred shape:

```text
crates/tui-vfx-player/
  src/lib.rs
  src/bin/tui-vfx-player.rs        # existing or K0 CLI
  src/bin/tui-vfx-player-ui.rs     # new K1 UI
```

If K0 currently put too much logic directly in the CLI binary, K1 may first extract shared player logic into library modules, but that extraction must be behavior-preserving.

Suggested shared API surface:

```text
PlayerConfig
PlayerState
PlayerRuntimeInputs
PlayerSnapshot
PlayerError
load_player_recipe(...)
initialize_player(...)
sample_player_at_elapsed(...)
render_player_snapshot(...)
```

The UI should call this shared surface, not duplicate it.

## Suggested UI keybindings

Borrow the useful concepts from `examples/demo.rs`, simplified for K1:

```text
q       quit
?       help
space   pause / resume
r       reset / reload
m       motion-disabled stable sample
[       previous phase
]       next phase
left    sample_t - 0.05
right   sample_t + 0.05
t       fire event-driven signal/trigger input, when applicable
D       dump current snapshot/grid to a temp/debug file, if cheap
```

For `t`, use canonical v3.1 trigger/signal vocabulary from I0, not old `pipeline.timing.dwell_until_binding` field names.

## Event-driven dwell behavior

K1 should expose a simple way to fire canonical signal-backed triggers.

If a recipe has a lifecycle dwell trigger backed by a signal such as:

```text
userDismissed
```

the UI can bind `t` to set that runtime signal to `true`.

Expected behavior:

```text
without signal: dwell continues until maxDuration
with signal: dwell terminates and exit begins
reset: clears latch/runtime input state
```

This should reuse K0’s runtime input and lifecycle logic.

## Non-goals

K1 should not attempt:

```text
full visual parity with old recipes
full old demo replacement
legacy recipe browser parity
legacy fallback loading
full corpus migration
all asset loading
all procedural rendering
all source kinds
all primitive families beyond K0-supported set
frame-level diff/probe tooling
production-grade UI polish
```

K1 is a basic visual shell over the K0 player.

## Acceptance criteria

K1 is acceptable when:

```text
K0 CLI player still passes its existing tests and recursive fixture smoke.
The new UI can open at least one canonical v3.1 fixture and render a visible surface.
The UI renders through the K0 player/snapshot path, not a duplicated execution path.
The UI can pause/resume and reset.
The UI can display phase, sample_t, render_hash, and non_empty_cells.
The UI can scrub or phase-cycle at least for deterministic inspection.
The UI can run against /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/ with the primitive descriptor pack.
Any unsupported primitive is reported explicitly rather than silently ignored.
```

## Suggested verification

Run the existing contract and descriptor-pack checks:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Run J2 validator regression:

```text
cargo run -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

Run K0 CLI player regression:

```text
cargo run -p tui-vfx-player -- render-recipes \
  --descriptor-pack /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

Run the new K1 UI manually against at least:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_dim.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_wipe.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/samplers/sampler_sinewave.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/event_driven_dwell/bool_binding_demo.json
```

Use exact fixture names as present in the repo; if the sine wave file uses camel casing or another spelling from J0/J2, use the actual path.

## Required evidence memo

At the end of K1, produce:

```text
docs/new_kernel/PHASE_K1_STATUS_MEMO_TO_ARCHITECT.md
```

The memo should report:

```text
what UI binary or command was added
which K0 player APIs it reuses
which canonical fixtures were manually opened
which CLI regressions still pass
which primitives render visibly
which primitives are unsupported or degraded
whether event-trigger input was tested
confirmation that old recipes were not modified
confirmation that K0 CLI behavior was preserved
```

## Draft implementer prompt

```text
We have completed K0 as a contract-native v3.1 CLI player. Do not replace it.

Implement Phase K1: Basic Visual Player UI on top of the established K0 CLI/player capability.

Use /usr/projects/tui-vfx as the implementation repo.
Use /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/ as the canonical recipe fixture corpus.
Use /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json as the descriptor pack.

The goal is to add a small visual UI/TUI player that reuses the existing K0 player logic. The UI must not create a second player engine and must not depend on the legacy tui-vfx-recipes runtime/cutover path as its execution model.

Use /usr/projects/tui-vfx-recipes/examples/demo.rs as interaction reference only. It is useful for frame loop, pause/resume, motion-disabled mode, reset, phase/sample_t controls, trigger key, render hash display, and help modal ideas. Do not copy its legacy runtime dependency model.

If K0 player logic currently lives only in a CLI binary, first extract the shared player logic into reusable library code without changing CLI behavior. Then add a UI binary that calls the same shared K0 functions.

The UI should support at least:
- load one canonical v3.1 recipe path
- load descriptor pack path
- validate before render
- render current player snapshot visibly
- pause/resume
- reset/reload
- motion-disabled stable sample
- phase cycle
- sample_t scrub
- display path, phase, sample_t, loop_t if present, render_hash, non_empty_cells, validation/render errors
- fire a canonical signal-backed lifecycle trigger when the recipe supports one

Preserve the K0 CLI. K1 is additive only. Run the K0 CLI recursive fixture smoke before and after K1 and confirm no drift.

Do not modify old recipes under:
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/

Do not create migrated recipes in the implementation repo. The canonical migrated corpus remains under:
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/

At completion, write:
docs/new_kernel/PHASE_K1_STATUS_MEMO_TO_ARCHITECT.md

The memo must include verification commands, fixture coverage, what UI command was added, what K0 APIs it reuses, and explicit confirmation that the CLI player remains intact.
```
