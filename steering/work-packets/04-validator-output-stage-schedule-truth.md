# Packet 04 — validator output-stage schedule truth

## Task first
Make the V3 validator output-stage messaging and behavior truthful about fixed deterministic samples versus real scheduling.

## Objective
Tighten `pipeline-validator` output-stage behavior and messaging so V3 compiled output validation accurately communicates when it is using one fixed deterministic sample versus a real schedule.

## Why this matters
The current validator can be technically correct but still misleading if it implies schedule-aware behavior while using a fixed deterministic bridge sample. Users need truthful stage feedback.

## Mode
BLOCKER_MODE

## Success condition
- output-stage behavior and warnings are accurate
- targeted tests prove the intended message/behavior
- no broad runtime changes

## Task-scope paths for grounding
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/fnc_validate_v3_compiled_recipe.rs`
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/stages/functions/fnc_validate_output.rs`
- targeted validator tests under `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/tests/`

## Exact write scope
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/fnc_validate_v3_compiled_recipe.rs`
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/stages/functions/fnc_validate_output.rs`
- the smallest relevant validator test file(s) under `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/tests/`

## Out of scope
- debug recipe corpus
- core compiled runtime semantics unless absolutely required to preserve truthful messaging
- unrelated validator stages

## Must-read docs in order
1. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
2. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
3. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
4. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
5. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`
6. `/usr/projects/global_prompts/standards/60_file_centric_execution.md`

## Repo-boundary guardrails
- Keep this lane inside `/usr/projects/tui-vfx-recipes` validator tooling.
- Do not reopen broader compiled-runtime timing work unless the validator seam truly cannot be made truthful without it.

## First steps
1. Inspect current warning and stage messaging.
2. Confirm whether the behavior is:
   - fixed-sample deterministic bridge
   - partially schedule-aware
   - or inconsistent between CLI and tests
3. Tighten the smallest seam that restores truthfulness.

## OFPF guidance
- orientation on `/usr/projects/tui-vfx-recipes`
- inspect the validator files first
- read targeted tests before changing behavior

## Verification required
Use the narrowest commands that prove the seam:
- targeted `pipeline-validator` tests you add or touch
- representative CLI JSON run for a compiled V3 recipe

## Performance note
This is tooling/validator work. Runtime hot-path optimization is not expected, but do not add unnecessary heavy multi-sample behavior if the lane is only about truthful reporting.

## Reporting format
Report:
- previous misleading behavior
- new truthful behavior
- exact changed files
- exact test commands
- any remaining caveat about fixed deterministic sampling

## Task reminder
Your task is still: make validator output-stage schedule truth accurate and provable, not to expand into broad validator redesign.
