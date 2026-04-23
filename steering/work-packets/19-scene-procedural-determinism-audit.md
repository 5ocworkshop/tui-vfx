# Packet 19 — scene procedural determinism audit

## Objective
Audit whether procedural scene sources and scene-layer composition remain deterministic and well-covered in the current V3 tooling path.

## Why this matters
Procedural scene sources are a likely place for determinism drift, especially in probes/preview/tooling.

## Mode
BLOCKER_MODE

## Success condition
- identify one concrete determinism or coverage gap (or confirm current stability)
- exact files/tests are named for the next lane

## In scope
- `/usr/projects/tui-vfx-recipes/src/scene/procedural/`
- `/usr/projects/tui-vfx-recipes/tests/scene/procedural/`
- scene-layer tests if directly relevant
- probe/validator evidence only when needed

## Out of scope
- broad procedural feature design
- recipe corpus edits
- unrelated runtime timing work

## Verification required
- targeted scene/procedural tests if present
- evidence from deterministic helper/probe/validator surfaces when needed

## Reporting format
Report:
- current deterministic guarantees
- uncovered or weakly covered seam
- exact files/tests for next implementation lane

## Task reminder
Your task is still: identify the next procedural determinism gap, not redesign procedural source capabilities.
