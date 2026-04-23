# Packet 08 — Madeira next-slice plan

## Objective
Turn the Madeira parity audit into one narrow next implementation slice that can be executed safely.

## Why this matters
The audit should feed a concrete next step, not a vague promise of future parity.

## Mode
BLOCKER_MODE

## Success condition
- one next slice is chosen
- exact files to touch are named
- exact proof commands are named
- risks are explicit

## In scope
- Madeira audit results
- the exact seam identified by the audit

## Out of scope
- implementing the slice itself
- widening into multiple Madeira gaps at once

## Deliverable
Produce:
- slice name
- exact seam
- exact files
- why this slice is first
- what commands prove it when done

## Reporting format
Be concrete. If more than one slice seems possible, explain why the chosen one is best first.

## Task reminder
Your task is still: pick the next slice, not perform Madeira implementation.
