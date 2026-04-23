# Packet 45 — probe/validator ordered-preview truth fix

## Task first
Fix the seam where validator stage reporting loses sampler/shader/spec truth on supported ordered V3 trees.

## Why this matters
Probe surfaces currently preserve more bridge truth than validator stage output for the same ordered V3 recipe. That makes diagnostics inconsistent and understates actual supported behavior.

## Success condition
- validator stage output preserves the same meaningful bridge truth that probe still sees for the target ordered-preview recipe class
- targeted tests prove the fix
- no broad validator redesign

## Mode
BLOCKER_MODE

## Task-scope paths for grounding
- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_render_compiled_plan_deterministically.rs`
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/fnc_validate_v3_compiled_recipe.rs`
- `/usr/projects/tui-vfx-recipes/src/probe/fnc_build_probe_spec_from_preview.rs`
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/tests/test_v3_compiled_bridge.rs`

## Exact write scope
- the smallest seam in `fnc_render_compiled_plan_deterministically.rs` and/or validator reporting needed to preserve ordered-preview truth
- the narrowest validator tests proving the restored truth

## Out of scope
- broad replay/runtime redesign
- unrelated timing changes
- recipe corpus edits

## Must-read docs in order
1. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
2. `/usr/projects/gt-design/docs/superpowers/handoffs/2026-04-23-v3-session-audit-synthesis.md`
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
5. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
6. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`

## Verification required
- one representative validator JSON run on an ordered V3 tree
- one representative probe JSON run on the same recipe
- targeted validator tests for the restored truth surface

## Task reminder
Your task is still: restore ordered-preview truth in validator/probe reporting, not redesign the bridge.
