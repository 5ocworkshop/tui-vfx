# Packet 05 — debug-recipes QC for V3

## Task first
Strengthen the V3 debug-recipes QC tooling path so it gives useful, trustworthy quality signals without widening into full corpus cleanup.

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

## Task-scope paths for grounding
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/fnc_run_debug_recipes_qc.rs`
- `/usr/projects/tui-vfx-recipes/tests/test_debug_recipes_qc.rs`
- nearby probe/QC helpers only if required

## Exact write scope
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/fnc_run_debug_recipes_qc.rs`
- `/usr/projects/tui-vfx-recipes/tests/test_debug_recipes_qc.rs`
- the smallest nearby probe/QC helper only if clearly required

## Out of scope
- mass visual rewriting of recipe fixtures
- broad validator refactor
- runtime render changes unless directly required for QC truthfulness

## Must-read docs in order
1. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
2. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
3. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-recipe-vocabulary.md`
4. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
5. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
6. `/usr/projects/global_prompts/standards/60_file_centric_execution.md`

## Repo-boundary guardrails
- This is tooling/QC work, not recipe-corpus cleanup.
- Do not broaden into mass fixture rewriting in this packet.

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
