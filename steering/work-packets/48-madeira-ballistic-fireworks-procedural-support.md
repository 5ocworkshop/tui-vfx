# Packet 48 — Madeira ballistic fireworks procedural support

## Task first
Implement stock procedural support for `ballistic_fireworks` as the next bounded Madeira effect-capability slice.

## Why this matters
The clearest remaining effect-focused Madeira gap is that `ballistic_fireworks` is authored in the recipe but missing from the stock procedural registry/runtime.

## Success condition
- `ballistic_fireworks` exists as a stock procedural source
- the registry resolves it without fallback
- one Madeira-specific proof confirms the recipe no longer treats that layer as missing support

## Mode
BLOCKER_MODE

## Task-scope paths for grounding
- `/usr/projects/tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json`
- `/usr/projects/tui-vfx-recipes/src/scene/procedural/cls_procedural_registry.rs`
- `/usr/projects/tui-vfx-recipes/src/scene/procedural/sources/mod.rs`
- `/usr/projects/tui-vfx-recipes/src/scene/procedural/`
- `/usr/projects/tui-vfx-recipes/tests/scene_procedural.rs`

## Exact write scope
- the new fireworks procedural source file
- registry wiring
- the narrowest scene/procedural tests needed
- one Madeira-specific proof seam if justified

## Out of scope
- flag asset parity
- text stack fidelity
- broad Madeira redesign

## Must-read docs in order
1. `/usr/projects/gt-design/docs/superpowers/handoffs/2026-04-23-v3-session-audit-synthesis.md`
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
5. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
6. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`

## Verification required
- scene/procedural tests proving registry resolution and non-fallback behavior
- one Madeira-specific proof command if available

## Performance note
This is hot-path-sensitive. Avoid per-frame allocations and repeated palette parsing.

## Task reminder
Your task is still: add `ballistic_fireworks` support narrowly, not broaden Madeira work.
