# Packet 04 — validator output-stage schedule truth

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

## In scope
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/fnc_validate_v3_compiled_recipe.rs`
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/stages/functions/fnc_validate_output.rs`
- targeted validator tests under `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/tests/`

## Out of scope
- debug recipe corpus
- core compiled runtime semantics unless absolutely required to preserve truthful messaging
- unrelated validator stages

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
