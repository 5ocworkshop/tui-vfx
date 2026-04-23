# Packet 26 — debug-recipes content-family cleanup tranche

## Objective
Run one bounded cleanup tranche on a content-effect family in the debug recipes using the improved packet style and fixture rules.

## Why this matters
The content recipes are numerous and user-facing. Once the prompt experiment improves delegation quality, this is a good place to apply the refined packet style.

## Mode
FAMILY_MODE

## Prerequisites
- relevant audit/shortlist exists
- chosen content family is explicitly named

## Success condition
- one content-family tranche is made trustworthy and legible
- message/body/description/layout rules are followed
- no cross-family drift

## In scope
- one named content family only
- related QC/tests if directly needed

## Out of scope
- all content recipes
- unrelated filters/styles/masks
- runtime content-engine changes unless clearly justified

## Verification required
- relevant recipe/QC checks
- any narrow tests that prove the family still loads/renders as intended

## Reporting format
Report the exact family, exact fixtures changed, and what rule violations were corrected.
