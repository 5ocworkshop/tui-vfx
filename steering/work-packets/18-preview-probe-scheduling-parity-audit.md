# Packet 18 — preview/probe scheduling parity audit

## Objective
Audit whether preview-facing and probe-facing V3 sampling/scheduling semantics are aligned after the recent timing normalization work.

## Why this matters
We normalized elapsed-time handling for compiled replay. The next risk is that adjacent sampling/reporting surfaces still describe or exercise timing differently.

## Mode
BLOCKER_MODE

## Success condition
- identify one concrete scheduling/sampling parity issue or confirm the surfaces are aligned
- recommend one next correction seam if needed

## In scope
- `/usr/projects/tui-vfx-recipes/src/preview/`
- `/usr/projects/tui-vfx-recipes/src/probe/`
- relevant V3 compile timing helpers for evidence only

## Out of scope
- changing core replay timing again unless a clear bug is proven
- recipe content
- unrelated validator stages

## Verification required
- exact representative commands/tests for preview/probe timing surfaces
- evidence-backed mismatch analysis

## Reporting format
Report:
- representative surface pairs compared
- exact commands/tests
- parity or mismatch result
- one recommended next fix lane if needed

## Task reminder
Your task is still: audit preview/probe timing parity, not reopen the core replay timing patch without evidence.
