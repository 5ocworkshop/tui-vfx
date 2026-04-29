# Phase K2.15 graph/descriptor migration status memo to architect

## Verdict

K2.15 materially moved the player from `graph.order`-only evidence toward real topology/value-bus execution, and added 10 canonical v3.1 debug fixtures while preserving the zero-gap fixture gates.

## Final counters

```text
canonical v3.1 fixtures: 57 -> 67
validate-recipe: 67 valid / 0 invalid
render-recipe: 67 rendered / 0 unsupported / 0 errors
render-frame: 67 rendered / 0 unsupported / 0 errors
fixture-qc: pass, totalRecipes=67, playerErrors=0
primitive-field-coverage: 422 used / 422 handled / 0 unhandled
primitive-adapter-gap: 43 rendered / 0 unresolved
schema-readiness: canDeclareSchemaReady=true, explicitOwnerDecisionNeeded=0
migration-mapping: canonicalExists=50, schemaDecisionNeeded=91, descriptorDecisionNeeded=113
```

## Player graph execution

Implemented:

- topology-first execution with `graph.order` fallback;
- sequence graph-value visibility;
- parallel branch input snapshots;
- branch value/surface merge at join;
- deterministic warnings for last-writer graph-value and surface conflicts;
- input re-emission node outputs.

Not complete:

- real effect-output publication;
- full runtime missing-value/kind diagnostics beyond contract validation and authored fallbacks;
- backend/compositor lowering.

## Demo oracle reading

`/usr/projects/tui-vfx-recipes/examples/demo.rs` was used only as an oracle. The relevant lesson is not its UI structure; it is the clean separation between sampled runtime state, compiled/rendered snapshot, grid-first rendering, and ratatui as an adapter. K2.15 keeps our boundary as:

```text
RecipeDocument v3.1 -> contract validation -> player/runtime evidence -> future backend IR/render adapter -> UI
```

The UI still does not construct compositor internals.

## Higher-level blockers blocking faster progress

1. We need a **player-owned render IR** that can carry rows, styled cells, roles, channel-write metadata, graph diagnostics, and sample-clock state before backend lowering.
2. We need a **backend/compositor adapter seam** so the player can lower v3.1 data into compositor-compatible IR without importing the legacy recipe runtime or pushing compositor construction into the UI.
3. We need a **complete scene/layer runtime model**: visibility predicates, transparent/clear behavior, style/base surface handling, and element-attributed diagnostics.
4. We need a **source fidelity strategy**: ANSI styled-cell extraction, real image rasterization or a formal asset-renderer boundary, and bounded procedural generators.
5. We need a **descriptor backlog burn-down lane** for the 113 remaining descriptor decisions; these are no longer broad schema questions, so they should be processed as implementation/evidence tasks.
6. We need a **visual/owner audit throughput plan** for 280 owner-audit records. Repeatedly reporting that they exist is no longer useful; forward progress requires batching and explicit disposition criteria.

## Recommended next packet

Do not reopen schema-readiness. Start the player IR/backend adapter packet and split scene/layer fidelity plus descriptor backlog into separate bounded lanes.

