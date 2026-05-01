<!-- <FILE>docs/arch/INDEX.md</FILE> - <DESC>Foundational architecture documents index</DESC> -->
<!-- <VERS>VERSION: 0.11.0</VERS> -->
<!-- <WCTX>tui-vfx-compost docs: current-state fence removed because it must be regenerated at phase start.</WCTX> -->
<!-- <CLOG>0.11.0: PATCH — remove the compost current-state fence from the architecture index; it will be regenerated when implementation starts.</CLOG> -->

# Architecture Documents

Foundational, north-star, and cross-crate boundary documents live here so they do not get lost among implementation notes and migration reports.

## v3.1 north-star docs

- [tui-vfx-tui-vfx-compost-vertical-implementation-plan.md](tui-vfx-tui-vfx-compost-vertical-implementation-plan.md) — Formal implementation plan for the tui-vfx-compost clean-sheet pure v3.1 compositor build: stable schema, substrate-first runtime migration, then primitive slices.
- [tui-vfx-compost-agent-workflow-handoff.md](tui-vfx-compost-agent-workflow-handoff.md) — Restartable handoff for the tui-vfx-compost clean-sheet pure v3.1 compositor build, completed schema/structure checkpoints, preserved recovery worktrees, exact write-scope rules, and future slice gates.
- [CLOCKS_AND_TIMING.md](CLOCKS_AND_TIMING.md) — Architecture note separating lifecycle clocks, phase timing, native transition timing/variants, reduced-motion terminal policies, sample time, presentation cadence, semantic update cadence, and primitive motion parameters.
- [v31-schema-boundary-north-star.md](v31-schema-boundary-north-star.md) — North-star architecture for schema-owned crate boundaries, native transitions, recipe-oracle transition boundaries, data models, responsibilities, co-located primitive source trees, and validation discipline.
- [v31-native-transition-model.md](v31-native-transition-model.md) — Official rationale and canonical shape for native v3.1 transitions, tracks, lifecycle alignment, reduced-motion policy, and recipe-oracle mapping rules.
- [v31-ai-authoring-prompt-guidance.md](v31-ai-authoring-prompt-guidance.md) — Reusable prompt anchors and classification rules for AI-assisted v3.1 recipe authoring without app/design-system or legacy execution assumptions.
- [primitive-workbench-schema-driven-workflow.md](primitive-workbench-schema-driven-workflow.md) — Draft Primitive Workbench workflow for schema-driven primitive scaffolding, generated assets, migration mappings, commonality extraction, and validation gates.

<!-- <FILE>docs/arch/INDEX.md</FILE> - <DESC>Foundational architecture documents index</DESC> -->
<!-- <VERS>END OF VERSION: 0.11.0</VERS> -->
