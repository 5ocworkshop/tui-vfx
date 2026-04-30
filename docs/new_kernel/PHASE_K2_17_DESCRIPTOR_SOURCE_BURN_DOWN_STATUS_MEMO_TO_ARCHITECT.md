# K2.17 Descriptor and Source Burn-Down Status Memo to Architect

Status: implemented and under final review/verification.

What changed:

- Added 54 canonical v3.1 debug fixtures, raising the corpus from 88 to 142.
- Expanded descriptor-backed effects from 45 to 75.
- Added implementation-readiness and control-catalog CLIs.
- Corrected source/content vocabulary: content effects now surface as `content.*` descriptor backlog plus `source.text` evidence, not durable pseudo-sources.
- Added content, filter, mask, sampler, shader, style, source, scene visibility, render IR, and backend-seam evidence.

<!-- Generated K2.17 evidence memo; update when counters or gates change. -->

Evidence date: 2026-04-30

Core gate summary:

- validate-recipe: 142/142 valid, 0 invalid.
- render-recipe: 142/142 rendered, 0 unsupported, 0 errors.
- render-frame: 142/142 rendered, 0 unsupported, 0 errors.
- fixture-qc: overall pass; 142 validated, 142 rendered, 0 unhandled fields, 0 unresolved adapter gaps.
- primitive-field-coverage: 894/894 used fields handled; 0 missing descriptor fields.
- primitive-adapter-gap: 75/75 effects rendered; 0 unsupported.
- schema-readiness: 293 schema-ready records; 0 explicit owner decisions; 40 source-blocked; 76 descriptor-blocked.
- migration mapping: canonicalExists 97; descriptorDecisionNeeded 76; sourceDecisionNeeded 40; blockedByFieldCoverage 8.
- implementation-readiness: canonicalExists 159; contentBacklog 39; sourceBacklog 1; descriptorBacklog 84; graphRuntimeBacklog 83; sceneRuntimeBacklog 16; explicitOwnerDecisionNeeded 0.
- control-catalog: 367 controls (16 source, 351 effect).

Artifacts are derived from `/tmp/k217-current/*.json` and current `descriptors/v3.1/packs/primitive.json`.


Architect blockers needing deliberate follow-up:

1. Descriptor backlog is still large: mapping `descriptorDecisionNeeded` is 76 and readiness `descriptorBacklog` is 84. We need another implementation tranche, not another report-only loop.
2. Content/source backlog improved but missed the preferred target: readiness has 39 content records and 1 true source record. The next packet should target roughly 10-15 more content fixtures/adapters or formally lower the target.
3. Graph and scene runtime backlog remains significant: graphRuntimeBacklog 83 and sceneRuntimeBacklog 16. Forward progress needs concrete runtime closures for these, not repeated classification.
4. Backend holdbacks remain intentional: visual parity/compositor lowering is still separate from player evidence. The new backend seam is the starting point, not completion of compositor integration.

