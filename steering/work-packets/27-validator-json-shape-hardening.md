# Packet 27 — validator JSON shape hardening

## Objective
Audit and tighten the machine-readable JSON output shape for validator/probe surfaces so downstream tooling and experiments can rely on it more safely.

## Why this matters
As more of our workflow depends on machine-readable validator/probe results, shape drift or inconsistent field semantics becomes costly.

## Mode
BLOCKER_MODE

## Success condition
- one clear JSON-shape inconsistency or weakness is fixed
- tests prove the intended stable shape
- no broad redesign of the whole CLI

## In scope
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/`
- `/usr/projects/tui-vfx-recipes/tools/recipe-probe/` when directly relevant
- JSON-focused tests

## Out of scope
- recipe content
- non-JSON text-output polishing
- unrelated validator stage logic unless needed to stabilize the JSON contract

## Verification required
- targeted JSON-shape tests
- one representative CLI JSON run showing the intended shape

## Reporting format
Report the old ambiguous shape, the new stabilized shape, and exact affected consumers if known.
