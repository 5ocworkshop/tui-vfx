# K2.16 graph executor hardening report

## Implemented

- Added player runtime diagnostic `missingGraphValue` when a graph-value input has no published value and no fallback.
- Added runtime kind-mismatch diagnostic scaffolding for graph-value inputs consumed by known descriptor input kinds.
- Render IR now carries the graph value snapshot after graph execution.

## Verification

```text
cargo nextest run -p tui-vfx-player graph_missing_required_value_emits_structured_player_diagnostic --no-fail-fast: pass
cargo nextest run -p tui-vfx-player test_fnc_player_render_ir_carries_rows_styles_provenance_and_graph_values --no-fail-fast: pass
```

## Holdbacks

Real effect-output publication remains limited to current input re-emission semantics. Full descriptor-driven effect outputs remain future player IR/backend work.
