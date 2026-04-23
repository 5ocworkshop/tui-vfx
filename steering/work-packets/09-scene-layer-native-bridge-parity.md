# Packet 09 — scene-layer/native bridge parity

## Task first
Close one concrete scene-bearing native/direct bridge gap without widening into a scene-system redesign.

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

## Task-scope paths for grounding
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_build_scene_source_from_compiled_plan.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_execute_compiled_step_tree_to_scene.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_render_compiled_plan_deterministically.rs`
- narrow tests that prove the closed gap

## Exact write scope
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_build_scene_source_from_compiled_plan.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_execute_compiled_step_tree_to_scene.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_render_compiled_plan_deterministically.rs`
- the smallest scene-bearing test file(s) that prove the closed gap

## Out of scope
- debug recipe corpus
- unrelated timing work
- broad scene architecture redesign

## Must-read docs in order
1. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
2. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
3. relevant scene-bearing audit result(s)
4. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-recipe-vocabulary.md`
5. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
6. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`

## Repo-boundary guardrails
- Keep this lane in `/usr/projects/tui-vfx-recipes` scene-bearing compile/render seams.
- Do not reopen generalized timing normalization unless the exact scene gap demands it.

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
