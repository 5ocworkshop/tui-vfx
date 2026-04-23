# Packet 09 — scene-layer/native bridge parity

## Objective
Improve the scene-bearing V3 direct/native path where current rendering still relies on a shallower bridge than desired for richer scene-layer behavior.

## Why this matters
Scene-bearing plans are a major part of end-to-end V3 usefulness. If they only barely pass through the bridge, showcase recipes can appear “supported” without true parity.

## Mode
BLOCKER_MODE

## Success condition
- one specific scene-bearing gap is closed
- targeted scene-bearing tests pass
- no broad scene-engine rewrite

## In scope
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_build_scene_source_from_compiled_plan.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_execute_compiled_step_tree_to_scene.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_render_compiled_plan_deterministically.rs`
- narrow tests that prove the closed gap

## Out of scope
- debug recipe corpus
- unrelated timing work
- broad scene architecture redesign

## Performance watchpoints
- avoid repeated large scene/grid copies where a smaller seam fix works
- call out any new per-frame costs introduced by the parity fix

## Verification required
- narrow scene-bearing compiled tests
- representative validator or deterministic render proof

## Reporting format
Report:
- exact gap fixed
- exact changed files
- exact tests/commands
- performance caveats if any

## Task reminder
Your task is still: close one concrete scene-bearing native-bridge gap, not redesign scene composition wholesale.
