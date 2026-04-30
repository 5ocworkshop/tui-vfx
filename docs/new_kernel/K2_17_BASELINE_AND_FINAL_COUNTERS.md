# K2.17 Baseline and Final Counters

Baseline was captured at main commit `9c4ab6b` and recipe commit `a360c19`: 88 canonical fixtures, 45 descriptor-backed effects, 113 descriptor-decision records, 61 source-decision records, and 55 migration `canonicalExists` records.

Final counters after this packet:

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


Net movement:

- Canonical v3.1 debug fixtures increased from 88 to 142 (+54), excluding deprecated legacy recipes and excluding `/usr/projects/tui-vfx-recipes/pro/`.
- Descriptor-backed effect coverage increased from 45 to 75 (+30).
- Player adapter gap closed for the active canonical corpus: 0 unresolved adapter gaps and 0 unsupported rendered effects.
- Mapping descriptor decisions decreased from 113 to 76.
- Mapping source decisions decreased from 61 to 40; readiness now separates this into 39 content backlog records and 1 true source backlog record.
- Preferred targets not yet met: descriptorDecisionNeeded remains above 65, and source/content backlog remains above 30.

