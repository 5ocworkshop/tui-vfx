# Packet 03 — reusable task-packet template

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

## In scope
- a new or updated template file under `/usr/projects/tui-vfx/steering/`
- related steering references if needed

## Out of scope
- runtime code
- experiment reruns
- large prose rewrites across unrelated docs

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
