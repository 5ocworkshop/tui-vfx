# Packet 51 — Madeira diagnostic example PreviewItem Option fix

## Task first
Repair the current diagnostic example compile seam caused by the `loaded.item -> Option<PreviewItem>` API change.

## Why this matters
Packet 40 found that the bounded diagnostic surfaces `diag_render_dump` and `diag_timeline_dump` no longer compile, which weakens Madeira end-to-end operator proof even though the core recipe path is green.

## Success condition
- `examples/diag_render_dump.rs` and `examples/diag_timeline_dump.rs` compile again
- the fix is limited to the `Option<PreviewItem>` seam
- no broad diagnostic-example redesign

## Mode
BLOCKER_MODE

## Task-scope paths for grounding
- `/usr/projects/tui-vfx-recipes/examples/diag_render_dump.rs`
- `/usr/projects/tui-vfx-recipes/examples/diag_timeline_dump.rs`
- the current `loaded.item` / `PreviewItem` call sites they depend on

## Exact write scope
- the two diagnostic example files
- one smallest supporting helper only if strictly necessary

## Out of scope
- broad preview API redesign
- Madeira runtime changes
- packet/library work

## Must-read docs in order
1. `/usr/projects/gt-design/docs/superpowers/handoffs/2026-04-23-v3-session-audit-synthesis.md`
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
5. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
6. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`

## Verification required
- `cargo build -p tui-vfx-recipes --example diag_render_dump --example diag_timeline_dump`
- one smallest smoke run or test if available

## Task reminder
Your task is still: repair the bounded diagnostic-example compile seam, not redesign preview loading.
