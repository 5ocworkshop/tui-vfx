# Packet 52 — Madeira direct-preview allocation/cache slice

## Task first
Reduce the clearest per-frame structural overhead in Madeira’s direct-preview path without broad engine optimization.

## Why this matters
Packet 42 concluded that Madeira is at risk for sustained 60 FPS because the direct-preview path still does avoidable per-render work: scene lowering/layer mapping churn, a leaked timing context seam, and repeated structural setup before rendering.

## Success condition
- one narrowly chosen direct-preview allocation/cache seam is improved
- the chosen seam is verified with the same Madeira path that motivated the audit
- no broad whole-engine optimization campaign

## Mode
BLOCKER_MODE

## Task-scope paths for grounding
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_build_scene_source_from_compiled_plan.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_render_compiled_plan_deterministically.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_apply_compiled_pipeline_replay_to_scene.rs`
- `/usr/projects/tui-vfx-recipes/src/scene/procedural/sources/cls_ballistic_fireworks.rs`
- Madeira-specific proof commands from Packet 42

## Exact write scope
- only the exact seam chosen before editing
- the narrowest supporting proof/test surface required

## Out of scope
- broad engine-wide optimization
- unrelated recipe/runtime redesign
- changing Madeira semantics while chasing performance

## Must-read docs in order
1. `/usr/projects/gt-design/docs/superpowers/handoffs/2026-04-23-v3-session-audit-synthesis.md`
2. current Packet 42 audit output
3. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
4. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
5. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
6. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
7. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`

## Verification required
- exact command(s) showing the chosen Madeira preview path still works
- one bounded before/after measurement or evidence-backed justification for reduced per-frame structural cost

## Task reminder
Your task is still: improve one Madeira-specific direct-preview overhead seam, not optimize the whole engine.
