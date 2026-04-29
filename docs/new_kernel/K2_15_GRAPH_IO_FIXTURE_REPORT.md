# K2.15 graph I/O fixture report

## Added canonical fixtures

Added under `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/complex/`:

- `graph_io_sequence_filter_to_tint.json`
- `graph_io_parallel_merge_tint.json`
- `graph_nested_parallel_sequences.json`
- `graph_parallel_overlap_conflict_snapshot.json`

## Evidence purpose

These fixtures prove the player graph executor path rather than only `tui-vfx-next` proof semantics:

- sequence producer/consumer value flow;
- parallel branch publication visible after join;
- nested sequence/parallel traversal;
- deterministic conflict warnings for overlapping parallel writes.

They are structural player-evidence fixtures, not visual-parity claims.

## Gates

```text
validate-recipe: 67 valid / 0 invalid
render-recipe: 67 rendered / 0 unsupported / 0 errors
render-frame: 67 rendered / 0 unsupported / 0 errors
fixture-qc: pass, visualFrames=67, playerErrors=0
```

