# Packet 28 — V3 tooling command reference

## Task first
Create or tighten a concise V3 tooling command reference so humans and agents can quickly run the right commands.

## Objective
Create or tighten a concise command-reference artifact for the current V3 tooling surfaces so humans and agents can quickly run the right validation/probe/doc commands.

## Why this matters
We are accumulating many useful commands. A compact command reference reduces drift and repeated rediscovery.

## Mode
BLOCKER_MODE

## Success condition
- one concise command reference exists or is updated
- commands are grouped by purpose
- stale commands are removed or clearly marked

## Task-scope paths for grounding
- tooling/docs reference surface in `/usr/projects/tui-vfx-recipes` or `/usr/projects/tui-vfx`
- command documentation only

## Exact write scope
- the chosen command-reference artifact only
- the smallest nearby doc pointer if needed

## Out of scope
- changing tool behavior
- broad docs rewrite
- recipe content

## Must-read docs in order
1. `/usr/projects/tui-vfx-recipes/steering/INTENTIONS.md`
2. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
3. current tooling command surfaces and relevant packet outputs
4. `/usr/projects/global_prompts/standards/60_file_centric_execution.md`

## Repo-boundary guardrails
- This is command documentation only.
- Do not change tool behavior in this packet.

## Verification required
- manually re-run the listed commands or a representative subset and confirm they still work

## Reporting format
Report what command groups were documented and any stale commands removed.
