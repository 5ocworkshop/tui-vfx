# Packet 28 — V3 tooling command reference

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

## In scope
- tooling/docs reference surface in `/usr/projects/tui-vfx-recipes` or `/usr/projects/tui-vfx`
- command documentation only

## Out of scope
- changing tool behavior
- broad docs rewrite
- recipe content

## Verification required
- manually re-run the listed commands or a representative subset and confirm they still work

## Reporting format
Report what command groups were documented and any stale commands removed.
