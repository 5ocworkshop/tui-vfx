# K2.15 baseline and final counters

## Scope

K2.15 stayed on the v3.1 debug-recipes pathway only. Legacy `recipes/debug_recipes/` stayed read-only evidence; canonical additions landed only under `recipes/v3.1/debug_recipes/`.

## Counter table

| Gate | Baseline | Final | Result |
| --- | ---: | ---: | --- |
| Canonical v3.1 fixtures | 57 | 67 | +10 fixtures |
| validate-recipe | 57 valid / 0 invalid | 67 valid / 0 invalid | clean |
| render-recipe | 57 rendered / 0 unsupported / 0 errors | 67 rendered / 0 unsupported / 0 errors | clean |
| render-frame | 57 rendered / 0 unsupported / 0 errors | 67 rendered / 0 unsupported / 0 errors | clean |
| fixture-qc | pass, 57 rendered, 0 player errors | pass, 67 rendered, 0 player errors | clean |
| primitive-field-coverage | 361 used / 361 handled / 0 unhandled | 422 used / 422 handled / 0 unhandled | clean |
| primitive-adapter-gap | 43 rendered / 0 unresolved | 43 rendered / 0 unresolved | clean |
| schema-readiness | canDeclareSchemaReady=true, 249 schema-ready | canDeclareSchemaReady=true, 251 schema-ready | improved |
| migration canonicalExists | 48 | 50 | improved |
| migration schemaDecisionNeeded | 93 | 91 | improved |

## Final command evidence

```text
validate-recipe: total=67 valid=67 invalid=0
render-recipe: total=67 rendered=67 unsupported=0 errors=0
render-frame: total=67 rendered=67 unsupported=0 errors=0
fixture-qc: totalRecipes=67 validated=67 rendered=67 unsupported=0 playerErrors=0 fieldCoverageUnhandled=0 adapterGapUnresolved=0 overallStatus=pass
primitive-field-coverage: totalPrimitiveInstances=156 usedInputFields=422 handledInputFields=422 usedButUnhandledInputFields=0 missingDescriptorInputFields=0 schemaDecisionNeededFields=0
primitive-adapter-gap: totalEffects=43 rendered=43 stillUnsupported=0 missingDescriptor=0
schema-readiness: canDeclareSchemaReady=true explicitOwnerDecisionNeeded=0 unresolvedSchemaBlockers=0 estimatedSchemaReadinessPercent=41.6
migration-mapping: records=603 canonicalExists=50 candidateReady=5 schemaDecisionNeeded=91 descriptorDecisionNeeded=113 adapterDecisionNeeded=0 sourceDecisionNeeded=61
```

## Remaining holdbacks

The remaining high-count buckets are not hidden player failures:

- `ownerAuditNeeded=280`: still needs owner/aesthetic review against legacy intent.
- `descriptorDecisionNeeded=113`: descriptor backlog remains for families not in this tranche.
- `sourceDecisionNeeded=61`: source fidelity remains bounded; image and ANSI are not visual-parity complete.
- `backendHoldback=15` and GUI/human-review holdbacks remain future adapter/visual-review work.

