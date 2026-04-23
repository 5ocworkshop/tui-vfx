# Packet 02 — briefing experiment integration

## Objective
Integrate the validated findings from the subagent-briefing experiment into the permanent steering surfaces.

## Why this matters
The point of the experiment is not the experiment itself; it is to improve all later subagent work. Once the experiment converges, the winning structure must be merged into the real leader and subagent-facing artifacts.

## Mode
BLOCKER_MODE

## Success condition
- permanent briefing/orchestration docs reflect the experiment’s evidence-backed improvements
- changes are justified by explicit experiment results, not by preference
- no unrelated wording churn

## In scope
- `/usr/projects/tui-vfx/steering/ORCHESTRATION.md`
- `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
- experiment results files only as evidence inputs

## Out of scope
- running the experiment itself
- runtime/library code
- recipe content
- debug recipe corpus

## Required inputs before editing
Do not begin edits until you have:
- the completed experiment results log
- the per-cycle conclusions
- the winning packet structure summary

## First steps
1. Read the results log carefully.
2. Extract the strongest durable lessons only.
3. Group them into:
   - ordering changes
   - wording changes
   - verification changes
   - scope/ownership guardrails
4. Apply only those changes that are clearly supported by the experiment.

## Verification required
- `git diff --check`
- manual consistency check:
  - read order matches experiment result
  - task-first/bounds/guardrails/verification/task-again structure is reflected if that won
  - subagents are not incorrectly assigned leader-facing docs

## Reporting format
Report:
- which experiment findings were integrated
- which were deliberately not integrated and why
- exact changed files
- exact verification performed

## Task reminder
Your task is still: merge proven experiment learnings into the permanent orchestration surfaces, not to invent new process policy from scratch.
