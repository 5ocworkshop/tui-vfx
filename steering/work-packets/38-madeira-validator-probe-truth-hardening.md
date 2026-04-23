# Packet 38 — Madeira validator/probe truth hardening

## Objective
Make sure the validator/probe surfaces tell the truth about Madeira’s current support level as implementation slices land.

## Why this matters
A recipe can appear “supported” too early if the diagnostic surfaces do not distinguish between parse/compile/bridge success and actual operational parity.

## Mode
BLOCKER_MODE

## Prerequisites
- at least one Madeira implementation slice has landed

## Success condition
- validator/probe surfaces accurately communicate Madeira’s current state
- no misleading pass status remains for known unimplemented semantics
- the reporting is evidence-backed

## In scope
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/`
- `/usr/projects/tui-vfx-recipes/src/probe/`
- Madeira-specific proof commands/tests

## Out of scope
- new Madeira runtime features unless strictly necessary for truthful reporting
- broad validator redesign

## Verification required
- representative Madeira validator run(s)
- representative Madeira probe run(s)
- tests for any new diagnostic wording or classification logic

## Reporting format
Report:
- prior misleading truth surface
- new truthful surface
- exact files changed
- exact commands/tests
- any remaining caveat still not expressible cleanly

## Task reminder
Your task is still: harden diagnostic truth for Madeira, not broaden feature implementation.
