# Packet 40 — Madeira end-to-end operational check

## Objective
Run a bounded end-to-end operational check for Madeira across authoring, loading, validation, deterministic rendering, and any current probe/preview surfaces.

## Why this matters
After several slices, we need a concrete checkpoint that says what Madeira can actually do end-to-end today.

## Mode
BLOCKER_MODE

## Prerequisites
- relevant Madeira implementation slices complete
- validator/probe truth surfaces reasonably current

## Success condition
- one end-to-end checklist run is performed
- every stage is marked pass/fail/unproven with evidence
- one next blocker is identified if not yet fully operational

## In scope
- Madeira recipe
- current supporting loader/validator/probe/deterministic-render surfaces
- no broad new implementation unless a trivial proof fix is necessary

## Out of scope
- fixing every discovered issue in the same packet
- broad docs rewrite

## Verification required
At minimum, gather fresh evidence for:
- load/parse
- normalize/validate/compile
- validator output truth
- deterministic render/probe
- any Madeira-specific baseline tests already created

## Reporting format
Report an end-to-end matrix with:
- stage
- command
- result
- evidence
- next blocker if failed or degraded

## Task reminder
Your task is still: assess end-to-end operational state, not immediately fix everything discovered.
