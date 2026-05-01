<!-- <FILE>docs/arch/INDEX.md</FILE> - <DESC>Foundational architecture documents index</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>v3.1 north-star architecture: gather schema-boundary and primitive-workbench docs under a stable architecture hub.</WCTX> -->
<!-- <CLOG>0.3.0: MINOR — add clocks and timing architecture note.
0.2.0: MINOR — add compositor-next vertical implementation plan.
0.1.0: INIT — add architecture hub for foundational v3.1 schema and primitive workflow docs.</CLOG> -->

# Architecture Documents

Foundational, north-star, and cross-crate boundary documents live here so they do not get lost among implementation notes and migration reports.

## v3.1 north-star docs

- [compositor-next-vertical-implementation-plan.md](compositor-next-vertical-implementation-plan.md) — Detailed plan for copying the hardened compositor into compositor-next, then migrating primitive-by-primitive through vertical schema/runtime/validation slices.
- [CLOCKS_AND_TIMING.md](CLOCKS_AND_TIMING.md) — Architecture note separating lifecycle clocks, phase timing, sample time, presentation cadence, semantic update cadence, and primitive motion parameters.
- [v31-schema-boundary-north-star.md](v31-schema-boundary-north-star.md) — North-star architecture for schema-owned crate boundaries, data models, responsibilities, co-located primitive source trees, and validation discipline.
- [primitive-workbench-schema-driven-workflow.md](primitive-workbench-schema-driven-workflow.md) — Draft Primitive Workbench workflow for schema-driven primitive scaffolding, generated assets, migration mappings, commonality extraction, and validation gates.

<!-- <FILE>docs/arch/INDEX.md</FILE> - <DESC>Foundational architecture documents index</DESC> -->
<!-- <VERS>END OF VERSION: 0.3.0</VERS> -->
