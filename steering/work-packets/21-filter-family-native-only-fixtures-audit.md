# Packet 21 — filter family native-only fixtures audit

## Objective
Audit the growing set of `complex_filter_*_native_only.json` and related native-mix fixtures to identify the next highest-value fixture/coverage gap.

## Why this matters
These fixtures are supposed to demonstrate and regression-lock the direct/native path. If they are inconsistent, mislabeled, or under-verified, they lose value.

## Mode
BLOCKER_MODE

## Success condition
- one prioritized shortlist of native-only/native-mix fixture gaps
- evidence-backed next cleanup tranche recommendation
- no bulk editing in this audit packet

## In scope
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/complex/complex_filter_*`
- related filter coverage tests
- current debug recipe quality rules

## Out of scope
- rewriting the full native-only corpus
- runtime filter semantics changes
- unrelated content/style/mask recipes

## Verification required
- exact fixture inspection evidence
- any narrow coverage test evidence if needed

## Reporting format
Report:
- top offending native-only/native-mix fixtures
- why they are risky or misleading
- recommended cleanup order and next packet scope

## Task reminder
Your task is still: audit and prioritize the native-only filter fixtures, not rewrite them all.
