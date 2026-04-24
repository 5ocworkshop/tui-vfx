<!-- <FILE>docs/design/tui-vfx-v3-migration-outcome-policy.md</FILE> - <DESC>Accepted provisional V3 migration outcome policy for equivalence, replacements, and retired recipes without deleting legacy recipes.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Record the project-owner-approved migration outcome classification so VC-09 and release-gate work can proceed while the owner completes the recipe audit.</WCTX> -->
<!-- <CLOG>0.1.0: initial accepted policy for equivalent/replacement/retired tracks, with explicit caveats that classification is subject to owner audit and legacy recipes are not removed.</CLOG> -->

# V3 migration outcome policy

This document records the accepted provisional policy for judging V2 → V3 recipe
outcomes while the project owner completes the recipe audit.

## Decision

V3 migration uses three outcome tracks:

| Track | Meaning | Validation bar |
|---|---|---|
| `equivalent` | V3 should preserve V2 behavior | VC-09 render/probe equivalence within tolerance. |
| `replacement` | V3 intentionally replaces or improves V2 behavior | Explicit replacement rationale plus V3 validator/probe evidence. |
| `retired` | V2 recipe will not carry forward as a V3 recipe | Inventory records the reason; excluded from V3 migration gates. |

## Owner caveats

This policy is approved with two caveats:

1. **Subject to owner recipe audit.** Final classification depends on the project
   owner's recipe audit. Provisional labels may change.
2. **Do not remove legacy recipes.** Even when a recipe is provisionally marked
   `retired`, do not delete V2/legacy recipe files as part of this policy. V2
   recipe removal remains out of scope until the final V2-retirement gate.

## Provisional defaults before audit

Until the owner audit is complete, use these defaults only to keep VC-09 and
migration tooling moving:

| Recipe class | Provisional track |
|---|---|
| critical/demo/documented visual contracts | `equivalent` |
| debug recipes | `replacement`, unless the recipe exists specifically to pin a regression |
| deprecated recipes | `retired`, but keep files in place |
| exploratory/lab recipes | `replacement` or `retired`, pending owner audit |

These defaults are working assumptions, not final curation decisions.

## Evidence requirements

### `equivalent`

Required evidence:

- V2 reference fixture or paired source
- V3 recipe or compiled plan
- render/probe comparison through VC-09 or release-gate tooling
- tolerance result: pass, fail, or whitelist-needed

### `replacement`

Required evidence:

- explicit rationale explaining why V3 behavior intentionally differs
- V3 validator/probe evidence showing the replacement works
- authoring/debug recipe docs when the replacement demonstrates a new V3
  capability or changed composition pattern

### `retired`

Required evidence:

- inventory row with reason
- no deletion of the legacy recipe file
- not included in equivalence or migration-gate required sets unless owner later
  reclassifies it

## Relationship to V2 retention

This policy does not authorize V2 removal.

V2 recipe support, legacy recipes, and fallback paths stay in place until the
final V2-retirement plan is created and explicitly approved after migration,
stability, and downstream adaptation.

## Plan impact

This unblocks provisional VC-09 work and release-gate manifest work without
waiting for the full recipe audit.

It updates the decision status for:

- VC-09 migration-equivalence harness
- Chapter 50 migration workflow
- Chapter 60 release gates
- the V3 outstanding master punch list

<!-- <FILE>docs/design/tui-vfx-v3-migration-outcome-policy.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
