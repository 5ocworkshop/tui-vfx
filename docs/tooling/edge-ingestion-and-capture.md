<!-- <FILE>docs/tooling/edge-ingestion-and-capture.md</FILE> - <DESC>V3 edge ingestion and offline capture tooling contract.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Document the Chapter 63 edge-ingestion contract for ANSI input, command-output capture, and adapter-owned runtime boundaries.</WCTX> -->
<!-- <CLOG>0.1.0: initial edge-ingestion guide for ANSI/captured command output and runtime boundary rules.</CLOG> -->

# Edge ingestion and capture tooling

Edge ingestion turns external terminal-like content into recipe/grid/source data.
It does not move terminal ownership into the compositor.

## Supported direction

| Source | Intended handling |
|---|---|
| ANSI-styled text | Parse into grid/source data at authoring/tool time or through a source adapter. |
| Command output | Capture offline into a fixture/source artifact. Runtime recipe playback must not spawn commands. |
| Files/assets | Load through explicit source specs or authoring tools. Keep effect semantics asset-agnostic. |
| Host app widgets | Host renders or snapshots its own content, then tui-vfx effects consume grid/source data. |

## Runtime boundary

Recipe execution must be deterministic from its loaded inputs, runtime bindings,
clock, and target grid. A recipe should not shell out during playback. If a user
wants command output in a recipe, capture it first and reference the captured
artifact.

## Debug recipe expectation

Each ingestion mode should have a small debug recipe that demonstrates:

1. a source/captured input,
2. one downstream shader/filter/mask/style step that consumes it,
3. a validator/probe command proving non-empty output and stage activity.

The basic recipe should be visually plain. More elaborate chains can be added
after the primitive source path is easy to inspect.

<!-- <FILE>docs/tooling/edge-ingestion-and-capture.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
