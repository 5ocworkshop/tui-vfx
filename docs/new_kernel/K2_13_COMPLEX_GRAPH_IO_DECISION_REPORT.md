<!-- <FILE>docs/new_kernel/K2_13_COMPLEX_GRAPH_IO_DECISION_REPORT.md</FILE> - <DESC>K2.13 complex graph I/O decision report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.13 schema decision burn-down: settle complex graph I/O and sequence/parallel dispositions.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document accepted graph I/O runtime IR semantics and holdbacks.</CLOG> -->

# K2.13 Complex Graph I/O Decision Report

## Decision

Graph I/O and sequence/parallel composition are schema-level runtime IR.

Accepted semantics:

```text
Sequence:
  child N+1 sees mutations and graph values emitted by child N.

Parallel:
  branches read the same input snapshot;
  branch surfaces merge at the join;
  graph values merge at the join;
  conflicts use explicit policy or authored-order default with diagnostics.
```

Accepted I/O shape:

```text
io.outputs:
  hint
  kind
  source

io.inputs:
  input
  hint
  kind
```

## Disposition

Complex records are not generic owner-audit blockers after K2.13. They classify into:

- `acceptedSchema` where graph/scene semantics are representable.
- `descriptorBacklog` where descriptor vocabulary is the missing work.
- `backendHoldback` for shadow/subcell/backend renderer work.
- `guiHumanReviewHoldback` for authored visual-conflict evidence.
- `oracleOnly` for offline/capture evidence.

## Remaining backlog

The next forward-progress blockers are implementation lanes, not schema indecision: descriptor coverage, fixture migration, graph executor evidence, backend/compositor packet, and visual-conflict signoff.

<!-- <FILE>docs/new_kernel/K2_13_COMPLEX_GRAPH_IO_DECISION_REPORT.md</FILE> - <DESC>K2.13 complex graph I/O decision report</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
