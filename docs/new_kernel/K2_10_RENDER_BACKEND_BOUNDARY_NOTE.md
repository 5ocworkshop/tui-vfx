<!-- <FILE>docs/new_kernel/K2_10_RENDER_BACKEND_BOUNDARY_NOTE.md</FILE> - <DESC>K2.10 render backend boundary note</DESC> -->
<!-- <VERS>VERSION: 0.1.1</VERS> -->
<!-- <WCTX>K2.10 corpus-wide migration mapping and backlog board.</WCTX> -->
<!-- <CLOG>0.1.1: PATCH — add required metadata footer.
0.1.0: INIT — record corpus-wide migration mapping evidence and next-packet backlog.</CLOG> -->

# K2.10 Render Backend Boundary Note

## Boundary

- v3.1 DTOs are not compositor DTOs.
- The compositor must remain stable while the clean-room contract model matures.
- A future backend adapter lowers validated/player runtime data into compositor-compatible IR.
- The GUI should select or consume a player backend. It must not construct compositor internals itself.
- Compositor-backed output is a future backend, not the current migration-mapping authority.

## Current authority chain

```text
RecipeDocument v3.1
  -> contract validation
  -> player/runtime IR and evidence reports
  -> fixture-qc / render-frame / migration-mapping-batch
  -> Ratatui player UI as human inspection surface
```

## Future backend shape

```text
RecipeDocument v3.1
  -> contract validation
  -> player/runtime IR
  -> explicit render-backend adapter
  -> compositor-compatible IR / SemanticScene / CompositionSpec
  -> Ratatui display
```

## GUI guidance

- `tui-vfx-player-ui` consumes `tui-vfx-player` evidence.
- It is useful for human inspection after fixture-qc passes.
- It should eventually display mapping and fixture-QC status.
- It must not depend on the legacy recipe runtime.
- It must not directly depend on compositor internals.

## K2.10 decision

Do not wire the compositor in K2.10. The backlog shows more descriptor, source, schema, and migration-classification work before backend rendering should become the critical path.

<!-- <FILE>docs/new_kernel/K2_10_RENDER_BACKEND_BOUNDARY_NOTE.md</FILE> - <DESC>K2.10 render backend boundary note</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.1</VERS> -->
