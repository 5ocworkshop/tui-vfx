# Packet 46 — procedural layer frame-equivalence regression

## Task first
Add a focused regression proving that `ProceduralLayer::paint()` preserves deterministic output equivalently to direct source rendering on a nontrivial rect.

## Why this matters
Current procedural determinism is broadly good, but the weakest remaining proof seam is whether the scene-layer composition path preserves the same output as the direct source path.

## Success condition
- one focused frame-equivalence regression exists
- it uses a stable stock source on a non-1x1 rect
- it proves scene-layer composition does not distort the source output

## Mode
BLOCKER_MODE

## Task-scope paths for grounding
- `/usr/projects/tui-vfx-recipes/src/scene/layers/cls_procedural_layer.rs`
- `/usr/projects/tui-vfx-recipes/tests/scene/layers/test_cls_procedural_layer.rs`
- `/usr/projects/tui-vfx-recipes/src/scene/procedural/sources/mod.rs`
- `/usr/projects/tui-vfx-recipes/tests/scene_procedural.rs`

## Exact write scope
- the procedural layer test file
- the smallest supporting helper/test file only if clearly necessary

## Out of scope
- procedural feature design
- large performance work
- probe/validator rewrites

## Must-read docs in order
1. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
2. `/usr/projects/gt-design/docs/superpowers/handoffs/2026-04-23-v3-session-audit-synthesis.md`
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
5. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
6. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`

## Verification required
- the new focused regression
- existing procedural scene tests still green

## Task reminder
Your task is still: strengthen deterministic proof, not redesign procedural sources.
