<!-- <FILE>docs/design/tui-vfx-v3-validator-canonicalization-checklist.md</FILE> - <DESC>Execution checklist for the V3 validator/canonicalization phase. Tracks the minimum checks and canonical outputs needed before broad family runtime implementation should proceed.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Initial checklist companion to Chapter 59. Focuses on the minimum viable validation/canonicalization surface for implementation-readiness.</WCTX> -->
<!-- <CLOG>0.1.0: initial checklist. Seeds the concrete validation/canonicalization work items following the schema, catalog, lowering, and normalized-IR phases.</CLOG> -->

# tui-vfx V3 validator / canonicalization checklist

## Status tracker

| ID | Check area | Status | Notes |
|---|---|---|---|
| VC-01 | Authoring schema validation | OPEN | first implementation slice |
| VC-02 | Region-ref / compression resolution | OPEN | required for normalized IR |
| VC-03 | Style normalization validation | OPEN | required before style-heavy runtime work |
| VC-04 | Hint producer/consumer validation | OPEN | required before hint-bound execution |
| VC-05 | Scene-layer placement validation | OPEN | required before scene-heavy runtime work |
| VC-06 | Contract discovery validation | OPEN | `requires_*` checks |
| VC-07 | Lowering invariant checks | OPEN | V2→V3 migration correctness |
| VC-08 | Normalized IR dump / debug output | OPEN | shared tooling surface |
| VC-09 | Migration-equivalence checks | OPEN | critical fixtures first |
| VC-10 | Human-review-needed report | OPEN | unresolved lowering classes |

## Minimum go/no-go rule

Broad runtime family work should not begin until at least:

- VC-01
- VC-02
- VC-03
- VC-04
- VC-05
- VC-07
- VC-08

have credible implementation plans or initial code in place.

## Immediate next action

Start by turning the normalized IR design into actual internal types, then build:

1. region-ref resolution
2. style normalization
3. hint validation
4. canonical IR dump

That gives the smallest useful validator/canonicalizer spine.

<!-- <FILE>docs/design/tui-vfx-v3-validator-canonicalization-checklist.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
