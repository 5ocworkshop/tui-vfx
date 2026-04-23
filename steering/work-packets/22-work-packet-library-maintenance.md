# Packet 22 — work-packet library maintenance

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

## In scope
- `/usr/projects/tui-vfx/steering/work-packets/`
- experiment results and winning template as evidence

## Out of scope
- dispatching the packets
- runtime/library code
- broad steering rewrites outside packet-library maintenance

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
