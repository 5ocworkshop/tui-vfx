# Packet 49 — scene-layer compile-normalize proof fix

## Task first
Repair the stale scene-layer compile-normalize proof surface so it matches the current typed V3 scene-layer source shape.

## Why this matters
A stale proof surface makes the scene-semantics story noisier than it should be. The audit found a red test caused by an old fixture shape, not by a real runtime regression.

## Success condition
- the stale scene-layer compile-normalize proof is updated to the current typed `type/spec` source shape
- the focused test passes
- no broad scene rewrite

## Mode
BLOCKER_MODE

## Task-scope paths for grounding
- `/usr/projects/tui-vfx-recipes/src/v3/compile/test_compile_normalized_recipe.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_compile_normalized_recipe.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/authoring/cls_v3_scene.rs`
- `/usr/projects/tui-vfx-recipes/tests/recipe_schema/test_cls_ra_scene_layer.rs`

## Exact write scope
- the stale proof/test file
- the smallest supporting fixture/helper only if required

## Out of scope
- runtime scene semantics changes
- Madeira effect work
- broad schema rewrite

## Must-read docs in order
1. `/usr/projects/gt-design/docs/superpowers/handoffs/2026-04-23-v3-session-audit-synthesis.md`
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
5. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
6. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`

## Verification required
- targeted compile-normalize scene-layer test command that was previously red

## Task reminder
Your task is still: fix the stale proof surface, not broaden into scene implementation.
