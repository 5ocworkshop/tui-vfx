# Packet 39 — Madeira reference fixtures and baselines

## Task first
Establish the smallest trustworthy Madeira-specific baselines needed to catch regressions after implementation slices land.

## Objective
Create or tighten the Madeira-specific reference/baseline fixtures or proof artifacts needed to guard against regressions once implementation slices land.

## Why this matters
Madeira is a showcase target. Once behavior improves, we need trustworthy regression anchors rather than relying on memory.

## Mode
FAMILY_MODE

## Prerequisites
- at least one Madeira implementation tranche complete
- enough behavior exists to capture a meaningful baseline

## Success condition
- one or more Madeira-focused baseline/reference artifacts exist or are improved
- regression intent is explicit
- artifacts are trustworthy and explain what they prove

## Task-scope paths for grounding
- Madeira-specific tests, deterministic render locks, validator/probe expectations, or documented baseline artifacts
- supporting docs/comments explaining what the baseline proves

## Exact write scope
- the smallest Madeira-specific baseline/test/artifact set needed
- the narrowest supporting doc/comment updates

## Out of scope
- wide recipe corpus edits unrelated to Madeira
- speculative new behavior without proof

## Must-read docs in order
1. current Madeira implementation/audit outputs
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`

## Repo-boundary guardrails
- Keep the packet on regression anchors and proof surfaces.
- Do not use it as a broad feature-expansion lane.

## Verification required
- commands showing the baseline artifacts/tests are green
- clear statement of what each baseline guards

## Reporting format
Report:
- exact baselines added/updated
- exact commands proving them
- what future regressions they are meant to catch

## Task reminder
Your task is still: establish Madeira regression anchors, not broaden feature scope.
