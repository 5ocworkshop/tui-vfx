<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/110_appendix_audits.md</FILE> - <DESC>Chapter 110 — appendix: the three audit workflows (A shader catalog, B corpus curation, C structural translation sample) that produce empirical inputs the plan can't answer abstractly. Lives as a sibling document; this chapter summarizes and cross-references.</DESC> -->
<!-- <VERS>VERSION: 1.0.1</VERS> -->
<!-- <WCTX>Extracted from the monolithic plan (v0.16.0) "Appendix — audits and curation" section. Thin chapter that points at the sibling docs/design/history/tui-vfx-v3-upgrade-audit-workflow.md file; the three audits produce their own sibling appendix files when executed.</WCTX> -->
<!-- <CLOG>1.0.1: fix the debug-recipes migration-log reference to the sibling path from the chapter directory.</CLOG> -->

# 110 — Appendix — audits and curation

The V3 migration depends on three audit workflows that will produce the empirical inputs the main plan can't answer abstractly: the shape of the primitive catalog, the set of recipes that earn their place in V3, and the validation of the proposed tree structure against diverse real recipes.

- **[Audit & curation workflows](../history/tui-vfx-v3-upgrade-audit-workflow.md)** — captures three deferred workflows:
  - **Workflow A — Shader catalog decomposition:** per-named-shader evaluation against the `debug_recipes/shaders/` interactive preview. Classifies each of ~27 named shaders as trivial composition / earned name / primitive-itself. Resolves Open Questions #2 and #5.
  - **Workflow B — Recipe corpus curation:** William Morris principle applied to the 200–300 recipe corpus. *"Have nothing in your house that is not useful or you do not think is beautiful."* Classifies every recipe for port / consolidate / archive / delete. Produces the V3 port list. This is Phase 1 of the mainline migration workflow (see `50_migration_workflow.md`).
  - **Workflow C — Structural translation sample:** 6–8 representative ported recipes re-expressed in the V3 tree shape to stress-test structural diversity. Resolves Open Questions #3, #4, #6.

  **Status: all three deferred.** Workflows documented for execution in a future session. None blocking; sequencing recommendation included in the workflow doc.

  Each workflow produces its own sibling appendix file when executed:
  - `../tui-vfx-v3-upgrade-appendix-shader-catalog.md` (from Workflow A)
  - `../tui-vfx-v3-upgrade-appendix-corpus-audit.md` (from Workflow B)
  - `../tui-vfx-v3-upgrade-appendix-structural-translations.md` (from Workflow C)

The debug-recipes migration exercise (`../history/tui-vfx-v3-upgrade-debug-recipes-migration-log.md`) served as a pre-execution pressure test for all three workflows, validating the tree schema against the debug corpus's single-capability recipes and surfacing 34 schema questions (Q1–Q34 in the migration-log journal) that the full audits will need to address at corpus scale.

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/110_appendix_audits.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.1</VERS> -->
