# K2.17 Schema API Docs Gate

Schema/API/docs touched by this packet:

- Primitive descriptor pack expanded; no schema version bump was made because v3.1 is pre-release and the report/contract versions remain compatible.
- Public player report DTOs and backend/source seams were added with rustdoc-bearing public exports.
- VOCABULARY.md and the v3.1 feature checklist were updated for implementation-readiness, control-catalog, content descriptor vocabulary, and render-backend seam language.

Verification evidence is listed below; final command evidence belongs in the status memo after full workspace gates complete.

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


