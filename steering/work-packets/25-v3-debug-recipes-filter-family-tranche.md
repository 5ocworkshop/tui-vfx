# Packet 25 — V3 debug-recipes filter-family cleanup tranche

## Objective
Run one bounded cleanup tranche on the highest-priority filter-family debug recipes identified by the audit packets, applying the approved fixture-quality rules.

## Why this matters
The native-only/native-mix filter fixtures are part of how we assess migration quality. Once the audit identifies the worst offenders, we need a disciplined cleanup tranche.

## Mode
FAMILY_MODE

## Prerequisites
- Packet 10 or Packet 21 complete with a ranked shortlist
- the chosen tranche is explicitly named

## Success condition
- one filter-family tranche is cleaned up
- descriptions/body text/layout/contrast/timing are corrected where needed
- validator/QC expectations still pass

## In scope
- only the selected filter-family fixtures and closely related tests/QC references

## Out of scope
- the whole debug-recipes corpus
- unrelated content/style/mask fixture work
- runtime filter semantics changes unless clearly required and separately approved

## Verification required
- targeted fixture/QC checks
- any related coverage tests
- proof that the cleaned fixtures now clearly show the intended effect

## Reporting format
Report:
- exact fixtures touched
- exact quality issues fixed
- exact verification
- remaining filter-family fixtures left for later tranches
