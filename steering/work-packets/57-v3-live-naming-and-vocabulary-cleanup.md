# Packet 57 — V3 live naming and vocabulary cleanup

## Task first
Remove stale future-facing terminology from live docs/prompts/guidance so the project stops teaching the wrong V3 naming and vocabulary during cutover.

## Why this matters
The rename inventory explicitly says the next concrete rename-bearing bucket is live docs/comments/prompts in `tui-vfx` proper. Until that cleanup happens, the project keeps teaching stale `Ra*`-era framing and inconsistent V3 vocabulary even when the target direction is already agreed.

## Success condition
- one bounded live-docs/prompts terminology tranche is cleaned up
- docs prefer the intended `Vfx*` / current V3 vocabulary where appropriate
- historical/archive docs remain historically accurate

## Mode
FAMILY_MODE

## Task-scope paths for grounding
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-ra-to-vfx-rename-inventory.md`
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-schema-overview.md`
- `/usr/projects/tui-vfx/steering/INTENTIONS.md`
- live prompts / docs / guidance files in `tui-vfx` proper that still teach stale future-facing terms

## Exact write scope
- only the chosen live docs/comments/prompts tranche in `tui-vfx` proper
- the smallest supporting note explaining the cutover if needed

## Out of scope
- archive-wide mass rename
- the main Rust public surface rename event in `tui-vfx-recipes`
- broad generated-doc rewrites

## Must-read docs in order
1. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-ra-to-vfx-rename-inventory.md`
2. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-schema-overview.md`
3. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
4. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
5. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
6. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`

## Verification required
- diff review showing only live terminology cleanup, not history rewriting
- one explicit note that archives/decision docs were left accurate where history itself is the fact

## Task reminder
Your task is still: clean up live V3 naming/vocabulary guidance, not perform the whole Rust public-surface rename in one packet.
