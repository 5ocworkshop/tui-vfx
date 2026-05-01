<!-- <FILE>docs/arch/INDEX.md</FILE> - <DESC>Foundational architecture documents index</DESC> -->
<!-- <VERS>VERSION: 0.5.0</VERS> -->
<!-- <WCTX>Compositor-next execution: add restartable warm-agent vertical-slice workflow handoff.</WCTX> -->
<!-- <CLOG>0.5.0: MINOR — add compositor-next agent workflow handoff for restartable two-agent slice execution.
0.4.0: MINOR — add compositor-next current-state fence.
0.3.0: MINOR — add clocks and timing architecture note.
0.2.0: MINOR — add compositor-next vertical implementation plan.
0.1.0: INIT — add architecture hub for foundational v3.1 schema and primitive workflow docs.</CLOG> -->

# Architecture Documents

Foundational, north-star, and cross-crate boundary documents live here so they do not get lost among implementation notes and migration reports.

## v3.1 north-star docs

- [compositor-next-vertical-implementation-plan.md](compositor-next-vertical-implementation-plan.md) — Detailed plan for copying the hardened compositor into compositor-next, then migrating primitive-by-primitive through vertical schema/runtime/validation slices.
- [compositor-next-agent-workflow-handoff.md](compositor-next-agent-workflow-handoff.md) — Restartable execution handoff for warmed low-level coding agents, lead review/integration responsibilities, active worktrees, scoreboard, and per-slice gates.
- [compositor-next-current-state-fence.md](compositor-next-current-state-fence.md) — Phase 0 current-state fence separating pre-existing dirty/exploratory work from compositor-next-owned vertical implementation.
- [CLOCKS_AND_TIMING.md](CLOCKS_AND_TIMING.md) — Architecture note separating lifecycle clocks, phase timing, sample time, presentation cadence, semantic update cadence, and primitive motion parameters.
- [v31-schema-boundary-north-star.md](v31-schema-boundary-north-star.md) — North-star architecture for schema-owned crate boundaries, data models, responsibilities, co-located primitive source trees, and validation discipline.
- [primitive-workbench-schema-driven-workflow.md](primitive-workbench-schema-driven-workflow.md) — Draft Primitive Workbench workflow for schema-driven primitive scaffolding, generated assets, migration mappings, commonality extraction, and validation gates.

<!-- <FILE>docs/arch/INDEX.md</FILE> - <DESC>Foundational architecture documents index</DESC> -->
<!-- <VERS>END OF VERSION: 0.5.0</VERS> -->
