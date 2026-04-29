Below is the formal response to send after J2, assigning **K0**.

````md
# ARCH-RESP-TO-PHASE_J2.md

## Decision

Assuming Phase J2 is complete and the shared primitive descriptor catalog is now in place, the next phase is:

Phase K0 — Contract-Native Skeleton Player

K0 should build the first player that consumes canonical v3.1 `RecipeDocument` files directly. It should not wrap or depend on the old `/usr/projects/tui-vfx-recipes/src` runtime.

## Core instruction

Build a tiny contract-native player in `/usr/projects/tui-vfx` that can:

- load a canonical v3.1 recipe from `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/`
- load/use the J2 shared descriptor catalog
- validate before rendering
- render one sampled frame into a semantic surface/scene
- emit deterministic JSON frame output
- report unsupported sources/effects explicitly instead of silently succeeding

## Important constraint

Do not mutate the legacy recipe corpus:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/
````

Only use it as evidence. The canonical migrated fixtures live under:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

## K0 scope

Minimum player support:

```text
source.card
source.text if present in J2
filter.dim
filter.tint
filter.invert
filter.greyscale
mask.none
mask.wipe
mask.checkers
sampler.sineWave if present in the migrated primitive set
basic lifecycle sampling: enter / dwell / exit
trigger-terminated dwell using the I0 lifecycle trigger contract
deterministic render hash
non-empty cell count
JSON frame report
```

Do not implement:

```text
legacy recipe loading
old V3 authoring syntax
template expansion
full visual parity
full runtime stores
all procedurals
all image / ANSI sources
all effect ports
studio controls
live UI playback
old tui-vfx-recipes runtime dependency
```

## Suggested deliverables

Add a player library/API, either inside `tui-vfx-next` or a focused new crate. Suggested vocabulary:

```text
RecipePlayer
PlayerSession
PlayerSampleRequest
PlayerFrame
PlayerFrameReport
PlayerError
UnsupportedSource
UnsupportedEffect
```

Add a CLI command, preferably a new player CLI rather than expanding the validator beyond its purpose:

```text
cargo run -p tui-vfx-player-cli -- render-recipe \
  --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json \
  --descriptor-pack <J2 descriptor pack path> \
  --phase dwell \
  --phase-t 1.0 \
  --width 80 \
  --height 24 \
  --json
```

Also support recursive smoke rendering:

```text
cargo run -p tui-vfx-player-cli -- render-recipe \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
  --descriptor-pack <J2 descriptor pack path> \
  --json
```

## Expected JSON frame shape

Use a stable schema label, for example:

```json
{
  "schemaVersion": "v3.1.player.frame.1",
  "recipeId": "debug.baseline",
  "path": "...",
  "status": "rendered",
  "phase": "dwell",
  "phaseT": 1.0,
  "loopT": null,
  "width": 35,
  "height": 3,
  "renderHash": 123,
  "nonEmptyCells": 42,
  "rows": ["..."],
  "errors": [],
  "warnings": []
}
```

Unsupported features should be explicit:

```json
{
  "status": "unsupported",
  "errors": [
    {
      "code": "unsupportedEffectAdapter",
      "path": "graph.nodes.x.effect",
      "message": "No player adapter registered for shader.radialSpiral"
    }
  ]
}
```

## Reference evidence from old player

Use these old files as behavioral reference only:

```text
/usr/projects/tui-vfx-recipes/src/preview/cls_direct_v3_preview_state.rs
/usr/projects/tui-vfx-recipes/src/preview/cls_direct_v3_preview_snapshot.rs
/usr/projects/tui-vfx-recipes/src/preview/fnc_render_direct_v3_snapshot.rs
/usr/projects/tui-vfx-recipes/src/v3/compile/cls_v3_playback_timing.rs
/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_render_compiled_plan_deterministically.rs
/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_execute_compiled_step_tree_to_scene.rs
/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_build_scene_source_from_compiled_plan.rs
/usr/projects/tui-vfx-recipes/tests/test_packet_69e_event_driven_dwell.rs
```

Do not copy the old architecture. Extract only the stable lessons:

```text
phase/sample timing must be explicit
absolute time and loop time are distinct
trigger latch state belongs in player session state
reset clears latch state
render output needs grid + roles + hash + non-empty count
unsupported runtime paths must fail loudly
valid canonical recipe does not imply visual parity
```

## Tests required

At minimum:

```text
cargo test -p tui-vfx-contract
cargo test -p tui-vfx-next
cargo test -p tui-vfx-player-cli
cargo run -q -p tui-vfx-contract-cli -- validate-recipe --json --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
cargo run -q -p tui-vfx-player-cli -- render-recipe --json --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
cargo tree -p tui-vfx-player-cli
cargo test --workspace
git diff --check
```

Dependency guardrail:

```text
tui-vfx-player-cli must not depend on /usr/projects/tui-vfx-recipes/src
tui-vfx-player-cli must not depend on legacy recipe runtime crates
```

## Acceptance criteria

K0 is accepted when:

```text
1. canonical J2 fixtures still validate
2. the K0-supported primitive fixture subset renders frames
3. unsupported features produce structured unsupported diagnostics
4. repeated renders with the same input produce the same hash
5. event-driven dwell trigger behavior has a stateful session test
6. no old recipe files are modified
7. docs/VOCABULARY.md is updated with Player / Frame / PlayerSession / RenderHash terms
8. a PHASE_K0_STATUS_MEMO_TO_ARCHITECT.md report is produced
```

## Request after K0

Report:

```text
which fixtures rendered
which fixtures were unsupported
which adapters were implemented
which descriptor IDs remain unimplemented
whether any schema pressure was discovered
whether the player needed any contract changes
```

```
```
