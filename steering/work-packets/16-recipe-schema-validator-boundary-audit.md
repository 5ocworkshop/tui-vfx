# Packet 16 — recipe_schema validator boundary audit

## Objective
Audit the current `recipe_schema` validation boundary and identify the highest-value next blocker in schema-level validation for V3-related authoring flows.

## Why this matters
The repo increasingly depends on schema-level truth surfaces. We need to know where schema validation still stops too early or becomes ambiguous.

## Mode
BLOCKER_MODE

## Success condition
- identify one concrete schema-level blocker
- exact files and tests are named
- evidence distinguishes schema validation from compiled/runtime validation

## In scope
- `/usr/projects/tui-vfx-recipes/src/recipe_schema/`
- `/usr/projects/tui-vfx-recipes/tests/recipe_schema.rs`
- nearby schema validator helpers only if directly relevant

## Out of scope
- compiled runtime code
- validator CLI stages unless used only as contrast/evidence
- recipe corpus edits

## Verification required
- evidence from schema validator files and tests
- if you can run a targeted schema test, do so
- do not drift into compiled-validation fixes

## Reporting format
Report:
- top schema-boundary gap
- exact file/test seam
- why it belongs at schema level rather than compiled/runtime level
- exact proof commands for the future implementation lane

## Task reminder
Your task is still: find the next schema-validation blocker, not fix compiled/runtime validation.
