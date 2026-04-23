# Packet 17 — centralized loader dispatch audit

## Objective
Audit the centralized recipe loading/dispatch seam to identify the next most valuable blocker for keeping all tools on one authoritative load path.

## Why this matters
Multiple tools depend on `load_recipe_document` / V3 load helpers. If dispatch drifts or bypasses creep in, tooling truth diverges.

## Mode
BLOCKER_MODE

## Success condition
- identify one concrete loader/dispatch consistency blocker
- map which tools already use the centralized seam and which still risk bypassing it
- recommend one next fix lane

## In scope
- `/usr/projects/tui-vfx-recipes/src/recipe/`
- `/usr/projects/tui-vfx-recipes/src/v3/fnc_load_v3_document.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/fnc_load_v3_normalized.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/fnc_load_v3_compiled.rs`
- tool entrypoints that call loading code directly, if any

## Out of scope
- recipe content
- runtime rendering changes
- broad schema/export work

## Verification required
- evidence of where the centralized seam is used
- evidence of any bypass or inconsistency
- exact commands/tests if a narrow loader test exists

## Reporting format
Report:
- current authoritative load path
- any bypasses or confusing alternatives
- one recommended next fix lane
- exact files involved

## Task reminder
Your task is still: audit centralized loading/dispatch consistency, not rewrite the whole loader architecture.
