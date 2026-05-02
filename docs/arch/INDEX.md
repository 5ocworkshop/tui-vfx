<!-- <FILE>docs/arch/INDEX.md</FILE> - <DESC>Foundational architecture documents index</DESC> -->
<!-- <VERS>VERSION: 0.13.0</VERS> -->
<!-- <WCTX>Update the primitive architecture index after peer review accepted the Rust-owned v3.1 primitive catalog instance direction and reconciled the workbench as scaffolding/migration/validation companion tooling.</WCTX> -->
<!-- <CLOG>0.13.0: MINOR — mark the Rust-SSOT primitive catalog architecture and implementation plan as accepted direction, add the peer-review memo, and update the workbench entry to companion status.
0.12.0: MINOR — index the new v31-primitive-rust-ssot architecture and implementation plan drafts.</CLOG> -->

# Architecture Documents

Foundational, north-star, and cross-crate boundary documents live here so they do not get lost among implementation notes and migration reports.

## v3.1 north-star docs

- [tui-vfx-tui-vfx-compost-vertical-implementation-plan.md](tui-vfx-tui-vfx-compost-vertical-implementation-plan.md) — Formal implementation plan for the tui-vfx-compost clean-sheet pure v3.1 compositor build: stable schema, substrate-first runtime migration, then primitive slices.
- [tui-vfx-compost-agent-workflow-handoff.md](tui-vfx-compost-agent-workflow-handoff.md) — Restartable handoff for the tui-vfx-compost clean-sheet pure v3.1 compositor build, completed schema/structure checkpoints, preserved recovery worktrees, exact write-scope rules, and future slice gates.
- [CLOCKS_AND_TIMING.md](CLOCKS_AND_TIMING.md) — Architecture note separating lifecycle clocks, phase timing, native transition timing/variants, reduced-motion terminal policies, sample time, presentation cadence, semantic update cadence, and primitive motion parameters.
- [v31-schema-boundary-north-star.md](v31-schema-boundary-north-star.md) — North-star architecture for schema-owned crate boundaries, native transitions, recipe-oracle transition boundaries, data models, responsibilities, co-located primitive source trees, and validation discipline.
- [v31-native-transition-model.md](v31-native-transition-model.md) — Official rationale and canonical shape for native v3.1 transitions, tracks, lifecycle alignment, reduced-motion policy, and recipe-oracle mapping rules.
- [v31-ai-authoring-prompt-guidance.md](v31-ai-authoring-prompt-guidance.md) — Reusable prompt anchors and classification rules for AI-assisted v3.1 recipe authoring without app/design-system or legacy execution assumptions.
- [v31-primitive-rust-ssot.md](v31-primitive-rust-ssot.md) — Accepted architecture for Rust-owned v3.1 primitive catalog instance declarations in `tui-vfx-compost`, bootstrap carry-forward, domain-specific runtime traits, generated `primitive.json`, and the descriptor round-trip lock.
- [v31-primitive-rust-ssot-implementation-plan.md](v31-primitive-rust-ssot-implementation-plan.md) — Production implementation plan for the Rust-SSOT primitive port: Phase 0 substrate gates, Phase 0.5 commonality extraction, registry/codegen/bootstrap, first three domain-spanning ports, derive macro, bootstrap burndown, loader rewiring, and workbench reconciliation.
- [v31-primitive-rust-ssot-peer-review-memo.md](v31-primitive-rust-ssot-peer-review-memo.md) — Peer-review artifact and author disposition that led to the accepted Rust-owned primitive catalog direction and its amended production plan.
- [primitive-workbench-schema-driven-workflow.md](primitive-workbench-schema-driven-workflow.md) — Companion Primitive Workbench workflow for schema-constrained Rust scaffolding, migration evidence, fixture/control generation, commonality extraction, and validation gates under the accepted Rust-SSOT direction.

<!-- <FILE>docs/arch/INDEX.md</FILE> - <DESC>Foundational architecture documents index</DESC> -->
<!-- <VERS>END OF VERSION: 0.13.0</VERS> -->
