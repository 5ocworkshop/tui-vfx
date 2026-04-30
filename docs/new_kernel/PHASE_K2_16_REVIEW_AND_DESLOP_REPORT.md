# Phase K2.16 review and deslop report

## Subagent review summaries

- **Gauss** mapped the player/report/CLI architecture and recommended a player-owned render IR plus thin `render-ir` CLI shim.
- **Kepler** identified low-risk fixture/adaptor tranche candidates using already implemented player adapters.
- **Kuhn** mapped schema-readiness and migration-mapping report surfaces and recommended disposition-first backlog summaries without false green.
- **Dewey** mapped scene/source fidelity and recommended visibility short-circuiting, write-policy skip behavior, bounded ANSI/image/procedural support, and explicit source diagnostics.
- **Linnaeus** reviewed the touched-file diff for de-slop risks and requested fixes for rendered-scene provenance, descriptor-driven graph-value kind diagnostics, fixture naming consistency, and exact docs-gate evidence.
- **Zeno** performed the formal third-party review and requested changes for graph-value kind diagnostics, rendered-scene provenance, and style writes under `skipTransparentEmpty`.

## Deslop scope

Touched files only. Cleanup performed:

- kept CLI render logic in a thin command module;
- kept render IR construction in player-owned modules;
- extracted shared recipe render flow so `render_recipe` and `render_recipe_ir` cannot drift;
- restricted render IR provenance to the scene actually rendered by the current player path;
- made `skipTransparentEmpty` protect style writes for blank overlay cells as well as glyph writes;
- removed hard-coded input-name kind inference and added descriptor-backed graph-value kind mismatch diagnostics;
- renamed the reveal-wipe right-to-left fixture so path, id, title, and authored behavior agree;
- preserved existing `render-frame` output compatibility;
- avoided compositor imports and UI backend construction;
- added deterministic fixture metadata with `expectedVisual`;
- avoided phase shorthand in public code symbols and fixture IDs.

## Current verification evidence

```text
cargo nextest run -p tui-vfx-player graph_value_kind_mismatch_emits_structured_player_diagnostic graph_missing_required_value_emits_structured_player_diagnostic test_fnc_player_render_ir_carries_rows_styles_provenance_and_graph_values --no-fail-fast: 3 passed
cargo nextest run -p tui-vfx-player --no-fail-fast: pass
cargo nextest run -p tui-vfx-player-cli --no-fail-fast: pass
cargo nextest run -p tui-vfx-contract-cli --no-fail-fast: pass
cargo nextest run -p tui-vfx-player -p tui-vfx-player-cli -p tui-vfx-contract -p tui-vfx-contract-cli -p tui-vfx-player-ui --no-fail-fast: 260 passed
cargo nextest run --workspace --no-fail-fast: 2836 passed
cargo clippy -p tui-vfx-player -p tui-vfx-player-cli -p tui-vfx-player-ui -p tui-vfx-contract -p tui-vfx-contract-cli --all-targets --all-features -- -D warnings: pass
cargo xtask docs check/api/api-check/api-validate and audit configschema: pass, with pre-existing docs-check warnings noted in K2_16_SCHEMA_API_DOCS_GATE.md
validate-recipe recursive: 88 valid / 0 invalid
render-recipe recursive: 88 rendered / 0 unsupported / 0 errors
render-frame recursive: 88 rendered / 0 unsupported / 0 errors
fixture-qc recursive: pass, 88 rendered, 0 playerErrors
primitive-field-coverage recursive: 541 used / 541 handled / 0 unhandled
primitive-adapter-gap recursive: 45 rendered / 0 unresolved
```

## Remaining risks

- Full scene visibility predicates are not implemented in the recipe scene runtime path; this is an explicit K2.16 acceptance deviation because the current recipe DTO/player path has no implemented visibility evaluator yet.
- ANSI styled-cell fidelity remains bounded; image support remains fallback/resolver-boundary only.
- Content/source backlog families still require descriptor/adapter design.
- Backend/compositor adapter is a preflight boundary, not an implemented backend.
