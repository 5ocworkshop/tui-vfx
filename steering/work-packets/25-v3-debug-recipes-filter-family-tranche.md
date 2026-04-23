# Packet 25 — V3 debug-recipes filter-family cleanup tranche

## Task first
Clean up exactly one selected filter-family debug-recipe tranche using the approved fixture-quality rules.

## Objective
Run one bounded cleanup tranche on the highest-priority filter-family debug recipes identified by the audit packets, applying the approved fixture-quality rules.

## Why this matters
The native-only/native-mix filter fixtures are part of how we assess migration quality. Once the audit identifies the worst offenders, we need a disciplined cleanup tranche.

## Mode
FAMILY_MODE

## Prerequisites
- Packet 10 or Packet 21 complete with a ranked shortlist
- the chosen tranche is explicitly named

## Success condition
- one filter-family tranche is cleaned up
- descriptions/body text/layout/contrast/timing are corrected where needed
- validator/QC expectations still pass

## Task-scope paths for grounding
- only the selected filter-family fixtures and closely related tests/QC references

## Exact write scope
- only the selected filter-family fixtures
- the smallest related QC/test file(s) if directly required by the tranche

## Out of scope
- the whole debug-recipes corpus
- unrelated content/style/mask fixture work
- runtime filter semantics changes unless clearly required and separately approved

## Must-read docs in order
1. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
2. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
3. relevant audit shortlist for the chosen tranche
4. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
5. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-recipe-vocabulary.md`
6. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`

## Repo-boundary guardrails
- This is recipe-fixture cleanup, not runtime filter semantics work.
- Keep the tranche confined to the chosen family only.

## Verification required
- targeted fixture/QC checks
- any related coverage tests
- proof that the cleaned fixtures now clearly show the intended effect

## Reporting format
Report:
- exact fixtures touched
- exact quality issues fixed
- exact verification
- remaining filter-family fixtures left for later tranches
