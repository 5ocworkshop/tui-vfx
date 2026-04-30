<!-- <FILE>docs/new_kernel/K2_19_SCHEMA_API_DOCS_GATE.md</FILE> - <DESC>K2.19 schema API docs gate</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.19 visible playback: compositor backend and studio-control pilot evidence.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture K2.19 results, commands, artifacts, limits, and verification evidence.</CLOG> -->

# K2.19 schema/API/docs gate

## Schema impact

- No schema version bump. v3.1 is pre-release and this packet did not lock or increment the canonical schema version.
- No v3.1 recipe DTO mutation was required. The new backend lowers from existing `PlayerRenderIrReport` and existing descriptor/control data.

## API impact

- New internal workspace crate: `tui-vfx-player-backend-compositor`.
- Extended `PlayerRenderBackendOutput` with backend evidence fields needed by machine demos.
- New CLI commands: `render-backend`, `render-backend-timeline`, `studio-snapshot`, and `play-backend`.
- UI CLI adds `--backend`; interactive mode advances time in a 16 ms frame loop and renders backend styled cells into ratatui spans.

## Docs/rustdoc/OFPF impact

- New crate/files include OFPF file headers and rustdoc for exported types/functions.
- Impacted vocabulary/checklist/index docs were updated.
- No unrelated docs were touched just for churn.

## Verification evidence

- `./scripts/k219_visual_demo.sh` PASS table generated `/tmp/k219-visual-results/`.
- Targeted nextest for new CLI/UI paths includes bounded `play-backend` JSON/ANSI playback and styled ratatui preview coverage.
- Full workspace `cargo nextest run --workspace --no-fail-fast`: PASS, 2872/2872.
- Targeted clippy gate: PASS with `-D warnings`.
- Full verification is recorded in `PHASE_K2_19_REVIEW_AND_DESLOP_REPORT.md`.
