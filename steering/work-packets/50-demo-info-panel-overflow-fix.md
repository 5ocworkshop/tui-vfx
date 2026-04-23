# Packet 50 — demo info-panel overflow fix

## Task first
Fix the preview info-panel overflow so long descriptions remain visible without overlapping other fields.

## Why this matters
The current demo can obscure important info when long descriptions collide with the fixed info layout. This hurts evaluation quality even if playback controls work.

## Success condition
- long descriptions no longer sit on top of other items
- the info area still shows important metadata clearly
- behavior is proven in the demo/example surface

## Mode
BLOCKER_MODE

## Task-scope paths for grounding
- `/usr/projects/tui-vfx-recipes/examples/demo.rs`
- any exact helper functions in the same file that render the info pane

## Exact write scope
- `examples/demo.rs`
- only one nearby helper if needed and kept in the same demo surface

## Out of scope
- unrelated demo redesign
- recipe/runtime changes
- packet/library work

## Must-read docs in order
1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
5. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
6. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`

## Verification required
- `cargo build -p tui-vfx-recipes --example demo`
- `cargo test -p tui-vfx-recipes --example demo -- --nocapture`
- add a focused test if the seam is subtle

## Task reminder
Your task is still: fix info-panel overflow in the demo, not broaden into general demo redesign.
