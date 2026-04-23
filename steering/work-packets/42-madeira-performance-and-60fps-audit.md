# Packet 42 — Madeira performance and 60 FPS audit

## Objective
Audit Madeira-specific performance risk against the 16.7 ms / 60 FPS goal once enough of the recipe is operational to make the audit meaningful.

## Why this matters
A showcase recipe that is visually correct but too slow is not truly production-worthy.

## Mode
BLOCKER_MODE

## Prerequisites
- enough Madeira behavior is implemented to exercise realistic paths

## Success condition
- one evidence-backed Madeira performance audit exists
- major hot paths and likely bottlenecks are identified
- one next optimization slice is recommended if needed

## In scope
- Madeira execution path and its immediate runtime/probe surfaces
- no speculative global performance work beyond what Madeira reveals

## Out of scope
- broad engine-wide optimization campaign
- unrelated performance tuning elsewhere in the repo

## Verification required
- exact commands/measurements used
- clear explanation of whether the 60 FPS target looks safe, borderline, or at risk

## Reporting format
Report:
- measured or inferred hot spots
- risk level against 16.7 ms/frame
- recommended next optimization packet if needed

## Task reminder
Your task is still: audit Madeira performance readiness, not optimize the whole engine in one go.
