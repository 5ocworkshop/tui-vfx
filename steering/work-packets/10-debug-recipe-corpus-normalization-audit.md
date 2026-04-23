# Packet 10 — debug-recipe corpus normalization audit

## Objective
Identify the next highest-value family of debug recipes that still violates the current fixture-quality rules, so later cleanup packets can be prioritized intelligently.

## Why this matters
The corpus is large. We should not spray edits randomly; we should target the most misleading or lowest-quality clusters first.

## Mode
BLOCKER_MODE

## Success condition
- one prioritized shortlist of bad families/fixtures
- each recommendation tied to specific rule violations
- no mass editing in this packet

## In scope
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/`
- relevant fixture-quality rules in INTENTIONS and briefing docs

## Out of scope
- bulk fixing the corpus
- validator code changes unless absolutely needed to prove a violation

## What to score
Look for:
- missing/weak descriptions
- misleading body text
- bad layout/contrast
- unclear effect visibility
- wrong directory classification
- stale `_DEPRECATED_` drift where it still matters

## Verification required
- evidence from exact fixture files
- a short ranked list with reasons

## Reporting format
Report:
- top offenders
- violated rules per offender
- recommended cleanup order

## Task reminder
Your task is still: audit and prioritize, not rewrite the corpus.
