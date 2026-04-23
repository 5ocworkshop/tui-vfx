# Packet 33 — V3 end-to-end readiness audit

## Objective
Conduct a bounded audit of how close the current V3 pipeline is to being operational end-to-end across authoring, loading, validation, preview, probe, and representative showcase recipes.

## Why this matters
We need a periodically refreshed view of overall readiness so we know whether we are converging or just fixing isolated seams.

## Mode
BLOCKER_MODE

## Success condition
- one readiness matrix exists
- top remaining blockers are ranked
- evidence is tied to actual commands/tests, not intuition

## In scope
- all major V3 surfaces in `/usr/projects/tui-vfx-recipes`
- representative showcase checks (including Madeira where appropriate)

## Out of scope
- solving all blockers
- broad implementation

## Verification required
- representative commands for each major surface
- exact pass/fail/readiness notes

## Reporting format
Report a readiness matrix and the next 3 highest-value blockers.
