# Phase K2.14 Review and De-slop Report

## Scope

This report records the formal third-party review and AI de-slop cycle for the v3.1 debug-recipes descriptor/adapter/fixture migration tranche. The review scope is limited to files touched for this tranche and to canonical v3.1 fixtures under `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/`.

## Briefing discipline

All review and de-slop agents were briefed with `.omx/context/k214-subagent-briefing-20260429T220000Z.md`. The briefing restated the v3.1-only pathway, legacy read-only source corpus, canonical fixture target directory, no schema-version bump constraint, and the prohibition on phase shorthand in durable public API vocabulary.

## Agent findings and closure

- **Lane C/D source and content analysis** — confirmed the descriptor pack already had a source descriptor foothold, but the player needed bounded source adapters and content adapters. This led to `source.ansi`, `source.image`, `source.procedural`, and content adapter work in the player.
- **Lane B/E/F runtime, graph, and scene analysis** — confirmed runtime value support exists in contract/proof layers, while player graph-topology/value-bus support remains a higher-level blocker. This tranche added graph I/O proof-test evidence rather than claiming full player graph execution.
- **Lane G/H descriptor analysis** — identified filters, masks, samplers, shaders, and styles as the lowest-friction descriptor/adaptor burn-down lane. This tranche expanded the primitive descriptor pack and canonical fixtures for those families.
- **Lane A/I/J coordination analysis** — confirmed the baseline/final gates to preserve schema-readiness and explicit backend/gui/oracle holdback classification.
- **AI de-slop review** — requested changes for over-claimed handled fields, a `wrapIndicator` request-context bug, fixture tags that used transient wording, a stale packet title, and stale process/readiness wording. These were corrected.
- **Code review** — requested changes for honest field coverage, graph I/O proof evidence, `source.ansi` descriptor wording, and stale readiness process text. These were corrected.

## De-slop fixes made

- Downgraded `source.ansi` descriptor wording from styled-cell parity to bounded text-grid ANSI evidence; SGR parsing remains a future styled-source adapter task.
- Fixed `content.wrapIndicator` to use the actual player sample request instead of a default request.
- Ensured newly claimed handled fields are actually consumed by player adapters, including content, filter, shader, style, source, sampler, and mask inputs touched by this tranche.
- Replaced transient fixture metadata tag `descriptor-tranche` with durable `adapter-evidence`.
- Renamed the player sampler adapter module from vague “extra” wording to `fnc_apply_distortion_sampler_primitives`; the module name is internal implementation vocabulary and not a schema/public value.
- Added graph I/O proof-test coverage for sequence output consumption, parallel join visibility, and conflict-policy evidence.
- Updated the active packet title and April 29 process update so they no longer contradict schema-readiness evidence.

## Fresh verification evidence

Latest local gate outputs after review fixes:

```text
validate-recipe: total=57 valid=57 invalid=0
render-recipe: total=57 rendered=57 unsupported=0 errors=0
render-frame: total=57 rendered=57 unsupported=0 errors=0
fixture-qc: totalRecipes=57 validated=57 rendered=57 unsupported=0 playerErrors=0 overallStatus=pass
primitive-field-coverage: usedInputFields=361 handledInputFields=361 usedButUnhandledInputFields=0 schemaDecisionNeededFields=0
primitive-adapter-gap: totalEffects=43 rendered=43 stillUnsupported=0 missingDescriptor=0
schema-readiness: canDeclareSchemaReady=true fieldCoverageBlockedRecords=0 adapterBlockedRecords=0 unknownRecords=0
rustfmt touched files: pass
cargo fmt scoped packages: pass
cargo clippy --workspace --all-targets --all-features -- -D warnings: pass
nextest: 2822 passed, 0 skipped
cargo xtask docs check: pass with 3 pre-existing warnings
```

## Remaining risks and higher-level blockers

- **Graph/player integration remains the largest blocker.** The proof crate demonstrates sequence/parallel/value behavior, but the player does not yet execute a full graph topology/value bus. Significant forward progress needs a dedicated graph-execution adapter packet.
- **Source fidelity is bounded.** `source.ansi` currently strips SGR to prove text-grid ingestion; styled ANSI cells and richer image/procedural sources need future substrate work before visual parity claims are valid.
- **Backend/compositor effects remain intentionally held back.** Shadows, subcell shapes, and richer renderer-only behavior need a backend adapter seam rather than direct UI compositor construction.
- **GUI-human-review and oracle-only buckets remain signed off, not migrated.** They are not schema blockers, but they require future owner/review or oracle-comparison packets before migration can be called visually complete.

