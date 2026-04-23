# Packet 22 — work-packet library maintenance

## Task first
Audit the packet library itself and identify which packets still need structural updates to match the improved packet standard.

## Objective
Audit the on-disk work-packet library itself after the experiment converges and identify what should be updated so the packets match the winning briefing format.

## Why this matters
We have a packet library on disk. Once the experiment converges, those packets may need structural updates to align with the proven template.

## Mode
BLOCKER_MODE

## Success condition
- identify which packet-library files need structural updates
- categorize them by simple mechanical update vs packet-specific rewrite
- propose a safe update strategy

## Task-scope paths for grounding
- `/usr/projects/tui-vfx/steering/work-packets/`
- experiment results and winning template as evidence

## Exact write scope
- none by default; this is an audit packet
- if you need to add a small audit artifact, keep it in the work-packets area only

## Out of scope
- dispatching the packets
- runtime/library code
- broad steering rewrites outside packet-library maintenance

## Must-read docs in order
1. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
2. experiment results used as evidence
3. `/usr/projects/tui-vfx/steering/TASK_PACKET_TEMPLATE.md`
4. `/usr/projects/tui-vfx/steering/work-packets/COMMON_EXECUTION_RULES.md`
5. `/usr/projects/global_prompts/standards/60_file_centric_execution.md`

## Repo-boundary guardrails
- This is packet-library maintenance only.
- Do not revise orchestration policy here unless a later dedicated packet explicitly says so.

## Verification required
- evidence from the winning experiment structure
- exact packet files likely needing update
- no speculative changes without citing the winning structure

## Reporting format
Report:
- packet-library files that are already compliant
- packet-library files that need updates
- suggested batching strategy for updating them

## Task reminder
Your task is still: maintain the packet library based on the experiment winner, not rewrite orchestration policy again.
