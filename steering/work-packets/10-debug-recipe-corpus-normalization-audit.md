# Packet 10 — debug-recipe corpus normalization audit

## Task first
Audit the debug-recipe corpus and rank the next highest-value cleanup family without rewriting the corpus in this packet.

## Objective
Identify the next highest-value family of debug recipes that still violates the current fixture-quality rules, so later cleanup packets can be prioritized intelligently.

## Why this matters
The corpus is large. We should not spray edits randomly; we should target the most misleading or lowest-quality clusters first.

## Mode
BLOCKER_MODE

## Success condition
- one prioritized shortlist of bad families/fixtures
- each recommendation tied to specific rule violations
- no mass editing in this packet

## Task-scope paths for grounding
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/`
- relevant fixture-quality rules in INTENTIONS and briefing docs

## Exact write scope
- none by default; this is an audit packet
- if you must add a tiny note or shortlist artifact, state why and keep it narrowly scoped

## Out of scope
- bulk fixing the corpus
- validator code changes unless absolutely needed to prove a violation

## Must-read docs in order
1. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
2. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-recipe-vocabulary.md`
5. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`

## Repo-boundary guardrails
- Keep this packet at the audit/prioritization level.
- Do not “fix as you go” through the corpus in this lane.

## What to score
Look for:
- missing/weak descriptions
- misleading body text
- bad layout/contrast
- unclear effect visibility
- wrong directory classification
- stale `_DEPRECATED_` drift where it still matters

## Verification required
- evidence from exact fixture files
- a short ranked list with reasons

## Reporting format
Report:
- top offenders
- violated rules per offender
- recommended cleanup order

## Task reminder
Your task is still: audit and prioritize, not rewrite the corpus.
