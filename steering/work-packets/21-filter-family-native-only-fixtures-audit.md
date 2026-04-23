# Packet 21 — filter family native-only fixtures audit

## Task first
Audit the native-only/native-mix filter fixtures and rank the next highest-value cleanup tranche without rewriting the corpus in this packet.

## Objective
Audit the growing set of `complex_filter_*_native_only.json` and related native-mix fixtures to identify the next highest-value fixture/coverage gap.

## Why this matters
These fixtures are supposed to demonstrate and regression-lock the direct/native path. If they are inconsistent, mislabeled, or under-verified, they lose value.

## Mode
BLOCKER_MODE

## Success condition
- one prioritized shortlist of native-only/native-mix fixture gaps
- evidence-backed next cleanup tranche recommendation
- no bulk editing in this audit packet

## Task-scope paths for grounding
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/complex/complex_filter_*`
- related filter coverage tests
- current debug recipe quality rules

## Exact write scope
- none by default; this is an audit packet
- if you must capture a shortlist artifact, keep it narrow and report why

## Out of scope
- rewriting the full native-only corpus
- runtime filter semantics changes
- unrelated content/style/mask recipes

## Must-read docs in order
1. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
2. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-recipe-vocabulary.md`
5. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
6. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`

## Repo-boundary guardrails
- Keep this packet at the audit/prioritization layer.
- Do not broaden into runtime filter semantics or whole-corpus rewriting.

## Verification required
- exact fixture inspection evidence
- any narrow coverage test evidence if needed

## Reporting format
Report:
- top offending native-only/native-mix fixtures
- why they are risky or misleading
- recommended cleanup order and next packet scope

## Task reminder
Your task is still: audit and prioritize the native-only filter fixtures, not rewrite them all.
