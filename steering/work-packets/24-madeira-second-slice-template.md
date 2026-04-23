# Packet 24 — Madeira second-slice template

## Objective
Provide the next bounded Madeira execution lane after the first implementation slice lands.

## Why this matters
If the first slice succeeds, we want the next step already framed without improvising under pressure.

## Mode
BLOCKER_MODE

## Prerequisites
- Packet 23 complete
- Packet 23 verification green

## Success condition
- one next follow-up slice is selected and framed clearly
- exact files and proof commands are named

## In scope
- the next Madeira gap only

## Out of scope
- broad restructuring
- multiple slices at once

## Verification required
- exact commands named
- evidence that the next slice depends on what landed in Packet 23

## Reporting format
Report the next exact slice and why it follows logically from the first.
