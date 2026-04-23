# Packet 60 — V3 doc-autogen and authoring-guide cutover

## Task first
Advance the outstanding V3 doc/autogen cutover so generated artifacts, editorial templates, and authoring guides describe the real V3 surface rather than a mixed or legacy-biased picture.

## Why this matters
Chapter 100 still lists several documentation blockers: V3-shape generated artifacts, editorial template alignment, and V3 rewrites for authoring/validator guides. The doc system exists, but the V3 cutover is still incomplete.

## Success condition
- one bounded doc/autogen cutover tranche lands
- the chosen generated or editorial artifacts now describe the real V3 surface more accurately
- one or more authoring guides stop teaching outdated V2-biased structure

## Mode
FAMILY_MODE

## Task-scope paths for grounding
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-upgrade-plan/100_tooling_ci_migration.md`
- `/usr/projects/tui-vfx/docs/INDEX.md`
- `/usr/projects/tui-vfx/docs/RECIPE_AUTHORING_WORKFLOW.md`
- `/usr/projects/tui-vfx/docs/RECIPE_VISUAL_QA.md`
- `/usr/projects/tui-vfx/docs/templates/capabilities.toml`
- `/usr/projects/tui-vfx/xtask/src/docs/`
- `/usr/projects/tui-vfx-recipes/docs/generated/V3_API.md`
- `/usr/projects/tui-vfx-recipes/tools/fnc_generate_v3_docs.py`

## Exact write scope
- the smallest coherent doc/autogen tranche among:
  - `xtask` V3-shaped generated artifacts
  - editorial template alignment
  - V3 authoring/validator guide rewrite
  - CI/Justfile doc-generation gate alignment if directly required

## Out of scope
- whole-docs rewrite in one pass
- unrelated runtime feature work
- archive migration

## Must-read docs in order
1. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-upgrade-plan/100_tooling_ci_migration.md`
2. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
3. `/usr/projects/tui-vfx/docs/INDEX.md`
4. the exact authoring/generated doc files chosen for the tranche
5. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
6. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
7. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`

## Verification required
- exact doc-generator / freshness / validation commands for the chosen tranche
- explicit statement of which V3 doc/autogen blocker from Chapter 100 was advanced

## Task reminder
Your task is still: land one bounded V3 doc/autogen tranche, not finish every documentation blocker in one pass.
