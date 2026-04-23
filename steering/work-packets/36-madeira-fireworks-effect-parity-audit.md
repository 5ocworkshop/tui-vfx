# Packet 36 — Madeira fireworks/effect parity audit

## Objective
Audit the fireworks/effect layers of Madeira separately from scene semantics so we know which remaining gaps are visual/effect capability issues rather than scene-composition issues.

## Why this matters
Madeira is not only a scene/layout problem. It is also an effect-stack problem, and mixing those concerns makes the next implementation slice harder to choose.

## Mode
BLOCKER_MODE

## Success condition
- one effect-focused Madeira gap list exists
- effect gaps are separated from scene/placement gaps
- one next effect-focused implementation slice is recommended

## In scope
- `/usr/projects/tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json`
- effect-bearing scene layer/pipeline sections of the Madeira recipe
- supporting compile/render/probe/validator seams only as evidence

## Out of scope
- implementing the fixes
- broad scene-semantics work
- general recipe corpus work

## Verification required
- exact commands showing what Madeira currently proves today
- explicit list of effect semantics that are still unproven, degraded, or missing

## Reporting format
Report:
- effect semantic matrix: proven / unproven / degraded
- top remaining effect-focused blocker
- exact likely seam for the next implementation packet

## Task reminder
Your task is still: isolate the Madeira effect-capability gap, not fix it yet.
