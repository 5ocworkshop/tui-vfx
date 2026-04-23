# Packet 02 — briefing experiment integration

## Task first
Integrate only the strongest evidence-backed packet/briefing learnings from the experiment into the permanent steering surfaces.

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

## Task-scope paths for grounding
- `/usr/projects/tui-vfx/steering/ORCHESTRATION.md`
- `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
- experiment results files only as evidence inputs

## Exact write scope
- `/usr/projects/tui-vfx/steering/ORCHESTRATION.md`
- `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`

## Out of scope
- running the experiment itself
- runtime/library code
- recipe content
- debug recipe corpus

## Must-read docs in order
1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
2. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. completed experiment result files used as evidence
5. `/usr/projects/global_prompts/standards/60_file_centric_execution.md`
6. `/usr/projects/global_prompts/standards/65_subagent_orchestration.md`

## Repo-boundary guardrails
- This is process/steering work, not runtime/library work.
- Do not broaden into packet-library normalization unless explicitly assigned separately.
- Only integrate findings that are actually supported by experiment evidence.

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
