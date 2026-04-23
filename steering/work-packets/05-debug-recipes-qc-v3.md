# Packet 05 — debug-recipes QC for V3

## Objective
Improve the V3 debug-recipes QC path so it provides trustworthy, useful quality checks instead of shallow pass-through behavior.

## Why this matters
Debug recipes are visual references and release baselines. If the QC path is weak for V3, the corpus becomes less trustworthy during migration.

## Mode
FAMILY_MODE

## Success condition
- V3 debug-recipes QC provides useful checks for representative V3 fixtures
- results are machine-readable and meaningful
- the lane remains inside tooling/QC seams

## In scope
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/fnc_run_debug_recipes_qc.rs`
- `/usr/projects/tui-vfx-recipes/tests/test_debug_recipes_qc.rs`
- nearby probe/QC helpers only if required

## Out of scope
- mass visual rewriting of recipe fixtures
- broad validator refactor
- runtime render changes unless directly required for QC truthfulness

## Extra guardrail
Because this is FAMILY_MODE, you may improve adjacent QC/reporting seams if they are clearly part of making V3 QC useful. Do not widen into the entire recipe corpus.

## Required checks to think about
- description presence
- fixture categorization
- paired legacy bridge caveats
- output/probe usefulness
- whether QC reports explain what is actually being validated

## Verification required
- targeted QC tests
- representative `--debug-recipes-qc` runs on V3 fixtures
- proof that the report is more informative, not just still green

## Reporting format
Report:
- what V3 QC was missing before
- what new signals/checks it now provides
- exact files changed
- commands run
- remaining limitations

## Task reminder
Your task is still: strengthen V3 debug-recipes QC as a tooling surface, not rewrite the debug recipe corpus itself.
