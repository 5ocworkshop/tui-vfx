# Phase K2.15 review and deslop report

## Process note

A clean red/green sequence was not fully preserved for the initial graph-executor edits; implementation had begun before all regression tests were added. The remaining work was corrected by adding targeted regression tests and running nextest gates before documentation and final review.

## Subagent review summaries

- **Ptolemy** mapped the player graph/scene hotspots and recommended parity checks against `tui-vfx-next` semantics.
- **Hilbert** produced the verification matrix and flagged missing diagnostic/fixture risks.
- **Chandrasekhar** inspected the legacy demo oracle and recommended an explicit sample clock, player-owned render IR, semantic surface, and backend adapter seam.
- **Schrodinger** ran the ai-de-slop pass and recommended sharper graph assertions, non-overclaiming fixture names, repaired index history, and explicit diagnostic vocabulary.
- **Goodall** ran the third-party code review and recommended `metadata.expectedVisual` on new fixtures, image fallback warnings, focused source/scene tests, and docs/API gate evidence.

## Deslop pass

Scoped cleanup performed on touched files:

- removed unused graph helper/imports;
- replaced broad graph-value style predicates with exact foreground assertions and a negative sibling-isolation assertion;
- added focused source and scene fixture tests for ANSI stripping, image fallback warnings, procedural dots-spinner evidence, and scene-local style placement;
- renamed overclaiming canonical fixtures from shader/mask/visibility language to tint/signal-binding language where that is what the player actually proves;
- added `expectedVisual` metadata to new canonical fixtures and documented it in `docs/VOCABULARY.md`;
- added a structured non-fatal image fallback warning;
- repaired the new-kernel index changelog so K2.13/K2.14 history remains visible;
- kept graph execution names readable and avoided durable phase shorthand in code symbols;
- preserved `graph.order` fallback instead of adding a second compatibility path;
- kept scene styled evidence placement local to `fnc_render_scene.rs` rather than pushing compositor concepts into the UI.

## Verification evidence

```text
cargo clippy -p tui-vfx-player -p tui-vfx-player-cli -p tui-vfx-player-ui -p tui-vfx-contract -p tui-vfx-contract-cli --all-targets --all-features -- -D warnings: pass
cargo nextest run -p tui-vfx-player --no-fail-fast: 31 passed / 0 failed
cargo nextest run --workspace --no-fail-fast: 2832 passed / 0 failed
validate-recipe recursive: 67 valid / 0 invalid
render-recipe recursive: 67 rendered / 0 unsupported / 0 errors
render-frame recursive: 67 rendered / 0 unsupported / 0 errors
fixture-qc recursive: pass, 67 rendered, 0 playerErrors
primitive-field-coverage recursive: 422 used / 422 handled / 0 unhandled
primitive-adapter-gap recursive: 43 rendered / 0 unresolved
schema-readiness recursive: canDeclareSchemaReady=true, explicitOwnerDecisionNeeded=0
migration-mapping-batch recursive: canonicalExists=50, schemaDecisionNeeded=91, descriptorDecisionNeeded=113
schema/docs gates: schema generation/check, docs check/api/api-check/api-validate, configschema audit all passed
legacy debug_recipes root: unchanged
```

## Remaining risks

- Runtime graph-value missing/kind diagnostics are not as rich as the future player IR should provide.
- Scene/layer visibility remains incomplete; current evidence is element-local pipeline/style placement and signal-backed source content.
- Source fixtures are bounded evidence, not visual parity.

