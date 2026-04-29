# K2.15 holdback register

## Current holdbacks after K2.15

```text
ownerAuditNeeded=280
descriptorDecisionNeeded=113
sourceDecisionNeeded=61
backendHoldback=15
guiHumanReviewHoldback=2
oracleOnly=195
duplicateOrVariant=3
explicitOwnerDecisionNeeded=0
```

## High-level blockers for architect attention

1. **Backend/compositor lowering seam** — we now have player-owned graph/value-bus evidence, but no durable player IR that lowers to the `tui-vfx-compositor` backend without the UI constructing compositor internals.
2. **Scene/layer fidelity** — element-local style evidence works, but full layer visibility, transparent blending/clear semantics, and element-attributed diagnostics remain incomplete.
3. **Source fidelity** — ANSI is text-only after SGR stripping; image is a deterministic fallback; procedural is a tiny bounded generator set. This blocks serious visual parity.
4. **Descriptor backlog** — 113 descriptor-decision records remain; these are not schema blockers, but they block broader canonical fixture migration.
5. **Owner/visual audit volume** — 280 records still require owner or visual judgment. Forward progress needs explicit triage lanes, not repeated rediscovery.

