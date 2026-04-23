# Packet 07 — Madeira flag parity audit

## Objective
Determine the exact remaining gap between the current “loads/compiles/validator passes” state of `madeira_flag.json` and the eventual full V3 realized scene/effect parity target.

## Why this matters
Madeira is the showcase target. We should not jump into implementation without a precise gap list.

## Mode
BLOCKER_MODE

## Success condition
- a concrete gap list exists
- each gap is tied to a specific seam or file area
- one next implementation slice is suggested

## In scope
- `/usr/projects/tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json`
- V3 scene/compile/preview/render seams in `/usr/projects/tui-vfx-recipes/src/v3/`
- validator/probe surfaces only as evidence

## Out of scope
- implementing all Madeira gaps
- broad scene-engine redesign
- unrelated recipe cleanup

## First steps
1. Confirm current pass state:
   - loads
   - compiles
   - validator output-stage pass
2. Identify what that does **not** guarantee.
3. Compare expected scene/effect semantics against what current bridge/native paths actually exercise.

## Verification required
- exact commands showing current pass state
- evidence-backed list of unproven or unsupported semantics

## Reporting format
Report:
- what is already proven
- what is not yet proven
- exact remaining gaps
- recommended next implementation slice

## Task reminder
Your task is still: audit the Madeira gap precisely, not solve it end-to-end in one packet.
