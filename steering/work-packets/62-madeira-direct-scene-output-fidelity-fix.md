# Packet 62 — Madeira direct-scene output fidelity fix

## Task first
Fix the direct V3 Madeira scene-render bridge so the full 4-layer scene renders with resolved tokens and visible layer output before any finer aesthetic tuning.

## Why this matters
Packet 61 showed that the current Madeira recipe is structurally faithful on paper but not yet visually faithful in live direct output: the comparison against `/usr/projects/madeira-flag` found missing visible fireworks/flag output and unresolved placeholder text in the current scene-render path. Until the direct bridge renders the whole scene correctly, deeper motion/shading parity work is premature.

## Success condition
- direct V3 output renders all 4 intended Madeira layers visibly
- text tokens resolve instead of leaking placeholders
- the corrective slice is proven with focused commands before any broader parity tuning

## Mode
BLOCKER_MODE

## Task-scope paths for grounding
- `/usr/projects/tui-vfx/steering/work-packets/61-madeira-reference-repo-faithfulness-audit.md`
- `/usr/projects/madeira-flag/`
- `/usr/projects/tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json`
- the exact direct-scene render/preview bridge files involved in current Madeira output
- current Madeira baselines and direct-preview proofs

## Exact write scope
- only the exact direct-scene render/preview bridge seam that explains the missing layer output / unresolved tokens
- the narrowest supporting tests or proof surfaces required

## Out of scope
- deeper flag-wave/shading aesthetic retuning
- broad Madeira redesign
- non-Madeira runtime work

## Must-read docs in order
1. `/usr/projects/tui-vfx/steering/work-packets/61-madeira-reference-repo-faithfulness-audit.md`
2. `/usr/projects/gt-design/docs/superpowers/handoffs/2026-04-23-v3-session-audit-synthesis.md`
3. `/usr/projects/tui-vfx/steering/work-packets/41-madeira-visual-vetting-protocol.md`
4. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
5. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
6. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
7. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
8. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`

## Verification required
- exact commands proving the direct V3 Madeira output now shows all intended scene layers
- exact commands proving token resolution no longer leaks placeholders
- at least one side-by-side or structured comparison against `/usr/projects/madeira-flag` showing improvement

## Task reminder
Your task is still: fix the direct-scene output fidelity blocker, not solve every remaining Madeira parity issue in one packet.
