# Packet 30 — codex-spark doc-task experiment design

## Task first
Design a separate codex-spark experiment for doc-oriented tasks using the improved briefing structure, but do not run it yet.

## Objective
Design a separate experiment specifically for codex-spark agents on doc-oriented tasks, using the improved briefing structure from the main experiment.

## Why this matters
Spark agents may be a strong fit for doc-only work, but that should be tested explicitly rather than assumed.

## Mode
BLOCKER_MODE

## Success condition
- one clean experiment design exists
- candidate doc-only tasks are chosen
- scoring/quality rubric is defined
- the experiment stays separate from the main briefing experiment

## Task-scope paths for grounding
- experiment design docs only
- packet/task selection for doc-oriented work

## Exact write scope
- the spark experiment design artifact(s) only

## Out of scope
- running the spark experiment itself unless explicitly requested later
- product/runtime code

## Must-read docs in order
1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
2. experiment learnings from the main briefing experiment
3. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
4. `/usr/projects/global_prompts/standards/65_subagent_orchestration.md`

## Repo-boundary guardrails
- This is experiment design only.
- Do not launch spark tasks from this packet.

## Verification required
- clear written protocol
- clearly named candidate doc tasks
- clear pass/fail rubric

## Reporting format
Report the experiment design, candidate tasks, and expected failure modes to watch.
