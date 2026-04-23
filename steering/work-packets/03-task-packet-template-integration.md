# Packet 03 — reusable task-packet template

## Task first
Create or update the canonical reusable task-packet template so future packets inherit the experiment-backed structure by default.

## Objective
Create or update a durable reusable task-packet template based on the experiment’s winning packet structure.

## Why this matters
Even if ORCHESTRATION and the briefing improve, the leader will keep dispatching uneven work unless there is a concrete reusable packet template.

## Mode
BLOCKER_MODE

## Success condition
- one durable reusable packet template exists on disk
- it reflects the winning experiment structure
- it is easy for the leader to fill in for future lanes

## Task-scope paths for grounding
- a new or updated template file under `/usr/projects/tui-vfx/steering/`
- related steering references if needed

## Exact write scope
- the canonical template file under `/usr/projects/tui-vfx/steering/`
- only the smallest related steering reference(s) if needed to point people at it

## Out of scope
- runtime code
- experiment reruns
- large prose rewrites across unrelated docs

## Must-read docs in order
1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
2. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
3. experiment result files used as evidence
4. `/usr/projects/tui-vfx/steering/ORCHESTRATION.md`
5. `/usr/projects/global_prompts/standards/60_file_centric_execution.md`
6. `/usr/projects/global_prompts/standards/65_subagent_orchestration.md`

## Repo-boundary guardrails
- Keep this work in steering/template surfaces only.
- Do not use this packet to revise unrelated orchestration policy.

## Template requirements
The template should make room for:
- task first
- success condition
- mode (`BLOCKER_MODE` / `FAMILY_MODE`)
- write scope
- out-of-scope bullets
- must-read order
- repo-boundary guardrails
- hot-path watchpoints
- exact verification commands
- reporting contract
- closing task reminder

## Verification required
- `git diff --check`
- manual audit that the template reflects the experiment winner rather than a guess

## Reporting format
Report:
- exact template path
- why the structure was chosen
- what future lanes it is intended to support

## Task reminder
Your task is still: create a practical reusable packet template from experiment evidence, not to revise unrelated strategy docs.
