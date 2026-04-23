# Packet 20 — Madeira scene semantics audit

## Objective
Go one level deeper than the basic Madeira parity audit and identify which scene semantics (layer placement, scene source behavior, nested pipelines, etc.) are still unproven or degraded in the current path.

## Why this matters
Madeira is scene-heavy. We need to separate general “passes validator output” from real scene semantic parity.

## Mode
BLOCKER_MODE

## Success condition
- map Madeira’s scene semantics to current proof status
- identify the single most important unproven or degraded scene semantic
- name the exact seam for the next slice

## In scope
- `/usr/projects/tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json`
- V3 scene compile/render/placement files used by Madeira
- proof surfaces only (tests, validator, deterministic render helper)

## Out of scope
- implementing the fix
- fireworks/effect aesthetics tuning
- broad Madeira redesign

## Verification required
- exact commands showing what is already proven
- evidence tying each unproven semantic to specific files or helpers

## Reporting format
Report:
- semantic matrix (proven / unproven / degraded)
- top remaining scene-semantic blocker
- exact next seam

## Task reminder
Your task is still: isolate the scene-semantic gap, not perform Madeira implementation.
